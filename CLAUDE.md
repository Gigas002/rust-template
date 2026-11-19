@AGENTS.md

## Claude Code

The workflows in `docs/agents/` are also available as skills in `.claude/skills/`:
`new-project`, `feature`, `fix`, `audit` and `release`. They load the right documents for
the task; the documents stay the single source of truth for every agent.

Put scratch files (probe fixtures, measurement data, base builds) in your scratchpad
directory, never in the repository.
