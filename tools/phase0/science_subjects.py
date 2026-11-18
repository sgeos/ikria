"""Count science subjects and their capped value, Phase 0 experiment 7.

Inputs: tmp/inventory.json from the running game (bodies, biomes, atmospheres), the
experiment caps and masks from the game's ScienceDefs.cfg, the wiki's body multipliers,
and the wiki's list of Space Center location biomes. Excludes asteroid and comet samples,
which C4 bounds per class, Breaking Ground's deployed experiments, and the laboratory.
Writes tmp/science_subjects.json."""
import json

HARD = 0.6
LANDED, SPLASHED, FLOW, FHIGH, SLOW, SHIGH = 1, 2, 4, 8, 16, 32
SITS = [("landed", LANDED), ("splashed", SPLASHED), ("flying_low", FLOW),
        ("flying_high", FHIGH), ("space_low", SLOW), ("space_high", SHIGH)]
# id: (cap, situation mask, biome mask, requires atmosphere)
EXPS = {
    "crewReport": (5, 63, 7, False), "evaReport": (8, 63, 23, False),
    "mysteryGoo": (13, 63, 3, False), "surfaceSample": (40, 3, 3, False),
    "mobileMaterialsLab": (32, 63, 3, False), "temperatureScan": (8, 63, 7, False),
    "barometerScan": (12, 63, 3, False), "seismicScan": (22, 1, 1, False),
    "gravityScan": (22, 51, 51, False), "atmosphereAnalysis": (24, 13, 13, True),
    "infraredTelescope": (22, 32, 0, False), "magnetometer": (45, 48, 0, False),
    "evaScience": (25, 63, 0, False),
}
# Wiki multipliers: landed, splashed, flying low, flying high, space low, space high.
MULT = {
    "Sun": (None, None, 1, 1, 11, 2), "Moho": (10, None, None, None, 8, 7),
    "Eve": (8, 8, 6, 6, 7, 5), "Gilly": (9, None, None, None, 8, 6),
    "Kerbin": (0.3, 0.4, 0.7, 0.9, 1, 1.5), "Mun": (4, None, None, None, 3, 2),
    "Minmus": (5, None, None, None, 4, 2.5), "Duna": (8, None, 5, 5, 7, 5),
    "Ike": (8, None, None, None, 7, 5), "Dres": (8, None, None, None, 7, 6),
    "Jool": (None, None, 12, 9, 7, 6), "Laythe": (14, 12, 11, 10, 9, 8),
    "Vall": (12, None, None, None, 9, 8), "Tylo": (12, None, None, None, 10, 8),
    "Bop": (12, None, None, None, 9, 8), "Pol": (12, None, None, None, 9, 8),
    "Eeloo": (15, None, None, None, 12, 10),
}
KSC_BIOMES = 37
# The game's 34 surface feature definitions, 12 at a cap of 30 and 11 each at 45 and 60, in
# Breaking Ground's ScienceDefs.cfg. Corrected on 2026-11-18 from a 12, 12 and 10 split.
ROC_CAPS_TOTAL = 30 * 12 + 45 * 11 + 60 * 11

inv = {b["name"]: b for b in json.load(open("tmp/inventory.json"))}
subjects, value, per_body = 0, 0.0, {}
for body, m in MULT.items():
    biomes = len(inv[body]["biomes"]) or 1
    atm = inv[body]["has_atmosphere"]
    n_body, v_body = 0, 0.0
    for exp, (cap, smask, bmask, req_atm) in EXPS.items():
        if req_atm and not atm:
            continue
        for i, (sit, bit) in enumerate(SITS):
            mult = m[i]
            if mult is None or not (smask & bit):
                continue
            count = biomes if (bmask & bit) else 1
            n_body += count
            v_body += count * cap * mult
    per_body[body] = {"subjects": n_body, "normal": round(v_body, 1), "hard": round(v_body * HARD, 1)}
    subjects += n_body
    value += v_body

ksc_caps = sum(cap for exp, (cap, smask, bmask, req) in EXPS.items() if (smask & LANDED) and (bmask & LANDED))
ksc_subjects = KSC_BIOMES * sum(1 for exp, (cap, smask, bmask, req) in EXPS.items() if (smask & LANDED) and (bmask & LANDED))
ksc_value = KSC_BIOMES * ksc_caps * MULT["Kerbin"][0]
out = {
    "bodies": per_body,
    "body_subjects": subjects, "body_normal": round(value), "body_hard": round(value * HARD),
    "ksc_location_subjects": ksc_subjects, "ksc_normal": round(ksc_value), "ksc_hard": round(ksc_value * HARD),
    "surface_feature_subjects": 34, "surface_feature_hard_if_unscaled": round(ROC_CAPS_TOTAL * HARD),
}
out["total_subjects"] = subjects + ksc_subjects + 34
out["total_hard"] = out["body_hard"] + out["ksc_hard"] + out["surface_feature_hard_if_unscaled"]
out["total_normal_without_features"] = out["body_normal"] + out["ksc_normal"]
json.dump(out, open("tmp/science_subjects.json", "w"), indent=1)
for b, d in sorted(per_body.items(), key=lambda kv: -kv[1]["hard"]):
    print(f"  {b:7} subjects {d['subjects']:4}  hard {d['hard']:9,.0f}")
print("bodies:", subjects, "subjects,", f"{out['body_normal']:,} normal, {out['body_hard']:,} hard")
print("Space Center location biomes:", ksc_subjects, "subjects,", f"{out['ksc_hard']:,} hard")
print("surface features:", 34, f"subjects, {out['surface_feature_hard_if_unscaled']:,} hard if unscaled")
print("total:", out["total_subjects"], "subjects,", f"{out['total_hard']:,} hard;", f"{out['total_normal_without_features']:,} normal without features")
