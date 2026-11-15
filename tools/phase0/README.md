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
| `monolith_research_edit.py` | Researches the test rover's nodes by editing and loading a copy of the save | TP-0001 |

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
