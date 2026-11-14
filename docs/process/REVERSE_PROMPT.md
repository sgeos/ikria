# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Harness design, 2026-11-14

**Done.**
- Drafted [`docs/architecture/HARNESS_DESIGN.md`](../architecture/HARNESS_DESIGN.md)
  for the operator's review. It covers three processes, the host abstraction, how the
  harness runs Keleusma loops, the record types, the enforcement of the rules of play,
  the bridge to kRPC, lockstep stepping with kRPC's tick hold, the MCP tools, the
  repositories, and eight Phase 0 experiments.
- Recorded the full Hard settings from the new career save in the campaign plan's
  Hard appendix, and added open question O9, since Hard disables other launch sites.
- Pointed open questions 2, 4 and 5 at the design.

**Decided.** Nothing new. The design is proposed.

**Verified.**
- The two new saves exist, both on Hard, and their settings were read from their files.
- The career's technology tree matches the stock tree node for node.
- Keleusma's embedding interface was read from its repository without changes.
- kRPC's protocol, tick hold and licences were read from its documentation, its release
  archive and its repository.
- The kRPC Python client 0.7.0 installed in `tmp/krpc-venv`.
- `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.**
- The kRPC server was not reachable, because the game was at its main menu, so no
  experiment has run.
- The licensing analysis is not legal advice.

**For the operator.**
- D1, the process layout.
- D2, whether the bridge lives in a new GPL-3.0 repository.
- O9, how C16 is met.
- Loading the sandbox save, so the experiments can run.
