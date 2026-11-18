"""Write an edited copy of a verification save in which every researched node lists
every part that requires it as purchased.

Debug.unlock_technology_tree researched every node in kRPC 0.7.0 but left no part
recorded as purchased, as TP-0002 records, and Hard requires entry purchases. This
script repairs that in a copy, which the operator loads from the game's menu, since
kRPC's load hangs. It refuses a save other than the verification save, and refuses to
overwrite an existing output.

Arguments: the game's GameData directory, the source .sfs, the output .sfs."""
import pathlib
import re
import sys

game_data, source, output = (pathlib.Path(a) for a in sys.argv[1:4])
if "Ikria Physical Verification" not in str(source.parent):
    raise SystemExit("refusing a save outside the verification save")
if output.exists():
    raise SystemExit(f"refusing to overwrite {output.name}")

parts_by_tech = {}
for cfg in game_data.rglob("*.cfg"):
    # Many stock configs begin with a byte-order mark, which defeats a match at the
    # start of the first line, so it is removed first.
    text = cfg.read_text(encoding="utf-8-sig", errors="replace")
    text = re.sub(r"//[^\n]*", "", text)
    if not re.search(r"^\s*PART\s*$", text, re.M):
        continue
    name = re.search(r"^\s*name\s*=\s*(\S+)", text, re.M)
    tech = re.search(r"^\s*TechRequired\s*=\s*(\S+)", text, re.M)
    if name and tech:
        # Craft and saves name parts with dots where the config has underscores.
        parts_by_tech.setdefault(tech.group(1), set()).add(name.group(1).replace("_", "."))

text = source.read_text(encoding="utf-8")
start = text.index("name = ResearchAndDevelopment")
end = text.index("\n\t}\n", start)
section = text[start:end]
added = 0


def add_parts(match):
    global added
    block = match.group(0)
    tech = re.search(r"\n\t\t\tid = (\S+)", block).group(1)
    have = set(re.findall(r"\n\t\t\tpart = (\S+)", block))
    new = sorted(parts_by_tech.get(tech, set()) - have)
    added += len(new)
    return block[:-len("\t\t}")] + "".join(f"\t\t\tpart = {p}\n" for p in new) + "\t\t}"


section = re.sub(r"\t\tTech\n\t\t\{\n.*?\n\t\t\}", add_parts, section, flags=re.S)
output.write_text(text[:start] + section + text[end:], encoding="utf-8")
print(f"nodes: {section.count(chr(10) + chr(9) * 2 + 'Tech')}, part entries added: {added}")
