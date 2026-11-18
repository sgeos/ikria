"""Experiment B3, a retro-rocket soft landing under a parachute on Kerbin, TP-0002.

The article IKR-SEP-A5 is placed by the debug service 1,500 m above the ground near the
Space Center, falling, and its parachute is armed. When the radar altitude falls to the
firing height, both reduced-load sepratrons fire. The touchdown speed is compared with
the 5.0 and 5.2 m/s measured under the same parachute without retro motors in A2D.

No kerbal flies: any seated at launch goes on EVA and is recovered first.

Argument: the game's `saves` directory. Writes tmp/sepratron/b-softland.json."""
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from a2_abort import surface_state, unload_crew  # noqa: E402
from flightlib import Run  # noqa: E402

CRAFT = "IKR-SEP-A5B"
# Calibrated from the first run: contact near 0.68 m of radar altitude, an ignition lag
# of about 0.04 s at 5.2 m/s, and a 1.68 m braking distance. The first run fired at
# 2.17 m and touched down at 3.88 m/s.
FIRE_RADAR_M = float(sys.argv[2]) if len(sys.argv) > 2 else 2.6
SITE = (-0.05, -74.45)


def main():
    run = Run(f"b-softland-{FIRE_RADAR_M:g}", sys.argv[1])
    try:
        fly(run)
    finally:
        run.save()


def fly(run):
    sc = run.sc
    kerbin = sc.bodies["Kerbin"]
    pod = run.launch(CRAFT)
    crew = unload_crew(run, pod)
    run.event("crew_removed", names=crew, crew_now=len(pod.crew))
    if len(pod.crew) != 0:
        raise SystemExit("refusing to fly crewed")
    h = kerbin.surface_height(*SITE)
    run.debug("set_flight", kerbin, SITE[0], SITE[1], h + 1500.0, 60.0, 90.0, -80.0, 0.0, 0.0,
              altitude_m=h + 1500.0, speed_mps=60.0)
    time.sleep(0.5)
    pod = sc.active_vessel
    run.parts(pod, "parachuteSingle")[0].parachute.deploy()
    seps = run.parts(pod, "sepMotor1")
    rows = run.log["samples"].setdefault("descent", [])
    fired = False
    end = sc.ut + 400.0
    while sc.ut < end:
        s = surface_state(pod, kerbin)
        rows.append({"ut": sc.ut, **s})
        if not fired and s["radar_m"] <= FIRE_RADAR_M:
            run.fire(seps)
            fired = True
            run.event("retro_fired", **s)
        if s["situation"] in ("VesselSituation.landed", "VesselSituation.splashed"):
            break
        time.sleep(0.01 if s["radar_m"] < 30 else 0.25)
    air = [r for r in rows if r["situation"] == "VesselSituation.flying"]
    run.log["analysis"] = {"last_airborne_vspeed_mps": air[-1]["vspeed"],
                           "vspeed_at_fire_mps": next((e["vspeed"] for e in run.log["events"]
                                                       if e["event"] == "retro_fired"), None),
                           "baseline_without_retro_mps": [5.0, 5.16], "fire_radar_m": FIRE_RADAR_M}
    run.event("analysis", **run.log["analysis"])
    time.sleep(3)
    run.event("landed", parts=len(pod.parts.all))
    pod.recover()
    run.event("recovered")


if __name__ == "__main__":
    main()
