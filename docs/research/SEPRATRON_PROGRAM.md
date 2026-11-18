# The Sepratron Research Program

> **Navigation**: [Research](README.md) | [Documentation Root](../README.md)

**Status.** Program run on 2026-11-18 in the verification save "Ikria Physical
Verification", Kerbal Space Program 1.12.5 with kRPC 0.7.0. The procedure is
[TP-0002](../../record/verification/TP-0002.md), the measured results are
[VR-0011](../../record/verification/VR-0011.md), and the scripts that reproduce every
run are in [`tools/sepratron/`](../../tools/sepratron/README.md). The operator asked for
the program on 2026-11-18 and named its use cases. Where this document recommends a
design rule, the rule is a proposal until the operator adopts it.

**Audience.** Future sessions of this project, and anyone designing with small solid
motors in KSP1. Each claim is marked as measured, *(files)* for the game's own
configuration files, *(web)* for community sources, or as an inference.

**The part.** The operator's "small SRBs" are the Sepratron I, part `sepMotor1`, in
Precision Propulsion at 160 science. Earlier campaign text named the FM1 Mite first for
low-gravity ascent. The Sepratron is the intended part.

## The motor *(files)*

| Property | Value |
|---|---|
| Mass, loaded and empty | 72.5 kg and 12.5 kg |
| Solid fuel | 8 units, 60 kg at 7.5 kg per unit |
| Thrust in vacuum and at sea level | 18.0 kN and 13.8 kN |
| Specific impulse in vacuum and at sea level | 154 s and 118 s |
| Burn time at full thrust | 5.03 s |
| Impulse in vacuum and at sea level | 90.6 kN·s and 69.4 kN·s |
| Cost and entry cost | 75 and 1,100 funds |
| Mounting | Surface attachment only, 0.538 m long |

Its neighbours among solid motors are the FM1 Mite at 12.5 kN for 49 s and the F3S0
Shrimp at 30 kN for 47 s, both slow burners, the Launch Escape System at 750 kN for
0.53 s, and the FL-C1000 tank, whose built-in separation motors give 32 kN for 1.4 s.

**Measured to match.** In vacuum, two sepratrons changed a 892.5 kg stage's velocity by
218.09 m/s, against 218.07 m/s from the rocket equation with the masses kRPC reported.
Every other burn in the program agreed with the same equation within its measurement
error. The motor can be designed with by calculation.

## Tuning a fixed burn

A sepratron cannot be throttled or stopped once lit. Two settings in the editor still
shape its burn, and both were confirmed in flight.

- **Fuel load sets the impulse.** The motor burns at a fixed rate, so a reduced load
  gives a shorter burn and a proportionally smaller impulse. Loads of 0.197, 0.22, 0.229,
  1.59 and 1.60 units all burned exactly as sized.
- **The thrust limiter sets the burn time.** It scales thrust and fuel flow together, so
  the impulse stays the same and the burn lengthens. kRPC sets the limiter of a solid
  motor through `Engine.thrust_limit`, which a Flea reported as 0.25 when set.
- **Size by mass, not by count.** The rocket equation gives the fuel for a target velocity
  change, `m0 (1 - exp(-dv / (Isp g0)))`. Two sizing errors in this program both came from
  using the wrong initial mass, never from the motor. `tools/sepratron/sizing.py` holds
  the calculation.

**Stock craft carry tweaks.** Stock craft files store each part's editor settings. The
program's first articles, copied from stock craft, inherited sepratrons at half fuel and
half thrust, and a parachute set to open at 50 m. Any craft built from another craft's
parts should reset each part to its defaults before use.

## Use case 1, separation

**Practice** *(web)*. Sepratrons go on the spent part, angled so their exhaust misses
everything else, and fire with or just after the decoupler. Community advice ranges
from two to six per booster, often at a reduced thrust limit. A reported pitfall is
motors on a detaching part failing to light when they share a stage with the decoupler.

