"""Research the nodes the test rover needs, in the verification save only, by editing a
copy of the save. Part of task T14 of V0.1.0.

The edit adds one `Tech` entry per node, with only the rover's parts purchased, so the
rest of the tree stays locked for the Green Monolith test. Refuses to run unless the
loaded save is "Ikria Physical Verification". Arguments: the game's `saves` directory.
Writes tmp/monolith_research_edit.json.

The operator loads the edited copy from the game's own Load menu. kRPC 0.7.0's `load` hung
twice at the Space Center without applying the file, as TP-0001 records, so this script
does not call it."""
import json
import pathlib
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from guard import loaded_save  # noqa: E402

SAVE = "Ikria Physical Verification"
# Node, its cost in the tree, and the rover's parts in it, as the save names parts.
NODES = [
    ("engineering101", 5, ["longAntenna", "sensorThermometer"]),
    ("basicScience", 45, ["probeCoreSphere.v2"]),
    ("electrics", 90, ["batteryBankMini", "spotLight1"]),
    ("spaceExploration", 90, ["roverWheel2"]),
    ("fieldScience", 300, ["roverBody"]),
    ("advElectrics", 160, ["LgRadialSolarPanel"]),
    ("advScienceTech", 550, ["sensorGravimeter"]),
    ("precisionEngineering", 160, ["stackDecouplerMini", "structuralMiniNode", "strutCube"]),
    ("composites", 300, ["structuralIBeam3"]),
]
LOG = {"nodes_added": [n for n, _, _ in NODES]}


def tech_block(node, cost, parts):
    lines = ["\t\tTech", "\t\t{", f"\t\t\tid = {node}", "\t\t\tstate = Available", f"\t\t\tcost = {cost}"]
    lines += [f"\t\t\tpart = {p}" for p in parts]
    lines += ["\t\t}"]
    return "\n".join(lines) + "\n"


conn = krpc.connect(name="ikria-monolith-research-edit", address="127.0.0.1")
sc = conn.space_center
saves = pathlib.Path(sys.argv[1])
save = loaded_save(conn, saves)
assert save == SAVE, f"refusing to run in {save!r}"
LOG["loaded_save"] = save

sc.save("ipv-pre-edit")
src = saves / SAVE / "ipv-pre-edit.sfs"
deadline = time.time() + 15
while not src.exists() and time.time() < deadline:
    time.sleep(0.5)
text = src.read_text(encoding="utf-8")
marker = "\t\tname = ResearchAndDevelopment\n"
assert text.count(marker) == 1, "research scenario not found exactly once"
start = text.index(marker)
end = text.index("\n\t}\n", start)
section = text[start:end]
present = [n for n, _, _ in NODES if f"\t\t\tid = {n}\n" in section]
assert not present, f"nodes already present, refusing to duplicate: {present}"
addition = "".join(tech_block(n, c, p) for n, c, p in NODES)
edited = text[:end] + "\n" + addition.rstrip("\n") + text[end:]
dst = saves / SAVE / "ipv-edited.sfs"
dst.write_text(edited, encoding="utf-8")
LOG["edited_file"] = dst.name

LOG["next"] = "load ipv-edited from the game's Load menu"
print(json.dumps(LOG, indent=1))
pathlib.Path("tmp").mkdir(exist_ok=True)
json.dump(LOG, open("tmp/monolith_research_edit.json", "w"), indent=1)
conn.close()
