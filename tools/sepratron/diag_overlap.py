"""Report bounding-box overlaps between parts of each article on the pad, without firing.

kRPC's Part.bounding_box gives each part's axis-aligned box in a reference frame. Boxes
of attached parts touch by design, so the report lists overlaps deeper than a tolerance,
between a sepratron and any part other than its parent. After a separation, such a
sepratron sits inside another vessel's collider, which TP-0002 identifies as the likely
cause of the loss of the first abort pod.

Arguments: the game's `saves` directory, then article names."""
import itertools
import pathlib
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from flightlib import Run  # noqa: E402

TOLERANCE_M = 0.01


def overlap(a, b):
    depth = []
    for axis in range(3):
        depth.append(min(a[1][axis], b[1][axis]) - max(a[0][axis], b[0][axis]))
    return min(depth)


def main():
    saves, articles = sys.argv[1], sys.argv[2:]
    run = Run("diag-overlap", saves)
    try:
        for craft in articles:
            vessel = run.launch(craft)
            frame = vessel.reference_frame
            boxes = {p: p.bounding_box(frame) for p in vessel.parts.all}
            found = []
            for a, b in itertools.combinations(boxes, 2):
                if "sepMotor1" not in (a.name, b.name):
                    continue
                sep, other = (a, b) if a.name == "sepMotor1" else (b, a)
                if sep.parent == other:
                    continue
                depth = overlap(boxes[a], boxes[b])
                if depth > TOLERANCE_M:
                    found.append({"sepratron_box": boxes[sep], "other": other.name,
                                  "depth_m": round(depth, 3)})
            sep_box = next(boxes[p] for p in boxes if p.name == "sepMotor1")
            size = [round(hi - lo, 3) for lo, hi in zip(*sep_box)]
            run.event("overlaps", craft=craft, sepratron_box_size_m=size, count=len(found),
                      overlaps=[{k: v for k, v in f.items() if k != "sepratron_box"} for f in found])
            vessel.recover()
            time.sleep(4)
    finally:
        run.save()


if __name__ == "__main__":
    main()
