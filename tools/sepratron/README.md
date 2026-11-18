# Sepratron Program Tools

> **Navigation**: [Procedure TP-0002](../../record/verification/TP-0002.md) | [Program](../../docs/research/SEPRATRON_PROGRAM.md)

The scripts behind the Sepratron research program, kept so that every run can be
repeated. They run only in the verification save, through the kRPC Python client
0.7.0, from the repository root, and write their logs to `tmp/sepratron/`. Each refuses
any other save. Debug calls go through `Run.debug`, which logs them.

| Script | What it does |
|---|---|
| `setup_program.py` | Checkpoints the save and unlocks the tree through the debug service |
| `purchase_edit.py` | Writes a save copy in which every researched node's parts are purchased |
| `craftgen.py` | Builds craft files from stock part blocks, reset to part defaults |
| `build_articles.py` | Writes every test article, refusing to change one that exists |
| `sizing.py` | Rocket-equation sizing and a planar ascent model for fixed burns |
| `flightlib.py` | Save guard, logged debug calls, launch with crew-safe site clearing, sampling |
| `diag_orientation.py` | Each sepratron's force direction, without firing |
| `diag_overlap.py` | Sepratrons overlapping parts other than their parent, without firing |
| `a1_separation.py` | Separation and deorbit in orbit |
| `a2_abort.py` | Pad and in-flight abort |
| `a3_minmus_ascent.py` | Sepratron ascent from Minmus, abandoning the descent stage |
| `b_braking.py` | Capture braking at Minmus and descent braking on the Mun |
| `b_softland.py` | Retro soft landing under a parachute |
| `b_takeoff.py` | Rocket-assisted aircraft takeoff |

**Running them.** Use `python -u -I`, so that output is not lost if a run is stopped, and
a time limit longer than the script's longest path. Stop touching the game while a run
is in progress. A Flight Results dialog after a lost vessel blocks the next launch until
it is closed.
