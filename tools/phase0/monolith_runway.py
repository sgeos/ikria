"""Put a fresh test rover on the runway for the KerbNet scan, task T14 of V0.1.0.

In orbit the rover's RoveMate core limits KerbNet to a 10-degree field of view, which
the operator confirmed on 2026-11-15. Its part file allows up to 179.5 degrees when the
vessel is landed, splashed or before launch, so a rover on the runway should see about a
hemisphere at once. Refuses to run unless the loaded save is "Ikria Physical
Verification". Saves the result as "ipv-monolith-runway".

Arguments: the game's `saves` directory. Writes tmp/monolith_runway.json."""
import json
import pathlib
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from guard import loaded_save  # noqa: E402
from launch_site import clear_crewed_launch_site  # noqa: E402

SAVE = "Ikria Physical Verification"
LOG = {}


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


conn = krpc.connect(name="ikria-monolith-runway", address="127.0.0.1")
k, sc = conn.krpc, conn.space_center
save = loaded_save(conn, sys.argv[1])
LOG["loaded_save"] = save
assert save == SAVE, f"refusing to run in {save!r}"
assert sc.game_mode == sc.GameMode.career

# Rule R20. Recover crewed vessels at the launch site explicitly before any launch.
clear_crewed_launch_site(sc, lambda crew: print("recovered before launch:", crew, flush=True))
sc.launch_vessel(craft_directory="SPH", name="IkriaKN", launch_site="Runway", recover=True)
assert wait_for(lambda: k.game_scene == k.GameScene.flight
                and str(sc.active_vessel.situation) == "VesselSituation.pre_launch", 120)
time.sleep(3)
v = sc.active_vessel
flight = v.flight()
LOG["vessel"] = {"parts": len(v.parts.all), "situation": str(v.situation),
                 "latitude": flight.latitude, "longitude": flight.longitude,
                 "can_communicate": v.comms.can_communicate}
LOG["vessels_in_save"] = [(x.name, str(x.situation)) for x in sc.vessels]
LOG["debug_members"] = sorted(m for m in dir(conn.debug) if not m.startswith("_"))
sc.save("ipv-monolith-runway")
LOG["saved_as"] = "ipv-monolith-runway"
print(json.dumps(LOG, indent=1))
json.dump(LOG, open("tmp/monolith_runway.json", "w"), indent=1)
conn.close()
