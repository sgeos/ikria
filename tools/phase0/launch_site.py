"""Recover crewed vessels from the launch sites before a launch, as rule R20 requires.

Campaign rule R20, item 1, says that every launch first recovers any crewed vessel at
the launch site explicitly, refuses to launch if it cannot, and never relies on the
launch call to clear crew. In TP-0002 a launch with `recover=True` removed a pod with
Jebediah aboard and he was marked Missing. Every launching script in `tools/phase0`
and `tools/sepratron/flightlib.py` uses this module.

Scope. Only a crewed vessel that is before launch or landed, on Kerbin, and inside one
of the launch areas below counts. A crewed vessel elsewhere on Kerbin or on another
body is neither recovered nor allowed to block a launch, because it may be meant to
stay where it is.

Scenes. kRPC 0.7.0 lists vessels only in the flight scene. Elsewhere it raises
UnsupportedSceneException, observed during the Sepratron program. In the flight scene
the vessels are read through kRPC and the crewed ones in a launch area are recovered.
In any other scene a temporary save is written under a unique neutral name, read and
deleted, as `guard.py` does. A crewed vessel found there in a launch area refuses the
launch, because no recovery is possible from that scene.

Failure. Every doubt refuses the launch. That includes an unreadable vessel, an
unreadable scene, a missing saves directory outside the flight scene, a save that does
not appear or does not parse, and crew still present after recovery. Temporary save
files are deleted in every case, including on error.
"""
import math
import pathlib
import time
import uuid

LAUNCH_BODY = 'Kerbin'
# Kerbin's ORBIT REF in a save file. Kerbol is 0 and Kerbin is 1 in the stock system.
LAUNCH_BODY_REF = '1'
KERBIN_RADIUS_M = 600_000.0
# Launch areas, from the game's waypoints and measured vessel positions. The pad is a
# circle about its centre, and the runway a latitude and longitude rectangle.
PAD_LATITUDE_DEG = -0.0972
PAD_LONGITUDE_DEG = -74.5577
PAD_RADIUS_M = 300.0
RUNWAY_LATITUDE_DEG = (-0.060, -0.035)
RUNWAY_LONGITUDE_DEG = (-74.730, -74.490)
SAVE_SITUATIONS = ('PRELAUNCH', 'LANDED')
SETTLE_S = 3.0
SAVE_WAIT_S = 15.0
SAVE_POLL_S = 0.5
MAX_SAVE_BYTES = 64 * 1024 * 1024


class LaunchSiteError(RuntimeError):
    """The launch site could not be shown free of crew, so nothing may launch."""


def near_launch_site(latitude, longitude):
    """Whether a position in degrees on Kerbin lies in a launch area."""
    if not (math.isfinite(latitude) and math.isfinite(longitude)):
        raise LaunchSiteError('non-finite vessel position')
    lat0 = math.radians(PAD_LATITUDE_DEG)
    north = math.radians(latitude - PAD_LATITUDE_DEG) * KERBIN_RADIUS_M
    east = math.radians(longitude - PAD_LONGITUDE_DEG) * KERBIN_RADIUS_M * math.cos(lat0)
    on_pad = math.hypot(north, east) <= PAD_RADIUS_M
    on_runway = (RUNWAY_LATITUDE_DEG[0] <= latitude <= RUNWAY_LATITUDE_DEG[1]
                 and RUNWAY_LONGITUDE_DEG[0] <= longitude <= RUNWAY_LONGITUDE_DEG[1])
    return on_pad or on_runway


def in_flight_scene(conn):
    try:
        return conn.krpc.game_scene == conn.krpc.GameScene.flight
    except Exception as error:
        raise LaunchSiteError(f'could not read the game scene: {error}') from error


def crewed_in_flight_scene(sc):
    """Return (vessel, crew names) for each crewed vessel in a launch area, read
    through kRPC. Any error while reading a vessel is raised as LaunchSiteError."""
    down = (sc.VesselSituation.pre_launch, sc.VesselSituation.landed)
    found = []
    try:
        for vessel in list(sc.vessels):
            crew = [member.name for member in vessel.crew]
            if not crew or vessel.situation not in down:
                continue
            body = vessel.orbit.body
            if body.name != LAUNCH_BODY:
                continue
            frame = body.reference_frame
            position = vessel.position(frame)
            latitude = body.latitude_at_position(position, frame)
            longitude = body.longitude_at_position(position, frame)
            if near_launch_site(latitude, longitude):
                found.append((vessel, crew))
    except LaunchSiteError:
        raise
    except Exception as error:
        raise LaunchSiteError(f'could not read the vessels at the launch site: {error}') from error
    return found


