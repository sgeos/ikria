"""Read-only inventory of KSP 1.12.5 career data. Argument: path to GameData.

Parses ConfigNode files with a small brace parser. Prints counts and short lists only.
"""
import collections
import os
import re
import sys

ROOT = sys.argv[1]


def parse(text):
    """Return a list of (key, value) where value is str or a nested list."""
    text = text.lstrip("﻿")
    lines = [re.sub(r"//.*", "", ln).strip() for ln in text.splitlines()]
    stack = [[]]
    pending = None
    for ln in lines:
        if not ln:
            continue
        while ln:
            if ln.startswith("{"):
                node = []
                stack[-1].append((pending or "", node))
                stack.append(node)
                pending = None
                ln = ln[1:].strip()
            elif ln.startswith("}"):
                if len(stack) > 1:
                    stack.pop()
                ln = ln[1:].strip()
            elif "=" in ln and ("{" not in ln or ln.index("=") < ln.index("{")):
                k, v = ln.split("=", 1)
                v = v.strip()
                if "}" in v:
                    v, rest = v.split("}", 1)
                    stack[-1].append((k.strip(), v.strip()))
                    ln = "}" + rest
                else:
                    stack[-1].append((k.strip(), v))
                    ln = ""
            else:
                m = re.match(r"([^{}]*)(.*)", ln)
                if m.group(1).strip():
                    pending = m.group(1).strip()
                ln = m.group(2).strip()
    return stack[0]


def nodes(tree, name):
    out = []
    for k, v in tree:
        if isinstance(v, list):
            if k.split(":")[0].strip() == name:
                out.append(v)
            out.extend(nodes(v, name))
    return out


def val(node, key, default=""):
    for k, v in node:
        if k == key and isinstance(v, str):
            return v
    return default


def load_all():
    data = []
    for base in ["Squad", "SquadExpansion"]:
        for dp, dn, fn in os.walk(os.path.join(ROOT, base)):
            if "zDeprecated" in dp:
                continue
            for f in fn:
                if f.endswith(".cfg"):
                    p = os.path.join(dp, f)
                    data.append((os.path.relpath(p, ROOT), parse(open(p, encoding="utf-8", errors="replace").read())))
    return data


DATA = load_all()

# Tech tree
tt = parse(open(os.path.join(ROOT, "Squad/Resources/TechTree.cfg"), encoding="utf-8").read())
rd = nodes(tt, "RDNode")
costs = {val(n, "id"): int(val(n, "cost", "0")) for n in rd}
print("TECH: nodes", len(rd), "total science", sum(costs.values()))
by_cost = collections.Counter(costs.values())
print("  nodes by cost:", dict(sorted(by_cost.items())))

# Parts
parts = []
for path, tree in DATA:
    for k, v in tree:
        if k == "PART" and isinstance(v, list):
            parts.append((path, v))
real = [(p, n) for p, n in parts if val(n, "TechRequired") not in ("", "Unresearcheable")
        and val(n, "TechHidden").lower() != "true" and val(n, "category").lower() != "none"]
print("PARTS: blocks", len(parts), "researchable and listed", len(real))
print("  by source:", dict(collections.Counter(p.split("/")[0] + "/" + p.split("/")[1] for p, _ in real)))
print("  by category:", dict(collections.Counter(val(n, "category") for _, n in real)))
unknown_tech = sorted({val(n, "TechRequired") for _, n in real} - set(costs))
print("  TechRequired not in tree:", unknown_tech)
empty = sorted({k for k in costs if k != "start"} - {val(n, "TechRequired") for _, n in real})
print("  nodes with no listed part:", empty)
entry = sum(int(float(val(n, "entryCost", "0") or 0)) for _, n in real)
cost = sum(int(float(val(n, "cost", "0") or 0)) for _, n in real)
print("  sum entryCost", entry, "sum unit cost", cost)

# Part upgrades
ups = [(p, n) for p, t in DATA for n in nodes(t, "PARTUPGRADE")]
print("PARTUPGRADES:", len(ups), sorted(val(n, "techRequired") for _, n in ups)[:40])

# Science experiments
exps = [(p, n) for p, t in DATA for n in nodes(t, "EXPERIMENT_DEFINITION")]
print("EXPERIMENTS:", len(exps))
for p, n in exps:
    print("  ", val(n, "id"), "| base", val(n, "baseValue"), "cap", val(n, "scienceCap"),
          "| surf", val(n, "situationMask"), "biome", val(n, "biomeMask"), "|", p.split("/")[1])

# Contracts
ctr = [(p, n) for p, t in DATA for n in nodes(t, "CONTRACTS")]
print("CONTRACT config nodes:", len(ctr), [p for p, _ in ctr])
for p, n in ctr:
    print("  sub-nodes:", [k for k, v in n if isinstance(v, list)])

# Strategies
strat = [(p, n) for p, t in DATA for n in nodes(t, "STRATEGY")]
print("STRATEGIES:", len(strat), [val(n, "name") for _, n in strat])

# Experience traits
traits = [(p, n) for p, t in DATA for n in nodes(t, "EXPERIENCE_TRAIT")]
print("TRAITS:", [val(n, "name") for _, n in traits])
