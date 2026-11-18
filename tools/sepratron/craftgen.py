"""Generate simple test-article craft files for the Sepratron research program.

kRPC 0.7.0 cannot build craft in the editor, so test articles are written as craft
files. Each part's block, with its module and resource data, is copied from a stock
craft or an operator-built craft that already holds that part, and given a fresh
identifier, position, rotation and attachment. Nothing is distributed. The output goes
only to the verification save's Ships folder, and the stock files are read, never
changed.

Geometry. A stack child sits below its parent, at the parent's position plus the
parent's bottom-node height minus the child's top-node height, which matches the
operator-built craft IKR-TS-001A. Node heights come from the part configuration, times
its rescaleFactor, which defaults to 1.25 when absent. A radial part sits on a circle of
a stated radius around the stack axis. At angle phi, measured from +z toward +x, its
position is (r sin phi, y, r cos phi) and its rotation is phi about the vertical axis,
which matches the stock Two-Stage Lander's sepratrons. A retro mount adds a turn of 180
degrees about the part's surface normal, which reverses its thrust and keeps it on the
surface.

Normalization. Stock craft carry their designers' tweaks, such as a sepratron at half
thrust and half fuel, or a parachute set to open at 50 m. Every copied part is therefore
reset to the part's own configuration. Each field of a module that the configuration
sets takes the configuration's value, the thrust limiter returns to 100 percent, and
every resource is filled, unless the article sets a value explicitly. The first A1 and
A2 runs flew with inherited tweaks, as TP-0002 records, before these resets existed.

Limits. Positions are computed, not checked against colliders, so a radius that is too
small can clip a part into its neighbour. Staging fields are left neutral, because the
flight scripts fire each part through kRPC. Parts without a template cannot be used.
"""
import math
import pathlib
import random
import re

KSP_VERSION = "1.12.5"


def craft_name_to_cfg(name):
    return name.replace(".", "_")


def module_blocks(text, indent):
    """Return (start, end, name, fields) for each MODULE block at the given indent, where
    fields maps each key at the block's top level to its value."""
    blocks = []
    marker = "\n" + indent + "MODULE\n" + indent + "{\n"
    pos = 0
    while True:
        start = text.find(marker, pos)
        if start < 0:
            return blocks
        depth, i = 0, start + len(marker) - 1
        while i < len(text):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        body = text[start + len(marker):i]
        fields, level = {}, 0
        for line in body.splitlines():
            stripped = line.strip()
            if stripped == "{":
                level += 1
            elif stripped == "}":
                level -= 1
            elif level == 0 and "=" in stripped:
                key, _, value = stripped.partition("=")
                fields.setdefault(key.strip(), value.strip())
        blocks.append((start + len(marker), i, fields.get("name"), fields))
        pos = i


def reset_modules(craft_tail, cfg_text):
    """Set each top-level module field that the part configuration defines to the
    configuration's value, matching modules by name and occurrence."""
    cfg_modules = {}
    for _, _, name, fields in module_blocks(re.sub(r"[ \t]+", "\t", "\n" + cfg_text), "\t"):
        cfg_modules.setdefault(name, []).append(fields)
    out, last, seen = [], 0, {}
    for start, end, name, _ in module_blocks(craft_tail, "\t"):
        index = seen.get(name, 0)
        seen[name] = index + 1
        defaults = cfg_modules.get(name, [])
        body = craft_tail[start:end]
        if index < len(defaults):
            for key, value in defaults[index].items():
                if key == "name":
                    continue
                body = re.sub(rf"(?m)^(\t\t{re.escape(key)} = ).*$", lambda m, v=value: m.group(1) + v, body)
        out.append(craft_tail[last:start] + body)
        last = end
    out.append(craft_tail[last:])
    return "".join(out)


