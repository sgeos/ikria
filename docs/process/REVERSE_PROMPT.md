# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such. Written
by a Claude Code line.

## Readiness for Codex and Grok Build, 2026-11-16

**Done.**
- **Session start.** `AGENTS.md` now says to read `CLAUDE.md` at the start of every
  session. Codex loads only `AGENTS.md`. Grok Build loads both once the folder is
  trusted, according to its local user guide.
- **Worktrees.** `scripts/worktree.sh new` copies the main checkout's local material
  into a new worktree. `scripts/check.sh` falls back to the main checkout's terms list
  in a worktree that lacks one, and names the list by a relative label, so that its
  output carries no home directory.
- **Shared state files.** Lines rewrite the state files on top of the latest `main`, keep
  the other lines' open items, and name the assistant that wrote the report.
- **Gate discipline.** Stage before the gate, and commit only on its exit status.
- **The blog line.** Only a Claude Code line can message it. Others ask the operator.
- **Standing authority.** It stays with Claude Code lines until the operator says
  otherwise.

**Decided.** Nothing by the operator. The line held the standing authority to Claude
Code lines by default.

**Verified.** In a throwaway worktree, on macOS.
- **The copy.** `worktree.sh new` produced an identical copy with private modes.
- **The fallback.** With the copy removed, the gate scanned with the main checkout's
  list.
- **A negative test.** A planted term made the gate fail, naming only the file and line.
- **Cleanup.** `worktree.sh rm` removed the tree and its copy.

The gate exits zero on this branch with every file staged.

**Not verified.** No Codex or Grok Build session was run against the repository.

**For the operator.**
1. Whether the standing authority to push and merge extends to Codex and Grok Build
   lines.
2. The two script changes diverge from `sgeos/project-template`, and a later template
   sync would undo them unless they are upstreamed.
