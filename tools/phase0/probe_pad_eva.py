"""Phase 0, experiments 3 and 4, in the sandbox save only.

Launches the craft IkriaT0 to the pad without firing anything, measures lockstep cycles,
sends the kerbal out, lists what the kerbal can do, tries the reports and any flag event,
boards again and recovers the vessel. Writes tmp/probe_pad_eva.json.
"""
import json
import time

import krpc

OUT = {}


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


conn = krpc.connect(name="ikria-probe-pad-eva", address="127.0.0.1")
k, sc = conn.krpc, conn.space_center
assert sc.game_mode == sc.GameMode.sandbox, "refusing to run outside the sandbox"

sc.launch_vessel(craft_directory="VAB", name="IkriaT0", launch_site="LaunchPad", recover=True)
assert wait_for(lambda: k.game_scene == k.GameScene.flight and sc.active_vessel is not None, 120), "no flight scene"
time.sleep(5)
v = sc.active_vessel
OUT["vessel"] = v.name
OUT["situation"] = str(v.situation)
OUT["crew"] = [c.name for c in v.crew]
OUT["parts"] = len(v.parts.all)
print("launched:", OUT["vessel"], OUT["situation"], OUT["crew"])

# Experiment 3. Lockstep cycles reading a telemetry set and writing controls.
ref = v.orbit.body.reference_frame
ctl = v.control
cycles, n = [], 100
try:
    k.hold_tick()
    for _ in range(n):
        t0 = time.perf_counter()
        ut = sc.ut
        pos = v.position(ref)
        vel = v.velocity(ref)
        rot = v.rotation(ref)
        mass = v.mass
        thrust = v.available_thrust
        flight = v.flight(ref)
        alt = flight.mean_altitude
        spd = flight.speed
        ctl.throttle = 0.0
        ctl.pitch = 0.0
        ctl.yaw = 0.0
        ctl.roll = 0.0
        k.next_tick()
        cycles.append(time.perf_counter() - t0)
finally:
    k.release_tick()
cycles_ms = sorted(c * 1000 for c in cycles)
OUT["lockstep_cycle_ms"] = {"median": cycles_ms[n // 2], "p95": cycles_ms[int(n * 0.95)], "max": cycles_ms[-1], "calls_per_cycle": 15}
print("lockstep cycle ms:", OUT["lockstep_cycle_ms"])

# Experiment 4. The kerbal outside the vessel.
crew = v.crew[0]
kerbal = crew.eva()
assert wait_for(lambda: sc.active_vessel.name != OUT["vessel"], 30), "no EVA vessel"
time.sleep(3)
kerbal = sc.active_vessel
OUT["eva_vessel"] = kerbal.name
OUT["eva_situation"] = str(kerbal.situation)
parts = []
for p in kerbal.parts.all:
    entry = {"name": p.name, "title": p.title, "modules": [], "experiments": []}
    for m in p.modules:
        entry["modules"].append({"name": m.name, "events": list(m.events), "actions": list(m.actions)})
    for e in p.experiments:
        entry["experiments"].append({"name": e.name, "title": e.title, "available": e.available,
                                     "has_data": e.has_data, "rerunnable": e.rerunnable})
    parts.append(entry)
OUT["eva_parts"] = parts
print(json.dumps(parts, indent=1)[:4000])

trials = {}
for p in kerbal.parts.all:
    for e in p.experiments:
        try:
            e.run()
            time.sleep(2)
            trials[e.name] = {"ran": True, "has_data": e.has_data,
                              "data": [{"value": d.science_value, "transmit": d.transmit_value} for d in e.data] if e.has_data else []}
        except Exception as ex:
            trials[e.name] = {"ran": False, "error": f"{type(ex).__name__}: {ex}"}
    for m in p.modules:
        for ev in m.events:
            if "flag" in ev.lower() or "sample" in ev.lower():
                try:
                    m.trigger_event(ev)
                    time.sleep(3)
                    trials["event:" + ev] = {"triggered": True}
                except Exception as ex:
                    trials["event:" + ev] = {"triggered": False, "error": f"{type(ex).__name__}: {ex}"}
OUT["eva_trials"] = trials
print("trials:", json.dumps(trials, indent=1))
OUT["vessels_after_trials"] = [x.name + " " + str(x.type) for x in sc.vessels]

# Board again and recover.
try:
    kerbal.control.board()
    time.sleep(3)
    OUT["boarded"] = True
except Exception as ex:
    OUT["boarded"] = f"{type(ex).__name__}: {ex}"
print("boarded:", OUT["boarded"])
with open("tmp/probe_pad_eva.json", "w") as f:
    json.dump(OUT, f, indent=1)
for x in sc.vessels:
    if x.situation in (sc.VesselSituation.pre_launch, sc.VesselSituation.landed) and x.name in (OUT["vessel"], OUT.get("eva_vessel", "")):
        pass
try:
    sc.active_vessel.recover()
    OUT["recovered"] = True
except Exception as ex:
    OUT["recovered"] = f"{type(ex).__name__}: {ex}"
print("recovered:", OUT["recovered"])
with open("tmp/probe_pad_eva.json", "w") as f:
    json.dump(OUT, f, indent=1)
conn.close()