class PartLibrary:
    """Templates from existing craft, and node geometry from part configurations."""

    def __init__(self, game_root):
        self.root = pathlib.Path(game_root)
        self.templates = {}
        sources = sorted((self.root / "Ships").rglob("*.craft"))
        sources += sorted((self.root / "saves" / "Ikria Physical Verification" / "Ships").rglob("*.craft"))
        for path in sources:
            text = path.read_text(encoding="utf-8-sig", errors="replace")
            for block in re.split(r"\nPART\n", text)[1:]:
                match = re.search(r"\n\tpart = (\S+)_\d+\n", "\n" + block)
                if match and match.group(1) not in self.templates:
                    self.templates[match.group(1)] = block.rstrip()
        self.configs = {}
        for cfg in (self.root / "GameData").rglob("*.cfg"):
            text = re.sub(r"//[^\n]*", "", cfg.read_text(encoding="utf-8-sig", errors="replace"))
            if not re.search(r"^\s*PART\s*$", text, re.M):
                continue
            name = re.search(r"^\s*name\s*=\s*(\S+)", text, re.M)
            if name:
                self.configs.setdefault(name.group(1), text)

    def node_height(self, name, node):
        text = self.configs[craft_name_to_cfg(name)]
        scale = re.search(r"^\s*rescaleFactor\s*=\s*([0-9.]+)", text, re.M)
        scale = float(scale.group(1)) if scale else 1.25
        match = re.search(rf"^\s*node_stack_{node}\s*=\s*([^\n]+)", text, re.M)
        if not match:
            raise ValueError(f"{name} has no {node} stack node")
        return float(match.group(1).split(",")[1]) * scale

    def template(self, name):
        if name not in self.templates:
            raise KeyError(f"no template craft holds {name}")
        return self.templates[name]


def quat_mul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return (aw * bx + ax * bw + ay * bz - az * by,
            aw * by - ax * bz + ay * bw + az * bx,
            aw * bz + ax * by - ay * bx + az * bw,
            aw * bw - ax * bx - ay * by - az * bz)


def fmt(values):
    return ",".join(f"{v:.7g}" for v in values)


class Part:
    def __init__(self, name, ident, pos, rot, attm):
        self.name, self.ident, self.pos, self.rot, self.attm = name, ident, pos, rot, attm
        self.links, self.attn, self.srfn = [], [], None
        self.resources = {}
        self.thrust_pct = 100.0

    @property
    def tag(self):
        return f"{self.name}_{self.ident}"


