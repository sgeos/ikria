"""Experiment A1, separation and deorbit, TP-0002.

The article IKR-SEP-A1 is placed by the debug service in a circular orbit of Kerbin at
100 km and pointed prograde, so that its FL-T100 spent stage trails. The decoupler fires
alone for half a second, which measures its own push. Then one opposed pair of the
stage's four retro sepratrons fires, which pushes the stage away from the probe and
retrograde. Positions, velocities and masses of both vessels are sampled in Kerbin's
non-rotating frame. The probe is then landed near the launch pad by the debug service and
recovered, so that the run leaves nothing in orbit but the deorbiting stage.

Argument: the game's `saves` directory. Writes tmp/sepratron/a1-separation.json."""
import itertools
import math
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from flightlib import G0, Run, dot, norm, sub  # noqa: E402

ISP_VAC = 154.0
SEP_FUEL_T = 0.06
PAD = (-0.0972, -74.5577)


def opposed_pair(run, vessel):
    seps = run.parts(vessel, "sepMotor1")
    frame = vessel.reference_frame
    pos = {p: p.position(frame) for p in seps}
    best = min(itertools.combinations(seps, 2),
               key=lambda ab: dot(pos[ab[0]], pos[ab[1]]) / (norm(pos[ab[0]]) * norm(pos[ab[1]])))
    return list(best)


def main():
    run = Run(sys.argv[2] if len(sys.argv) > 2 else "a1-separation", sys.argv[1])
    sc = run.sc
    kerbin = sc.bodies["Kerbin"]
    frame = kerbin.non_rotating_reference_frame
    probe = run.launch(sys.argv[3] if len(sys.argv) > 3 else "IKR-SEP-A1B")
    run.debug("set_circular_orbit", kerbin, 100_000.0, body="Kerbin", altitude_m=100_000.0)
    time.sleep(4)
    probe = sc.active_vessel
    run.debug("set_direction", (0.0, 1.0, 0.0), 0.0, probe.orbital_reference_frame, probe,
              direction="prograde")
    probe.control.sas = True
    time.sleep(2)
    o = probe.orbit
    run.event("placed", apoapsis_m=o.apoapsis_altitude, periapsis_m=o.periapsis_altitude,
              speed=o.speed, mass_kg=probe.mass)
    decoupler = run.parts(probe, "Decoupler.1")[0].decoupler
    stage = decoupler.decouple()
    run.event("decoupled", stage_mass_kg=stage.mass, probe_mass_kg=probe.mass)
    vessels = {"probe": probe, "stage": stage}
    run.sample("decoupler_only", vessels, frame, 0.5, every=0.02)
    pair = opposed_pair(run, stage)
    run.event("stage_parts_before", parts=[p.name for p in stage.parts.all])
    run.fire(pair)
    run.sample_engines("engines", pair, 6.0, every=0.1)
    run.event("stage_parts_after", parts=[p.name for p in stage.parts.all])
    run.sample("burn", vessels, frame, 1.0, every=0.05)
    run.sample("coast", vessels, frame, 10.0, every=0.5)
    so, po = stage.orbit, probe.orbit
    run.event("after", stage_periapsis_m=so.periapsis_altitude, stage_apoapsis_m=so.apoapsis_altitude,
              probe_periapsis_m=po.periapsis_altitude, probe_apoapsis_m=po.apoapsis_altitude,
              stage_mass_kg=stage.mass)

    # Analysis from the samples.
    d = run.log["samples"]["decoupler_only"]
    rel_v = norm(sub(d[-1]["stage"]["vel"], d[-1]["probe"]["vel"]))
    b = run.log["samples"]["burn"]
    # The probe coasts during the burn, so the change in the stage's velocity relative to
    # the probe is the burn's own velocity change, with gravity cancelled.
    v0 = sub(d[-1]["stage"]["vel"], d[-1]["probe"]["vel"])
    v1 = sub(b[-1]["stage"]["vel"], b[-1]["probe"]["vel"])
    m0, m1 = d[-1]["stage"]["mass"], b[-1]["stage"]["mass"]
    dv_measured = norm(sub(v1, v0))
    dv_theory = ISP_VAC * G0 * math.log(m0 / m1)
    prograde = d[-1]["probe"]["vel"]
    along = dot(sub(v1, v0), prograde) / norm(prograde)
    sep_at = {}
    for row in run.log["samples"]["burn"] + run.log["samples"]["coast"]:
        t = row["ut"] - d[0]["ut"]
        dist = norm(sub(row["stage"]["pos"], row["probe"]["pos"]))
        for mark in (1, 2, 5, 10, 15):
            if t >= mark and mark not in sep_at:
                sep_at[mark] = round(dist, 2)
    run.log["analysis"] = {
        "decoupler_relative_speed_mps": round(rel_v, 3),
        "stage_mass_before_after_kg": [round(m0, 1), round(m1, 1)],
        "stage_dv_measured_mps": round(dv_measured, 2),
        "stage_dv_rocket_equation_mps": round(dv_theory, 2),
        "stage_dv_along_velocity_mps": round(along, 2),
        "distance_after_decouple_m": sep_at,
    }
    run.event("analysis", **run.log["analysis"])

    # Clean up: land the probe beside the pad and recover it.
    sc.active_vessel = probe
    time.sleep(2)
    run.debug("set_landed", kerbin, PAD[0], PAD[1], 0.0, purpose="cleanup")
    time.sleep(3)
    try:
        probe.recover()
        run.event("probe_recovered")
    except Exception as error:
        run.event("probe_not_recovered", error=str(error).splitlines()[0])
    run.save()


if __name__ == "__main__":
    main()
