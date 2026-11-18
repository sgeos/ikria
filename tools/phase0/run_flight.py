"""Run the separate bridge and harness against an already prepared verification article.

This does not launch, empty or recover a vessel. The bridge performs its own save,
crew and profile checks. Every output path must be new. A timeout is an uncertain
outcome, never a reason to repeat the command. Inspect the retained journals first.
"""
import argparse
import os
import pathlib
import subprocess
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('saves', type=pathlib.Path)
    parser.add_argument('bridge', type=pathlib.Path)
    parser.add_argument('harness', type=pathlib.Path)
    parser.add_argument('output', type=pathlib.Path)
    parser.add_argument('--mode', choices=['monitor', 'pad-abort', 'hop'], default='monitor')
    parser.add_argument('--ticks', type=int, default=50)
    parser.add_argument('--port', type=int, default=50120)
    parser.add_argument('--abort-file', type=pathlib.Path)
    args = parser.parse_args()
    if not 1 <= args.ticks <= 29995 or not 1024 <= args.port <= 65535:
        parser.error('ticks or port outside supported bounds')
    args.output.mkdir(parents=True, exist_ok=False)
    bridge_args = [str(args.bridge.resolve()), str(args.saves.resolve()), 'IKR-TS-001B',
                   str(args.port), str(args.ticks + 5)]
    if args.mode != 'monitor':
        bridge_args.append('--arm-verification')
    bridge_log = args.output / 'bridge.txt'
    with bridge_log.open('x') as stderr:
        bridge = subprocess.Popen(bridge_args, stdout=subprocess.DEVNULL, stderr=stderr)
        try:
            for _ in range(200):
                if bridge.poll() is not None:
                    raise RuntimeError('bridge refused startup, inspect bridge.txt')
                if 'ready on' in bridge_log.read_text():
                    break
                time.sleep(0.1)
            else:
                raise TimeoutError('bridge did not become ready within 20 seconds')
            env = os.environ.copy()
            if args.abort_file:
                env['IKRIA_ABORT_FILE'] = str(args.abort_file.resolve())
            else:
                env.pop('IKRIA_ABORT_FILE', None)
            with (args.output / 'harness.txt').open('x') as harness_log:
                result = subprocess.run(
                    [str(args.harness.resolve()), f'127.0.0.1:{args.port}',
                     str((args.output / 'flight.jsonl').resolve()), args.mode, str(args.ticks)],
                    env=env, stdout=harness_log, stderr=subprocess.STDOUT, timeout=840,
                    check=False)
            bridge_status = bridge.wait(timeout=5)
            if result.returncode or bridge_status:
                raise RuntimeError(f'harness exit {result.returncode}, bridge exit {bridge_status}, inspect logs')
            print('session completed, evidence in', args.output)
        finally:
            if bridge.poll() is None:
                bridge.terminate()
                try:
                    bridge.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    bridge.kill()
                    bridge.wait(timeout=3)


if __name__ == '__main__':
    main()