class Craft:
    """A craft built from a root part downward."""

    def __init__(self, library, name, root, rng=None):
        self.lib = library
        self.name = name
        self.rng = rng or random.Random(name)
        self.parts = []
        self.root = self._new(root, (0.0, 10.0, 0.0), (0, 0, 0, 1), 0)

    def _new(self, name, pos, rot, attm):
        self.lib.template(name)
        part = Part(name, self.rng.randrange(4_200_000_000, 4_294_000_000), pos, rot, attm)
        self.parts.append(part)
        return part

    def stack_below(self, parent, name):
        """Attach `name` by its top node to the parent's bottom node."""
        parent_bottom = self.lib.node_height(parent.name, "bottom")
        child_top = self.lib.node_height(name, "top")
        x, y, z = parent.pos
        child = self._new(name, (x, y + parent_bottom - child_top, z), (0, 0, 0, 1), 0)
        parent.links.append(child.tag)
        parent.attn.append(f"bottom,{child.tag}_0|{parent_bottom:.7g}|0")
        child.attn.append(f"top,{parent.tag}_0|{child_top:.7g}|0")
        return child

    def stack_above(self, parent, name):
        """Attach `name` by its bottom node to the parent's top node."""
        parent_top = self.lib.node_height(parent.name, "top")
        child_bottom = self.lib.node_height(name, "bottom")
        x, y, z = parent.pos
        child = self._new(name, (x, y + parent_top - child_bottom, z), (0, 0, 0, 1), 0)
        parent.links.append(child.tag)
        parent.attn.append(f"top,{child.tag}_0|{parent_top:.7g}|0")
        child.attn.append(f"bottom,{parent.tag}_0|{child_bottom:.7g}|0")
        return child

    def radial(self, parent, name, count, radius, dy=0.0, retro=False, phase_deg=0.0, rot=None):
        """Attach `count` copies of `name` around the parent, evenly spaced."""
        made = []
        for i in range(count):
            phi = math.radians(phase_deg + 360.0 * i / count)
            pos = (parent.pos[0] + radius * math.sin(phi), parent.pos[1] + dy,
                   parent.pos[2] + radius * math.cos(phi))
            q = rot or (0.0, math.sin(phi / 2), 0.0, math.cos(phi / 2))
            if retro:
                q = quat_mul(q, (0.0, 0.0, 1.0, 0.0))
            part = self._new(name, pos, q, 1)
            part.srfn = f"srfAttach,{parent.tag}"
            parent.links.append(part.tag)
            made.append(part)
        return made

    def set_resource(self, part, resource, amount):
        part.resources[resource] = amount

    def set_thrust(self, part, percent):
        if not 0.0 <= percent <= 100.0:
            raise ValueError("a thrust limiter lies between 0 and 100 percent")
        part.thrust_pct = percent

    def _block(self, part):
        body = self.lib.template(part.name)
        start = body.index("\tEVENTS")
        tail = "\n" + body[start:]
        tail = reset_modules(tail, self.lib.configs[craft_name_to_cfg(part.name)])[1:]
        tail = re.sub(r"(\n\t\tthrustPercentage = )[0-9.Ee+-]+", rf"\g<1>{part.thrust_pct:g}", tail)

        def fill(match):
            return f"{match.group(1)}{match.group(3)}{match.group(2)}{match.group(3)}"
        tail = re.sub(r"(RESOURCE\n\t\{\n\t\tname = \S+\n\t\tamount = )[0-9.Ee+-]+(\n\t\tmaxAmount = )([0-9.Ee+-]+)",
                      fill, tail)
        for resource, amount in part.resources.items():
            pattern = rf"(RESOURCE\n\t\{{\n\t\tname = {resource}\n\t\tamount = )[0-9.Ee+-]+"
            tail, count = re.subn(pattern, rf"\g<1>{amount}", tail)
            if count != 1:
                raise ValueError(f"{part.name} has no single {resource} resource to set")
        head = [f"\tpart = {part.tag}", "\tpartName = Part",
                f"\tpersistentId = {self.rng.randrange(1, 4_294_967_295)}",
                f"\tpos = {fmt(part.pos)}", "\tattPos = 0,0,0", "\tattPos0 = 0,0,0",
                f"\trot = {fmt(part.rot)}", "\tattRot = 0,0,0,1", "\tattRot0 = 0,0,0,1",
                "\tmir = 1,1,1", "\tsymMethod = Radial", "\tautostrutMode = Off",
                "\trigidAttachment = False", "\tistg = 0", "\tresPri = 0", "\tdstg = 0",
                "\tsidx = -1", "\tsqor = -1", "\tsepI = -1", f"\tattm = {part.attm}",
                "\tmodCost = 0", "\tmodMass = 0", "\tmodSize = 0,0,0"]
        head += [f"\tlink = {tag}" for tag in part.links]
        head += [f"\tattN = {a}" for a in part.attn]
        if part.srfn:
            head.append(f"\tsrfN = {part.srfn}")
        return "PART\n{\n" + "\n".join(head) + "\n" + tail + "\n"

    def text(self):
        header = [f"ship = {self.name}", f"version = {KSP_VERSION}",
                  "description = Generated test article of the Ikria Sepratron program.",
                  "type = VAB", "size = 2,4,2", "steamPublishedFileId = 0",
                  f"persistentId = {self.rng.randrange(1, 4_294_967_295)}", "rot = 0,0,0,1",
                  "missionFlag = Squad/Flags/default", "vesselType = Ship"]
        return "\n".join(header) + "\n" + "".join(self._block(p) for p in self.parts)

    def write(self, directory):
        path = pathlib.Path(directory) / f"{self.name}.craft"
        path.write_text(self.text(), encoding="utf-8")
        return path
