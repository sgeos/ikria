# Handoff

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This is a state file. It is overwritten whole on every update and holds only what is in
flight, why, what is owed and the next step, within a budget of 8 KB. History lives in
git. A rewrite that drops an open item says in its commit message whether the item was
completed, moved or abandoned.

## State on 2026-11-14

- **Seeded** by the admin session from `sgeos/project-template` at `3954eba`, tag
  `v1.0.8`. Nothing is implemented. The public repository is
  `https://github.com/sgeos/ikria`.
- **Seed review done** on 2026-11-13. The public rulings are in charter item 10.
- **Campaign plan final.** [`CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md) holds
  the criteria, rules, constraints and phases, settled by the operator on 2026-11-14
  and merged in pull requests 4 and 5, with proposed appendices on technology order and
  community practice. Charter item 2 is amended, and `CLAUDE.md` records the standing
  authority to push.
- **Sequence proposed.** [`CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md)
  gives the early game as stages E0 to E6, with research, purchases, upgrades, science
  and a funds plan, and checkpoints for recalibration. Its end-of-E0 measurements join
  the Phase 0 questions below.
- **Record decided.** [`AGENCY_RECORD.md`](../architecture/AGENCY_RECORD.md) defines
  the agency's documents, from vehicle sheets and mission plans to program and
  financial records. The operator decided its scope on 2026-11-14, and its methodology
  is to hold up to professional scrutiny. Formats wait for the blog articles.
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

1. Run the remaining Phase 0 experiments of
   [`HARNESS_DESIGN.md`](../architecture/HARNESS_DESIGN.md) in the sandbox save. They
   need a vessel in flight, so a small hand-written craft file comes first. The Python
   client 0.7.0 is in `tmp/krpc-venv`, and the probes are in `tmp/`, both local scratch.
2. Define the first milestone in `docs/roadmap/`, which the plan sets as Phase 0, and
   the record crate that the bridge repository waits on.

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
- **Bridge repository, 2026-11-14.** `sgeos/ikria-bridge`, public, GPL-3.0-only, seeded
  by this line at the operator's instruction and checked out at
  `~/projects/keleusma/ikria-bridge`, outside this line's sandbox, so writing there
  needs escalation.
- **Saves, 2026-11-14.** The operator created "Ikria Automated Space Agency", a career on
  Hard, and "Ikria Simulated Research and Dev", a sandbox on Hard whose name the game
  shortened. The older "Rookie" and "Sandbox" saves are left untouched.

## Validity

```sh
git status --short          # clean
git log --oneline -5        # the newest commit should describe this state
./scripts/check.sh          # exits zero
```

Report any check not run, and any commit after the one that last rewrote this file.

## Mailbox

None assigned. The blog line does not use the Keleusma interline mailbox, so this line
messages its running Claude Code session. On 2026-11-14 it asked for articles on
incident reports, drafted as A391 and unpublished, and on vehicle specification sheets
and mission documents, and on program and financial management. The
templates follow the published articles. Before writing the first of each document,
ask the operator whether its article exists.
