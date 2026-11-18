"""Experiments B1 and B2, short-burst braking with full sepratrons, TP-0002.

Mode `capture`: the article IKR-SEP-A3C is placed by the debug service on a hyperbolic
approach to Minmus with a 15 km periapsis and an excess speed of about 245 m/s, points
retrograde under its own autopilot, and fires one opposed pair of its upright descent
motors centred on periapsis. The resulting orbit is compared with the prediction of a
near-circular orbit at 15 km.

Mode `descent`: the article is placed 4 km above a Mun site, falling at 200 m/s, points
straight up under its own autopilot and fires one opposed pair. The velocity shed is
compared with the rocket equation less gravity over the burn. A safety net, logged, sets
the article down if it is still fast near the ground, and the article is then returned
to Kerbin and recovered.

Arguments: the game's `saves` directory and the mode. Writes tmp/sepratron/b-<mode>.json."""
import math
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from flightlib import G0, Run  # noqa: E402

CRAFT = "IKR-SEP-A3C"
PAD = (-0.0972, -74.5577)
ISP_VAC = 154.0


def descent_pair(run, vessel):
    tanks = [p for p in vessel.parts.all if p.name == "fuelTankSmallFlat"
             and p.resources.amount("LiquidFuel") > 0]
    motors = [p for p in run.parts(vessel, "sepMotor1") if p.parent == tanks[0]]
    frame = vessel.reference_frame
    pos = {p: p.position(frame) for p in motors}
    best = None
    for i, a in enumerate(motors):
        for b in motors[i + 1:]:
            c = sum(x * y for x, y in zip(pos[a], pos[b]))
            if best is None or c < best[0]:
                best = (c, [a, b])
    return best[1]


def settle(run, ap, limit=1.5, timeout=40.0):
    held, end = None, run.sc.ut + timeout
    while run.sc.ut < end:
        if ap.error < limit:
            held = held or run.sc.ut
            if run.sc.ut - held >= 1.0:
                return ap.error
        else:
            held = None
        time.sleep(0.05)
    return ap.error


def capture(run, vessel):
    sc = run.sc
    minmus = sc.bodies["Minmus"]
    mu = minmus.gravitational_parameter
    rp = minmus.equatorial_radius + 15_000.0
    v_esc = math.sqrt(2 * mu / rp)
    v_p = 327.0
    v_inf = math.sqrt(v_p ** 2 - v_esc ** 2)
    a = -mu / v_inf ** 2
    e = 1 - rp / a
    n = math.sqrt(mu / abs(a) ** 3)
    m_anom = -n * 90.0
    run.debug("set_orbit", minmus, a, e, 0.0, 0.0, 0.0, m_anom, sc.ut, body="Minmus",
              sma_m=a, eccentricity=e, periapsis_alt_m=15_000.0, v_inf_mps=v_inf)
    time.sleep(3)
    vessel = sc.active_vessel
    o = vessel.orbit
    run.event("placed", periapsis_m=o.periapsis_altitude, eccentricity=o.eccentricity,
              time_to_periapsis_s=o.time_to_periapsis, mass_kg=vessel.mass)
    ap = vessel.auto_pilot
    ap.reference_frame = vessel.orbital_reference_frame
    ap.target_direction = (0.0, -1.0, 0.0)
    ap.engaged = True
    err = settle(run, ap)
    t_peri = sc.ut + vessel.orbit.time_to_periapsis
    run.event("pointing", error_deg=err, time_to_periapsis_s=t_peri - sc.ut)
    while sc.ut < t_peri - 2.5:
        time.sleep(0.02)
    v0 = vessel.orbit.speed
    m0 = vessel.mass
    run.fire(descent_pair(run, vessel))
    time.sleep(6.0)
    ap.engaged = False
    o = vessel.orbit
    m1 = vessel.mass
    run.log["analysis"] = {
        "dv_rocket_equation_mps": ISP_VAC * G0 * math.log(m0 / m1),
        "speed_before_mps": v0, "apoapsis_after_m": o.apoapsis_altitude,
        "periapsis_after_m": o.periapsis_altitude, "eccentricity_after": o.eccentricity,
        "captured": o.eccentricity < 1.0, "predicted": "near-circular at 15 km"}
    run.event("analysis", **run.log["analysis"])
    return vessel


