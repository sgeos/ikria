# IKR-TS-001, starting-parts hop test article

> **Navigation**: [Vehicles](README.md) | [Verification](../verification/VR-0009.md)

**Status.** Provisional specification by Codex, 2026-11-17. Revisions A and B are
verification articles for E0. Neither is cleared for a crewed or career mission.
The operator authorized provisional record sections pending the blog articles.

## Purpose and configuration

Exercise the first hop and parachute recovery using only parts already purchased in
"Ikria Automated Space Agency". The observed career has the start node, zero science,
10,000 funds and initial facility levels. Its file was read without modification.

The local craft is derived from `IkriaT0`, a copy of the stock Jumping Flea. A command
pod sits above a Flea motor, with a nose parachute, two Mystery Goo units and three
fins. There is no decoupler. The whole article returns under one parachute. Eight
parts and the measured pad masses fit the planned 30-part and 18-tonne initial limits.
The game accepted crewed placement on the pad.

| Part | Quantity | Technology | Purchased in the career |
|---|---|---|---|
| Mk1 command pod | 1 | Start | Yes |
| Mk16 parachute | 1 | Start | Yes |
| RT-5 Flea motor | 1 | Start | Yes |
| Mystery Goo | 2 | Start | Yes |
| Basic fin | 3 | Start | Yes |

Revision A carries 70 solid-fuel units at a 35 percent thrust setting. Revision B
carries 35 units at 20 percent. Other template settings remain the same. The generator
refuses a different part inventory, unexpected source settings, unpurchased parts, or
an existing destination with different contents. Craft files remain local game assets.
Their hashes and the source hash are in the verification data.

## Performance and mass

Measured flight results are in [VR-0009](../verification/VR-0009.md). They are observations
from one flight per revision, not predicted performance envelopes. The full pod's pad
masses were 2,045.0 kg and 1,782.5 kg. After the crew member left and was recovered, the
empty flight masses were 1,951.0 kg and 1,688.5 kg. The dry mass reported by kRPC also
changed. The cause and contents of that difference need accounting before a crewed
qualification, and these empty flights do not establish crewed performance.

No delta-v budget, priced procurement sheet or structural margin is claimed yet.
The recorded difficulty groups matched. No debug-service setter was called. Relevant
debug flags read false after the flights but were not sampled before them.

## Control and qualification

The empty capsule reports no control authority and no communication link. The probe
used part-level engine activation and parachute deployment despite those states.
This is an expressly bounded verification experiment. Those calls cannot be treated
as career-valid command authority under R2 and R3.

The Python probe performs no active attitude control. It ignites the motor, observes
burnout and descent, and deploys the parachute when descending safely below 1,500 m
above terrain. It records the returned game safety state and each command. A solid
motor cannot provide a commanded shutdown in this profile.

A Keleusma development program now flies this profile through the Rust bridge, as
[VR-0010](../verification/VR-0010.md) records. A qualified career program is not
established. Broader phase-specific abort qualification,
repeatability, crewed mass and loading, a chosen landing footprint with measured
error, and a campaign-valid command path remain required. These tests do not complete
T12. The operator authorized rehearsal in the verification save on 2026-11-17.

## Flight history and disposal

Two uncrewed mechanics flights are documented in VR-0009. VR-0010 adds one lost
article, an intact landing with a software fault, and completed normal and coast-abort
flights. The latter three articles were recovered with all eight parts. There are no
career flights.
The intended disposal is recovery of all eight parts together, with no separated
stage or planned debris. Jebediah was used only to place each article on the pad and
was recovered before ignition.

## Historical comparison

The article is a small solid-motor sounding-rocket model with parachute recovery.
A sourced comparison to historical vehicles remains unfinished. The present record
makes no claim about real-world buildability or qualification.
