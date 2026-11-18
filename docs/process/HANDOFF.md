# Handoff

> **Navigation**: [Process](README.md) | [Documentation Root](../README.md)

Keep this state file within 8 KB. History lives in git. Account for removed open
items in the commit message.

## State on 2026-11-18

Written by Codex for Claude on `docs/codex-claude-handoff`, based on main `583e1d4`.
Ikria [pull request 34](https://github.com/sgeos/ikria/pull/34) merged as `583e1d4`,
then bridge [pull request 5](https://github.com/sgeos/ikria-bridge/pull/5) merged as
`5d63344`, both on 2026-11-18 in Coordinated Universal Time. A Claude line received
this handoff on 2026-11-18 and found it valid.

- **Seed.** `sgeos/project-template` at `3954eba`, tag `v1.0.8`. Seed review finished
  on 2026-11-13. Public rulings remain in charter item 10.
- **Campaign.** `CAMPAIGN_PLAN.md` and `CAMPAIGN_SEQUENCE.md` retain E0 to E6,
  recalibration checkpoints, deferred rescues under R5 and the solvency reserve R18.
  Appendices remain proposed.
- **Record.** Vehicle and mission articles remain unpublished. The operator authorized
  provisional sections on 2026-11-17. M-0001 and VR-0010 use them. I-0001 and I-0002
  follow R11's provisional contents while incident article A391 is unpublished.
  Program and financial articles remain pending. Check their status before the first
  records of those types.
- **Completed and merged.** T10 is the scripted harness, T16 the two Python mechanics
  hops, T11 the separate Rust bridge and T17 the single-vessel Keleusma flight path.
  The latter includes checked dialogue loading, resource admission, independent abort,
  command policy, timeouts, journals and replay. Commands and limits are in
  [FLIGHT_CONTROL.md](../architecture/FLIGHT_CONTROL.md).
- **Live evidence.** [VR-0010](../../record/verification/VR-0010.md) records a complete
  normal flight and a coast-abort flight with intact eight-part recovery. Their 6,824
  and 6,798 samples replay to identical commands. I-0001 records an empty article lost
  to a velocity-frame defect. I-0002 records a touchdown bounce that stopped software
  after intact landing. Both were corrected.

## Worktrees

Preserve `tmp/worktrees/feat-codex-flight-control`, which holds Codex's raw logs and
rehearsal tooling. The bridge's sibling `ikria` symlink supplies its records, and its
workflow pins `504d4c4`.

## Operator scope and last observed game state

Scope remains in `PROJECT_CHARTER.md`, questions in `OPEN_QUESTIONS.md`. The operator
authorized rehearsals in "Ikria Physical Verification", including debug features,
while keeping designs
within actual starting career limits. No debug setter was used in the recorded series.

On 2026-11-17 the career had only the start node, zero science and 10,000 funds, and
was not loaded since. On 2026-11-18 the Sepratron program unlocked the verification
save's whole tree. Its checkpoints are "ipv-before-sepratron-program" and
"ipv-after-sepratron-program". Jebediah is Missing there, I-0007. Recheck before use.

The empty starting capsule has no control source or link. The flights used part-level
verification instrumentation and passive fins. Continuous controls were zero, so
active attitude actuation is not qualified. No career mission is cleared.

## Open work and recommended resumption

1. Fix the known defects in `FLIGHT_CONTROL.md` first. Read VR-0010, M-0001, I-0001
   and I-0002 before more flights. Use a fresh abort-request
   path for a normal run. The flight worktree still contains `tmp/abort-flight-4`.
   Never retry an uncertain command.
2. T12 is partial. The original sandbox qualification remains open. Resolve a
   campaign-valid control source, qualify attitude actuation, broaden abort tests,
   establish repeatability, account for crewed mass and loading, and include mission
   science. E1 and E2 designs remain open.
3. T8 is done as `crates/ikria-save`. T9, the generated science ledger, remains open. Career
   authorization must come from the actual save. Multi-vessel scheduling, general
   phase modules and in-flight replacement, calibrated timing admission and the
   mission controller remain unimplemented. JavaScript Object Notation timing and
   atomic bridge batching remain measurement and optimization work.
4. T18, the Sepratron program, is done. Its design rules are in
   `docs/research/SEPRATRON_PROGRAM.md`. The ejectable cockpit and hybrid ascent are
   designs, not yet flown. Its rules are adopted as R16, R19 and R20. Jebediah's restored
   copy "ipv-jeb-restored" awaits the operator's load, I-0007.
5. T14 remains paused after step 4 of TP-0001, until a vessel meets a Green Monolith.
   The earlier kRPC load hang is unresolved. Training is in `CREW_TRAINING.md`, C13
   sites in `record/sites/`, and body hazards in `HAZARDS.md`.

## Other open items

- **Template divergence.** `scripts/worktree.sh`, `scripts/check.sh` and
  `scripts/scan_binary_terms.py` diverge from the template, and a sync could undo them.
  Items 1 to 8 of the feedback to the admin line are upstream as template commit
  `8a79622`, pull request 18, merged 2026-11-18, and item 9 is deferred. A sync may adopt
  them after a comparison. Wording that differs from Ikria's rules needs the operator's
  confirmation, under `CLAUDE.md`.
- Main remains protected and requires a pull request with a green `CI result`.
- Cite and consider Massachusetts Institute of Technology Lincoln Laboratory's KSPDG,
  which uses kRPC and PhysicsRangeExtender for pursuit-evasion games.
- Measure game determinism rather than infer it from software replay. The journals
  establish command replay and observed clock increments only.

## Environment and retained observations

Kerbal Space Program 1.12.5 build 03190 with both expansions, kRPC 0.7.0 and Module
Manager 4.2.3 runs under Rosetta 2. Rosetta removal in macOS 28 is a
risk. CKAN instance `ksp1` uses CKAN 1.36.4 under MacPorts Mono.

- kRPC uses ports 50000 and 50001. Bridge loopback port 50120 was exercised.
  `krpc.game_scene` is the scene property. Recheck the server hold timeout before
  changing the installation. The bridge deadline assumes a one-second server hold.
- Empty-capsule placement timed out. Crewed placement, then crew removal and recovery
  before ignition, works. Missing control sources remain the unproved cause.
- Four verification difficulty groups matched the career in VR-0009. Five debug flags
  were false before and after the new series, not sampled continuously.
- Measure velocity in the rotating body frame. The vessel-moving surface frame
  returned zero during flight. Transform directions separately from velocity sampling.
- Rails, physics range, boarding, jetpack, flags and inventory observations remain in
  the Phase 0 reports. Multi-client limits remain undocumented.
- The separate GPL-3.0-only bridge is at `~/projects/keleusma/ikria-bridge`.
- Saves remain "Ikria Automated Space Agency", "Ikria Simulated Research and Dev" and
  "Ikria Physical Verification". Rookie and Sandbox remain untouched. The sandbox
  holds IkriaT0, IkriaT1 and one flag. The monolith rover and checkpoint
  `ipv-before-early-rehearsal`, revision A and B craft, pad and landing saves, and the
  Python client remain available. Compressed telemetry and hashes are in VR-0010.

## Validity

```sh
git status --short
git log --oneline -5
RUSTC_WRAPPER= ./scripts/check.sh
```

A valid checkout is clean, its newest commit describes this state, and its gate exits
zero. Report checks not run and later commits. The inherited handoff was stale about
publication. The local gates pass on macOS with Rust 1.100.0, covering 60 Rust and 13
Python tests here and three bridge tests, none ignored. Post-merge continuous
integration passed for Ikria in run `44255410252` and bridge in run `44256254190`.
Those results cover the implementations. Check the handoff pull requests for their
required checks and merge state. Live observations are separate evidence.

## Mailbox

None assigned. The blog line does not use the Keleusma interline mailbox. Coordinate
through the operator. Article requests remain context.
