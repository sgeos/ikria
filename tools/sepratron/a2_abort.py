"""Experiment A2, abort and crew-module escape, TP-0002.

Mode `pad`: the article, IKR-SEP-A2D since revision D, stands on the pad with its Flea unlit. The pod
decouples and its four upright sepratrons fire together, the pad-abort case. The
parachute is armed at the apex.

Mode `flight`: the Flea is lit with its thrust limiter set through kRPC, and after
`abort_after` seconds the pod decouples and its sepratrons fire while the Flea still
burns, the in-flight case. The Flea is limited so that, by analysis, the pod outruns it,
since a solid motor cannot be shut down and a full-thrust Flea would overtake the pod.

No kerbal flies. If the game seats one at launch, the kerbal goes on EVA and is
recovered before anything fires, and the pod flies uncrewed. Its sepratrons, decoupler
and parachute are commanded part by part, which needs no control source.

Arguments: the game's `saves` directory, the mode, then for `flight` the Flea's thrust
limit as a fraction and the abort time in seconds. Writes tmp/sepratron/<label>.json."""
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from flightlib import Run, norm  # noqa: E402

CRAFT = "IKR-SEP-A2D"
SAFETY_RADAR_M = 300.0
SAFETY_VSPEED = 15.0


def unload_crew(run, vessel):
    """Send every kerbal aboard on EVA and recover them, returning their names."""
    names = []
    for kerbal in list(vessel.crew):
        names.append(kerbal.name)
        eva = None
        try:
            for part in vessel.parts.all:
                if kerbal.name in [c.name for c in part.crew]:
                    break
            run.sc.active_vessel = vessel
            eva = kerbal.eva()
        except Exception as error:
            run.event("eva_failed", kerbal=kerbal.name, error=str(error).splitlines()[0])
            raise
        time.sleep(3)
        eva_vessel = run.sc.active_vessel
        eva_vessel.recover()
        time.sleep(3)
    run.sc.active_vessel = vessel
    time.sleep(2)
    return names


def surface_state(vessel, body):
    flight = vessel.flight(body.reference_frame)
    attitude = vessel.flight(vessel.surface_reference_frame)
    return {"alt_m": flight.mean_altitude, "radar_m": flight.surface_altitude,
            "vspeed": flight.vertical_speed, "speed": flight.speed,
            "pitch_deg": attitude.pitch, "heading_deg": attitude.heading,
            "lat": flight.latitude, "lon": flight.longitude, "situation": str(vessel.situation)}


def track(run, key, vessels, body, seconds, every=0.1, stop=None):
    rows = run.log["samples"].setdefault(key, [])
    end = run.sc.ut + seconds
    while run.sc.ut < end:
        row = {"ut": run.sc.ut}
        for name, v in vessels.items():
            try:
                row[name] = surface_state(v, body)
                row[name]["pos"] = list(v.position(body.reference_frame))
            except Exception as error:
                row[name] = {"absent": type(error).__name__}
        rows.append(row)
        if stop and stop(row):
            break
        time.sleep(every)
    return rows


