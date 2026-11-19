# Planning

How to turn a request into `docs/PLAN.md`: a plan an agent can execute step by step, and
that another agent (or the same one, after its context is gone) can pick up halfway.
Start from [`templates/PLAN.md`](../templates/PLAN.md).

## What a good plan has

1. **Goal**: two or three sentences: who uses it, for what, and what "done" looks like.
2. **Interface**: everything a user sees, decided before code: commands and flags, config
   keys with defaults, output formats, exit codes, files it reads and writes. It becomes the
   README.
3. **Architecture mapping**: which modules go in the library and which in the binary,
   which Cargo features exist (one per optional capability), and the dependencies, each
   with a reason and checked against ARCHITECTURE.md §10.
4. **Risks and unknowns**: anything that could change the design (an API you haven't used,
   a format you don't know, a performance requirement). Each gets a spike step early: a
   small experiment whose result decides the design, recorded in `DECISIONS.md`.
5. **Milestones**: each one leaves a working program a user could try. The first is a
   walking skeleton: the thinnest end-to-end path (input → library → output), with real
   behaviour replacing the placeholder.
6. **Steps**: the units of work; one commit each.
7. **Verification**: how each milestone is checked by hand (phase 5 of the workflow), and
   which scenarios get measured, with what command.
8. **Progress**: a log kept up to date as steps land.

## Steps

A good step:

- Is one commit that leaves the gates green, small enough to review in one sitting
  (as a rule of thumb, under ~400 changed lines).
- Says what changes for the user or the code, which files or modules, and which tests
  prove it ("`config/`: `--output` flag; test: flag overrides file").
- Has a "done when" that can be checked: a test passes, a command prints X.
- Depends only on earlier steps.

Order: the walking skeleton first; then features, most important first; then hardening
(error paths, edge cases, robustness checklist of ARCHITECTURE.md §8); then the
documentation pass and the audit. Put risky work early, so a surprise changes the plan
while it's cheap.

## Keeping the plan alive

- Tick a step in Progress when its commit lands: `- [x] 2.3 config file (a1b2c3d)`, with
  one line on anything the next steps need to know.
- When reality disagrees with the plan, edit the plan first, and note the change and why
  in the log. Don't silently drift.
- A new feature request in an existing project adds a milestone to the same plan.
- When everything is done, the plan stays as the record of how the project was built.

## Sizing the questions

Planning surfaces decisions. Those that change what the user gets (scope, interface,
platforms, external effects) go to the user, in one batch (WORKFLOW.md phase 2). The rest
are engineering choices: make them, write them in `DECISIONS.md`, and list the notable
ones in the summary so the user can overrule them.
