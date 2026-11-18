# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by a Claude line on 2026-11-18.

## The Sepratron research program, T18

**Done.** The program the operator asked for, in the verification save, with a writeup
for future sessions and outside readers,
[`SEPRATRON_PROGRAM.md`](../research/SEPRATRON_PROGRAM.md). Its procedure is TP-0002, its
evidence VR-0011, and its scripts are in `tools/sepratron/`.
- **The motor matches its numbers.** Every measured burn agreed with the rocket equation
  within its measurement error, as close as 0.01 percent.
- **All six use cases flew.** Separation and deorbit in orbit, pad and in-flight aborts,
  a rocket-assisted takeoff, capture braking at Minmus, descent braking on the Mun, a
  retro soft landing on Kerbin, and an ascent from Minmus to orbit on about 3.7 units of
  solid fuel that left the descent stage behind.
- **Findings with design force.** A burning solid booster returned toward an escaped pod
  in both in-flight aborts. A fixed burn cannot absorb pointing or launch errors, which
  supports the operator's hybrid ascent. Motors must clear the other vessel and the hatch.
  A coast needs power for pointing.
- **The operator's notes.** The hybrid ascent and the reusable ejectable cockpit are
  written up as designs. Neither has flown.
- **The campaign sequence** now names the Sepratron I as the operator's small solid motor.

**Losses.** Four uncrewed articles were lost, each to a fault in my test design or
software, I-0003 to I-0006. **Jebediah Kerman is Missing in the verification save,**
I-0007. A script stopped with him aboard a pod on the pad, and the next launch removed
the pod. The save does not respawn missing crew. The career save was not loaded.
Launches now recover crewed vessels explicitly first.

**Decided.** By the operator on 2026-11-18, the program and its use cases, and that the
small motors meant are the Sepratron I.

**Verified.** In game, uncrewed, with debug placement in the verification save. The gate
exits zero on this branch with every file staged, on macOS.

**Not verified.** Repeatability beyond the pad abort, crewed flight, the cockpit design, a
full-thrust soft landing and exhaust heating.

**For the operator.**
1. Whether to restore Jebediah by editing the verification save's roster, or leave the
   record as it stands.
2. Whether to adopt the program's design rules for the campaign, especially that escape
   motors must outrun a solid booster for its whole burn or leave its line.
