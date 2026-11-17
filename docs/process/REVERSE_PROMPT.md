# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. Written by Codex on 2026-11-16.

## Task T10

**Facts.** The operator requested the session startup checks and task T10 on a separate
branch, with approval required before pushing. The startup handoff was valid. The
working tree was clean, the gate passed 16 tests with none ignored, and merge `1ce12bb`
was the only commit after the handoff rewrite at `bf64ba3`. No mailbox was assigned.
The previous report, documentation index and required session-start reading were read.

**Implemented.** Branch `feat/codex-scripted-harness` has an isolated worktree. Remote
`main` was fetched and matched its base `1ce12bb` before this state rewrite.

- A single vessel `Host` contract and finite `ScriptedHost`, with a held tick until a
  matching command is applied and an ordered command log.
- A scheduler that validates vessel identity and increasing game time before stepping
  its loop, then validates command identity and requests before applying the result.
  Errors permanently stop the scheduler, and clean exhaustion is also terminal.
- A `RateDamper` adapter around the existing verified Keleusma attitude loop. It uses
  owned record types and decodes loop output before another resume. The scripted axis
  convention and fixed-point conversion are documented.
- Thirteen deterministic tests and a runnable `scripted_host` example. T10 is marked
  done, and the design and changelog describe the implementation and its limits.

**Verified locally.** macOS on Apple silicon, Rust 1.100.0. The staged gate ran as
`RUSTC_WRAPPER= ./scripts/check.sh` and exited zero, including formatting, Clippy with
warnings denied, the disclosure scan and all 29 workspace tests. None were ignored.
The 13 new tests cover replay against the independent Rust reference, identical runs,
conversion boundaries, lockstep ordering, identity and time refusals, invalid requests,
runtime faults, invalid loop outputs, host read and write failures, and terminal states.
The example emitted three version 1 command messages, checked for their game times,
vessel, pitch values and neutral throttle.

**Limits.** The rate damper is a test adapter. It has no reference-frame transformation
or target orientation and is not qualified for flight. No bridge connection, protocol
handshake, live hold release, campaign gate, vehicle safe state, program swap, timing
admission or multiple vessel scheduling was implemented. Only the telemetry fields the
adapter consumes have type-enforced checks here. Script and command storage are in
memory. No game run or remote continuous integration result was observed.

**Decisions.** The scheduler stops on every failure because the loop or host can have
partially advanced. It does not retry an uncertain command application. No replacement
safe command is sent. These are implementation choices for the scripted skeleton.
The operator made no new campaign or authority ruling.

**For the operator.** Review the local commit and approve pushing this branch if ready.
Standing authority for Codex and Grok Build remains unresolved. The readiness changes
to `scripts/worktree.sh` and `scripts/check.sh` still need a template upstream decision.
Those follow-ups remain open in `HANDOFF.md`, along with the other line's roadmap and
Green Monolith work. Nothing was pushed or merged.
