"""Write the Sepratron program's test articles into the verification save's VAB folder.

Each article is described in TP-0002. Existing files with different contents are
refused, so a rerun cannot silently change an article that has flown.

Argument: the game's root directory."""
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from craftgen import Craft, PartLibrary  # noqa: E402

TANK_R = 0.62   # FL-T100 surface radius, from the stock Two-Stage Lander's sepratrons
POD_R = 0.56    # Mk1 pod surface radius about 0.15 m below its origin, from its slope
POD_R_C = 0.50  # between the pod's surface radius at the motor's top and bottom


def a1(lib):
    """Separation and deorbit. A probe over a TR-18A over an FL-T100 spent stage with four
    retro sepratrons, which push the stage away from the probe and toward retrograde."""
    c = Craft(lib, "IKR-SEP-A1C", "probeCoreOcto.v2")
    dec = c.stack_below(c.root, "Decoupler.1")
    tank = c.stack_below(dec, "fuelTankSmallFlat")
    c.radial(tank, "sepMotor1", 4, TANK_R, retro=True)
    return c


def a2(lib):
    """Abort. A Mk1 pod with a parachute and four upright sepratrons, over a TR-18A over a
    Flea. The Flea's thrust limit is set by the flight script."""
    c = Craft(lib, "IKR-SEP-A2D", "mk1pod.v2")
    c.stack_above(c.root, "parachuteSingle")
    # Revision B mounted the motors 0.15 m lower, where their 0.538 m length reached
    # 0.05 m into the decoupler that leaves with the booster, and the first pod was lost
    # at ignition. Revision C raises them to the pod's origin, clear of the decoupler.
    c.radial(c.root, "sepMotor1", 4, POD_R_C, dy=0.0, phase_deg=45)
    dec = c.stack_below(c.root, "Decoupler.1")
    c.stack_below(dec, "solidBooster.sm.v2")
    return c


def a3(lib):
    """Low-gravity lander. A probe over an empty FL-T100 ascent stage with four upright
    sepratrons, over a TR-18A over an FL-T100 descent stage with four upright braking
    sepratrons. Upright mounting thrusts along the vessel's top, which is retrograde in a
    capture burn and upward in a feet-first descent."""
    c = Craft(lib, "IKR-SEP-A3C", "probeCoreOcto.v2")
    ascent = c.stack_below(c.root, "fuelTankSmallFlat")
    c.set_resource(ascent, "LiquidFuel", 0)
    c.set_resource(ascent, "Oxidizer", 0)
    c.radial(ascent, "sepMotor1", 4, TANK_R)
    dec = c.stack_below(ascent, "Decoupler.1")
    descent = c.stack_below(dec, "fuelTankSmallFlat")
    c.radial(descent, "sepMotor1", 4, TANK_R, phase_deg=45)
    return c


def a3d(lib):
    """Minmus ascent with fixed-impulse burns. As A3C, except that the ascent stage's
    motors carry fuel loads sized by tools/sepratron/sizing.py: one opposed pair with
    3.0 units each for the first burn, at 15 degrees above the horizon, and one opposed
    pair with 0.37 units each for circularization at an apoapsis of about 11 km."""
    c = Craft(lib, "IKR-SEP-A3D", "probeCoreOcto.v2")
    ascent = c.stack_below(c.root, "fuelTankSmallFlat")
    c.set_resource(ascent, "LiquidFuel", 0)
    c.set_resource(ascent, "Oxidizer", 0)
    motors = c.radial(ascent, "sepMotor1", 4, TANK_R)
    for i, part in enumerate(motors):
        c.set_resource(part, "SolidFuel", 3.0 if i % 2 == 0 else 0.37)
    dec = c.stack_below(ascent, "Decoupler.1")
    descent = c.stack_below(dec, "fuelTankSmallFlat")
    c.radial(descent, "sepMotor1", 4, TANK_R, phase_deg=45)
    return c


def a3e(lib):
    """As A3D, with fuel loads resized against the stage's true mass. A3D's sizing counted
    every ascent motor as fully loaded, 452.5 kg, where the stage weighs about 239 kg, so
    its first burn would have exceeded Minmus escape velocity. It did not fly. Burn 1 is
    1.59 units per motor and burn 2 is 0.197."""
    c = Craft(lib, "IKR-SEP-A3E", "probeCoreOcto.v2")
    ascent = c.stack_below(c.root, "fuelTankSmallFlat")
    c.set_resource(ascent, "LiquidFuel", 0)
    c.set_resource(ascent, "Oxidizer", 0)
    motors = c.radial(ascent, "sepMotor1", 4, TANK_R)
    for i, part in enumerate(motors):
        c.set_resource(part, "SolidFuel", 1.59 if i % 2 == 0 else 0.197)
    dec = c.stack_below(ascent, "Decoupler.1")
    descent = c.stack_below(dec, "fuelTankSmallFlat")
    c.radial(descent, "sepMotor1", 4, TANK_R, phase_deg=45)
    return c


