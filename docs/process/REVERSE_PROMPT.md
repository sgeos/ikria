# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Seeding report, 2026-11-13

From the admin session, which seeded the repository. This line has not yet run.

**Done.**
- Seeded from `sgeos/project-template` v1.0.8 at `3954eba`, the template's latest commit
  on that day, with the placeholders filled and the Keleusma family constraint added.
- Recorded the operator brief and the kRPC findings in `docs/process/HANDOFF.md`, the
  operator's rulings in `docs/decisions/PROJECT_CHARTER.md`, and the remaining questions
  in `docs/architecture/OPEN_QUESTIONS.md`.
- Created `sgeos/ikria`, applied template step 10 in full, and made it public after the
  operator's disclosure scan of the history came back clean. The default-branch ruleset
  is active, so changes reach `main` only by pull request with green CI.

**Verified.** The gate passes, CI passes on `main`, and the disclosure scan found no
listed term in the tracked files or the history. A read-only kRPC client connected to the
running game on 2026-11-13.

**Not verified.** No line has compared the repository with Appendix B beyond the terms
scan, because the seeding session may not read it. Nothing is implemented, and no Rust
kRPC client has been chosen.

**For the operator.** The seven open questions, and the design document that follows
them.
