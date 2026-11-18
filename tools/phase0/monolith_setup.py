"""Set up the Green Monolith test in the verification save, task T14 of V0.1.0.

Refuses to run unless the loaded save is "Ikria Physical Verification". Every use of the
debug service is logged. Launches the rover `IkriaKN`, a copy of the stock Bug-E Buggy,
whose RoveMate core has full KerbNet anomaly detection, and places it in a polar orbit
of Kerbin for the KerbNet scan. Saves the result as "ipv-monolith-scan".

Arguments: the game's `saves` directory. Writes tmp/monolith_setup.json."""
import json
import math
import pathlib
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from guard import loaded_save  # noqa: E402
from launch_site import clear_crewed_launch_site  # noqa: E402

SAVE = "Ikria Physical Verification"
LOG = {"debug_uses": []}


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


conn = krpc.connect(name="ikria-monolith-setup", address="127.0.0.1")
k, sc, dbg = conn.krpc, conn.space_center, conn.debug
save = loaded_save(conn, sys.argv[1])
LOG["loaded_save"] = save
assert save == SAVE, f"refusing to run in {save!r}"
assert sc.game_mode == sc.GameMode.career

LOG["funds_before"] = sc.funds
dbg.funds = 2_000_000.0
LOG["debug_uses"].append({"call": "Debug.funds", "value": 2_000_000.0, "ut": sc.ut})

try:
    # Rule R20. Recover crewed vessels at the launch site explicitly before any launch.
    clear_crewed_launch_site(sc, lambda crew: print("recovered before launch:", crew, flush=True))
    sc.launch_vessel(craft_directory="SPH", name="IkriaKN", launch_site="Runway", recover=True)
    LOG["launch"] = "ok"
except Exception as ex:
    LOG["launch"] = f"ERROR {type(ex).__name__}: {str(ex).splitlines()[0]}"
    print(json.dumps(LOG, indent=1))
    json.dump(LOG, open("tmp/monolith_setup.json", "w"), indent=1)
    sys.exit(1)

assert wait_for(lambda: k.game_scene == k.GameScene.flight and sc.active_vessel is not None, 120)
time.sleep(5)
v = sc.active_vessel
LOG["vessel"] = {"name": v.name, "parts": len(v.parts.all), "situation": str(v.situation)}
kerbin = sc.bodies["Kerbin"]
sma = kerbin.equatorial_radius + 250_000.0
dbg.set_orbit(kerbin, sma, 0.0, math.pi / 2, 0.0, 0.0, 0.0, sc.ut)
LOG["debug_uses"].append({"call": "Debug.set_orbit", "body": "Kerbin", "sma_m": sma,
                          "eccentricity": 0.0, "inclination_rad": math.pi / 2, "ut": sc.ut})
time.sleep(5)
o = sc.active_vessel.orbit
LOG["orbit"] = {"apoapsis_alt_m": o.apoapsis_altitude, "periapsis_alt_m": o.periapsis_altitude,
                "inclination_deg": math.degrees(o.inclination)}
try:
    LOG["can_communicate"] = sc.active_vessel.comms.can_communicate
except Exception as ex:
    LOG["can_communicate"] = f"ERROR {str(ex).splitlines()[0]}"
sc.save("ipv-monolith-scan")
LOG["saved_as"] = "ipv-monolith-scan"
print(json.dumps(LOG, indent=1))
json.dump(LOG, open("tmp/monolith_setup.json", "w"), indent=1)
conn.close()
