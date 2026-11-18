"""Experiment B4, rocket-assisted takeoff, TP-0002.

The article IKR-SEP-B4N, a stock Aeris 3A with an OKTO probe for uncrewed control, or
IKR-SEP-B4R, the same with four sepratrons in line with its jet, takes off from the
runway. The jet runs at full throttle for 5 s against the brakes, the brakes release,
and in B4R the four sepratrons fire at release. The autopilot holds 8 degrees of pitch
on the runway heading throughout, and the roll distance, time and speed to the first
flying sample are recorded. The aircraft is then set back on the runway by the debug
service, logged, and recovered, so that neither run risks a crash. No kerbal flies.

Arguments: the game's `saves` directory and the article name."""
import math
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from a2_abort import unload_crew  # noqa: E402
from flightlib import Run  # noqa: E402

RUNWAY = (-0.0486, -74.7247)


def ground_distance(body, a, b):
    lat1, lon1, lat2, lon2 = (math.radians(x) for x in (*a, *b))
    h = (math.sin((lat2 - lat1) / 2) ** 2
         + math.cos(lat1) * math.cos(lat2) * math.sin((lon2 - lon1) / 2) ** 2)
    return 2 * body.equatorial_radius * math.asin(math.sqrt(h))


def main():
    saves, craft = sys.argv[1], sys.argv[2]
    run = Run(f"b-takeoff-{craft}", saves)
    try:
        fly(run, craft)
    finally:
        run.save()


def fly(run, craft):
    sc = run.sc
    kerbin = sc.bodies["Kerbin"]
    run.clear_crewed_launch_site()
    sc.launch_vessel(craft_directory="SPH", name=craft, launch_site="Runway", recover=True)
    deadline = time.time() + 90
    while time.time() < deadline:
        try:
            v = sc.active_vessel
            if v is not None and str(v.situation) == "VesselSituation.pre_launch":
                break
        except Exception:
            pass
        time.sleep(0.5)
    time.sleep(3)
    plane = sc.active_vessel
    run.event("launched", craft=craft, parts=len(plane.parts.all), mass_kg=plane.mass)
    crew = unload_crew(run, plane)
    run.event("crew_removed", names=crew, crew_now=len(plane.crew))
    if len(plane.crew) != 0:
        raise SystemExit("refusing to fly crewed")
    ctrl = plane.control
    ctrl.brakes = True
    ctrl.sas = True
    ctrl.throttle = 1.0
    for p in plane.parts.all:
        if p.engine is not None and p.name != "sepMotor1":
            p.engine.active = True
    ap = plane.auto_pilot
    ap.reference_frame = plane.surface_reference_frame
    ap.target_pitch_and_heading(8.0, 90.0)
    ap.engaged = True
    time.sleep(5.0)
    f = plane.flight(kerbin.reference_frame)
    start = (f.latitude, f.longitude)
    t0 = sc.ut
    ctrl.brakes = False
    seps = run.parts(plane, "sepMotor1")
    if seps:
        run.fire(seps)
    run.event("released", jet_thrust_kN=sum(p.engine.thrust for p in plane.parts.all
                                             if p.engine is not None and p.name != "sepMotor1") / 1000.0,
              sepratrons=len(seps), mass_kg=plane.mass)
    rows = run.log["samples"].setdefault("roll", [])
    lifted = None
    while sc.ut < t0 + 60.0:
        f = plane.flight(kerbin.reference_frame)
        row = {"t": sc.ut - t0, "dist_m": ground_distance(kerbin, start, (f.latitude, f.longitude)),
               "speed": f.speed, "situation": str(plane.situation), "radar_m": f.surface_altitude}
        rows.append(row)
        if row["situation"] == "VesselSituation.flying" and row["radar_m"] > 1.5:
            lifted = row
            break
        time.sleep(0.05)
    run.log["analysis"] = {"craft": craft, "liftoff": lifted,
                           "last": rows[-1] if rows else None}
    run.event("analysis", **run.log["analysis"])
    ap.engaged = False
    ctrl.throttle = 0.0
    run.debug("set_landed", kerbin, RUNWAY[0], RUNWAY[1], 0.0, purpose="cleanup")
    time.sleep(4)
    try:
        sc.active_vessel.recover()
        run.event("recovered")
    except Exception as error:
        run.event("not_recovered", error=str(error).splitlines()[0])


if __name__ == "__main__":
    main()
