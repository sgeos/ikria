"""Prepare the verification save for the Sepratron research program.

Refuses to run unless the loaded save is "Ikria Physical Verification". Writes the
checkpoint "ipv-before-sepratron-program", then researches every node and purchases
every part through the debug service, which the operator allows in this save only, and
logs the call. Writes tmp/sepratron/setup.json.

Argument: the game's `saves` directory."""
import json
import pathlib
import re
import sys
import time

import krpc

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "phase0"))
from guard import loaded_save  # noqa: E402

SAVE = "Ikria Physical Verification"


def research_ids(path):
    text = path.read_text(encoding="utf-8")
    start = text.index("name = ResearchAndDevelopment")
    section = text[start:text.index("\n\t}\n", start)]
    return re.findall(r"\n\t\t\tid = (\S+)", section), len(re.findall(r"\n\t\t\tpart = ", section))


def wait_file(path, timeout=20.0):
    deadline = time.time() + timeout
    while not path.exists() and time.time() < deadline:
        time.sleep(0.5)
    if not path.exists():
        raise RuntimeError(f"the game did not write {path.name}")


saves = pathlib.Path(sys.argv[1])
conn = krpc.connect(name="ikria-sepratron-setup", address="127.0.0.1")
sc = conn.space_center
save = loaded_save(conn, saves)
assert save == SAVE, f"refusing to run in {save!r}"
log = {"loaded_save": save, "debug_uses": []}
sc.save("ipv-before-sepratron-program")
before = saves / SAVE / "ipv-before-sepratron-program.sfs"
wait_file(before)
log["before"] = dict(zip(("nodes", "parts"), research_ids(before)))
conn.debug.unlock_technology_tree()
log["debug_uses"].append({"call": "Debug.unlock_technology_tree", "ut": sc.ut})
sc.save("ipv-sepratron-tree")
after = saves / SAVE / "ipv-sepratron-tree.sfs"
wait_file(after)
nodes, parts = research_ids(after)
log["after"] = {"node_count": len(nodes), "purchased_part_entries": parts,
                "precisionPropulsion": "precisionPropulsion" in nodes}
log["before"] = {"node_count": len(log["before"]["nodes"]), "purchased_part_entries": log["before"]["parts"]}
out = pathlib.Path("tmp/sepratron")
out.mkdir(parents=True, exist_ok=True)
(out / "setup.json").write_text(json.dumps(log, indent=1))
print(json.dumps(log, indent=1))
conn.close()
