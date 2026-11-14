# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Campaign plan final, 2026-11-14

**Done.**
- Recorded the operator's last two rulings in
  [`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md), which
  omit the training lessons and grant a standing authority to push. Marked the plan
  final.
- Recorded the standing authority in `CLAUDE.md` and `AGENTS.md`. It covers pushing
  routine documentation and record pull requests, and nothing else.
- Recorded in R11 that incident reports are to follow space agency and aerospace
  industry standards, and asked the blog line to research and publish them.

**Decided.** By the operator on 2026-11-14, as the plan's third rulings table records.

**Verified.**
- The message to the blog line's running session was accepted into its inbox. Whether
  it was read, and whether the pilot confirmed it there, is not known.
- `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.** Facts the plan marks as recalled remain to be checked in Phase 0.

**For the operator.** Nothing is blocking. The pull request for the plan is to be
merged by the operator, since the standing authority does not cover merging.
