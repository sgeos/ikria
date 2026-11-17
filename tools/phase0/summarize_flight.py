"""Summarize a plain or gzip flight journal without treating a failed run as a pass."""
import argparse
import gzip
import hashlib
import json
import pathlib


def summarize(path):
    """Read at most 64 MiB and report observed telemetry, commands and terminal state."""
    path = pathlib.Path(path)
    opener = gzip.open if path.suffix == '.gz' else open
    samples = []
    commands = []
    acknowledgements = 0
    header = None
    terminal = None
    completion = None
    compute = []
    total = 0
    digest = hashlib.sha256()
    with opener(path, 'rb') as stream:
        for _ in range(100000):
            raw = stream.readline(65536)
            if not raw:
                break
            total += len(raw)
            if len(raw) >= 65536 or total > 64 * 1024 * 1024:
                raise ValueError('journal exceeds record or total size limit')
            digest.update(raw)
            row = json.loads(raw)
            if row.get('event') == 'session':
                header = row
            if row.get('type') == 'sample':
                samples.append(row)
            if row.get('type') == 'applied':
                acknowledgements += 1
            if row.get('event') == 'command_intent':
                compute.append(row['compute_ns'])
                if row['request']['command']['requests']:
                    commands.append(row)
            if row.get('event') == 'complete':
                completion = row
            if row.get('event') == 'session_end':
                terminal = row
        else:
            raise ValueError('too many journal records')
    if not header or not samples:
        raise ValueError('missing session header or samples')
    times = [row['ut'] for row in samples]
    deltas = [b-a for a, b in zip(times, times[1:])]
    return {
        'journal_sha256': digest.hexdigest(), 'uncompressed_bytes': total,
        'session': header, 'samples': len(samples), 'acknowledged_commands': acknowledgements,
        'first_ut': times[0], 'last_ut': times[-1], 'duration_game_s': times[-1]-times[0],
        'tick_delta_min_s': min(deltas, default=None), 'tick_delta_max_s': max(deltas, default=None),
        'maximum_altitude_m': max(row['telemetry']['altitude_m'] for row in samples),
        'max_abs_vertical_speed_m_s': max(abs(row['telemetry']['vertical_speed_m_s']) for row in samples),
        'last_telemetry': samples[-1]['telemetry'], 'discrete_commands': commands,
        'max_compute_ns': max(compute, default=None),
        'completion': completion, 'terminal': terminal,
        'successful_session': bool(terminal and terminal['success']),
        'flight_completed': bool(terminal and terminal['success'] and completion),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('journals', nargs='+', type=pathlib.Path)
    args = parser.parse_args()
    print(json.dumps({path.name: summarize(path) for path in args.journals}, indent=2, allow_nan=False))


if __name__ == '__main__':
    main()
