"""Read-only: list parts per tech node up to a cost limit, with English titles,
plus node prerequisites. Arguments: GameData path, cost limit."""
import os
import re
import sys

ROOT, LIMIT = sys.argv[1], int(sys.argv[2])

loc = {}
for dp, dn, fn in os.walk(ROOT):
    for f in fn:
        if f.endswith(".cfg") and "Localization" in dp:
            t = open(os.path.join(dp, f), encoding="utf-8-sig", errors="replace").read()
            m = re.search(r"en-us\s*\{(.*?)\n\s*\}", t, re.S)
            if m:
                for k, v in re.findall(r"(#autoLOC_\w+)\s*=\s*(.*)", m.group(1)):
                    loc.setdefault(k, v.strip())


def en(s):
    s = s.split("//")[0].strip()
    return loc.get(s, s)


tt = open(os.path.join(ROOT, "Squad/Resources/TechTree.cfg"), encoding="utf-8-sig").read()
nodes = {}
for blk in re.split(r"\n\tRDNode\s*\n", tt)[1:]:
    nid = re.search(r"\n\t\tid = (\S+)", "\n" + blk).group(1)
    cost = int(re.search(r"cost = (\d+)", blk).group(1))
    title = en(re.search(r"title = (.*)", blk).group(1))
    parents = re.findall(r"parentID = (\S+)", blk)
    anyp = "True" in re.search(r"anyToUnlock = (\S+)", blk).group(1)
    nodes[nid] = dict(cost=cost, title=title, parents=parents, any=anyp, parts=[])

for base in ["Squad", "SquadExpansion"]:
    for dp, dn, fn in os.walk(os.path.join(ROOT, base)):
        if "zDeprecated" in dp:
            continue
        for f in fn:
            if not f.endswith(".cfg"):
                continue
            t = open(os.path.join(dp, f), encoding="utf-8-sig", errors="replace").read()
            for m in re.finditer(r"(?m)^PART\s*(?:\n\s*)?\{", t):
                blk = t[m.end():m.end() + 30000]

                def g(k):
                    mm = re.search(r"(?m)^\s*" + k + r"\s*=\s*(.*?)\s*$", blk)
                    return mm.group(1).strip() if mm else ""
                tech, cat, hid = g("TechRequired"), g("category"), g("TechHidden")
                if tech in nodes and cat.lower() != "none" and hid.lower() != "true":
                    nodes[tech]["parts"].append((cat, en(g("title")), g("entryCost")))

for nid, n in sorted(nodes.items(), key=lambda kv: (kv[1]["cost"], kv[0])):
    if n["cost"] > LIMIT:
        continue
    req = (" any of " if n["any"] else " all of ") + ",".join(n["parents"]) if n["parents"] else ""
    print(f"== {nid} [{n['title']}] cost {n['cost']}{req}")
    for cat, title, ec in sorted(n["parts"]):
        print(f"     {cat:13} {title}  (entry {ec})")