def after_abort(run, mode, pod, booster, chute, body, seconds=420.0, guard_m=20.0):
    """One loop, every tick, from the abort to touchdown. It records the pod and the
    booster, guards the pod against a still-burning booster in flight mode, arms the
    parachute at the apex, and applies the safety net. A single loop is required: an
    earlier version guarded the booster in a separate fixed-length loop, during which
    nothing armed the parachute, and the pod was lost (TP-0002)."""
    frame = body.reference_frame
    rows = run.log["samples"].setdefault("after_abort", [])
    start = run.sc.ut
    apex_done = guard_used = False
    while run.sc.ut < start + seconds:
        now = run.sc.ut - start
        pod_state = surface_state(pod, body)
        row = {"t": now, "pod": pod_state}
        try:
            gap_vec = [a - b for a, b in zip(pod.position(frame), booster.position(frame))]
            gap = norm(gap_vec)
            rel = [a - b for a, b in zip(booster.velocity(frame), pod.velocity(frame))]
            row["gap_m"] = gap
            row["closing_mps"] = sum(g * c for g, c in zip(gap_vec, rel)) / gap
        except Exception:
            gap = None
        rows.append(row)
        if (mode == "flight" and not guard_used and gap is not None and gap < guard_m
                and row["closing_mps"] > 0 and now > 0.5):
            guard_used = True
            run.event("near_collision", gap_m=round(gap, 2), closing_mps=round(row["closing_mps"], 2))
            run.view(booster)
            away = [h - 1000.0 * g / gap for h, g in zip(booster.position(frame), gap_vec)]
            run.debug("set_position", tuple(away), frame, purpose="booster guard")
            run.view(pod)
        if not apex_done and now > 5.5 and pod_state["vspeed"] < 0:
            apex_done = True
            chute.deploy()
            run.event("apex_and_chute", chute_state=str(chute.state), **pod_state)
            run.save()
        if pod_state["situation"] in ("VesselSituation.landed", "VesselSituation.splashed"):
            break
        if pod_state["radar_m"] < SAFETY_RADAR_M and pod_state["vspeed"] < -SAFETY_VSPEED and now > 5.5:
            run.event("safety_net", chute_state=str(chute.state), **pod_state)
            run.debug("set_landed", body, pod_state["lat"], pod_state["lon"], 0.0, purpose="safety net")
            break
        time.sleep(0.05 if now < 8 else 0.2)
    time.sleep(3)
    run.event("chute_after_descent", chute_state=str(chute.state))


def summarize(run):
    rows = run.log["samples"]["after_abort"]
    gaps = [r["gap_m"] for r in rows if "gap_m" in r]
    burn = [r for r in rows if r["t"] <= 5.2]
    air = [r for r in rows if r["pod"]["situation"] == "VesselSituation.flying"]
    apex = max(rows, key=lambda r: r["pod"]["alt_m"])
    return {"pod_speed_at_burnout_mps": round(max(r["pod"]["speed"] for r in burn), 1),
            "apex_alt_m": round(apex["pod"]["alt_m"], 1), "apex_t_s": round(apex["t"], 1),
            "min_gap_after_1s_m": round(min((r["gap_m"] for r in rows if "gap_m" in r and r["t"] > 1), default=-1), 1),
            "max_gap_m": round(max(gaps), 1) if gaps else None,
            "last_airborne_vspeed_mps": round(air[-1]["pod"]["vspeed"], 2) if air else None}


def main():
    saves, mode = sys.argv[1], sys.argv[2]
    limit = float(sys.argv[3]) if mode == "flight" else None
    abort_after = float(sys.argv[4]) if mode == "flight" else 0.0
    label = f"a2d-abort-{mode}" + (f"-{int(limit * 100)}pct" if mode == "flight" else "")
    run = Run(label, saves)
    try:
        fly(run, mode, limit, abort_after)
    finally:
        run.save()


def fly(run, mode, limit, abort_after):
    sc = run.sc
    kerbin = sc.bodies["Kerbin"]
    vessel = run.launch(CRAFT)
    crew = unload_crew(run, vessel)
    run.event("crew_removed", names=crew, crew_now=len(vessel.crew))
    if len(vessel.crew) != 0:
        raise SystemExit("refusing to fly a crewed abort test")
    flea = run.parts(vessel, "solidBooster.sm.v2")[0]
    seps = run.parts(vessel, "sepMotor1")
    chute = run.parts(vessel, "parachuteSingle")[0].parachute
    decoupler = run.parts(vessel, "Decoupler.1")[0].decoupler
    pod = vessel
    if mode == "flight":
        flea.engine.thrust_limit = limit
        run.event("flea_limit", requested=limit, reported=flea.engine.thrust_limit)
        run.fire([flea])
        track(run, "boost", {"article": vessel}, kerbin, abort_after, every=0.05)
    booster = decoupler.decouple()
    run.fire(seps)
    run.event("abort", pod_mass_kg=pod.mass, booster_mass_kg=booster.mass,
              pod=surface_state(pod, kerbin))
    after_abort(run, mode, pod, booster, chute, kerbin)
    final = surface_state(pod, kerbin)
    run.event("landed", parts=len(pod.parts.all), **final)
    run.log["analysis"] = summarize(run)
    try:
        pod.recover()
        run.event("pod_recovered")
    except Exception as error:
        run.event("pod_not_recovered", error=str(error).splitlines()[0])


if __name__ == "__main__":
    main()
