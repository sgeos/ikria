"""Read-only: unit cost, entry cost, mass and node of named parts, and the summed
entry cost of every part in nodes at or below a cost. Arguments: GameData, limit,
then substrings of English part titles."""
import os
import re
import sys

ROOT, LIMIT, WANT = sys.argv[1], int(sys.argv[2]), sys.argv[3:]
loc = {}
for dp, dn, fn in os.walk(ROOT):
    for f in fn:
        if f.endswith(".cfg") and "Localization" in dp:
            t = open(os.path.join(dp, f), encoding="utf-8-sig", errors="replace").read()
            m = re.search(r"en-us\s*\{(.*?)\n\s*\}", t, re.S)
            if m:
                for k, v in re.findall(r"(#autoLOC_\w+)\s*=\s*(.*)", m.group(1)):
                    loc.setdefault(k, v.strip())
tt = open(os.path.join(ROOT, "Squad/Resources/TechTree.cfg"), encoding="utf-8-sig").read()
cost = {}
for blk in re.split(r"\n\tRDNode\s*\n", tt)[1:]:
    cost[re.search(r"\n\t\tid = (\S+)", "\n" + blk).group(1)] = int(re.search(r"cost = (\d+)", blk).group(1))
parts = []
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
                    return mm.group(1).split("//")[0].strip() if mm else ""
                if g("category").lower() == "none" or g("TechHidden").lower() == "true" or g("TechRequired") not in cost:
                    continue
                title = g("title")
                title = loc.get(title, title)
                parts.append(dict(title=title, tech=g("TechRequired"), entry=float(g("entryCost") or 0),
                                  unit=float(g("cost") or 0), mass=g("mass")))
early = [p for p in parts if cost[p["tech"]] <= LIMIT]
print(f"parts in nodes costing at most {LIMIT}: {len(early)}, summed entry cost {sum(p['entry'] for p in early):,.0f}")
print(f"all parts: summed entry cost {sum(p['entry'] for p in parts):,.0f}")
for w in WANT:
    for p in parts:
        if w.lower() in p["title"].lower():
            print(f"  {p['title'][:44]:44} node {p['tech']:20} ({cost[p['tech']]:4}) unit {p['unit']:>8,.0f} entry {p['entry']:>8,.0f} mass {p['mass']}")