def a3f(lib):
    """As A3E, with a Z-100 battery on the ascent stage and loads resized from A3E's
    measured burn. In A3E the OKTO's 10 units of charge, drawn at 0.020 per second, ran
    out during the 540 s coast, so the reaction wheel could not point the stage for
    circularization. Burn 1 is 1.60 units per motor and burn 2 is 0.229."""
    c = Craft(lib, "IKR-SEP-A3F", "probeCoreOcto.v2")
    ascent = c.stack_below(c.root, "fuelTankSmallFlat")
    c.set_resource(ascent, "LiquidFuel", 0)
    c.set_resource(ascent, "Oxidizer", 0)
    motors = c.radial(ascent, "sepMotor1", 4, TANK_R)
    for i, part in enumerate(motors):
        c.set_resource(part, "SolidFuel", 1.60 if i % 2 == 0 else 0.229)
    c.radial(ascent, "batteryPack", 1, TANK_R, phase_deg=45)
    dec = c.stack_below(ascent, "Decoupler.1")
    descent = c.stack_below(dec, "fuelTankSmallFlat")
    c.radial(descent, "sepMotor1", 4, TANK_R, phase_deg=45)
    return c


def a5(lib):
    """Soft landing under a parachute. A Mk1 pod with a Mk16 parachute and two upright
    sepratrons, each with 0.22 units of fuel at a 25 percent thrust limit, which by
    sizing.py remove about 4 m/s in about 0.56 s just before touchdown."""
    c = Craft(lib, "IKR-SEP-A5B", "mk1pod.v2")
    c.stack_above(c.root, "parachuteSingle")
    # Revision A placed a motor at 0 degrees, over the Mk1 pod's hatch, and the kerbal
    # could not leave. The motors sit at 90 and 270 degrees, clear of the hatch.
    for part in c.radial(c.root, "sepMotor1", 2, POD_R_C, dy=0.0, phase_deg=90):
        c.set_resource(part, "SolidFuel", 0.22)
        c.set_thrust(part, 25.0)
    return c


def aircraft(lib, root, with_seps):
    """The stock Aeris 3A, with an OKTO probe on top of its intake fuselage for uncrewed
    control and, when `with_seps`, four sepratrons on the upper flanks of its radial engine
    body and intake fuselage, 45 degrees either side of the top, mounted in line with its
    jet. The aircraft hangar's craft point their nose along +z and turn each part 90
    degrees about x, so a part's upright axis faces forward. A radial offset of angle phi
    in the rocket hangar, (r sin phi, 0, r cos phi), becomes (r sin phi, -r cos phi, 0)."""
    import math
    import re
    from craftgen import Part, fmt, quat_mul
    name = "IKR-SEP-B4" + ("R" if with_seps else "N")
    stock = (root / "Ships" / "SPH" / "Aeris 3A.craft").read_text(encoding="utf-8-sig")
    head, *blocks = re.split(r"\nPART\n", stock)
    head = re.sub(r"(?m)^ship = .*$", f"ship = {name}", head)
    tags = [re.search(r"\n\tpart = (\S+)", "\n" + blk).group(1) for blk in blocks]
    craft = Craft(lib, name, "probeCoreOcto.v2")   # used only to render new part blocks
    craft.parts = []
    qx90 = (math.sqrt(0.5), 0.0, 0.0, math.sqrt(0.5))
    added = {}

    def place(parent_tag, name_, phi_deg, r):
        phi = math.radians(phi_deg)
        x0, y0, z0 = [float(v) for v in re.search(r"\n\tpos = (.*)",
                      blocks[tags.index(parent_tag)]).group(1).split(",")]
        pos = (x0 + r * math.sin(phi), y0 - r * math.cos(phi), z0)
        rot = quat_mul(qx90, (0.0, math.sin(phi / 2), 0.0, math.cos(phi / 2)))
        part = craft._new(name_, pos, rot, 1)
        part.srfn = f"srfAttach,{parent_tag}"
        added.setdefault(parent_tag, []).append(part.tag)
        return part

    intake = next(t for t in tags if t.startswith("MK1IntakeFuselage_"))
    body = next(t for t in tags if t.startswith("radialEngineBody_"))
    place(intake, "probeCoreOcto.v2", 180.0, 0.70)
    if with_seps:
        for parent in (body, intake):
            for phi in (135.0, 225.0):
                place(parent, "sepMotor1", phi, 0.625)
    for parent, new_tags in added.items():
        i = tags.index(parent)
        blocks[i] = blocks[i].replace("\n\tattm = ", "".join(f"\n\tlink = {t}" for t in new_tags) + "\n\tattm = ", 1)
    text = head + "".join("\nPART\n" + blk for blk in blocks) + "\n" + "".join(craft._block(p) for p in craft.parts)
    return name, text


def main():
    root = pathlib.Path(sys.argv[1])
    lib = PartLibrary(root)
    out = root / "saves" / "Ikria Physical Verification" / "Ships" / "VAB"
    for build in (a1, a2, a3, a3d, a3e, a3f, a5):
        craft = build(lib)
        path = out / f"{craft.name}.craft"
        text = craft.text()
        if path.exists() and path.read_text(encoding="utf-8") != text:
            raise SystemExit(f"refusing to change the existing article {path.name}")
        path.write_text(text, encoding="utf-8")
        masses = ", ".join(p.name for p in craft.parts)
        print(f"{path.name}: {len(craft.parts)} parts: {masses}")
    sph = root / "saves" / "Ikria Physical Verification" / "Ships" / "SPH"
    for with_seps in (False, True):
        name, text = aircraft(lib, root, with_seps)
        path = sph / f"{name}.craft"
        if path.exists() and path.read_text(encoding="utf-8") != text:
            raise SystemExit(f"refusing to change the existing article {path.name}")
        path.write_text(text, encoding="utf-8")
        print(f"{path.name}: {text.count(chr(10) + 'PART' + chr(10))} parts")


if __name__ == "__main__":
    main()
