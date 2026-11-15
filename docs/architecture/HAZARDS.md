# Hazards by Body

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Decided as a requirement by the operator on 2026-11-14, with the analysis
below. The operator noted that Ike can throw vessels around in orbit and cause accidents,
and asked that no other body be a similar pitfall. Orbital figures were computed from
the running game on 2026-11-14, as verification report
[VR-0008](../../record/verification/VR-0008.md) documents. Other hazards are marked
*(files)*, *(web)* or *(recall)* as elsewhere.

## Orbital hazards, the moons' bands

The game uses patched conics. A vessel orbiting a parent can enter a moon's sphere of
influence wherever the vessel's distance from the parent passes through the moon's range
of distances, widened by the moon's sphere of influence. That band is
`[a(1 - e) - SOI, a(1 + e) + SOI]`. An encounter can change a vessel's orbit sharply,
sending it into the moon, out of the system, or into the parent. Inclination can carry
an orbit clear of a moon but does not guarantee it, so each band is a keep-out zone
unless a mission plan shows the encounter is impossible or intended.

| Parent | Moon | Band, altitude above the parent | Note |
|---|---|---|---|
| Kerbin | Mun | 8,970 to 13,830 km | Relays and high orbits of Kerbin stay below or above it |
| Kerbin | Minmus | 44,153 to 48,647 km | Inclined 6 degrees, so many orbits pass above or below it |
| Duna | Ike | 1,734 to 4,026 km | **The operator's example.** Ike is large and close, so most high orbits of Duna, and many arrivals, meet it |
| Eve | Gilly | 13,349 to 48,251 km | Wide, because Gilly's orbit is eccentric and inclined, but its sphere of influence is only 126 km, so encounters are rare |
| Jool | Laythe | 17,460 to 24,908 km | |
| Jool | Vall | 34,746 to 39,558 km | |
| Jool | Tylo | 51,643 to 73,357 km | **A second Ike.** Tylo's sphere of influence is 10,857 km, and a capture at Jool often meets it |
| Jool | Bop | 91,081 to 153,919 km | Eccentric and inclined 15 degrees |
| Jool | Pol | 142,114 to 205,666 km | Eccentric |

**Findings.**
- **Ike and Tylo are the pitfall bodies** of the operator's kind. Both have spheres of
  influence large for their distance, and both sit on the paths that arrivals and
  relays most often use.
- **The Jool system is the densest.** Its five bands cover much of the space inside
  206,000 km, so any long-lived orbit or relay there needs an explicit encounter check.
- **Kerbin's synchronous orbit is clear.** At 2,863 km it sits well inside the Mun's
  band.
- An encounter can also be used deliberately, as a gravity assist. A mission plan that
  intends one says so.

**Rules.** *Proposed.*
1. Every mission plan lists the bands its trajectory crosses, and either shows no
   encounter can occur or plans the encounter.
2. Relays and stations are placed where a moon cannot perturb them unexpectedly, with a
   stated margin. The operator set out three options on 2026-11-14, and which one is
   feasible depends on the geometry of each network.
   - **Around the moon itself.** A relay orbiting Ike, for example, is inside Ike's
     sphere of influence by design, so Ike cannot perturb it by surprise.
   - **Very high around the parent,** above the outermost band and inside the parent's
     sphere of influence.
   - **Close around the parent,** below the innermost band, as low Duna orbits sit below
     Ike's.
3. The program-level tracking software checks each tracked vessel's predicted orbit
   against the bands, and warns before an unplanned encounter.

## Surface and environment hazards

| Body | Hazard | Source |
|---|---|---|
| Kerbin | Its oceans cover about 60 percent of the surface, so a landing off target is often a splashdown | *(web)* |
| Mun | Steep crater walls and slopes in most biomes | *(recall)* |
| Minmus | Very low gravity, 0.49 m/s², so a landing can bounce, and a kerbal's jump goes far. The flats are smooth and safe | *(files, recall)* |
| Gilly | Gravity of 0.05 m/s², so a vessel can drift off the surface, and a kerbal's jetpack exceeds escape speed | *(files)* |
| Eve | Gravity of 16.7 m/s² and a dense atmosphere make the return the hardest of the campaign. Landing high reduces it | *(files, web)* |
| Duna | A thin atmosphere, so parachutes need a large area and a powered final descent | *(recall)* |
| Moho | Close to the Sun and costly in Δv. The Mohole, near the north pole, is a deep hole that a landing must avoid | *(web)* |
| Jool | No surface. Descending far into the atmosphere destroys a vessel by pressure and heat | *(web)* |
| Laythe | Oceans with small islands, so a landing needs a precise target | *(web)* |
| Tylo | Gravity of 7.85 m/s² with no atmosphere, so every metre per second of descent and ascent is paid by engines | *(files)* |
| Bop and Pol | Low gravity with rough terrain | *(recall)* |
| Kerbol | Heat close to the Sun | *(recall)* |
| Any body | Defects of the game's physics, such as transitions between rails and physics, which R10 treats as game defects | *(recall)* |
