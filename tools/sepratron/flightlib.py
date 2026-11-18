"""Shared flight helpers for the Sepratron research program, verification save only.

Every script connects, identifies the loaded save with the Phase 0 guard and refuses any
save but "Ikria Physical Verification". Each debug-service call goes through `debug`,
which records it in the run log, as the operator's ruling on debug features requires.
Runs write JSON logs under tmp/sepratron/, and the summaries the reports cite are copied
into record/verification/data/ by hand after review.

Time is game time throughout. Samples are polled, not streamed, so their spacing
follows the game's tick and the kRPC round trip, and each sample records its own time.
"""
import json
import math
import pathlib
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "phase0"))
from guard import loaded_save  # noqa: E402
from launch_site import clear_crewed_launch_site as clear_site  # noqa: E402

SAVE = "Ikria Physical Verification"
G0 = 9.80665
OUT = pathlib.Path("tmp/sepratron")


class Run:
    def __init__(self, label, saves_dir):
        self.label = label
        self.saves_dir = saves_dir
        self.conn = krpc.connect(name=f"ikria-sep-{label}", address="127.0.0.1")
        self.sc = self.conn.space_center
        save = loaded_save(self.conn, saves_dir)
        if save != SAVE:
            raise SystemExit(f"refusing to run in {save!r}")
        self.log = {"label": label, "save": save, "debug_uses": [], "events": [], "samples": {}}

    # Debug calls, each logged.
    def debug(self, call, *args, **detail):
        getattr(self.conn.debug, call)(*args)
        self.log["debug_uses"].append({"call": f"Debug.{call}", "ut": self.sc.ut, **detail})

    def event(self, what, **detail):
        entry = {"ut": self.sc.ut, "event": what, **detail}
        self.log["events"].append(entry)
        print(json.dumps(entry), flush=True)

    def clear_crewed_launch_site(self):
        """Recover every crewed vessel in a launch area explicitly before any launch, or
        refuse, through the shared module `tools/phase0/launch_site.py`. In TP-0002 a pod
        left on the pad with Jebediah aboard was removed by the next
        launch_vessel(recover=True), and he was marked Missing. Launch never relies on
        that flag for crewed vessels. Outside the flight scene the module reads a
        temporary save from the saves directory given to the constructor."""
        clear_site(self.conn, self.saves_dir,
                   lambda crew: self.event("crewed_vessel_recovered_before_launch", crew=crew))

    def launch(self, craft, site="LaunchPad", timeout=90.0, directory="VAB"):
        self.clear_crewed_launch_site()
        self.sc.launch_vessel(craft_directory=directory, name=craft, launch_site=site, recover=True)
        k = self.conn.krpc
        deadline = time.time() + timeout
        while time.time() < deadline:
            try:
                if k.game_scene == k.GameScene.flight and self.sc.active_vessel is not None:
                    vessel = self.sc.active_vessel
                    if str(vessel.situation) == "VesselSituation.pre_launch":
                        time.sleep(3)
                        self.event("launched", craft=craft, parts=len(vessel.parts.all),
                                   mass_kg=vessel.mass)
                        return vessel
            except Exception:
                pass
            time.sleep(0.5)
        raise RuntimeError(f"{craft} did not reach the pad within {timeout} s")

    def view(self, vessel, distance=None):
        """Make `vessel` the active vessel, which the flight camera follows and which the
        game keeps in physics, and optionally set the camera's distance, clamped to its
        limits. In orbit a non-active vessel goes on rails 350 m away and gets no thrust,
        so a vessel whose burn is measured is made active first."""
        if self.sc.active_vessel != vessel:
            self.sc.active_vessel = vessel
            time.sleep(1.0)
        if distance is not None:
            cam = self.sc.camera
            cam.distance = max(cam.min_distance, min(cam.max_distance, distance))

    def parts(self, vessel, name):
        return [p for p in vessel.parts.all if p.name == name]

    def fire(self, parts):
        for p in parts:
            p.engine.active = True
        self.event("fired", parts=[p.name for p in parts], count=len(parts))

    def wait_ut(self, seconds, step=0.02):
        target = self.sc.ut + seconds
        while self.sc.ut < target:
            time.sleep(step)

    def sample_engines(self, key, parts, seconds, every=0.1):
        """Record each engine part's thrust and remaining solid fuel, and whether the part
        still exists, for `seconds` of game time."""
        rows = self.log["samples"].setdefault(key, [])
        end = self.sc.ut + seconds
        while self.sc.ut < end:
            row = {"ut": self.sc.ut}
            for i, p in enumerate(parts):
                try:
                    row[f"engine{i}"] = {"thrust_kN": p.engine.thrust / 1000.0,
                                         "active": p.engine.active,
                                         "fuel": p.resources.amount("SolidFuel"),
                                         "temp_K": p.temperature}
                except Exception as error:
                    row[f"engine{i}"] = {"absent": type(error).__name__}
            rows.append(row)
            time.sleep(every)
        return rows

    def sample(self, key, vessels, frame, seconds, every=0.05):
        """Record time, position, velocity and mass of each vessel for `seconds` of game
        time. Vessels that disappear are recorded as absent rather than ending the run."""
        rows = self.log["samples"].setdefault(key, [])
        end = self.sc.ut + seconds
        while self.sc.ut < end:
            row = {"ut": self.sc.ut}
            for name, v in vessels.items():
                try:
                    row[name] = {"pos": list(v.position(frame)), "vel": list(v.velocity(frame)),
                                 "mass": v.mass}
                except Exception as error:
                    row[name] = {"absent": type(error).__name__}
            rows.append(row)
            time.sleep(every)
        return rows

    def save(self):
        OUT.mkdir(parents=True, exist_ok=True)
        path = OUT / f"{self.label}.json"
        path.write_text(json.dumps(self.log, indent=1) + "\n")
        print(f"log: {path}")
        return path


def norm(v):
    return math.sqrt(sum(x * x for x in v))


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))
