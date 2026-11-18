"""Check each article's sepratron force directions on the pad, without firing.

kRPC's Thruster.thrust_direction gives the direction of the force a thruster produces.
For each sepratron, the script reports the cosine between that force and the unit
vector from the part toward the root part, in the vessel's frame. Near +1 means the
sepratron pushes toward the root, which is the upright mounting. Near -1 means it pushes
away from the root, which is the retro mounting. The article is then recovered.

Arguments: the game's `saves` directory, then article names."""
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from flightlib import Run, dot, norm, sub  # noqa: E402


def main():
    saves, articles = sys.argv[1], sys.argv[2:]
    run = Run("diag-orientation", saves)
    for craft in articles:
        vessel = run.launch(craft)
        frame = vessel.reference_frame
        root = vessel.parts.root.position(frame)
        report = []
        for part in run.parts(vessel, "sepMotor1"):
            force = part.engine.thrusters[0].thrust_direction(frame)
            toward_root = sub(root, part.position(frame))
            cosine = dot(force, toward_root) / (norm(force) * norm(toward_root))
            report.append(round(cosine, 3))
        run.event("orientation", craft=craft, cosines=report)
        vessel.recover()
        time.sleep(4)
    run.save()


if __name__ == "__main__":
    main()
