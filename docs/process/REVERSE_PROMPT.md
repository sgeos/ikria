# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Green Monolith test, steps 2 and 3, 2026-11-15

**Done.**
- **Step 2.** The operator loaded the edited save from the game's menu. A fresh save held
  the start node and the rover's nine nodes.
- **Step 3.** The rover `IkriaKN` is in a 250 km polar orbit of Kerbin, saved as
  "ipv-monolith-scan". The two debug uses are logged in
  [`TP-0001`](../../record/verification/TP-0001.md).
- **Save names.** Saves a script writes now carry the acronym of their save game, such
  as `ipv-`, at the operator's request. The files already in the verification save were
  renamed. The research edit script no longer calls kRPC's `load`, which hangs.

**Decided.** By the operator on 2026-11-15, that save names carry the save game's
acronym rather than `ikria-`. The acronyms themselves are the line's reading.

**Verified.** Read from the running game and the save files. The gate exits zero on this
branch with every file staged, on macOS.

**Not verified.** The edited scripts have not been rerun since the rename.

**For the operator.** Step 4, the KerbNet scan of Kerbin from the rover.
