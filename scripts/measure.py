#!/usr/bin/env python3
"""Time commands and their peak memory, and compare them (docs/ARCHITECTURE.md §9).

    scripts/measure.py -n 30 'base=/tmp/base/binary-crate world' 'new=target/release/binary-crate world'

Each LABEL=COMMAND runs N times (after warm-up runs), interleaved so that machine noise
hits every command alike. Prints a Markdown table: median and minimum wall time, median
peak RSS, and the change against the first command, ready for an AUDIT.md §3 row.

Unix only. Commands run without a shell; their stdout goes to /dev/null, and a run that
exits non-zero stops the measurement.

Each run is started by a small C helper (compiled with `cc` into target/ on first use),
not by Python itself: Linux carries a process's pre-exec memory into its peak RSS, so a
command forked straight from Python could never report less than Python's own ~10 MB.
The helper's floor is about 1.6 MB (an empty C program), and the time it reports excludes Python's overhead.
"""

import argparse
import os
import shlex
import statistics
import subprocess
import sys
from pathlib import Path

# Forks, execs argv[1..], waits; writes "<nanoseconds> <ru_maxrss>" to fd 3 and exits
# with the command's status (127 if it couldn't start, 128+N if killed by signal N).
HELPER_C = r"""
#include <stdio.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc < 2) return 2;
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    pid_t pid = fork();
    if (pid < 0) return 126;
    if (pid == 0) {
        close(3);
        execvp(argv[1], argv + 1);
        perror(argv[1]);
        _exit(127);
    }
    int status;
    struct rusage usage;
    if (wait4(pid, &status, 0, &usage) < 0) return 126;
    clock_gettime(CLOCK_MONOTONIC, &end);
    long long ns = (end.tv_sec - start.tv_sec) * 1000000000LL + (end.tv_nsec - start.tv_nsec);
    dprintf(3, "%lld %ld\n", ns, (long)usage.ru_maxrss);
    if (WIFSIGNALED(status)) return 128 + WTERMSIG(status);
    return WEXITSTATUS(status);
}
"""


def helper_path():
    """Builds the helper into the workspace's target dir if it isn't there yet."""
    root = Path(__file__).resolve().parent.parent
    target = Path(os.environ.get("CARGO_TARGET_DIR", root / "target"))
    helper = target / "measure-helper" / "spawn"
    source = helper.with_suffix(".c")
    if not helper.exists() or not source.exists() or source.read_text() != HELPER_C:
        helper.parent.mkdir(parents=True, exist_ok=True)
        source.write_text(HELPER_C)
        cc = os.environ.get("CC", "cc")
        try:
            subprocess.run([cc, "-O2", "-o", str(helper), str(source)], check=True)
        except (OSError, subprocess.CalledProcessError) as err:
            sys.exit(f"cannot build the measurement helper with {cc}: {err}")
    return str(helper)


def run_once(helper, argv, cwd, env):
    """Runs argv once through the helper; returns (seconds, peak RSS in bytes)."""
    read_end, write_end = os.pipe()
    pid = os.fork()
    if pid == 0:
        try:
            os.close(read_end)
            os.dup2(write_end, 3)
            devnull = os.open(os.devnull, os.O_WRONLY)
            os.dup2(devnull, 1)
            if cwd:
                os.chdir(cwd)
            os.execve(helper, [helper, *argv], env)
        except OSError as err:
            print(f"cannot run the helper: {err}", file=sys.stderr)
        os._exit(126)
    os.close(write_end)
    with os.fdopen(read_end) as pipe:
        report = pipe.read().split()
    _, status = os.waitpid(pid, 0)
    code = os.waitstatus_to_exitcode(status)
    if code != 0 or len(report) != 2:
        sys.exit(f"{shlex.join(argv)} exited with {code}")
    nanoseconds, maxrss = map(int, report)
    # ru_maxrss is in KiB on Linux and bytes on macOS.
    rss = maxrss if sys.platform == "darwin" else maxrss * 1024
    return nanoseconds / 1e9, rss


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

    helper = helper_path()
    for _ in range(args.warmup):
        for _, argv in commands:
            run_once(helper, argv, args.cwd, env)

    samples = {label: [] for label, _ in commands}
    for _ in range(args.runs):
        for label, argv in commands:
            samples[label].append(run_once(helper, argv, args.cwd, env))

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
