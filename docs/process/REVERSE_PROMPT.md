# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Campaign sequence, 2026-11-14

**Done.** Wrote
[`docs/architecture/CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md), the
prospective plan the operator asked for. It holds the facility gates and costs, the
early game as stages E0 to E6 with research, purchases, upgrades and expected science,
a funds plan for leaving the first research window, rules for buying, a sketch of the
stages after, and checkpoints. Linked it from the campaign plan and the architecture
index.

**Decided.** Nothing new. The sequence is proposed.

**Verified.**
- Node costs, part entry and unit costs, experiment values, strategy definitions and
  the base rewards of world firsts and rescues were read from the installed game's data
  files.
- Two finished contracts' payouts were read from the existing Normal career save.
- Facility costs and gates, biome lists, body multipliers and strategy rates were read
  from the official wiki through its public interface.
- The stage science was computed by a script, not by hand. `./scripts/check.sh` exits
  zero on this branch, on macOS.

**Not verified.**
- That the 60 percent reward scales each subject, the scientist level needed to reset
  experiments, and the recalled antenna powers.
- Whether a kerbal may leave a vessel in orbit at the first Astronaut Complex level.
- The weights of world-first rewards, the launch costs, and whether the wiki's strategy
  rates hold on Hard.
- That converting science to funds leaves the subjects counted for C4.

**For the operator.** Nothing is blocking. The main uncertainty is the funds gap of
about 0.9 to 1.0 million before the second level of Research and Development, which the
first checkpoint measures.
