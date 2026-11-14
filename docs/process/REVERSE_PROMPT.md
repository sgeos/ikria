# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Campaign plan, 2026-11-14

**Done.**
- Wrote [`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md) from
  the operator's brief of 2026-11-14. It holds 18 completion criteria, 8 rules of play,
  8 constraints, 7 phases and 8 open questions, with appendices on the Hard preset,
  achievement equivalents, anomalies and contract types.
- Indexed it in the architecture README, and pointed the milestone question in
  `OPEN_QUESTIONS.md` at it.

**Decided.** By the operator, the goal and the six named criteria. Everything else in
the plan is proposed.

**Verified.**
- The counts were computed by a read-only script over the installed game's data files on
  2026-11-14. They are 63 technology nodes costing 18,468 science, 461 researchable
  parts, 56 experiment definitions, 19 contract groups and 11 strategies. The script was
  not committed, and no game file was copied.
- `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.**
- The Hard preset's settings come from the official wiki, which predates some later
  settings. Several of them are unknown until a fresh Hard save is read.
- The facility gates and crew skills are recalled, not checked.
- The finding that Kerbal Space Program 2 has no Steam achievements rests on its Steam
  pages alone, since other sources refused automated reading.
- The body and biome lists, and so the full set of science subjects, were not read,
  because the game was not running.

**For the operator.** The eight open questions at the end of the plan. Q1 decides
whether the campaign plays Hard unmodified, which disables the reloading that charter
item 2 allows. Q2 decides what completing all missions means, and Q6 which achievement
list was meant.
