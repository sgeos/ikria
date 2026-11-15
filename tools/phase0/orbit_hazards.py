"""Read every moon's orbit and sphere of influence, read-only, and compute the band of
distances from its parent where an orbit can meet it.

A vessel orbiting a parent can enter a moon's sphere of influence when the vessel's
distance from the parent passes through the moon's range of distances, widened by the
moon's sphere of influence. That band is [a(1 - e) - SOI, a(1 + e) + SOI] for a moon with
semi-major axis a, eccentricity e and sphere of influence SOI. Inclination can carry an
orbit clear of the moon, but does not guarantee it, so the band is a conservative
keep-out zone. Writes tmp/orbit_hazards.json."""
import json
import math

import krpc

conn = krpc.connect(name="ikria-orbit-hazards", address="127.0.0.1")
sc = conn.space_center
rows = []
for b in sc.bodies.values():
    o = b.orbit
    if o is None:
        continue
    parent = o.body
    a, e = o.semi_major_axis, o.eccentricity
    soi = b.sphere_of_influence
    inner = a * (1 - e) - soi
    outer = a * (1 + e) + soi
    rows.append({
        "moon": b.name, "parent": parent.name,
        "parent_radius_km": parent.equatorial_radius / 1000,
        "parent_atmosphere_km": (parent.atmosphere_depth / 1000) if parent.has_atmosphere else 0.0,
        "parent_soi_km": parent.sphere_of_influence / 1000 if parent.name != "Sun" else None,
        "a_km": a / 1000, "e": e, "inclination_deg": math.degrees(o.inclination),
        "soi_km": soi / 1000,
        "band_altitude_km": [max(0.0, (inner - parent.equatorial_radius) / 1000),
                             (outer - parent.equatorial_radius) / 1000],
        "mass_kg": b.mass, "surface_gravity": b.surface_gravity,
    })
rows.sort(key=lambda r: (r["parent"], r["a_km"]))
json.dump(rows, open("tmp/orbit_hazards.json", "w"), indent=1)
for r in rows:
    lo, hi = r["band_altitude_km"]
    print(f"{r['parent']:7} {r['moon']:7} a {r['a_km']:>12,.0f} km  e {r['e']:.3f}  i {r['inclination_deg']:5.1f}  "
          f"SOI {r['soi_km']:>10,.0f} km  hazard band altitude {lo:>12,.0f} to {hi:>12,.0f} km")
conn.close()
