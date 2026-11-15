# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Monolith incidents, 2026-11-14

**Done.** Recorded the operator's ruling that an accidental monolith discovery needs an
incident report and an adjusted endgame strategy. R11 gains the trigger. The reserved
set in [`CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md) shrinks to 13
nodes, which leaves two monoliths of slack, and the section says how an accident adjusts
the set. Mission plans near a revealed monolith state their closest approach.

**Decided.** By the operator on 2026-11-14.

**Verified.** `./scripts/check.sh` exits zero on this branch with every file staged, on
macOS.

**Not verified.** The discovery distance, which task T13's test measures.

**For the operator.** Nothing new.
