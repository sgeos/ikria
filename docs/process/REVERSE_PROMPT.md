# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Green Monolith test, a replayed draw, 2026-11-16

**Done.** The test design in TP-0001 now allows for the operator's point. If the state
of the pseudorandom number generator persists in the save, plain reloads replay the same
draw. Each discovery now changes one input from the same quicksave.
- Plain reloads.
- Game restarts.
- Elapsed game time.
- Each of the save's four seeds.

The snipe and placement steps remain. The campaign sequence now points to TP-0001 for the
design instead of keeping a copy.

**Decided.** Nothing new. T14 stays paused until a vessel meets a Green Monolith.

**Verified.** Documentation only. The gate exits zero on this branch with every file
staged, on macOS.

**Not verified.** Which seed, if any, feeds the reward.

**For the operator.** Nothing. Next are T8, the save-file reader, and T10, the harness
skeleton.
