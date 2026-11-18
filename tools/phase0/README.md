# Phase 0 Tools

> **Navigation**: [Verification reports](../../record/verification/README.md)

The scripts behind the Phase 0 reports, kept for review and for repeating the runs. They
are tools, not the harness, and they are written in Python because kRPC's Python client
was the quickest way to probe the game. The probes run only against the sandbox save,
and refuse any other game mode.

| Script | What it does | Report |
|---|---|---|
| `probe_connect.py` | Connects, reads the scene and times calls | VR-0001 |
| `probe_inventory_tick.py` | Reads every body and biome, and steps the held tick | VR-0001, VR-0005 |
| `probe_pad_eva.py` | Launches `IkriaT0`, times lockstep cycles and sends the kerbal outside | VR-0002, VR-0003 |
| `probe_eva_continue.py` | Lists a kerbal's modules, events and experiments | VR-0003 |
| `probe_eva_actions.py` | Takes a report and a sample, tries the flag and boards | VR-0003 |
| `probe_range.py` | Flies the empty pod while its kerbal is active | VR-0004 |
| `probe_range2.py` | Plants a flag, times two vessels, and flies `IkriaT1` | VR-0002, VR-0003, VR-0004 |
| `science_subjects.py` | Counts the science subjects and their value | VR-0005 |
| `inv_game.py`, `inv_early.py`, `inv_costs.py` | Read the game's data files for nodes, parts and costs | Planning documents |
| `stage_science.py` | Computes the early stages' science | `CAMPAIGN_SEQUENCE.md` |
| `orbit_hazards.py` | Computes each moon's hazard band, read-only | VR-0008 |
| `guard.py` | Identifies the loaded save, since kRPC does not report it | TP-0001 |
| `monolith_setup.py` | Sets up the Green Monolith test in the verification save | TP-0001 |
| `monolith_research_edit.py` | Researches the test rover's nodes by editing a copy of the save, which the operator loads | TP-0001 |
| `monolith_runway.py` | Puts a fresh test rover on the runway for the KerbNet scan | TP-0001 |

**Save names.** A save a script writes is prefixed with the acronym of the save game it
belongs to, so that the game's Load menu shows where it came from. The prefixes are `iasa-`
for "Ikria Automated Space Agency", `isrd-` for "Ikria Simulated Research and Dev" and
`ipv-` for "Ikria Physical Verification". The operator asked for this on 2026-11-15, since
an `ikria-` prefix does not say which save is meant. The one exception is the guard's
temporary save, which is written before the guard knows which save is loaded and is
deleted at once.

**Running them.** The probes need the kRPC Python client 0.7.0, for example in a virtual
environment, and the game with the sandbox save loaded. Run them from the repository
root, where they write their output to `tmp/`. The game-file readers take the path of the
game's `GameData` directory as their first argument.

## Early hop development

The operator authorized verification-save rehearsals on 2026-11-17. These tools
refuse other saves and do not clear career flights. Their starting-parts test articles
have no uncrewed control authority, so the flight probe's part-level commands are
verification instrumentation. No debug-service setter is used. See
[VR-0009](../../record/verification/VR-0009.md) for results and limits.

| Script | Purpose |
|---|---|
| `prepare_early_hop.py` | Generate local revision A or B from `IkriaT0`, checking the purchased part inventory |
| `setup_early_hop.py` | Place a crewed article on the pad, remove and recover its crew, or recover the landed empty article |
| `launch_site.py` | Before any launch, recover crewed vessels at the pad or runway, or refuse outside the flight scene, failing closed, as rule R20 requires |
| `early_hop.py` | Fly the empty article with a bounded Python probe, recording a landing only after a settle period |
| `summarize_early_hops.py` | Generate the comparison table from completed flight logs |

Use the existing Python environment with kRPC for live operations. The scripts take
the game's saves directory explicitly. Placement does not ignite the motor. Before it
launches, placement recovers every crewed vessel at the launch sites and refuses to
launch if any crew cannot be recovered or read. Every other script here that launches
does the same. Crew
removal must complete before invoking the flight probe. A timeout has an uncertain
outcome and requires inspecting the game before another operation. Save names use the
`ipv-` prefix. Output paths must be new, so previous evidence is preserved.

```sh
python -I -B prepare_early_hop.py "$SAVES_DIR" B
python -I -B setup_early_hop.py "$SAVES_DIR" place pad.json --craft IKR-TS-001B
python -I -B setup_early_hop.py "$SAVES_DIR" empty empty.json --craft IKR-TS-001B
python -I -B early_hop.py "$SAVES_DIR" flight.json --craft IKR-TS-001B
python -I -B setup_early_hop.py "$SAVES_DIR" recover recovery.json --craft IKR-TS-001B
```

The commands above name the script files from this directory. Supply output paths
under the repository's `tmp/` when running them locally. Each step must succeed and its
output be checked before the next step. The following offline tests run from the
repository root and are included in the local gate. They never connect to the game.

```sh
python3 -I -B -m unittest discover -s tools/phase0/tests
```
