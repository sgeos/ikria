# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Hazards, disposal, landing sites, vehicle codes and relays, 2026-11-14

**Done.**
- Computed every moon's orbital hazard band from the running game, read-only, and wrote
  [`HAZARDS.md`](../architecture/HAZARDS.md) with the surface and environment hazards.
  Tylo is the second pitfall body of Ike's kind, and the Jool system is the densest.
  Verification report VR-0008 documents the computation.
- Amended R13 so that spent stages burn off their remaining Δv and deorbit, with a
  transfer stage able to bleed a lander's velocity before separating.
- Added R14 on chosen landing sites and R15 on orbital hazards.
- Set vehicle codes of the form IKR-[type]-[number], with eleven type codes.
- Added the relay registry to the agency record. The campaign plan records all of these
  as its ninth set of rulings.

**Decided.** By the operator on 2026-11-14, the five rulings above. The type codes and
the hazard rules are proposed.

**Verified.** The bands come from the game's own orbits and spheres of influence.
`RUSTC_WRAPPER= ./scripts/check.sh` exits zero on this branch with every file staged, on
macOS.

**Not verified.**
- The surface hazards marked as recalled.
- Whether an inclined orbit inside a band meets its moon, which each mission checks.

**For the operator.**
- Whether the proposed vehicle type codes suit.
- The monolith test still waits on any open dialog in the game window being closed.
