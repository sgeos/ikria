"""Bounded uncrewed mechanics probe for IKR-TS-001A in the verification save.

This is a Python experiment, not Keleusma qualification. Part-level ignition and
parachute deployment may bypass an empty capsule's control authority. Never use this
probe in the career or as evidence that such commands are career-valid. The caller
must first place the craft on the pad and recover every crew member from it.

Arguments are the game's saves directory and a new output JSON path. The raw log is
flushed after every sample. Failure leaves the game running and does not retry a
command or pretend that a burning solid motor can be shut down.
"""
import argparse
import collections
import datetime
import hashlib
import json
import math
import pathlib
import signal
import sys
import time

CRAFT = 'IKR-TS-001A'
SAVE = 'Ikria Physical Verification'


def should_deploy(altitude, vertical_speed, solid_fuel, safe):
    """Deploy only on a safe descent after burnout, with finite telemetry."""
    if not all(math.isfinite(x) for x in (altitude, vertical_speed, solid_fuel)):
        raise ValueError('non-finite descent telemetry')
    if solid_fuel < 0:
        raise ValueError('negative fuel')
    return safe and 5 < altitude < 1500 and vertical_speed < -1 and solid_fuel < 0.01


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('saves', type=pathlib.Path)
    parser.add_argument('output', type=pathlib.Path)
    parser.add_argument('--craft', choices=['IKR-TS-001A', 'IKR-TS-001B'], default=CRAFT)
    args = parser.parse_args()
    if args.output.exists():
        raise ValueError('refusing to overwrite an existing run')
    args.output.parent.mkdir(parents=True, exist_ok=True)

    # Imported only for an explicitly invoked live run. Unit tests need no kRPC.
    import krpc
    sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
    from guard import loaded_save

    log = {'date_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'purpose': 'uncrewed mechanics only, not career qualification',
           'debug_service_uses': [], 'actions': [], 'samples': []}

    def write_log():
        args.output.write_text(json.dumps(log, indent=2, allow_nan=False) + '\n')

    def deadline(*_):
        raise TimeoutError('verification probe exceeded 300 seconds')

    signal.signal(signal.SIGALRM, deadline)
    signal.alarm(300)
    conn = None
    try:
        conn = krpc.connect(name='ikria-early-hop', address='127.0.0.1',
                            stream_port=None, timeout=10)
        sc = conn.space_center
        log['save'] = loaded_save(conn, args.saves)
        if log['save'] != SAVE or sc.game_mode != sc.GameMode.career:
            raise RuntimeError('not the verification career')
        v = sc.active_vessel
        if v.name != args.craft or v.crew_count != 0:
            raise RuntimeError('wrong craft or crew aboard')
        if v.situation != sc.VesselSituation.pre_launch or v.packed:
            raise RuntimeError('craft is not on the pad and in physics')
        engines, chutes = v.parts.engines, v.parts.parachutes
        if len(engines) != 1 or len(chutes) != 1 or engines[0].active:
            raise RuntimeError('unexpected engine or parachute configuration')
        chute = chutes[0]
        if chute.state != sc.ParachuteState.stowed:
            raise RuntimeError('parachute is not stowed')
        expected = {'mk1pod.v2': 1, 'parachuteSingle': 1, 'GooExperiment': 2,
                    'solidBooster.sm.v2': 1, 'basicFin': 3}
        if collections.Counter(part.name for part in v.parts.all) != expected:
            raise RuntimeError('unexpected part inventory')
        fuel, thrust = (70, .35) if args.craft.endswith('A') else (35, .20)
        if abs(v.resources.amount('SolidFuel') - fuel) > .001 or abs(engines[0].thrust_limit - thrust) > .000001:
            raise RuntimeError('fuel or thrust differs from the test revision')
        craft_file = args.saves / SAVE / 'Ships' / 'VAB' / (args.craft + '.craft')
        log['craft_file_sha256'] = hashlib.sha256(craft_file.read_bytes()).hexdigest()
        log.update(name=v.name, control_state=str(v.control.state),
                   link=v.comms.can_communicate, crew=v.crew_count,
                   initial_parts=len(v.parts.all), initial_mass_kg=v.mass,
                   funds_before=sc.funds, science_before=sc.science,
                   start_ut=sc.ut)
        log['debug_flags_before'] = {
            name: getattr(conn.debug, name) for name in (
                'infinite_electricity', 'infinite_propellant', 'no_crash_damage',
                'unbreakable_joints', 'ignore_max_temperature')}
        write_log()
        # Mark before sending. A failed response can leave application uncertain.
        log['actions'].append({'call': 'Engine.active', 'value': True,
                               'ut': sc.ut, 'status': 'requested'})
        write_log()
        engines[0].active = True
        log['actions'][-1]['status'] = 'returned'
        deployed = False
        airborne = False
        start = time.monotonic()
        flight = v.flight(v.orbit.body.reference_frame)
        for _ in range(1000):
            if time.monotonic() - start > 240:
                raise TimeoutError('flight did not end in 240 seconds')
            sample = {'ut': sc.ut, 'surface_altitude_m': flight.surface_altitude,
                      'sea_level_altitude_m': flight.mean_altitude,
                      'vertical_speed_m_s': flight.vertical_speed,
                      'speed_m_s': flight.speed, 'pitch_deg': flight.pitch,
                      'latitude_deg': flight.latitude, 'longitude_deg': flight.longitude,
                      'g_force': flight.g_force, 'solid_fuel': v.resources.amount('SolidFuel'),
                      'situation': str(v.situation), 'parts': len(v.parts.all),
                      'chute_state': str(chute.state), 'chute_safe': str(chute.safe_state)}
            log['samples'].append(sample)
            write_log()
            if v.situation == sc.VesselSituation.flying:
                airborne = True
            if airborne and v.situation in (sc.VesselSituation.landed, sc.VesselSituation.splashed):
                log['outcome'] = 'landed' if len(v.parts.all) == log['initial_parts'] else 'landed with part loss'
                log['final_parts'] = len(v.parts.all)
                sc.save('ipv-' + args.craft.lower() + '-after')
                break
            if not deployed and should_deploy(sample['surface_altitude_m'],
                    sample['vertical_speed_m_s'], sample['solid_fuel'],
                    chute.safe_state == sc.ParachuteSafeState.safe):
                log['actions'].append({'call': 'Parachute.deploy', 'ut': sc.ut,
                                       'status': 'requested'})
                write_log()
                chute.deploy()
                log['actions'][-1]['status'] = 'returned'
                deployed = True
            if not airborne and time.monotonic() - start > 15:
                raise RuntimeError('ignition did not produce flight')
            time.sleep(0.25)
        else:
            raise TimeoutError('sample bound reached')
        log['duration_game_s'] = sc.ut - log['start_ut']
        log['peak_surface_altitude_m'] = max(s['surface_altitude_m'] for s in log['samples'])
        print(json.dumps({k: v for k, v in log.items() if k != 'samples'}, indent=2), flush=True)
    except BaseException as error:
        log['error'] = f'{type(error).__name__}: {error}'
        write_log()
        raise
    finally:
        write_log()
        signal.alarm(0)
        if conn is not None:
            conn.close()


if __name__ == '__main__':
    main()
