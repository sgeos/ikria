"""Print a comparison table from early-hop telemetry logs, without game access."""
import argparse
import json
import pathlib


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('logs', type=pathlib.Path, nargs='+')
    args = parser.parse_args()
    print('| Revision | Fuel units | Thrust limit | Empty mass, kg | Peak height above terrain, m | Peak sampled load, g | Duration, s | Final parts |')
    print('|---|---|---|---|---|---|---|---|')
    for path in args.logs:
        run = json.loads(path.read_text())
        if run.get('outcome') != 'landed' or run.get('error') or not run.get('samples'):
            raise ValueError(f'{path} is not a completed intact landing')
        if run['final_parts'] != run['initial_parts']:
            raise ValueError(f'{path} lost parts')
        revision = run['name'][-1]
        fuel, thrust = {'A': (70, 35), 'B': (35, 20)}[revision]
        peak_g = max(sample['g_force'] for sample in run['samples'])
        print(f"| {revision} | {fuel} | {thrust}% | {run['initial_mass_kg']:.1f} | "
              f"{run['peak_surface_altitude_m']:.1f} | {peak_g:.2f} | "
              f"{run['duration_game_s']:.2f} | {run['final_parts']} |")


if __name__ == '__main__':
    main()
