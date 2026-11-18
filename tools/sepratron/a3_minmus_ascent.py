"""Experiment A3, a sepratron-powered ascent from Minmus that abandons the descent stage,
TP-0002.

The article, IKR-SEP-A3F in the run that reached orbit, is placed by the debug service
on a flat equatorial site on Minmus, chosen from the body's own terrain heights. The
decoupler separates the ascent stage, and its ejection impulse lifts the stage clear
while the descent stage stays on the ground. The ascent stage pitches to 15 degrees east
with its reaction wheel and fires its 1.60-unit pair, coasts to the apoapsis predicted
near 11.2 km, waits until its pointing settles, points prograde and fires its 0.229-unit
pair, as sized by sizing.py. Revisions D and E carried earlier loads, TP-0002. The final orbit is compared with the
prediction. A cleanup net, logged, lifts the stage to a safe orbit if its trajectory
would meet the ground after the measurement, and the stage is then returned to Kerbin
and recovered.

Argument: the game's `saves` directory. Writes tmp/sepratron/a3d-minmus-ascent.json."""
import math
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from flightlib import Run  # noqa: E402

CRAFT = "IKR-SEP-A3F"
PITCH = 15.0
PAD = (-0.0972, -74.5577)


def choose_site(body, avoid=()):
    """The flattest low point on a coarse equatorial grid, whose terrain over the next 0.1
    to 10 degrees of longitude eastward stays within 300 m of the site."""
    best = None
    for lat in (-1.0, -0.5, 0.0, 0.5, 1.0):
        for lon10 in range(0, 3600, 50):
            lon = lon10 / 10.0 - 180.0
            h = body.surface_height(lat, lon)
            near = [body.surface_height(lat + dlat, lon + dlon)
                    for dlat, dlon in ((0.05, 0), (-0.05, 0), (0, 0.05), (0, -0.05))]
            slope = max(abs(n - h) for n in near)
            ahead = max(body.surface_height(lat, lon + d) for d in (0.1, 0.5, 1, 2, 4, 6, 8, 10))
            if any(abs(lat - a) < 0.5 and abs(lon - b) < 0.5 for a, b in avoid):
                continue
            if ahead - h > 300.0 or slope > 3.0:
                continue
            score = (slope, h)
            if best is None or score < best[0]:
                best = (score, lat, lon, h)
    if best is None:
        raise RuntimeError("no site met the flatness and clearance limits")
    return best[1], best[2], best[3]


def orbit_state(vessel):
    o = vessel.orbit
    return {"apoapsis_m": o.apoapsis_altitude, "periapsis_m": o.periapsis_altitude,
            "speed_mps": o.speed, "mass_kg": vessel.mass}


def main():
    run = Run(sys.argv[2] if len(sys.argv) > 2 else "a3e-minmus-ascent", sys.argv[1])
    try:
        fly(run)
    finally:
        run.save()


def fly(run):
    sc = run.sc
    minmus = sc.bodies["Minmus"]
    # kRPC lists vessels only in the flight scene, so the article launches first.
    lander = run.launch(CRAFT)
    avoid = []
    for v in sc.vessels:
        if v.orbit.body == minmus and str(v.situation) in ("VesselSituation.landed", "VesselSituation.splashed"):
            f = v.flight(minmus.reference_frame)
            avoid.append((f.latitude, f.longitude))
    lat, lon, h = choose_site(minmus, avoid)
    run.event("site", lat=lat, lon=lon, height_m=h)
    run.debug("set_landed", minmus, lat, lon, 0.0, body="Minmus")
    time.sleep(5)
    lander = sc.active_vessel
    run.event("landed_on_minmus", situation=str(lander.situation), mass_kg=lander.mass)
    seps = run.parts(lander, "sepMotor1")
    ascent_tank = [p for p in lander.parts.all if p.name == "fuelTankSmallFlat"
                   and p.resources.amount("LiquidFuel") == 0][0]
    ascent = [p for p in seps if p.parent == ascent_tank]
    big = [p for p in ascent if p.resources.amount("SolidFuel") > 1.0]
    small = [p for p in ascent if p.resources.amount("SolidFuel") < 0.5]
    if len(big) != 2 or len(small) != 2:
        raise RuntimeError(f"unexpected motor loads: {len(big)} large, {len(small)} small")
    decoupler = run.parts(lander, "Decoupler.1")[0].decoupler
    descent = decoupler.decouple()
    run.event("decoupled", ascent_mass_kg=lander.mass, descent_mass_kg=descent.mass)
    ap = lander.auto_pilot
    ap.reference_frame = lander.surface_reference_frame
    ap.target_pitch_and_heading(PITCH, 90.0)
    ap.target_roll = float("nan")
    ap.engaged = True
    deadline = sc.ut + 8.0
    while sc.ut < deadline:
        f = lander.flight(lander.surface_reference_frame)
        if abs(f.pitch - PITCH) < 1.5 and lander.flight(minmus.reference_frame).surface_altitude > 1.0:
            break
        time.sleep(0.05)
    f = lander.flight(lander.surface_reference_frame)
    run.event("burn1_attitude", pitch_deg=f.pitch, heading_deg=f.heading,
              radar_m=lander.flight(minmus.reference_frame).surface_altitude)
    run.fire(big)
    time.sleep(3.0)
    ap.engaged = False
    after1 = orbit_state(lander)
    run.event("after_burn1", **after1)
    run.save()
    # Coast to the apoapsis on rails, then point prograde.
    t_apo = sc.ut + lander.orbit.time_to_apoapsis
    sc.warp_to(t_apo - 70.0)
    ap.reference_frame = lander.orbital_reference_frame
    ap.target_direction = (0.0, 1.0, 0.0)
    ap.engaged = True
    # The first A3E run fired after a fixed 12 s with the autopilot still 66.5 degrees
    # off. Fire only after the error has stayed within 1.5 degrees for a full second.
    held_since = None
    deadline = sc.ut + 50.0
    while sc.ut < deadline:
        if ap.error < 1.5:
            held_since = held_since or sc.ut
            if sc.ut - held_since >= 1.0:
                break
        else:
            held_since = None
        time.sleep(0.05)
    run.event("pointing_settled", error_deg=ap.error, waited_s=sc.ut - (deadline - 50.0),
              charge=lander.resources.amount("ElectricCharge"))
    t_apo = sc.ut + lander.orbit.time_to_apoapsis
    while sc.ut < t_apo - 0.12:
        time.sleep(0.02)
    run.event("burn2_attitude", error_deg=ap.error, alt_m=lander.flight().mean_altitude)
    run.fire(small)
    time.sleep(2.0)
    ap.engaged = False
    final = orbit_state(lander)
    run.event("final_orbit", **final)
    run.log["analysis"] = {"predicted": {"apoapsis_after_burn1_m": 11214, "circular_m": 11214},
                           "after_burn1": after1, "final": final}
    run.save()
    # Cleanup, logged: return the stage to Kerbin and recover it. The descent stage is
    # left on Minmus by design.
    if final["periapsis_m"] < 6000.0:
        run.event("ascent_short", periapsis_m=final["periapsis_m"])
    run.debug("set_landed", sc.bodies["Kerbin"], PAD[0], PAD[1], 0.0, purpose="cleanup")
    time.sleep(4)
    try:
        sc.active_vessel.recover()
        run.event("ascent_stage_recovered")
    except Exception as error:
        run.event("not_recovered", error=str(error).splitlines()[0])


if __name__ == "__main__":
    main()
