#!/usr/bin/env python3
"""Time commands and their peak memory, and compare them (docs/ARCHITECTURE.md §9).

    scripts/measure.py -n 30 'base=/tmp/base/skel --no-config' 'new=target/release/skel --no-config'

Each LABEL=COMMAND runs N times (after warm-up runs), interleaved so that machine noise
hits every command alike. Prints a Markdown table: median and minimum wall time, median
peak RSS, and the change against the first command, ready for an AUDIT.md §3 row.

Unix only (it reads peak RSS from wait4). Commands run without a shell; their stdout
goes to /dev/null, and a run that exits non-zero stops the measurement.
"""

import argparse
import os
import shlex
import statistics
import sys
import time


def run_once(argv, cwd, env):
    """Runs argv once; returns (seconds, peak RSS in bytes)."""
    start = time.perf_counter()
    pid = os.fork()
    if pid == 0:
        try:
            devnull = os.open(os.devnull, os.O_WRONLY)
            os.dup2(devnull, 1)
            if cwd:
                os.chdir(cwd)
            os.execvpe(argv[0], argv, env)
        except OSError as err:
            print(f"cannot run {argv[0]}: {err}", file=sys.stderr)
        os._exit(127)
    _, status, usage = os.wait4(pid, 0)
    elapsed = time.perf_counter() - start
    code = os.waitstatus_to_exitcode(status)
    if code != 0:
        sys.exit(f"{shlex.join(argv)} exited with {code}")
    # ru_maxrss is in KiB on Linux and bytes on macOS.
    rss = usage.ru_maxrss if sys.platform == "darwin" else usage.ru_maxrss * 1024
    return elapsed, rss


def fmt_time(seconds):
    return f"{seconds * 1000:.2f} ms" if seconds < 1 else f"{seconds:.3f} s"


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("commands", nargs="+", metavar="LABEL=COMMAND")
    parser.add_argument("-n", "--runs", type=int, default=20, help="measured runs per command (20)")
    parser.add_argument("--warmup", type=int, default=2, help="unmeasured runs first (2)")
    parser.add_argument("--cwd", help="directory to run the commands in")
    parser.add_argument("--env", action="append", default=[], metavar="KEY=VALUE",
                        help="set a variable for the commands, e.g. --env HOME=/tmp/scratch")
    args = parser.parse_args()

    env = dict(os.environ)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value

    commands = []
    for item in args.commands:
        label, sep, command = item.partition("=")
        if not sep or not command.strip():
            parser.error(f"expected LABEL=COMMAND, got {item!r}")
        commands.append((label, shlex.split(command)))

    for _ in range(args.warmup):
        for _, argv in commands:
            run_once(argv, args.cwd, env)

    samples = {label: [] for label, _ in commands}
    for _ in range(args.runs):
        for label, argv in commands:
            samples[label].append(run_once(argv, args.cwd, env))

    print(f"| Command | Median | Min | Peak RSS (median) | vs {commands[0][0]} |")
    print("| ------- | ------ | --- | ----------------- | ------- |")
    base = None
    for label, _ in commands:
        times = [t for t, _ in samples[label]]
        rss = statistics.median(r for _, r in samples[label])
        median = statistics.median(times)
        if base is None:
            base, change = median, "—"
        else:
            change = f"{(median - base) / base * 100:+.1f} %"
        print(f"| {label} | {fmt_time(median)} | {fmt_time(min(times))} | "
              f"{rss / 1024 / 1024:.1f} MB | {change} |")
    print(f"\n{args.runs} runs each after {args.warmup} warm-up, interleaved.")


if __name__ == "__main__":
    main()