**Measured.** A TR-18A decoupler alone gave 11.1 to 11.3 m/s of relative speed between a
100 kg probe and its 892.5 kg stage in orbit. Two retro sepratrons on the stage then
carried it 1.7 km away in 10 s and 2.8 km in 15 s. The program always decoupled first and
fired second, which avoided the staging pitfall.

**Design rules.**
- **Clear the other vessel.** A separation motor must clear every part that leaves with
  the other vessel, by its full 0.538 m length. An abort pod whose motors reached 0.05 m
  into the decoupler, which stays with the booster, was destroyed at ignition
  ([I-0003](../../record/incidents/I-0003.md)). kRPC's `Part.bounding_box` checks this on
  the pad without firing anything, which `tools/sepratron/diag_overlap.py` does.
- **The decoupler stays with the lower part.** The TR-18A left with the stage below it in
  every run, so motors meant to push that stage away must clear it.
- **Fire after decoupling.** Command the decoupler, then the motors.

## Use case 2, deorbiting

**Practice** *(web)*. Retro-facing sepratrons on a final stage fire at separation, so the
stage both clears the payload and falls.

**Measured.** Retro sepratrons on a spent stage in a 100 km circular orbit, fired after
separation with the stack pointing prograde, cut the stage's periapsis to −35 km with one
motor's impulse and to −119 km with two.

**Design rules.**
- **One motor's impulse is ample at low orbit.** From 100 km a periapsis of 30 km needs
  about 60 m/s *(inference)*. A full sepratron gives a one-tonne stage about 90 m/s, so a
  reduced load usually suffices.
- **The stage burns along its separation attitude.** A spent stage has no control, so the
  carrier points it before release. Pointing prograde makes a retro mounting fire
  retrograde.
- **Keep the burning vessel active.** In orbit the game puts a vessel that is not active on
  rails 350 m from the active vessel, and a vessel on rails produces no thrust. A
  stage still burning beyond that range loses the rest of its burn unless it is the
  active vessel.

## Use case 3, abort and crew escape

**Practice** *(web)*. Sepratrons on the capsule, bound with the decoupler and parachute to
the Abort action group. The main engines must stop first, or the stack drives through
the pod. The stock Launch Escape System serves larger craft.

**Measured.** A 1,136 kg Mk1 pod on four upright sepratrons left the pad at 3.9 g on
average, reached 204 m/s at burnout, 4.92 s after ignition, and an apex of 1,855 m. Two
runs agreed within 0.3 m on apex height. It touched down under its Mk16 at 5.0 m/s, 447 m
from the pad. In flight, with the Flea booster still burning at a quarter thrust, the same
pod escaped to 2,072 m and was recovered.

**The hazard of a solid booster.** A solid motor cannot be shut down. In both in-flight
aborts the burning booster first fell behind, then closed again to between 200 m and 354 m
of the coasting pod about 10 to 14 s after the abort, before veering away on its own. It
missed twice by chance, not by design. A full-thrust Flea under a pod accelerates at
about 14 g against the pod's 4 g, and would overrun it *(inference)*.

**Design rules.**
- **Outrun the booster for its whole burn, or leave its line.** Escape motors must give
  more acceleration than the booster for as long as the booster burns, or move the
  capsule sideways out of its path.
- **Expect pitch-over.** An uncontrolled pod on symmetric motors tipped 10 degrees during
  its burn and was horizontal at the apex. Plan the parachute for any attitude.
- **Arm the parachute by condition.** The same loop that watches the booster must also
  detect the apex and arm the parachute. A test whose booster watch ran on its own for
  45 s lost its pod with the parachute unarmed
  ([I-0005](../../record/incidents/I-0005.md)).
- **Keep the hatch clear.** A sepratron over the Mk1 pod's hatch stopped the kerbal from
  leaving. The pod's motors at 45-degree offsets left the hatch free.

## Use case 4, rocket-assisted takeoff

**Practice** *(web)*. Groups of sepratrons fired at brake release, six to eight on heavy
aircraft, mounted near the centre of mass and in line with the engines.

