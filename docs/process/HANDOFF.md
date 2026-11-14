# Handoff

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This is a state file. It is overwritten whole on every update and holds only what is in
flight, why, what is owed and the next step, within a budget of 8 KB. History lives in
git. A rewrite that drops an open item says in its commit message whether the item was
completed, moved or abandoned.

## State on 2026-11-13

- **Seeded** by the admin session from `sgeos/project-template` at `3954eba`, tag
  `v1.0.8`. Nothing is implemented. The public repository is
  `https://github.com/sgeos/ikria`.
- **Seed review done.** The receiving line reviewed the tracked files, the full history
  and the text on GitHub on 2026-11-13, and reported to the operator in session. The
  operator's rulings that belong in the record are in the charter, in item 10 and in
  its context paragraph.
- **Campaign plan proposed.** On 2026-11-14 the operator asked for the game sequence.
  [`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md) defines the
  completion criteria, the rules of play, the constraints and seven phases. Its counts
  come from a read-only pass over the installed game's data files, and its preset,
  achievement and anomaly facts from the web. The operator answered its eight questions
  the same day, adopted every proposed criterion and rule, and amended charter item 2.
  The plan is held unpushed on `docs/campaign-plan` until it is final.
- **The operator's aim.** A technology demonstrator for Keleusma, with no physical-world
  impact. Keleusma stream processors fly craft in Kerbal Space Program 1 through the
  kRPC mod, an MCP server exposes the same controls to agents, and the record documents
  missions against a textbook model of the game, as a sandbox. The name is the Greek
  word for the land-based rowing benches on which crews learned to row in time.

## Operator brief, 2026-11-13

Ikria has three parts.

1. **A global mission controller** that runs outside KSP, probably with MCP
   functionality, and a way to run its own space-program-level driver loops.
2. **A harness** that drives individual craft and personnel with stream processors
   written in Keleusma. It supports many concurrent craft, whether or not that is wise
   for a given mission. It hot loads new stream processors to change behaviour, which
   is sometimes simpler than one complex state machine across mission phases.
3. **A bridge** to the KSP simulator.

The working loop is to design and test craft in a sandbox save, then fly the real
missions in a career save. The whole project and its progress are presented publicly as
a Keleusma technology demonstrator.

The operator's answers to the first questions are the charter,
[`docs/decisions/PROJECT_CHARTER.md`](../decisions/PROJECT_CHARTER.md). The questions
still open are in [`docs/architecture/OPEN_QUESTIONS.md`](../architecture/OPEN_QUESTIONS.md).

## Next step

1. Finalize `CAMPAIGN_PLAN.md`. Its remaining questions O7 and O8 need the operator.
   Push and open the pull request only once the operator calls the plan final.
2. Draft a design document in `docs/architecture/` for the operator to review. It should
   cover the Rust host that embeds Keleusma, the record types for telemetry in and
   commands out, the stepping contract between host and script, and the MCP tools. Keep
   KSP types behind the bridge, so that the harness and the MCP tools can be tested
   against a scripted host without the game running. That rationale is this line's
   recommendation, not an operator ruling.
3. Define the first milestone in `docs/roadmap/` once the design is approved. The
   plan proposes that it be Phase 0, the ground work.

## Open

- **The default branch is protected** by the template ruleset, so changes reach `main`
  only by pull request with a green `CI result` check.
- **Gate.** `scripts/check.sh` holds only the template's checks. Add Rust lint, format
  and test commands when code arrives.
- **Prior art to cite and perhaps reuse.** MIT Lincoln Laboratory's KSPDG drives KSP
  headlessly through kRPC for multi-agent pursuit-evasion games, and handles distant
  craft with PhysicsRangeExtender.
- **Determinism** is a result to measure, not an assumption. KSP slows the simulation
  under load rather than taking larger physics steps, so records use game time.
- **Mastery checklists.** KSP has no Steam achievements. Its own records are the
  progress milestones ("world firsts") and the science archive by body, situation and
  biome, both in `saves/<name>/persistent.sfs`.

## Environment

Observed by the admin session on 2026-11-13.

- **The game.** KSP 1.12.5, build 03190, from Steam, at
  `~/Library/Application Support/Steam/steamapps/common/Kerbal Space Program`, with
  both expansions. It is Intel-only and runs under Rosetta 2. Rosetta is expected to be
  removed in macOS 28, which would stop the game on this Mac.
- **Mods.** kRPC 0.7.0 and Module Manager 4.2.3, installed with CKAN as the instance
  `ksp1`. The `ckan` command wraps CKAN 1.36.4 under MacPorts Mono. kRPC is GPL-3.0.
- **Game settings.** Running in the background, advanced tweakables and the extra
  action groups are on. The exit and physics-warp confirmations are off. The window is
  1920 by 1080, not full screen.
- **kRPC server.** Verified 2026-11-13: version 0.7.0, listening on `127.0.0.1` ports
  50000 (RPC) and 50001 (streams) only. The official Python client 0.7.0 connected and
  read the save read-only. In this version the game scene is `krpc.game_scene`, not
  `current_game_scene`.
- **kRPC multi-vessel control,** from the installed 0.7.0 client's docstrings, since
  the online API pages returned 404 that day:
  - Each vessel has its own `control` and `auto_pilot`, not only the active vessel.
  - A vessel on rails "produces no thrust and ignores control input". In orbit the game
    puts a vessel on rails 350 m from the active vessel and unloads it at 2.5 km.
  - `vessel.physics_range` raises one vessel's range, as PhysicsRangeExtender does for
    KSPDG. It persists past client disconnect until the game exits, "costs
    performance, and accumulates positional error that can destroy it".
  - Rails time warp puts everything on rails, so multi-agent runs use real time or
    physical warp.
  - No documented limit on several clients at once was found.
- **kRPC EVA control,** same source:
  - `crew_member.eva()` returns the kerbal as a vessel that can be "walked around and
    flown using its `Vessel.control`".
  - `control.board()` boards only through a hatch the kerbal already stands at.
  - The jetpack and helmet lamp switch only for the active vessel.
  - Flags and carried items were not checked.
- **Observed 2026-11-14.** The game was not running, and the system Python has no kRPC
  client, so the body and biome lists could not be read. A career save named "Rookie"
  exists on the Normal preset, beside the sandbox save.

## Validity

```sh
git status --short          # clean
git log --oneline -5        # the newest commit should describe this state
./scripts/check.sh          # exits zero
```

Report any check not run, and any commit after the one that last rewrote this file.

## Mailbox

None assigned.
