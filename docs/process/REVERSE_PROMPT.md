# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Monoliths, the jetpack, and the first milestone, 2026-11-14

**Done.**
- Investigated the Green Monoliths at the operator's request, and recorded the facts,
  the open questions and a test plan in
  [`CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md).
- Analysed the EVA jetpack's Δv against every airless body, and recommended holding it in
  reserve outside R6's budgets.
- Settled Phase 0 experiments 5 and 6 from the kRPC client's interface, without touching
  the career save.
- Defined the first milestone, [`V0_1_0.md`](../roadmap/V0_1_0.md).

**Decided.** Nothing new. The test plan, the jetpack recommendation and the milestone
are proposed.

**Verified.**
- The KerbNet parts and their anomaly detection, the jetpack's mass and capacity, and
  EVA Propellant's density were read from the game's data files.
- The bodies' radii and gravity came from the running game.
- kRPC's lack of facility levels and of funds reasons was read from its client.
- `./scripts/check.sh` exits zero on this branch with every file staged, on macOS.

**Not verified.**
- How a Green Monolith chooses its node, and whether it ignores the research cap.
- The jetpack's real Δv, which rests on the wiki's figure of about 600 m/s.
- Whether the jetpack refills from a vessel's monopropellant.

**For the operator.** The monolith test needs a test career save on Hard, separate from
the campaign's career, which only the operator can create in the game. That is task
T13 of the milestone.
