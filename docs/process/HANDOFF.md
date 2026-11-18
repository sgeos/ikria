# Handoff

> **Navigation**: [Process](README.md) | [Documentation Root](../README.md)

This state file is overwritten whole within 8 KB. History lives in git. A rewrite
that drops an open item names it in the commit message as completed, moved or abandoned.

## State on 2026-11-17

Written by Codex on `feat/codex-flight-control`, based on local rehearsal commit
`8e7c9f5`, which includes T10 commit `614794e`. Remote main was refreshed and remains
`1ce12bb`. The paired bridge branch is `feat/codex-live-bridge`, based on `551bb7b`.
The operator approved this publication on 2026-11-17, Ikria first, then the bridge.
This is session-specific authority. Remote merge ancestry records completion.

- **Seed.** `sgeos/project-template` at `3954eba`, tag `v1.0.8`. Public repository
  `https://github.com/sgeos/ikria`. Seed review finished on 2026-11-13. Public rulings
  remain in charter item 10.
- **Campaign.** `CAMPAIGN_PLAN.md` and `CAMPAIGN_SEQUENCE.md` retain E0 to E6,
  recalibration checkpoints, deferred rescues under R5 and the solvency reserve R18.
  Appendices remain proposed.
- **Record.** Vehicle and mission articles remain unpublished. The operator authorized
  provisional sections on 2026-11-17. M-0001 and VR-0010 use them. I-0001 and I-0002
  follow R11's explicit provisional contents while incident article A391 is unpublished.
  Program and financial articles remain pending. The first records of those types
  still require the article-status check.
- **T10 and T16.** The scripted harness and two Python mechanics hops remain complete
  locally. They are retained in the branch history and VR-0009.
- **T11 and T17.** The separate Rust bridge and single-vessel Keleusma flight path are
  implemented locally. The runtime has checked dialogue loading, resource admission,
  normal and independent abort programs, command policy, timeouts, journals and replay.
  [FLIGHT_CONTROL.md](../architecture/FLIGHT_CONTROL.md) gives commands and limits.
- **Live evidence.** [VR-0010](../../record/verification/VR-0010.md) records a complete
  normal flight and a coast-abort flight with intact eight-part recovery. Their 6,824
  and 6,798 samples replay to identical commands. An earlier frame defect lost an
  empty article, I-0001. A later touchdown bounce stopped software after intact landing,
  I-0002. Both defects were corrected and their evidence retained.

## Operator scope and game state

Ikria remains a Keleusma technology demonstrator with no physical-world impact.
The architecture and scope remain in `PROJECT_CHARTER.md`, with open questions in
`OPEN_QUESTIONS.md`. The operator authorized early rehearsals and mechanics work in
"Ikria Physical Verification", including debug features, while keeping designs within
actual starting career limits. No debug setter was used in the recorded flight series.

The real career remains unchanged, with only the start node, zero science and 10,000
funds. Its persistent-file hash was checked before and after the series. The game was
left at the verification Space Center after recovery. Jebediah is available. The
starting empty capsule has no control source or link. The successful flights used
part-level verification instrumentation and passive fins, not qualified attitude
actuation. No career mission is cleared.

## Next step

1. Publish through green pull requests under the operator's 2026-11-17 approval,
   Ikria before the bridge. This branch includes T10 and the earlier rehearsals.
   Check remote merge ancestry before repeating publication.
2. T12 is partial. The original sandbox qualification remains open. Resolve a
   campaign-valid control source for the starting capsule, qualify attitude actuation,
   broaden abort tests, establish repeatability, account for crewed mass and loading,
   and include mission science. E1 and E2 designs remain open.
3. T8, the save reader, and T9, the generated science ledger, remain open. Career
   authorization must come from the actual save. Multi-vessel scheduling, general
   phase modules and in-flight replacement, calibrated timing admission and the
   mission controller remain unimplemented. JSON timing and atomic bridge batching
   remain measurement and optimization work.
4. T14 remains paused after step 4 of TP-0001, until a vessel meets a Green Monolith.
   The earlier kRPC load hang is unresolved. Training is in `CREW_TRAINING.md`, C13
   sites in `record/sites/`, and body hazards in `HAZARDS.md`.

## Other open items

- **Authority.** Standing push and merge authority has not been extended to Codex or
  Grok Build. This publication has explicit session approval.
- **Template upstream.** Readiness changes to `scripts/worktree.sh` and `scripts/check.sh`
  diverge from the template and need an upstream decision. A later sync could undo
  them. The rehearsal branch added Python tests to the gate.
- **Protected main.** Changes reach main through a pull request with a green `CI result`.
- **Prior art.** Cite and consider MIT Lincoln Laboratory's KSPDG, which uses kRPC
  and PhysicsRangeExtender for multi-agent pursuit-evasion games.
- **Determinism.** Measure game determinism rather than infer it from software replay.
  The journals establish command replay and observed clock increments only.

## Environment and retained observations

KSP 1.12.5 build 03190 with both expansions, kRPC 0.7.0 and Module Manager 4.2.3 runs
under Rosetta 2. Anticipated Rosetta removal in macOS 28 remains a risk. CKAN instance
`ksp1` uses CKAN 1.36.4 under MacPorts Mono. Earlier installation and mechanics evidence
remains in the Phase 0 reports.

- Local kRPC uses ports 50000 and 50001. The new bridge uses a configurable loopback
  port, demonstrated with 50120. `krpc.game_scene` is the scene property.
- Empty-capsule placement timed out. Crewed placement, followed by crew removal and
  recovery before ignition, works. Missing control sources remain the unproved cause.
- Four verification difficulty groups matched the career in VR-0009. Five debug flags
  were false before and after the new series, not sampled continuously.
- Body-frame velocity is required. Reading velocity in the vessel-moving surface
  frame returned zero while altitude changed. The bridge now transforms directions
  separately from the frame used to measure velocity.
- Rails, physics-range limits, boarding, jetpack, flags and inventory observations
  remain in the Phase 0 reports. No documented multi-client limit was found.
- The bridge is a separate public GPL-3.0-only repository at
  `~/projects/keleusma/ikria-bridge`. Its worktree is under this checkout's `tmp/worktrees`.
  Its local records dependency resolves through a sibling `ikria` symlink there.
- Saves remain the career "Ikria Automated Space Agency", sandbox "Ikria Simulated
  Research and Dev", and "Ikria Physical Verification". Older Rookie and Sandbox saves
  remain untouched. The sandbox still holds IkriaT0, IkriaT1 and one flag. The monolith
  rover setup and checkpoint `ipv-before-early-rehearsal` remain available.
- Revision A and B craft, pad and landing saves, and the Python client are
  retained. Generated raw journals and process logs are in the flight worktree's tmp.
  Compressed public telemetry and hashes are in VR-0010's data directory.

## Validity

```sh
git status --short
git log --oneline -5
RUSTC_WRAPPER= ./scripts/check.sh
```

A valid checkout is clean, its newest commit describes this state, and its gate exits
zero. Report any check not run and commits after this file's last rewrite. Startup
handoff was valid at `1ce12bb`, with only that merge after rewrite `bf64ba3`. The final
local gate covers 60 Rust and 13 Python tests here, plus three tests in the separate
bridge gate, none ignored. Local runs used macOS and Rust 1.100.0. Remote results are checked before merging.

## Mailbox

None assigned. The blog line does not use the Keleusma interline mailbox. Other
assistants ask the operator rather than sending unauthorized messages. Earlier article requests remain context.
