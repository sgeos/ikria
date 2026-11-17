# Handoff

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This state file is overwritten whole within 8 KB. History lives in git. A rewrite
that drops an open item names it in the commit message as completed, moved or abandoned.

## State on 2026-11-17

Written by Codex on `feat/codex-first-rehearsal`, based on local T10 commit `614794e`
on `feat/codex-scripted-harness`. Remote `main` was refreshed and remains `1ce12bb`.
Both branches are unpushed. No push or merge was authorized by these prompts.

- **Seed.** `sgeos/project-template` at `3954eba`, tag `v1.0.8`. Public repository
  `https://github.com/sgeos/ikria`. The seed review finished on 2026-11-13, with public
  rulings in charter item 10.
- **Campaign.** The operator settled
  [`CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md) on 2026-11-14, with proposed
  technology-order and community-practice appendices. R5 permits deferred rescues and
  R18 adds a solvency reserve. [`CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md)
  proposes E0 to E6 and recalibration checkpoints, including end-of-E0 measurements.
- **Record.** [`AGENCY_RECORD.md`](../architecture/AGENCY_RECORD.md) defines vehicle,
  mission, program and financial records with methodology open to professional review.
  The vehicle and mission articles remain unpublished. On 2026-11-17 the operator
  authorized provisional sections. Program and financial articles remain pending.
- **T10 complete locally.** The single vessel scheduler, scripted host and Keleusma
  rate-damping adapter have 13 deterministic tests and a runnable example. They are
  not a live bridge or a qualified flight program.
- **T16 complete locally.** Two starting-parts hop revisions flew empty in the
  verification save and were recovered with all eight parts. Revision A reached
  4,332.4 m above terrain and 8.93 g sampled peak load. B reached 1,239.4 m and 3.46 g.
  [VR-0009](../../record/verification/VR-0009.md) links telemetry, probes and the
  provisional vehicle sheet. No career mission was cleared.

## Operator scope

Ikria remains a Keleusma technology demonstrator with no physical-world impact.
The architecture and scope remain in [`PROJECT_CHARTER.md`](../decisions/PROJECT_CHARTER.md),
with open questions in [`OPEN_QUESTIONS.md`](../architecture/OPEN_QUESTIONS.md).

On 2026-11-17 the operator authorized early rehearsals and mechanics investigation
in "Ikria Physical Verification", including debug features, while keeping designs
within the actual starting career limits. The real career file was read without
modification and still holds only the start node, zero science and 10,000 funds.
The verification game was left at the Space Center after recovering both articles.
Jebediah was recovered before each ignition and remains available.

## Next step

1. Review the local rehearsal commit and T10 commit `614794e`. Ask before pushing
   either branch. Keep the parent commit when reviewing the rehearsal branch.
2. T12 remains open. Implement the live Keleusma flight path and phase-specific
   aborts, account for the mass change after crew removal, establish repeatability,
   and specify qualification of a starting-parts article without a probe core. The
   empty capsule has neither control authority nor a link. The Python probe's
   part-level commands were verification instrumentation, not career control policy.
3. T8, the save reader, remains open. The bridge can start T11 against `ikria-records`,
   including measurement of JSON encoding cost. E1 and E2 vehicle designs remain work.
4. T14 remains paused after step 4 of
   [`TP-0001.md`](../../record/verification/TP-0001.md), until a vessel meets a Green
   Monolith. The earlier kRPC `load` hang remains unresolved. Training is in
   `CREW_TRAINING.md`, C13 sites in `record/sites/`, and body hazards in `HAZARDS.md`.
   The sandbox still holds `IkriaT0`, `IkriaT1` and one flag. The monolith rover setup
   remains in the verification save. Local probes and Python client are in `tmp/`.

## Other open items

- **Authority.** Standing push and merge authority has not been extended to Codex or
  Grok Build. These sessions explicitly require approval before pushing.
- **Template upstream.** The readiness changes to `scripts/worktree.sh` and
  `scripts/check.sh` diverge from the template and need an upstream decision. A later
  sync could undo them. The rehearsal branch also adds Python tests to the gate.
- **Protected main.** Changes reach `main` through a pull request with a green
  `CI result` check.
- **Prior art.** Cite and consider MIT Lincoln Laboratory's KSPDG, which uses kRPC
  and PhysicsRangeExtender for multi-agent pursuit-evasion games.
- **Determinism.** Measure it rather than assume it. Game time is the record's clock.
  The hop probes sampled asynchronously and establish no timing determinism.

## Environment

Earlier details are in the Phase 0 reports. The installation is KSP 1.12.5 build 03190
with both expansions, kRPC 0.7.0 and Module Manager 4.2.3, under Rosetta 2. Anticipated
Rosetta removal in macOS 28 remains a risk. CKAN instance `ksp1` uses CKAN 1.36.4 under
MacPorts Mono. Other game settings remain documented in the earlier Phase 0 reports.

- **Connection.** Local kRPC on ports 50000 and 50001 was reached on 2026-11-17.
  `krpc.game_scene` is the scene property. Setting it to Space Center worked.
- **Launch limit.** Empty-capsule placement timed out in Flight and Space Center.
  Crewed placement worked. Missing control sources are a possible cause, unconfirmed.
  Crew removal and recovery before ignition allowed the empty mechanics tests.
- **Physics.** Four difficulty groups matched the career. No debug-service setter
  was used. Relevant debug flags read false after the flights, but were not sampled
  before them. The final probe now logs them before ignition.
- **Vessels.** Rails ignore controls and thrust. The documented orbital thresholds
  are 350 m for rails and 2.5 km for unloading. Raising `physics_range` persists until
  game exit and may accumulate destructive error. Rails warp puts all vessels on
  rails. No documented multi-client limit was found.
- **Crew.** Boarding requires reaching a hatch. Jetpack and lamp controls require the
  active vessel. Flag and inventory results remain in VR-0003.
- **Bridge.** `sgeos/ikria-bridge`, public and GPL-3.0-only, is checked out at
  `~/projects/keleusma/ikria-bridge`, outside this sandbox. No implementation was found
  when inspected. It was not modified.
- **Saves.** Career "Ikria Automated Space Agency", sandbox "Ikria Simulated Research
  and Dev", and test career "Ikria Physical Verification" use Hard, with reloading
  enabled in the test career. Older "Rookie" and "Sandbox" remain untouched.
- **Checkpoints.** `ipv-before-early-rehearsal` precedes the new tests. The local
  `IKR-TS-001A` and `IKR-TS-001B` craft and their pad and landing saves remain available.

## Validity

```sh
git status --short                # clean
git log --oneline -5              # newest commit describes this state
RUSTC_WRAPPER= ./scripts/check.sh  # exits zero
```

Report checks not run and commits after this file's last rewrite. The startup handoff
was valid at `1ce12bb`, with only that merge after its rewrite at `bf64ba3`, and all
16 then-existing tests passed. T10 passed 29 tests. The rehearsal gate passes 29 Rust
and 13 offline Python tests, none ignored, on macOS with Rust 1.100.0. Two live hops
were observed. No remote continuous integration result was observed.

## Mailbox and articles

None assigned. The blog line does not use the Keleusma interline mailbox. Claude Code
lines message its session. Other assistants ask the operator. The 2026-11-14 requests
for incident reports, vehicle and mission records, and program and financial records
remain the context. Incident article A391 was drafted but unpublished. The operator
has now authorized provisional vehicle and mission sections. Ask before the first
remaining document type whose article status is unknown.
