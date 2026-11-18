"""Phase 0, experiments 4 (flag, kerbal clear of the ladder), 10 (two vessels in a held
tick) and 9 (physics range beyond 13 km). Sandbox only. The rocket flies uncrewed while
its kerbal stays on the ground as the active vessel.
Arguments: craft name, physics range in metres for the rocket (0 for the default).
Writes tmp/probe_range2_<range>.json."""
import json
import math
import pathlib
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from launch_site import clear_crewed_launch_site  # noqa: E402

CRAFT = sys.argv[1]
RANGE = float(sys.argv[2])
OUT = {"craft": CRAFT, "range_set": RANGE}


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


def err(ex):
    return type(ex).__name__ + ": " + str(ex).splitlines()[0]


conn = krpc.connect(name="ikria-probe-range2", address="127.0.0.1")
k, sc = conn.krpc, conn.space_center
assert sc.game_mode == sc.GameMode.sandbox, "refusing to run outside the sandbox"
# Rule R20. Recover crewed vessels at the launch site explicitly before any launch.
# Sandbox probes take no saves directory, so outside the flight scene this refuses.
clear_crewed_launch_site(conn, None,
                         lambda crew: print("recovered before launch:", crew, flush=True))
sc.launch_vessel(craft_directory="VAB", name=CRAFT, launch_site="LaunchPad", recover=True)
assert wait_for(lambda: k.game_scene == k.GameScene.flight and sc.active_vessel is not None, 120)
time.sleep(5)
rocket = sc.active_vessel
OUT["crew"] = [c.name for c in rocket.crew]
rocket.crew[0].eva()
assert wait_for(lambda: str(sc.active_vessel.type).endswith("eva"), 30)
time.sleep(3)
kerbal = sc.active_vessel
ref = kerbal.orbit.body.reference_frame

# Experiment 4: release the ladder, walk clear, plant a flag.
try:
    kerbal.control.release_ladder()
    time.sleep(2)
except Exception as ex:
    OUT["release_ladder"] = err(ex)
start = kerbal.position(ref)
kerbal.control.forward = 1.0
t0 = time.time()
while time.time() - t0 < 30 and dist(kerbal.position(ref), start) < 40:
    time.sleep(0.25)
kerbal.control.forward = 0.0
time.sleep(2)
OUT["walked_m"] = round(dist(kerbal.position(ref), start), 1)
OUT["kerbal_situation"] = str(kerbal.situation)
OUT["kerbal_biome"] = kerbal.biome
eva = next(m for m in kerbal.parts.all[0].modules if m.name == "KerbalEVA")
flag = next(ev for ev in eva.event_list if ev.name == "PlantFlag")
OUT["flag_event"] = {"gui": flag.gui_name, "active": flag.active}
try:
    flag.trigger()
    time.sleep(10)
    OUT["flag_trigger"] = "triggered"
except Exception as ex:
    OUT["flag_trigger"] = err(ex)
OUT["flag_event_after"] = next(ev.gui_name for ev in eva.event_list if ev.name == "PlantFlag")
print("flag:", OUT["walked_m"], OUT["kerbal_situation"], OUT["flag_event"], OUT["flag_trigger"], OUT["flag_event_after"])

# Experiment 10: a held tick with two vessels, each read and commanded.
cycles = []
try:
    k.hold_tick()
    for _ in range(50):
        t0 = time.perf_counter()
        for v in (kerbal, rocket):
            v.position(ref); v.velocity(ref); v.rotation(ref); v.mass
            f = v.flight(ref); f.mean_altitude; f.speed
            v.control.pitch = 0.0
        k.next_tick()
        cycles.append((time.perf_counter() - t0) * 1000)
finally:
    k.release_tick()
cycles.sort()
OUT["two_vessel_cycle_ms"] = {"median": round(cycles[25], 2), "p95": round(cycles[47], 2), "max": round(cycles[-1], 2), "calls_per_cycle": 16}
print("two-vessel held cycle ms:", OUT["two_vessel_cycle_ms"])

# Experiment 9: light the uncrewed rocket's first-stage engines through its parts.
if RANGE > 0:
    rocket.physics_range = RANGE
OUT["range_read"] = rocket.physics_range
try:
    rocket.control.throttle = 1.0
    OUT["throttle_set"] = True
except Exception as ex:
    OUT["throttle_set"] = err(ex)
lit = []
stage = max(e.part.stage for e in rocket.parts.engines)
for e in rocket.parts.engines:
    if e.part.stage == stage:
        try:
            e.active = True
            lit.append(e.part.title)
        except Exception as ex:
            lit.append(e.part.title + " " + err(ex))
OUT["lit"] = lit
print("lit:", lit, "throttle:", OUT["throttle_set"], "range read:", OUT["range_read"])
name = rocket.name
log, t0, gone = [], time.time(), None
while time.time() - t0 < 420:
    try:
        if name not in {v.name for v in sc.vessels}:
            gone = round(time.time() - t0, 1)
            break
        p = rocket.position(ref)
        f = rocket.flight(ref)
        log.append({"t": round(time.time() - t0, 1), "dist_m": round(dist(p, kerbal.position(ref))),
                    "alt_m": round(f.mean_altitude), "speed": round(f.speed, 1),
                    "loaded": rocket.loaded, "packed": rocket.packed, "sit": str(rocket.situation).split(".")[-1]})
        if len(log) > 40 and log[-1]["sit"] in ("landed", "splashed"):
            break
    except Exception as ex:
        log.append({"t": round(time.time() - t0, 1), "error": err(ex)})
        if len(log) > 6 and all("error" in e for e in log[-6:]):
            break
    time.sleep(0.5)
OUT["log"] = log
OUT["gone_after_s"] = gone
trans, prev = [], None
for e in log:
    key = (e.get("loaded"), e.get("packed"), e.get("sit"), "error" in e)
    if key != prev:
        trans.append(e)
        prev = key
OUT["transitions"] = trans
print("transitions:")
for e in trans:
    print("  ", e)
print("gone after s:", gone, "max dist:", max((e.get("dist_m", 0) for e in log), default=0),
      "max alt:", max((e.get("alt_m", 0) for e in log), default=0))
print("last:", log[-3:])
with open(f"tmp/probe_range2_{int(RANGE)}.json", "w") as fh:
    json.dump(OUT, fh, indent=1)
conn.close()
