# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Campaign plan rulings, 2026-11-14

**Done.**
- Recorded the operator's rulings on the eight questions of
  [`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md). All 18
  criteria and rules R1 to R8 are now decided.
- Added R9 on information parity and R10 on losses and reloading. Each states the
  operator's ruling and the details this line proposes for it.
- Amended charter item 2, so that quickloads and reverts apply to the sandbox save
  only.
- Mapped the 41 missions of Kerbal Space Program 2 onto Kerbal Space Program 1 tasks,
  under the operator's rule for C6, and folded the new tasks into the phases.

**Decided.** By the operator on 2026-11-14, as the plan's rulings table records.

**Verified.**
- The game keeps up to five rolling backups of each save, from `SAVE_BACKUPS = 5` in
  its settings and the files in an existing save's backup folder. A save can therefore
  be restored from outside the game.
- `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.**
- The mission list of Kerbal Space Program 2 comes from its community wiki and its
  release notes on Steam. Several mappings rest on recalled names and values in Kerbal
  Space Program 1 and are marked as such.
- No restore of a save from outside the game has been tried.

**For the operator.** Questions O1 to O6 at the end of the plan. They cover the details
of R9 and R10, the mission mapping, Mission mode, the save names, and whether training
and scenarios are in scope. The plan is held unpushed until the operator calls it final.
