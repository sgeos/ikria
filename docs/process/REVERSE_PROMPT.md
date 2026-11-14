# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Campaign plan, second rulings, 2026-11-14

**Done.**
- Recorded the operator's second set of rulings in
  [`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md). R9 and R10
  are decided, everything from Kerbal Space Program 2 is a supplemental best effort,
  and Mission mode and the stand-alone scenarios repeat one-shot attempts until one
  completes without an incident.
- Added R11, which requires an incident report for every career reload and every loss
  of a kerbal, and R12, which keeps the operator's attention to three kinds of request.
- Named the saves, and answered the operator's question on mods in R9's enforcement
  note. The recommendation is to enforce the limits in the harness, adopt no mod that
  changes play, and write a read-only kRPC service only for state that kRPC lacks.

**Decided.** By the operator on 2026-11-14, as the plan's second rulings table records.

**Verified.** The game ships 16 stand-alone scenarios and 14 training lessons, counted
from its saves folder. `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.** Which state kRPC does not expose is recalled, not checked. Phase 0
settles it.

**For the operator.** O7, whether the training lessons are in scope, and O8, whether
to grant a standing authority to push routine pull requests. The plan is held unpushed
until the operator calls it final.
