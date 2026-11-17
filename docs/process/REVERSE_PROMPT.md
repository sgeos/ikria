# Reverse Prompt

> **Navigation**: [Process](./README.md)

Written by Codex on 2026-11-17. This state file replaces the prior report.

## Early verification rehearsals

**Operator authority.** The operator authorized rehearsals and game-mechanics
investigation in the verification save, including debug features, and emphasized
that the real career had only just started. The operator confirmed that the vehicle
and mission articles remain unpublished and authorized provisional sections.

**Facts.** Local T10 commit `614794e` remains unpushed. The new branch
`feat/codex-first-rehearsal` starts there. Remote `main` was refreshed and remains
`1ce12bb`. The startup protocol was completed earlier in this session. No mailbox is
assigned. Both branches still require approval before pushing.

**Completed.** Two eight-part hop revisions were derived from the local `IkriaT0`
using only career-purchased starting parts. Each flew empty and landed with eight
parts intact, then was recovered. A reached 4,332.4 m above terrain with a sampled
peak load of 8.93 g. B reached 1,239.4 m with 3.46 g. Jebediah only occupied the pad
craft and was removed and recovered before ignition. He remains available.

The career file remained unchanged, with only the start node, zero science and
10,000 funds. The game was left at the Space Center in the verification save. A
checkpoint preserves the state before this work. No debug-service setter was used.

**Implementation and record.** The branch adds guarded craft preparation, setup,
flight and summarization tools, 13 offline tests, raw telemetry, VR-0009 and a
provisional IKR-TS-001 vehicle sheet. T16 records these mechanics tests. T12 stays open.
The gate now includes the Python tests. The report records the exact limitations,
including changes between the initial probes and the final scripts.

**Verified locally.** The staged gate passes formatting, Clippy with warnings denied,
repository checks, the disclosure scan, 29 Rust tests and 13 Python tests. None were
ignored. Both live flight runs succeeded. The tests cover deployment boundaries,
non-finite inputs, unexpected craft inputs, absent purchases, and refusal to operate
on the wrong save, wrong vessel or a crewed flight article. No remote continuous
integration result was observed.

**Findings and uncertainty.** Empty-capsule placement timed out from two scenes.
Crewed placement succeeded, which supports a missing-control-source explanation but
does not prove it. The empty capsule reports no control authority or link, yet
part-level engine and parachute commands work. Those are verification instrumentation
and cannot silently become the career command policy. The reported dialog was the
crew recovery summary. A scene switch through kRPC worked without manual intervention.

**Not cleared.** These were Python mechanics tests, without a Keleusma flight program,
phase-specific abort testing, repeated trials, a crewed mass model or full mission
science collection. Debug flags read false only after the flights, so their state
throughout was not proved. Career qualification needs the live control path and those
checks. No early mission is declared cleared. E1 and E2 designs remain future work.

**Preserved open work.** T8, T11, T12 and the paused Green Monolith test remain open.
The template upstream decision, standing authority for other assistants, program and
financial articles, and T10 publication remain pending. No push or merge occurred.
