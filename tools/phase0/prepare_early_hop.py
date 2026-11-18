"""Derive a local hop test article from the operator's IkriaT0 craft.

No game files are distributed. Only the verification save receives a craft. Existing
files with different contents are refused. The source layout must match the measured
eight-part starting design. The career save is read only to check purchased parts.
"""
import argparse
import hashlib
import json
import pathlib
import re

PARTS = {'mk1pod.v2': 1, 'parachuteSingle': 1, 'GooExperiment': 2,
         'solidBooster.sm.v2': 1, 'basicFin': 3}
REVISIONS = {'A': (70, 35), 'B': (35, 20)}


def derive(source, revision, purchased):
    """Check the source and return a craft with bounded fuel and thrust settings."""
    if revision not in REVISIONS:
        raise ValueError('unknown revision')
    names = re.findall(r'^\s*part = (.*)_\d+$', source, re.M)
    if {name: names.count(name) for name in names} != PARTS:
        raise ValueError('source is not the expected eight-part craft')
    if not set(names) <= purchased:
        raise ValueError('the career has not purchased every part')
    fuel, thrust = REVISIONS[revision]
    for pattern, replacement in [
        (r'^ship = .*$', f'ship = IKR-TS-001{revision}'),
        (r'^(\s*)thrustPercentage = 100$', rf'\g<1>thrustPercentage = {thrust}'),
        (r'^(\s*)amount = 140$', rf'\g<1>amount = {fuel}'),
    ]:
        source, count = re.subn(pattern, replacement, source, flags=re.M)
        if count != 1:
            raise ValueError('source settings differ from the expected template')
    return source


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('saves', type=pathlib.Path)
    parser.add_argument('revision', choices=REVISIONS)
    args = parser.parse_args()
    source = args.saves / 'Ikria Simulated Research and Dev/Ships/VAB/IkriaT0.craft'
    career = args.saves / 'Ikria Automated Space Agency/persistent.sfs'
    # Tech blocks have scalar fields and no nested nodes. Refuse a changed layout.
    text = career.read_text()
    blocks = re.findall(r'\bTech\s*\{([^{}]*)\}', text)
    if not blocks:
        raise ValueError('no scalar Tech blocks found in the career save')
    purchased = set()
    for block in blocks:
        if re.search(r'^\s*state = Available\s*$', block, re.M):
            purchased.update(re.findall(r'^\s*part = (\S+)\s*$', block, re.M))
    result = derive(source.read_text(), args.revision, purchased)
    dest = args.saves / f'Ikria Physical Verification/Ships/VAB/IKR-TS-001{args.revision}.craft'
    if dest.exists() and dest.read_text() != result:
        raise ValueError('refusing to overwrite a different craft')
    dest.write_text(result)
    print(json.dumps({'name': dest.stem,
                      'sha256': hashlib.sha256(result.encode()).hexdigest(),
                      'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                      'career_sha256': hashlib.sha256(career.read_bytes()).hexdigest(),
                      'solid_fuel': REVISIONS[args.revision][0],
                      'thrust_percent': REVISIONS[args.revision][1],
                      'parts': PARTS}, indent=2))


if __name__ == '__main__':
    main()
