# Handoff

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This state file is overwritten whole and holds what is in flight, why, what is owed
and the next step, within 8 KB. History lives in git. A rewrite that drops an open item
names it in the commit message as completed, moved or abandoned.

## State on 2026-11-16

Written by Codex on `feat/codex-scripted-harness`, based on `main` at `1ce12bb`.
Remote `main` was fetched and matched that base before this rewrite.

- **Seed.** `sgeos/project-template` at `3954eba`, tag `v1.0.8`. The public repository
  is `https://github.com/sgeos/ikria`. The seed review finished on 2026-11-13, with
  public rulings in charter item 10.
- **Campaign plan final.** [`CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md)
  holds the operator's criteria, rules, constraints and phases of 2026-11-14, with
  proposed technology order and community practice appendices. Charter item 2 is
  amended. R5 permits deferred rescues, and R18 adds a solvency reserve.
- **Sequence proposed.** [`CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md)
  gives stages E0 to E6, research, purchases, upgrades, science and funds, with
  recalibration checkpoints. End-of-E0 measurements remain part of Phase 0.
- **Record decided.** [`AGENCY_RECORD.md`](../architecture/AGENCY_RECORD.md) defines
  vehicle, mission, program and financial documents. The operator decided the scope
  on 2026-11-14 and requires methodology that holds up to professional scrutiny.
  Formats wait for the blog articles.
- **T10 complete locally.** A single vessel scheduler, finite scripted host and
  Keleusma rate-damping adapter have 13 deterministic tests and a `scripted_host`
  example. The local gate passes 29 tests with none ignored. The adapter is not a
  qualified flight program. Bridge transport, campaign gates, frame conversion,
  safe commands after faults, timing admission and multiple vessels remain unimplemented.

## Operator brief, 2026-11-13

Ikria is a Keleusma technology demonstrator with no physical-world impact. Stream
processors fly Kerbal Space Program 1 craft through kRPC. A Model Context Protocol
server exposes the same controls to agents. The record documents missions against a
textbook model of the game. The name refers to the Greek land-based rowing benches
where crews learned to row in time.

There are three parts. A global mission controller runs outside the game, with agency
driver loops and probably the agent interface. A harness drives craft and personnel
through Keleusma stream processors, supports concurrent craft, and hot loads programs
between behaviours. A bridge connects to the simulator. Craft are designed and tested
in a sandbox, then flown in a career. The project is presented publicly as a technology
demonstrator.

The operator's initial answers are in
[`PROJECT_CHARTER.md`](../decisions/PROJECT_CHARTER.md). Remaining questions are in
[`OPEN_QUESTIONS.md`](../architecture/OPEN_QUESTIONS.md).

## Next step

1. Review the local T10 commit and ask the operator before pushing
   `feat/codex-scripted-harness`. No push or merge is authorized by this prompt.
2. Work the open tasks of [`V0_1_0.md`](../roadmap/V0_1_0.md). T4, T6, T7 and T10
   are done. T8, the save-file reader, remains open. The bridge can start T11 against
   `ikria-records` as a git dependency and measure JSON encoding at 50 ticks per second.
3. T14, the Green Monolith test, is paused after step 4 of
   [`TP-0001.md`](../../record/verification/TP-0001.md) until a vessel meets a Green
   Monolith. kRPC's `load` hangs, so the operator loads saves by hand. Training is in
   `CREW_TRAINING.md`, C13 sites in `record/sites/`, and body hazards in `HAZARDS.md`.
   The sandbox holds `IkriaT0`, `IkriaT1` and one flag. The Python client 0.7.0 and
   probes remain local scratch in `tmp/`.

## Open

- **Authority.** Codex and Grok Build still need the operator to extend standing push
  and merge authority. This session explicitly requires asking before pushing.
- **Template upstream.** The readiness changes to `scripts/worktree.sh` and
  `scripts/check.sh` diverge from the template and need an upstream decision. A later
  template sync could undo them.
- **Protected main.** Changes reach `main` through a pull request with a green
  `CI result` check.
- **Gate.** `scripts/check.sh` runs workspace formatting, Clippy and tests. Use
  `RUSTC_WRAPPER=` inside the sandbox because `sccache` cannot run there.
- **Prior art.** MIT Lincoln Laboratory's KSPDG drives the game headlessly through
  kRPC for multi-agent pursuit-evasion games and uses PhysicsRangeExtender for
  distant craft. Cite and consider reuse.
- **Determinism.** Measure it rather than assume it. The game slows under load instead
  of taking larger physics steps, so records use game time.

## Environment

Earlier observations from the admin session on 2026-11-13, with later dates where
noted. Codex did not recheck the live game during T10.

- **Game.** KSP 1.12.5, build 03190, from Steam, at
  `~/Library/Application Support/Steam/steamapps/common/Kerbal Space Program`, with
  both expansions. It runs under Rosetta 2. Expected Rosetta removal in macOS 28
  would prevent this Intel-only game from running on the Mac.
- **Mods.** kRPC 0.7.0 and Module Manager 4.2.3, installed with CKAN instance `ksp1`.
  The `ckan` wrapper runs CKAN 1.36.4 through MacPorts Mono. kRPC is GPL-3.0.
- **Settings.** Background running, advanced tweakables and extra action groups are
  enabled. Exit and physics-warp confirmations are disabled. Windowed, 1920 by 1080.
- **Server.** Verified 2026-11-13 on `127.0.0.1`, remote procedure calls on 50000 and
  streams on 50001. Python client 0.7.0 connected read-only. The scene property is
  `krpc.game_scene`, not `current_game_scene`.
- **Multi-vessel interface.** Each vessel has controls and autopilot. On rails it
  produces no thrust and ignores controls. The client documents orbital thresholds of
  350 m for rails and 2.5 km for unloading. Raising `physics_range` persists until
  game exit, costs performance and can accumulate destructive positional error.
  Rails warp puts everything on rails. No documented multi-client limit was found.
- **Crew interface.** `crew_member.eva()` returns a controllable vessel. Boarding
  requires reaching a hatch. Jetpack and lamp controls require the active vessel.
  The later flag and inventory results are in
  [`VR-0003.md`](../../record/verification/VR-0003.md).
- **Bridge, 2026-11-14.** `sgeos/ikria-bridge` is public, GPL-3.0-only, and checked
  out at `~/projects/keleusma/ikria-bridge`. It is outside this sandbox.
- **Saves, 2026-11-14.** The operator created career "Ikria Automated Space Agency",
  sandbox "Ikria Simulated Research and Dev", shortened by the game, and test career
  "Ikria Physical Verification". All use Hard, with reloading enabled in the test
  career. Older "Rookie" and "Sandbox" saves remain untouched.

## Validity

```sh
git status --short                # clean
git log --oneline -5              # newest commit describes this state
RUSTC_WRAPPER= ./scripts/check.sh  # exits zero
```

Report checks not run and commits after this file's last rewrite. The startup handoff
was valid at `1ce12bb`, with only that merge after its rewrite at `bf64ba3`. The startup
gate passed all 16 existing tests. The completed T10 gate passes 29 tests, none ignored,
on macOS with Rust 1.100.0. No remote continuous integration result was observed.

## Mailbox

None assigned. The blog line does not use the Keleusma interline mailbox. Claude Code
lines message its running session. Codex and Grok Build ask the operator. On
2026-11-14 the line requested articles on incident reports, drafted as A391 and
unpublished, vehicle sheets and mission documents, and program and financial management.
Templates follow published articles. Before writing the first of each document, ask
the operator whether its article exists.
