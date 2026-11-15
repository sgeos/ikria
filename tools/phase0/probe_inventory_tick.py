"""Phase 0, experiments 2 and 7, in the sandbox save only.

Experiment 7 reads every body and its biomes, which is read-only. Experiment 2 holds the
game's tick briefly, steps it with next_tick, measures game time per tick, and always
releases the hold. Writes tmp/inventory.json.
"""
import json
import time

import krpc

conn = krpc.connect(name="ikria-probe-inventory-tick", address="127.0.0.1")
k, sc = conn.krpc, conn.space_center
assert sc.game_mode == sc.GameMode.sandbox, "refusing to run outside the sandbox"

bodies = []
for b in sc.bodies.values():
    entry = {
        "name": b.name,
        "parent": b.orbit.body.name if b.orbit is not None else None,
        "radius_m": b.equatorial_radius,
        "surface_gravity": b.surface_gravity,
        "has_atmosphere": b.has_atmosphere,
        "atmosphere_depth_m": b.atmosphere_depth if b.has_atmosphere else 0.0,
        "has_atmospheric_oxygen": b.has_atmospheric_oxygen if b.has_atmosphere else False,
        "biomes": sorted(b.biomes),
        "flying_high_altitude_threshold": b.flying_high_altitude_threshold,
        "space_high_altitude_threshold": b.space_high_altitude_threshold,
        "sphere_of_influence_m": b.sphere_of_influence,
    }
    bodies.append(entry)
bodies.sort(key=lambda e: e["name"])
with open("tmp/inventory.json", "w") as f:
    json.dump(bodies, f, indent=1)
print("bodies:", len(bodies), "biomes in total:", sum(len(b["biomes"]) for b in bodies))
for b in bodies:
    print(f"  {b['name']:8} biomes {len(b['biomes']):3}  g {b['surface_gravity']:6.2f}  atm {b['has_atmosphere']}")

# Experiment 2: tick hold.
samples = []
try:
    k.hold_tick()
    t_wall = time.perf_counter()
    ut0 = sc.ut
    for _ in range(50):
        k.next_tick()
        samples.append(sc.ut)
    wall = time.perf_counter() - t_wall
finally:
    k.release_tick()
deltas = [b - a for a, b in zip([ut0] + samples[:-1], samples)]
nonzero = [d for d in deltas if d > 0]
print("tick hold: 50 next_tick cycles in", f"{wall*1000:.1f} ms wall,", f"{wall/50*1000:.2f} ms per cycle")
print("game time per tick, distinct values:", sorted(set(round(d, 6) for d in deltas)))
if nonzero:
    print("implied fixed update rate:", f"{1/(sum(nonzero)/len(nonzero)):.2f} per second of game time")
conn.close()
