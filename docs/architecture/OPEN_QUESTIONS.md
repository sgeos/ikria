# Open Questions

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

Raised by the admin session on 2026-11-13. The operator answered the first set the same
day, and those answers are recorded in
[`docs/decisions/PROJECT_CHARTER.md`](../decisions/PROJECT_CHARTER.md). The questions below
remain. Record each answer here with its date, or move it to `docs/decisions/`, and
delete the question once it is settled.

## Mission controller

1. **Agent authority through MCP.** Which actions may an agent take without the
   operator's approval, such as launching, accepting contracts or spending career funds?

## Harness

2. **Process layout.** One host process running many Keleusma programs, or one process
   per craft? Who owns the shared clock?
3. **Concurrent EVA.** The jetpack and helmet lamp switch only for the active vessel in
   kRPC 0.7.0, which limits how many kerbals can manoeuvre at once.

## Bridge

4. **Rust client.** Is there a maintained Rust kRPC client, or must one be generated from
   kRPC's service definitions? Check the client libraries' licences before linking any.
5. **Physics-tick stepping.** Measure whether kRPC can sustain one control step per
   physics tick, and at what cost.

## Space agency

6. **Building craft.** By what path does an agent build a craft: kRPC's editor vessel,
   generated craft files, or something else?
7. **Milestones.** Which career objectives make the first milestone? The proposed
   answer is Phase 0 of [`CAMPAIGN_PLAN.md`](CAMPAIGN_PLAN.md), whose own open questions
   are listed there.
