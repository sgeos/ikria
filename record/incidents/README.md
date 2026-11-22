# Incidents and Anomaly Log

> **Navigation**: [Record](../README.md)

Provisional reports use the contents required by campaign rule R11. Its instruction
allows reporting before the incident-format article is published. This table is the
R8 anomaly log for this development activity.

| Item | Observation | Disposition |
|---|---|---|
| A-0001 | JSON game-time mismatch during monitoring | Exact round trips enabled, 10,000-timestamp regression test |
| [I-0001](I-0001.md) | Uncrewed verification article lost with moving-frame velocity | Corrected bridge frames, subsequent deployment and landing observed |
| [I-0002](I-0002.md) | Touchdown bounce stopped the harness | Corrected phase transition and settled-contact detection, regression and live tests |
| [I-0003](I-0003.md) | Uncrewed abort pod destroyed at ignition, its motors inside the decoupler | Overlap check before flight, motors raised |
| [I-0004](I-0004.md) | Uncrewed abort pod lost, parachute inherited a 50 m opening height | Parts reset to their defaults, logs flushed and saved by phase, safety net |
| [I-0005](I-0005.md) | Uncrewed abort pod lost while a booster watch ran alone | One loop for every task, repeat flight recovered |
| [I-0006](I-0006.md) | Uncrewed lander destroyed by a velocity set in a moving frame | Placement by `set_flight`, plausibility check |
| [I-0007](I-0007.md) | Jebediah Kerman marked Missing in the verification save | Crewed vessels recovered before any launch, restoration for the operator |
| [I-0008](I-0008.md) | Empty article became Landed after crew removal, direct uncrewed placement timed out with a control warning | Open, monitor passed, armed checks deferred without changing policy |
