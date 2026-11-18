"""Phase 0, experiments 4 (flag off the pad), 9 (physics range) and 10 (two vessels).
Sandbox only. The pod flies uncrewed while its kerbal stays on the ground as the active
vessel. Argument: the physics range to set on the pod in metres, or 0 for the default.
Writes tmp/probe_range_<range>.json."""
import json
import math
import pathlib
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from launch_site import clear_crewed_launch_site  # noqa: E402

RANGE = float(sys.argv[1]) if len(sys.argv) > 1 else 0.0
OUT = {"pod_physics_range_set": RANGE}


def wait_for(pred, timeout, step=0.5):
    t0 = time.time()
    while time.time() - t0 < timeout:
        try:
            if pred():
                return True
        except Exception:
            pass
        time.sleep(step)
    return False


def dist(a, b):
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b)))


conn = krpc.connect(name="ikria-probe-range", address="127.0.0.1")
k, sc = conn.krpc, conn.space_center
assert sc.game_mode == sc.GameMode.sandbox, "refusing to run outside the sandbox"

# Rule R20. Recover crewed vessels at the launch site explicitly before any launch.
# Sandbox probes take no saves directory, so outside the flight scene this refuses.
clear_crewed_launch_site(conn, None,
                         lambda crew: print("recovered before launch:", crew, flush=True))
sc.launch_vessel(craft_directory="VAB", name="IkriaT0", launch_site="LaunchPad", recover=True)
assert wait_for(lambda: k.game_scene == k.GameScene.flight and sc.active_vessel is not None, 120)
time.sleep(5)
pod = sc.active_vessel
pod_name = pod.name
bill = pod.crew[0]
bill.eva()
assert wait_for(lambda: str(sc.active_vessel.type).endswith("eva"), 30)
time.sleep(3)
kerbal = sc.active_vessel
body_ref = kerbal.orbit.body.reference_frame
pod = next(v for v in sc.vessels if v.name == pod_name)

# Walk the kerbal off the pad, then try the flag.
start = kerbal.position(body_ref)
kerbal.control.forward = 1.0
t0 = time.time()
while time.time() - t0 < 25 and dist(kerbal.position(body_ref), start) < 60:
    time.sleep(0.25)
kerbal.control.forward = 0.0
time.sleep(2)
OUT["walked_m"] = dist(kerbal.position(body_ref), start)
OUT["kerbal_biome_after_walk"] = kerbal.biome
OUT["kerbal_situation_after_walk"] = str(kerbal.situation)
print("walked:", round(OUT["walked_m"], 1), OUT["kerbal_biome_after_walk"], OUT["kerbal_situation_after_walk"])
eva = next(m for m in kerbal.parts.all[0].modules if m.name == "KerbalEVA")
flag = next(ev for ev in eva.event_list if ev.name == "PlantFlag")
OUT["flag_active_off_pad"] = flag.active
before = {v.name for v in sc.vessels}
try:
    flag.trigger()
    time.sleep(8)
    OUT["flag_trigger"] = "triggered"
except Exception as ex:
    OUT["flag_trigger"] = "ERROR " + str(ex).splitlines()[0]
OUT["new_vessels_after_flag"] = [(v.name, str(v.type)) for v in sc.vessels if v.name not in before]
print("flag:", OUT["flag_active_off_pad"], OUT["flag_trigger"], OUT["new_vessels_after_flag"])

# Experiment 9. Fly the uncrewed pod on its Flea while the kerbal is active.
if RANGE > 0:
    pod.physics_range = RANGE
OUT["pod_physics_range_read"] = pod.physics_range
engines = pod.parts.engines
OUT["pod_engines"] = [e.part.title for e in engines]
for e in engines:
    e.active = True
OUT["ignited_by_part"] = [e.active for e in engines]
print("ignited by part:", OUT["pod_engines"], OUT["ignited_by_part"])
log, t0, gone_at = [], time.time(), None
while time.time() - t0 < 240:
    try:
        names = {v.name for v in sc.vessels}
        if pod_name not in names:
            gone_at = time.time() - t0
            break
        p = pod.position(body_ref)
        flight = pod.flight(body_ref)
        log.append({"t": round(time.time() - t0, 2), "ut": round(sc.ut, 2),
                    "dist_m": round(dist(p, kerbal.position(body_ref)), 1),
                    "alt_m": round(flight.mean_altitude, 1), "speed": round(flight.speed, 1),
                    "loaded": pod.loaded, "packed": pod.packed, "situation": str(pod.situation)})
        if len(log) > 20 and log[-1]["situation"].endswith(("landed", "splashed")):
            break
    except Exception as ex:
        log.append({"t": round(time.time() - t0, 2), "error": type(ex).__name__ + ": " + str(ex).splitlines()[0]})
        if len(log) > 5 and all("error" in e for e in log[-5:]):
            break
    time.sleep(0.25)
OUT["pod_log"] = log
OUT["pod_gone_after_s"] = gone_at
transitions = []
prev = None
for e in log:
    key = (e.get("loaded"), e.get("packed"), e.get("situation"))
    if key != prev:
        transitions.append(e)
        prev = key
OUT["transitions"] = transitions
print("transitions:", json.dumps(transitions, indent=1))
print("pod gone after s:", gone_at, " samples:", len(log))
print("max distance:", max((e.get("dist_m", 0) for e in log), default=None),
      "max altitude:", max((e.get("alt_m", 0) for e in log), default=None))
with open(f"tmp/probe_range_{int(RANGE)}.json", "w") as f:
    json.dump(OUT, f, indent=1)
conn.close()
