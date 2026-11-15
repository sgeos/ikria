"""Identify which save the running game has loaded, since kRPC does not report it.

The game is asked to write a uniquely named save, the saves directory is searched for
that file, and the file is deleted again. Import `loaded_save` and call it before doing
anything that must happen only in one save.

Argument when run directly: the path of the game's `saves` directory."""
import pathlib
import sys
import time
import uuid


def loaded_save(conn, saves_dir):
    """Return the folder name of the save the game has loaded."""
    name = "ikria-guard-" + uuid.uuid4().hex[:12]
    conn.space_center.save(name)
    deadline = time.time() + 15
    found = []
    while time.time() < deadline and not found:
        found = list(pathlib.Path(saves_dir).glob(f"*/{name}.sfs"))
        if not found:
            time.sleep(0.5)
    if len(found) != 1:
        raise RuntimeError(f"could not identify the loaded save, found {found}")
    folder = found[0].parent.name
    for extra in found[0].parent.glob(f"{name}.*"):
        extra.unlink()
    return folder


if __name__ == "__main__":
    import krpc

    c = krpc.connect(name="ikria-guard", address="127.0.0.1")
    print("loaded save:", loaded_save(c, sys.argv[1]), "| mode:", c.space_center.game_mode,
          "| scene:", c.krpc.game_scene)
    c.close()
