# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Community practice, 2026-11-14

**Done.**
- Ran a short research pass on how experienced players approach career on Hard, at the
  operator's request, and added its principles as an appendix of
  [`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md).
- Revised the technology order. The question of enough science now rests on the
  official wiki's total, and the first landing goes to Minmus after a flyby of the Mun.
- Added two kRPC questions to the front of Phase 0 in the handoff.

**Decided.** Nothing new. Both appendices are proposed.

**Verified.** `./scripts/check.sh` exits zero on this branch, on macOS. The research
read the official wiki through its public interface and Steam community pages. It did
not read Reddit or the official forum, which refused automated reading.

**Not verified.**
- Much community advice predates 1.12, and the wiki's total of science does not state
  its version.
- The wiki's facility costs were not cross-checked.
- Whether kRPC can take samples, run reports and plant flags with a kerbal outside the
  vessel is the largest open risk.

**For the operator.**
- Principle 6 of the new appendix reads R2 and R9 as allowing a Keleusma loop to plan
  its own transfers without the facility upgrades that manoeuvre nodes need. That could
  defer a large expense. The operator may overrule it.
- Otherwise nothing is blocking, and pull request 5 can be merged as it stands.
