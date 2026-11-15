"""Phase 0, experiment 4, continued from a kerbal already on EVA, sandbox only.
Writes tmp/probe_eva.json."""
import json
import time

import krpc

OUT = {}
conn = krpc.connect(name="ikria-probe-eva", address="127.0.0.1")
sc = conn.space_center
assert sc.game_mode == sc.GameMode.sandbox, "refusing to run outside the sandbox"
kerbal = sc.active_vessel
assert str(kerbal.type).endswith("eva"), f"active vessel is not a kerbal: {kerbal.type}"
OUT["eva_vessel"] = kerbal.name

def describe(obj):
    return {a: (lambda v: v if isinstance(v, (str, int, float, bool)) else str(v))(getattr(obj, a))
            for a in dir(obj) if not a.startswith("_") and not callable(getattr(type(obj), a, None)) or False}

parts = []
for p in kerbal.parts.all:
    entry = {"name": p.name, "title": p.title, "modules": [], "experiments": []}
    for m in p.modules:
        evs = []
        for ev in m.event_list:
            evs.append({a: str(getattr(ev, a)) for a in dir(ev) if not a.startswith("_") and a in ("name", "gui_name", "active", "guiName", "title")} or str(ev))
        entry["modules"].append({"name": m.name, "events": evs,
                                 "actions": [str(a) for a in m.action_list][:20],
                                 "event_attrs": [a for a in dir(m.event_list[0]) if not a.startswith("_")] if m.event_list else []})
    for e in p.experiments:
        rec = {}
        for a in ("name", "title", "available", "has_data", "rerunnable", "inoperable", "deployed"):
            try:
                rec[a] = getattr(e, a)
            except Exception as ex:
                rec[a] = "ERROR " + type(ex).__name__ + ": " + str(ex).splitlines()[0]
        entry["experiments"].append(rec)
    parts.append(entry)
OUT["eva_parts"] = parts
print(json.dumps(parts, indent=1)[:6000])
with open("tmp/probe_eva.json", "w") as f:
    json.dump(OUT, f, indent=1)
conn.close()