def parse_config(text):
    """Parse KSP's ConfigNode text into nested dictionaries with 'values' and 'nodes'
    lists. The loop is bounded by the line count. Unbalanced braces are refused."""
    root = {'name': None, 'values': [], 'nodes': []}
    stack = [root]
    pending = None
    for raw in text.splitlines():
        line = raw.split('//', 1)[0].strip()
        if not line:
            continue
        if line == '{':
            if pending is None:
                raise LaunchSiteError('save file has a brace without a node name')
            node = {'name': pending, 'values': [], 'nodes': []}
            stack[-1]['nodes'].append(node)
            stack.append(node)
            pending = None
        elif line == '}':
            if pending is not None or len(stack) == 1:
                raise LaunchSiteError('save file has an unbalanced brace')
            stack.pop()
        elif '=' in line:
            if pending is not None:
                raise LaunchSiteError('save file has a node name without a brace')
            key, value = line.split('=', 1)
            stack[-1]['values'].append((key.strip(), value.strip()))
        else:
            if pending is not None:
                raise LaunchSiteError('save file has a node name without a brace')
            pending = line
    if pending is not None or len(stack) != 1:
        raise LaunchSiteError('save file ended inside a node')
    return root


def value(node, key):
    return next((v for k, v in node['values'] if k == key), None)


def children(node, name):
    return [child for child in node['nodes'] if child['name'] == name]


def crewed_in_save_text(text):
    """Return (vessel name, crew names) for each crewed vessel in a launch area in a
    save file's text."""
    root = parse_config(text)
    games = children(root, 'GAME')
    states = children(games[0], 'FLIGHTSTATE') if len(games) == 1 else []
    if len(states) != 1:
        raise LaunchSiteError('save file has no single GAME FLIGHTSTATE node')
    found = []
    for vessel in children(states[0], 'VESSEL'):
        crew = [v for part in children(vessel, 'PART') for k, v in part['values']
                if k == 'crew' and v]
        orbits = children(vessel, 'ORBIT')
        if not crew or value(vessel, 'sit') not in SAVE_SITUATIONS:
            continue
        if len(orbits) != 1:
            raise LaunchSiteError(f'crewed vessel {value(vessel, "name")} has no single ORBIT')
        if value(orbits[0], 'REF') != LAUNCH_BODY_REF:
            continue
        try:
            latitude = float(value(vessel, 'lat'))
            longitude = float(value(vessel, 'lon'))
        except (TypeError, ValueError) as error:
            raise LaunchSiteError(
                f'crewed vessel {value(vessel, "name")} has no readable position') from error
        if near_launch_site(latitude, longitude):
            found.append((value(vessel, 'name'), crew))
    return found


def crewed_in_save(sc, saves_dir, sleep=time.sleep):
    """Write a temporary save, read the crewed vessels in a launch area from it, and
    delete its files in every case."""
    if saves_dir is None:
        raise LaunchSiteError('outside the flight scene the saves directory is required')
    saves = pathlib.Path(saves_dir)
    if not saves.is_dir():
        raise LaunchSiteError(f'saves directory {saves} does not exist')
    name = 'ikria-site-' + uuid.uuid4().hex[:12]
    try:
        sc.save(name)
        found = []
        for _ in range(int(SAVE_WAIT_S / SAVE_POLL_S) + 1):
            found = list(saves.glob(f'*/{name}.sfs'))
            if found:
                break
            sleep(SAVE_POLL_S)
        if len(found) != 1:
            raise LaunchSiteError(f'the temporary save did not appear once, found {found}')
        if found[0].stat().st_size > MAX_SAVE_BYTES:
            raise LaunchSiteError('the temporary save is too large to check')
        return crewed_in_save_text(found[0].read_text(encoding='utf-8', errors='strict'))
    except LaunchSiteError:
        raise
    except Exception as error:
        raise LaunchSiteError(f'could not check the launch site from a save: {error}') from error
    finally:
        for extra in saves.glob(f'*/{name}.*'):
            extra.unlink()


def clear_crewed_launch_site(conn, saves_dir, record, sleep=time.sleep):
    """Make sure no crewed vessel is in a launch area, or raise LaunchSiteError.

    In the flight scene, recover each such vessel, call `record(crew)` for it and check
    again. In any other scene, refuse if a save shows one. Nothing is launched here."""
    sc = conn.space_center
    if not in_flight_scene(conn):
        found = crewed_in_save(sc, saves_dir, sleep)
        if found:
            raise LaunchSiteError(
                f'crew at the launch site cannot be recovered from this scene, {found}')
        return
    found = crewed_in_flight_scene(sc)
    for vessel, crew in found:
        try:
            recoverable = vessel.recoverable
        except Exception as error:
            raise LaunchSiteError(f'could not check recovery of {crew}: {error}') from error
        if not recoverable:
            raise LaunchSiteError(f'refusing to launch, {crew} aboard an unrecoverable vessel')
        vessel.recover()
        record(crew)
    if found:
        # Recovering the active vessel leaves the flight scene, so check again in
        # whichever scene the game is now in.
        sleep(SETTLE_S)
        if in_flight_scene(conn):
            remaining = [crew for _, crew in crewed_in_flight_scene(sc)]
        else:
            remaining = [crew for _, crew in crewed_in_save(sc, saves_dir, sleep)]
        if remaining:
            raise LaunchSiteError(f'crew still at the launch site after recovery: {remaining}')