**Measured.** A stock Aeris 3A, flown uncrewed through an added OKTO probe, with four
sepratrons on the upper flanks of its fuselage in line with its jet, reached 40 m/s in
3.18 s and 64.9 m against 4.22 s and 82.7 m without them, despite 290 kg more mass. That
is about 25 percent less time and 22 percent less runway.

**Design rules.**
- **Assist where the jet is weakest.** The jet gave only about 50 kN at release, well
  below its rating at speed. Sepratrons supply thrust in the first seconds, which is
  where a jet lacks it *(inference)*.
- **Mount in line with the engine.** In the aircraft hangar a part's upright axis faces
  forward when the part carries the engine's rotation, a quarter turn about the x axis.

## Use case 5, short-burst braking

**Practice** *(web)*. Retro motors fired just before touchdown under parachutes, landers
that shed most of their velocity on paired sepratrons, and the editor's fuel slider to
set the impulse. No community source was found on capture braking with small solids.

**Measured.**
- **Orbital capture at Minmus.** A 1,345 kg lander on a hyperbolic approach, with an
  excess speed of 245 m/s and a 15 km periapsis, fired one pair retrograde at
  periapsis. Its speed fell by 141.15 m/s against 141.14 m/s from the rocket equation,
  and it was captured into a 15.0 by 146.4 km orbit.
- **Descent braking on the Mun.** The same lander, falling at 202.5 m/s, fired one pair
  pointing up and slowed to 69.9 m/s. It shed 132.60 m/s against 132.94 m/s predicted,
  the rocket equation less gravity over the burn.
- **Soft landing on Kerbin.** A Mk1 pod descending at 4.9 m/s under its Mk16 fired two
  sepratrons, each loaded with 0.22 units at a quarter thrust, and touched down at 2.87
  m/s against a 5.0 to 5.2 m/s baseline.

**Design rules.**
- **A pair sheds a known amount.** On an airless body the shed velocity is the rocket
  equation less gravity times burn time, accurate to a few tenths of a metre per second.
  The remainder is left to a throttleable engine or a second, timed pair.
- **Under a parachute, burn short and hard, fired low.** Parachute drag falls with the
  square of speed, so as the retro slows the pod, gravity takes back what the parachute
  stops carrying. The quarter-thrust burn lost about half its impulse that way. Full
  thrust burns the same load in about 0.14 s and would lose far less *(inference,
  untested)*.
- **Calibrate the firing height.** The pod first touched the ground at about 0.68 m of
  radar altitude, not the 0.49 m it reads at rest, and ignition lagged about 0.04 s.
  Firing at 2.17 m left too little travel, and firing at 2.6 m completed the burn.

## Use case 6, ascent from a low-gravity body

**Practice** *(web)*. Community budgets give about 180 m/s to reach orbit from Minmus. No
source was found describing an ascent on sepratrons alone or a seat lifted by them.

**Measured.** A lander on Minmus left its 892.5 kg descent stage on the surface and lifted
a 245 kg ascent stage, an OKTO probe, an empty FL-T100 tank, a battery and four sized
sepratrons, into an 11.6 by 12.2 km orbit. The whole ascent burned about 3.7 units of
solid fuel, under half of one sepratron's load.
1. **The decoupler as a first stage.** With the descent stage held by the ground, the
   decoupler's ejection lifted the ascent stage about 11 m, long enough for its reaction
   wheel to pitch it 16 degrees above the eastern horizon.
2. **Burn 1,** one pair at 1.60 units each for about 1.0 s, raised the apoapsis to 11.6
   km, 3.7 percent above the planar model.
3. **Circularization,** one pair at 0.229 units each at the apoapsis, 23.6 m/s by the
   rocket equation from the measured masses.

**What failed first, and why it matters.**
- **Power for the coast.** On the first attempts the OKTO's 10 units of charge, drawn at
  0.020 per second, ran out during the coast of about 550 s, so the reaction wheel could not point
  the stage for circularization. A battery fixed it.
