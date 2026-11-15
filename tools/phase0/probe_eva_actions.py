"""Phase 0, experiment 4, actions. Sandbox only. Runs the EVA report and the surface
sample, tries the flag, boards and recovers. Writes tmp/probe_eva_actions.json."""
import json
import time

import krpc

OUT = {}
conn = krpc.connect(name="ikria-probe-eva-actions", address="127.0.0.1")
sc = conn.space_center
assert sc.game_mode == sc.GameMode.sandbox, "refusing to run outside the sandbox"
kerbal = sc.active_vessel
assert str(kerbal.type).endswith("eva"), "active vessel is not a kerbal"
part = kerbal.parts.all[0]
OUT["situation"] = str(kerbal.situation)
try:
    OUT["biome"] = kerbal.biome
except Exception as ex:
    OUT["biome"] = "ERROR " + str(ex).splitlines()[0]


def exp(name):
    return next(e for e in part.experiments if e.name == name)


for name in ("evaReport", "surfaceSample"):
    e = exp(name)
    try:
        e.run()
        time.sleep(3)
        OUT[name] = {"has_data": e.has_data,
                     "data": [{"science_value": d.science_value, "transmit_value": d.transmit_value,
                               "data_amount": d.data_amount} for d in e.data]}
        try:
            OUT[name]["subject"] = {"title": e.science_subject.title, "science": e.science_subject.science,
                                    "cap": e.science_subject.science_cap,
                                    "value": e.science_subject.scientific_value}
        except Exception as ex:
            OUT[name]["subject"] = "ERROR " + str(ex).splitlines()[0]
    except Exception as ex:
        OUT[name] = "ERROR " + type(ex).__name__ + ": " + str(ex).splitlines()[0]

eva = next(m for m in part.modules if m.name == "KerbalEVA")
flag_events = [ev for ev in eva.event_list if "flag" in ev.gui_name.lower()]
OUT["flag_events"] = [{"name": ev.name, "gui": ev.gui_name, "active": ev.active, "visible": ev.visible} for ev in flag_events]
before = {v.name for v in sc.vessels}
for ev in flag_events:
    try:
        ev.trigger()
        time.sleep(5)
        OUT["flag_trigger"] = "triggered"
    except Exception as ex:
        OUT["flag_trigger"] = "ERROR " + type(ex).__name__ + ": " + str(ex).splitlines()[0]
after = {v.name: str(v.type) for v in sc.vessels}
OUT["new_vessels_after_flag"] = {n: t for n, t in after.items() if n not in before}
print(json.dumps(OUT, indent=1))

# Board the pod again, carrying the data, then recover.
try:
    kerbal = sc.active_vessel
    kerbal.control.board()
    time.sleep(4)
    OUT["boarded"] = True
except Exception as ex:
    OUT["boarded"] = "ERROR " + type(ex).__name__ + ": " + str(ex).splitlines()[0]
try:
    pod = sc.active_vessel
    OUT["after_board_active"] = pod.name + " " + str(pod.type)
    stored = []
    for p in pod.parts.all:
        for e in p.experiments:
            if e.has_data:
                stored.append(e.name)
    OUT["pod_experiments_with_data"] = stored
except Exception as ex:
    OUT["after_board_active"] = "ERROR " + str(ex).splitlines()[0]
print("boarded:", OUT["boarded"], OUT.get("after_board_active"))
with open("tmp/probe_eva_actions.json", "w") as f:
    json.dump(OUT, f, indent=1)
conn.close()
