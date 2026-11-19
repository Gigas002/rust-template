<!--
Template for docs/PLAN.md: copy it there and fill it in (docs/agents/PLANNING.md).
Replace every <...>, delete what doesn't apply, and delete this comment.
-->

# Plan: <name>

## Goal

<Who uses it, for what, and what "done" means, in two or three sentences.>

## Decisions

Asked: <the questions put to the user and their answers, or "none: the request was clear">.

Chosen (details in [DECISIONS.md](DECISIONS.md)): <the notable engineering choices, one line
each, so the user can overrule them>.

## Interface

**Commands**

```text
<name> [OPTIONS] <ARGS>        <what it does>
```

**Config** (`~/.config/<name>/config.toml`, all optional; `examples/config.toml`)

| Key | Default | Meaning | Flag |
| --- | ------- | ------- | ---- |
| `<key>` | `<value>` | <meaning> | `--<flag>` |

**Output**: <what goes to stdout, in which format>. **Exit codes**: 0 success, 1 failure,
2 bad usage or config<, others>.

**Files**: <what it reads and writes, where>.

## Architecture

| Crate | Module | Responsibility |
| ----- | ------ | -------------- |
| lib<name> | `<module>/` | <...> |
| <name> | `app/` | <...> |

**Features**: `default = [<...>]`; `<feature>`: <capability, optional dependencies>.

**Dependencies** (ARCHITECTURE.md §10: current, maintained, justified in DECISIONS.md):

| Crate | Version | For | Instead of |
| ----- | ------- | --- | ---------- |
| `<crate>` | `<x.y.z>` | <...> | <alternatives considered> |

## Risks and unknowns

| Risk | Spike step | Decides |
| ---- | ---------- | ------- |
| <...> | <1.1> | <...> |

## Milestones and steps

Each step is one commit that leaves `scripts/gates.sh` green.

### 1. Walking skeleton: <the thinnest end-to-end path>

- [ ] 1.1 <step>: <files/modules>; test: <what proves it>; done when <check>.
- [ ] 1.2 ...

### 2. <milestone>

- [ ] 2.1 ...

### N. Hardening, docs, audit

- [ ] N.1 Error paths and ARCHITECTURE.md §8 checklist
- [ ] N.2 README and examples match the program
- [ ] N.3 Audit (docs/agents/AUDIT.md) → docs/AUDIT.md
- [ ] N.4 Release prep (docs/agents/RELEASE.md), no tag

## Verification

| Milestone | Checked by hand | Measured (scripts/measure.py) |
| --------- | --------------- | ----------------------------- |
| 1 | <commands run in a scratch HOME, expected output> | <scenario, or "—"> |

## Progress

<!-- One line per landed step: `- [x] 1.1 <step> (<commit>): <note for later steps>`.
Also note plan changes here, with the reason. Newest last. -->
