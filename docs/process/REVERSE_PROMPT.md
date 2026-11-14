# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Plain KSP terms, multi-objective craft, and the physics range, 2026-11-14

**Done.**
- Returned the documents to the plain KSP words, such as "suborbital", "orbital
  elements", "satellite deployment" and the names of the technology nodes, at the
  operator's ruling not to talk around normal KSP terms.
- Recorded the operator's view that craft meeting several objectives are the goal, and
  that a lander left behind would need fitting to each contract's requirements.
- Ran Phase 0 experiment 9 in part, and started experiment 10, in the sandbox. Recorded
  the results in the design and the booster program.

**Decided.** By the operator on 2026-11-14, the plain use of KSP terms and the aim of
craft that meet several objectives.

**Verified.**
- A non-active pod in Kerbin's atmosphere stayed loaded and off rails out to 12.7 km, and
  was destroyed on impact rather than removed in flight.
- kRPC stages only the active vessel, but lights an engine on another through the part.
- kRPC's vessel list leaves out kerbals outside a vessel and asteroids, which the save
  file lists.
- `./scripts/check.sh`, with every file staged, and its scan of the full history both
  exit zero on this branch, on macOS.

**Not verified.**
- Planting a flag. The trials so far planted nothing, as the save file shows.
- Distances beyond 13 km, a raised physics range, and a held tick with two vessels.

**For the operator.** Bill Kerman is standing near the sandbox's launch pad, where kRPC
cannot reach him. Recovering him from the Tracking Station tidies the sandbox.