- **A fixed burn cannot absorb error.** An extra 1.2 degrees of pitch and the decoupler's
  hop raised one first burn's apoapsis 17 percent above prediction, and nothing could
  trim it. Near escape speed the apoapsis moves about half a kilometre per metre per
  second.

**Crewed variants** *(inference from the game's physics settings)*. A seated kerbal adds
45 kg, and the command-seat allowance, with parachute and jetpack, is 94 kg. A command
seat with a kerbal and two sepratrons would carry several hundred metres per second,
which is more than Minmus needs. Such an ascent was not flown in this program.

## Hybrid ascent, the operator's note of 2026-11-18

The operator noted that sepratrons can supply a fixed block of ascent velocity while a
throttleable engine does the rest, including later manoeuvres such as rendezvous or an
intercept. The program's findings support this directly *(inference)*.
- **Sepratrons for the bulk.** Their impulse is exact and cheap in mass. They carry the
  part of the ascent whose size is known in advance.
- **The engine for everything uncertain.** Attitude error, the decoupler's hop and terrain
  height all moved the first burn's result, and the circularization needed only about 24
  m/s. A small engine with a few tens of metres per second trims the orbit and then serves
  the rendezvous.
- **Sizing.** Size the sepratrons to fall short of the target by a margin the engine
  covers, never to exceed it, since excess solid impulse cannot be taken back.

## A reusable ejectable cockpit, the operator's note of 2026-11-18

The operator suggested a reusable ejectable cockpit, with variants, whose sepratrons are
arranged about the cockpit module's centre of mass, as the operator built it in KSP2. The
following is a design drawn from this program's findings. It has not been flown.
- **Thrust lines through the centre of mass.** Each motor's thrust line should pass
  through the ejected module's centre of mass, so that the escape does not tumble it. The
  symmetric abort pod still tipped 10 degrees, so the arrangement needs a flight check.
- **A sideways component when the booster is solid.** Since a solid booster follows its
  pod's line, a variant should cant its motors so the module leaves that line.
- **Clearances.** Motors clear of the hatch, clear of every part that stays behind, and
  out of each other's plumes.
- **Parachute and power.** The parachute at its defaults, armed by condition, and enough
  charge for any control the module carries.
- **Reuse.** The module is a subassembly with fixed geometry, so one qualification covers
  every vehicle that carries it unchanged.

## Limits and open questions

- **Single runs.** Most cases were flown once. The pad abort's apex repeated within 0.3 m,
  but repeatability was not established generally.
- **Uncrewed only.** No kerbal flew. Crew masses are from the game's physics file.
- **Placement by the debug service.** Orbits, approaches and starting states were set by
  the debug service, which the operator allows in the verification save only. Each use
  is logged in the run's data.
- **Not tested.** The ejectable cockpit, a crewed seat ascent, full-thrust soft landing,
  the editor slider's step size, and heat damage from exhaust on neighbouring parts.

## Sources

Community claims are from a web survey whose pages were mostly unreadable to automated
fetching, so most are search snippets and serve as leads, not quotations. Pages read in
full include [a Steam discussion on sepratrons](https://steamcommunity.com/app/220200/discussions/0/620713633858719867/),
[one on boosters striking the rocket](https://steamcommunity.com/app/220200/discussions/0/612823460270153275/),
[one on deorbiting space junk](https://steamcommunity.com/app/220200/discussions/0/3211505894122883417),
and [one on the Abort action group](https://steamcommunity.com/app/220200/discussions/0/864973124143527279/).
Leads from snippets include [the forum's report of motors not firing with a decoupler](https://forum.kerbalspaceprogram.com/topic/221396-engines-do-not-fire-when-triggered-in-the-same-action-group-as-a-decoupler/),
[its rocket-assisted takeoff thread](https://forum.kerbalspaceprogram.com/topic/73348-rato-rocket-assisted-take-off-give-your-spaceplanes-the-boost-they-deserve/)
and [the Solids to the Mun challenge](https://forum.kerbalspaceprogram.com/topic/177279-lowest-mass-challenge-solids-to-the-mun).
