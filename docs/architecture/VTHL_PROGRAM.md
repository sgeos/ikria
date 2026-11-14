# Booster Program, Vertical Takeoff and Horizontal Landing

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Decided as a top priority by the operator on 2026-11-14. The design and the
development steps below are proposed. Nothing has been flown. Facts are marked as in
[`CAMPAIGN_PLAN.md`](CAMPAIGN_PLAN.md), with *(files)* for the installed game's data,
*(web)* for the official wiki, *(measured)* for Phase 0 results, and *(recall)* for
memory not yet checked.

## Requirement

**Decided.** The operator's words of 2026-11-14 are as follows. A first-stage booster
takes off vertically, pushes the stages above it out of the thick atmosphere, separates,
and lands horizontally on the runway. A landing may need the booster to turn around, or
to make a full trip around Kerbin, depending on the mission and other constraints.

## Why it comes first

- **Return on investment.** The program is to be run on return on investment. A vehicle
  recovered on the runway is refunded in full *(web)*. A booster carries most of a
  launch vehicle's engines and tanks, so each recovered booster returns most of the
  cost of a launch. Without recovery, a stage that falls outside the physics range is
  removed and its value lost *(web)*.
- **The early funds gap.** Under the science ruling of 2026-11-14, surplus science is not
  converted to funds before the whole tree is unlocked. The early game's funds must
  therefore come from contracts, world firsts and lower launch costs, and the booster
  attacks the last directly.
- **A demonstration of Keleusma.** Flying the booster home while the upper stage flies
  on needs two vessels under control at once, each with its own loop, which is what the
  harness was designed to do.

## Concept of operations

1. **Vertical ascent.** The booster lifts the stack from the launch pad and flies the
   ascent through the thick lower atmosphere.
2. **Separation.** At the planned altitude and speed, the booster releases the upper
   stage, which continues under its own loop. The separation point trades upper stage
   performance against the booster's return.
3. **Return.** The booster flies one of two return modes.
   - **Turnaround.** The booster turns back towards the Space Center, by a rocket burn
     against its motion or by gliding and turning, then flies back.
   - **Once around.** The booster continues around Kerbin and approaches the runway from
     the far side, when its speed and the mission's direction make turning back too
     costly.
4. **Approach and landing.** Energy management to the runway, flare, touchdown and
   rollout, then recovery from the runway.

The mission plan chooses the return mode from the separation state, the launch
direction and the booster's margins. Both modes are qualified in the sandbox under R4.

## Parts within reach

*(files)* Every part a first booster needs lies inside the first research window, which
ends with the nodes costing 90 science.

| Need | Parts and node | Entry cost |
|---|---|---|
| Ascent engines | Swivel, Basic Rocketry, 5. Reliant, General Rocketry, 20. Thud for radial mounting, Advanced Rocketry, 45. | 3,500, 3,200 and 3,500 |
| Wings and control | Swept Wings, Elevon 1 and Structural Wing Type C, Aviation, 45. Delta-Deluxe Winglet, Flight Control, 45. | 3,600, 3,200, 6,400 and 4,600 |
| Landing gear | LY-01 and LY-05, Aviation, 45. LY-10, Landing, 90. | 500, 1,500 and 3,800 |
| Powered flyback, if used | Juno, Aviation, 45. Wheesley, Aerodynamics, 90. Small Circular Intake, Aviation, 45. | 2,000, 4,000 and 3,000 |
| Control | Stayputnik, Basic Science, 45. OKTO, Electrics, 90. | 1,400 and 2,800 |
| Separation | TD-12, Engineering 101, 5. TT-38K radial decoupler, Stability, 18. | 1,600 and 1,200 |

The cheapest path to all of the nodes for a first booster costs 153 science, the end of
stage E2 of [`CAMPAIGN_SEQUENCE.md`](CAMPAIGN_SEQUENCE.md).

## Design drivers

- **Facility limits.** The launch pad's first level caps a vessel at 18 tonnes, and the
  assembly buildings' first levels at 30 parts *(web)*. A winged booster spends parts on
  wings, control surfaces and gear, so the 30-part limit is expected to bind before the
  mass limit *(inference)*.
- **Separation altitude and speed.** Higher and faster helps the upper stage but makes
  the return harder, and raises heating on reentry. The first boosters separate low and
  slow, and later ones push outward as the return is proven.
- **Return energy.** A turnaround spends propellant or altitude to reverse its motion. A
  once-around trip spends time and needs the booster to survive a full pass. A booster
  with jets can cruise back at low altitude, at the cost of their mass.
- **Stability across regimes.** The same airframe flies tail-first on ascent and nose-first
  on return. Its centre of mass shifts as propellant drains, and its centre of lift must
  suit both the ascent and the glide.

## Risks

1. **Physics range after separation.** *(recall)* The game puts a vessel on rails once it
   is far enough from the active one, and may remove a vessel that goes on rails while
   flying in the lower atmosphere. A booster flying home will soon be far from an upper
   stage climbing away. kRPC's `vessel.physics_range` raises one vessel's range, but it
   costs performance and accumulates positional error *(read by the admin
   session in the kRPC client's documentation)*. This is the program's main technical risk.
   Phase 0 tests it before any design work depends on it.
2. **Two loops at once.** Both vessels need a loop stepped every tick in lockstep. The
   harness design allows it, and Phase 0 measures its cost.
3. **The landing itself.** A runway landing under automated control is the hardest
   control task in the early game. It is developed in the aircraft program of stage E2
   first, on aircraft that take off and land on the runway, before any booster flies.
4. **Loss of a booster.** A booster lost on return costs its value, but no crew, since
   boosters fly uncrewed. A lost booster in the career is documented, and an incident
   report is written only if R11 requires one.

## Control routines

The booster flies a phase table of Keleusma programs, in the vehicle-control routine
pairs of charter item 10.

| Phase | Program's task | Exit |
|---|---|---|
| Ascent | Pitch programme and throttle through the lower atmosphere | Separation state reached |
| Separation | Release the upper stage and move clear of it | Clear by a set distance |
| Return | Turnaround or once-around, as the plan chose | On the approach path |
| Approach | Energy management to the runway threshold | At the flare height |
| Landing | Flare, touchdown, braking and rollout | Stopped on the runway |
| Abort | Make the vessel safe, such as a controlled descent away from the Space Center | Any phase, on a fault |

The upper stage's own programs run alongside from the separation onward.

## Development steps

Each step is qualified in the sandbox under R4 before it flies in the career.

| Step | Vehicle | Proves |
|---|---|---|
| V1 | The aircraft program's runway aircraft | Takeoff, approach, flare and rollout under Keleusma control |
| V2 | A booster airframe dropped from a carrier aircraft, or launched short of separation altitude | Glide, energy management and landing of the booster shape |
| V3 | A full booster with a dummy upper stage, separating low and slow, with a turnaround | Ascent, separation, two vessels at once, the return |
| V4 | A booster lifting a working upper stage on a flight short of orbit | The first booster that pays for itself in the career |
| V5 | A booster lifting an upper stage to orbit, with turnaround and once-around variants | The standard launcher of the early game |

Steps V1 to V3 fly in the sandbox only. Steps V4 and V5 fly in the career once
qualified.

## Phase 0 additions

These join the experiments of [`HARNESS_DESIGN.md`](HARNESS_DESIGN.md).
- **Physics range.** At what distance and altitude does the game put a separated booster
  on rails or remove it? Does raising its `physics_range` keep it flying, and at what
  cost to the tick?
- **Two vessels in lockstep.** The cost of one held tick with two vessels each read and
  commanded.