def descent(run, vessel):
    sc = run.sc
    mun = sc.bodies["Mun"]
    lat, lon = 0.5, 30.0
    h = mun.surface_height(lat, lon)
    # Placement. set_landed with a 4 km height left an empty vessel record in two runs,
    # so the article is placed in flight, falling straight down at 200 m/s, and then
    # turned upright. Both calls are logged.
    run.debug("set_flight", mun, lat, lon, h + 4000.0, 200.0, 0.0, -90.0, 0.0, 0.0,
              body="Mun", altitude_m=h + 4000.0, speed_mps=200.0, pitch_deg=-90.0)
    time.sleep(0.5)
    vessel = sc.active_vessel
    run.debug("set_pitch_heading_roll", 90.0, 0.0, 0.0, vessel.surface_reference_frame, vessel,
              purpose="upright for braking")
    check = vessel.flight(mun.reference_frame)
    if len(vessel.parts.all) != 12 or not -215.0 < check.vertical_speed < -185.0:
        raise SystemExit(f"refusing to continue: {len(vessel.parts.all)} parts, "
                         f"{check.vertical_speed:.1f} m/s vertical")
    ap = vessel.auto_pilot
    ap.reference_frame = vessel.surface_reference_frame
    ap.target_pitch_and_heading(90.0, 0.0)
    ap.engaged = True
    err = settle(run, ap, timeout=3.0)
    flight = vessel.flight(mun.reference_frame)
    v0, m0, alt0 = flight.vertical_speed, vessel.mass, flight.surface_altitude
    t0 = sc.ut
    run.fire(descent_pair(run, vessel))
    time.sleep(5.5)
    flight = vessel.flight(mun.reference_frame)
    v1, m1, t1 = flight.vertical_speed, vessel.mass, sc.ut
    dv = ISP_VAC * G0 * math.log(m0 / m1)
    burn_s = 5.03
    run.log["analysis"] = {
        "terrain_m": h, "pointing_error_deg": err, "vspeed_before_mps": v0, "vspeed_after_mps": v1,
        "radar_before_m": alt0, "elapsed_s": t1 - t0, "dv_rocket_equation_mps": dv,
        "predicted_shed_mps": dv - mun.surface_gravity * burn_s,
        "measured_shed_mps": v1 - v0, "gravity_over_elapsed_mps": mun.surface_gravity * (t1 - t0)}
    run.event("analysis", **run.log["analysis"])
    run.save()
    # Safety net, then cleanup.
    end = sc.ut + 120.0
    while sc.ut < end:
        f = vessel.flight(mun.reference_frame)
        if str(vessel.situation) in ("VesselSituation.landed", "VesselSituation.splashed"):
            break
        if f.surface_altitude < 300.0 and f.vertical_speed < -15.0:
            f2 = vessel.flight(mun.reference_frame)
            run.event("safety_net", radar_m=f.surface_altitude, vspeed=f.vertical_speed)
            run.debug("set_landed", mun, f2.latitude, f2.longitude, 0.0, purpose="safety net")
            break
        time.sleep(0.1)
    ap.engaged = False
    return vessel


def main():
    saves, mode = sys.argv[1], sys.argv[2]
    run = Run(f"b-{mode}", saves)
    try:
        vessel = run.launch(CRAFT)
        vessel = capture(run, vessel) if mode == "capture" else descent(run, vessel)
        run.save()
        run.debug("set_landed", run.sc.bodies["Kerbin"], PAD[0], PAD[1], 0.0, purpose="cleanup")
        time.sleep(4)
        try:
            run.sc.active_vessel.recover()
            run.event("recovered")
        except Exception as error:
            run.event("not_recovered", error=str(error).splitlines()[0])
    finally:
        run.save()


if __name__ == "__main__":
    main()
