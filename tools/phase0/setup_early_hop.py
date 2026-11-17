"""Checkpoint, place, empty or recover a hop article in the verification save.

Placement carries Jebediah to the pad with the engine off. Empty-capsule placement
hung in the observed setup. The empty operation sends him outside, recovers him,
and returns to the empty article. None of these operations ignites an engine.
Every invocation identifies the loaded save before changing vessel state. A deadline
is an uncertain command outcome, so inspect the game before retrying any operation.
"""
import argparse
import datetime
import json
import pathlib
import signal
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('saves', type=pathlib.Path)
    parser.add_argument('operation', choices=['place', 'empty', 'recover'])
    parser.add_argument('output', type=pathlib.Path)
    parser.add_argument('--craft', choices=['IKR-TS-001A', 'IKR-TS-001B'], required=True)
    args = parser.parse_args()
    if args.output.exists():
        raise ValueError('refusing to overwrite a log')
    args.output.parent.mkdir(parents=True, exist_ok=True)

    import krpc
    sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
    from guard import loaded_save

    log = {'date_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'operation': args.operation, 'craft': args.craft, 'debug_service_uses': []}

    def write_log():
        args.output.write_text(json.dumps(log, indent=2, allow_nan=False) + '\n')

    def deadline(*_):
        raise TimeoutError('setup exceeded 60 seconds, inspect before retrying')

    signal.signal(signal.SIGALRM, deadline)
    signal.alarm(60)
    conn = None
    try:
        conn = krpc.connect(name='ikria-hop-setup', address='127.0.0.1',
                            stream_port=None, timeout=10)
        sc = conn.space_center
        log['save'] = loaded_save(conn, args.saves)
        if log['save'] != 'Ikria Physical Verification' or sc.game_mode != sc.GameMode.career:
            raise RuntimeError('not the verification save')
        log['start_ut'] = sc.ut
        write_log()
        if args.operation == 'place':
            # Never automatically recover an unrelated craft occupying the pad.
            sc.launch_vessel('VAB', args.craft, 'LaunchPad',
                             crew=['Jebediah Kerman'], recover=False)
            time.sleep(3)
            v = sc.active_vessel
            if v.name != args.craft or v.crew_count != 1:
                raise RuntimeError('placement did not produce the expected crewed craft')
        else:
            v = sc.active_vessel
            if v.name != args.craft:
                raise RuntimeError('wrong active vessel')
            if args.operation == 'empty':
                if v.crew_count != 1 or v.situation != sc.VesselSituation.pre_launch:
                    raise RuntimeError('expected a single crew member on the pad')
                if any(engine.active for engine in v.parts.engines):
                    raise RuntimeError('refusing crew removal with an active engine')
                kerbal = v.crew[0].eva()
                time.sleep(3)
                kerbal.control.release_ladder()
                time.sleep(3)
                kerbal.recover()
                time.sleep(3)
                matches = [craft for craft in sc.vessels if craft.name == args.craft]
                if len(matches) != 1:
                    raise RuntimeError('craft identity is ambiguous after recovery')
                sc.active_vessel = matches[0]
                time.sleep(3)
                v = sc.active_vessel
                if v.name != args.craft or v.crew_count != 0:
                    raise RuntimeError('crew removal failed')
            else:
                if v.crew_count != 0 or v.situation not in (
                        sc.VesselSituation.landed, sc.VesselSituation.splashed):
                    raise RuntimeError('recovery requires the empty craft safely landed')
                log.update(parts=len(v.parts.all), funds_before=sc.funds,
                           science_before=sc.science)
                v.recover()
                time.sleep(3)
                log.update(funds_after=sc.funds, science_after=sc.science,
                           scene=str(conn.krpc.game_scene))
                write_log()
                print(json.dumps(log, indent=2), flush=True)
                return
        log.update(name=v.name, crew=v.crew_count, control=str(v.control.state),
                   link=v.comms.can_communicate, situation=str(v.situation), ut=sc.ut,
                   mass_kg=v.mass, dry_mass_kg=v.dry_mass, parts=len(v.parts.all),
                   crew_status=str(sc.get_kerbal('Jebediah Kerman').roster_status))
        sc.save('ipv-' + args.craft.lower() + '-' + args.operation)
        write_log()
        print(json.dumps(log, indent=2), flush=True)
    except BaseException as error:
        log['error'] = f'{type(error).__name__}: {error}'
        write_log()
        raise
    finally:
        signal.alarm(0)
        if conn is not None:
            conn.close()


if __name__ == '__main__':
    main()
