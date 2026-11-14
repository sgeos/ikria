# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Push authority, 2026-11-14

**Done.**
- Recorded in `CLAUDE.md` and `AGENTS.md` that the operator extended the standing
  authority to pushing, for code and documentation alike, in this repository and
  `sgeos/ikria-bridge`.
- In the bridge, adopted the operator's no-outside-contributions stance, with forks and
  fork reports encouraged instead.

**Decided.** By the operator on 2026-11-14.

**Verified.** `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.** Nothing new.

**For the operator.** Nothing. The next work is the remaining Phase 0 experiments.
