# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Record types, version 1, 2026-11-14

**Done.** Task T7 of [`V0_1_0.md`](../roadmap/V0_1_0.md). The `ikria-records` crate holds
the record types and the messages between the bridge and the harness.
- **Validated types.** Game time, vessel and part identifiers, vectors, rotations, bounded
  controls, situations and resources. Each refuses a value that is not finite or is out of
  range, both when built and when decoded.
- **Telemetry and commands.** Telemetry per vessel, and commands per vessel with action
  group, abort, engine, decoupler and experiment requests. Staging is left out, since
  kRPC stages only the active vessel.
- **Messages.** A handshake that checks the protocol version, and the per-tick messages.
  Each is encoded as one line of JSON.

**Decided.** Nothing new.

**Verified.** `RUSTC_WRAPPER= ./scripts/check.sh` exits zero on this branch with every
file staged, on macOS. The tests ran 16, 10 in the new crate and 6 in the harness, and all
passed, with none skipped.

**Not verified.** The cost of JSON encoding at 50 ticks a second, which the bridge's
first run measures.

**For the operator.** Nothing.
