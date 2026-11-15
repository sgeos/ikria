"""Science per early stage, first collection at base value. Normal values, and Hard
at 0.6 on the assumption that the reward multiplier scales each subject.

Base values are from the game's experiment definitions. Multipliers are from the
official wiki. Biome counts are from the wiki. Which experiment is per biome in each
situation is from the experiment definitions' biome masks."""

HARD = 0.6
BASE = dict(crew=5, eva=8, goo=10, sample=30, mat=25, temp=8, baro=12)
KERBIN = dict(L=0.3, S=0.4, FL=0.7, FH=0.9, SL=1.0, SH=1.5)
MUN = dict(L=4, SL=3, SH=2)
MINMUS = dict(L=5, SL=4, SH=2.5)


def v(exps, mult):
    return sum(BASE[e] for e in exps) * mult


stages = []

# E0: day one. EVA reports on foot at about 12 Space Center area biomes, crew report
# and goo on the pad and the runway, one Flea hop flying low over Shores, and the
# recovery subject for flight.
e0 = 12 * v(["eva"], KERBIN["L"]) + 2 * v(["crew", "goo"], KERBIN["L"]) \
    + v(["crew", "goo"], KERBIN["FL"]) + 5
stages.append(("E0 day one", e0))

# E1: sounding rockets with crew, goo, thermometer and barometer through flying high,
# space low and space high, the flying-low subjects not yet taken, thermometer and
# barometer on the pad and runway, one splashdown in Water, and suborbit recovery.
four = ["crew", "goo", "temp", "baro"]
e1 = v(four, KERBIN["FH"]) + v(four, KERBIN["SL"]) + v(four, KERBIN["SH"]) \
    + v(["temp", "baro"], KERBIN["FL"]) + 2 * v(["temp", "baro"], KERBIN["L"]) \
    + v(four + ["eva"], KERBIN["S"]) + 8
stages.append(("E1 sounding rockets", e1))

# E2: aircraft. Flying low over the 10 biomes other than Shores, with the per-biome
# crew report and thermometer, and landings in 6 land biomes with crew, goo,
# thermometer, barometer, extravehicular report and materials.
e2 = 10 * v(["crew", "temp"], KERBIN["FL"]) \
    + 6 * v(["crew", "goo", "temp", "baro", "eva", "mat"], KERBIN["L"])
stages.append(("E2 aircraft", e2))

# E3: crewed passes of the Mun and Minmus, high and low space, with crew, goo,
# thermometer, barometer and materials, recovered, and the orbit recovery subject.
five = ["crew", "goo", "temp", "baro", "mat"]
e3 = v(five, MUN["SH"]) + v(five, MUN["SL"]) + v(five, MINMUS["SH"]) \
    + v(five, MINMUS["SL"]) + 10
stages.append(("E3 Mun and Minmus from space", e3))

# E4: landings without extravehicular activity, before the Astronaut Complex level 2.
mun_l = v(five, MUN["L"])
min_l = v(five, MINMUS["L"])
stages.append(("E4 one landed biome, Mun", mun_l))
stages.append(("E4 one landed biome, Minmus", min_l))
stages.append(("E4 all 17 Mun biomes", 17 * mun_l))
stages.append(("E4 all 9 Minmus biomes", 9 * min_l))

# E5: what the second level of the Astronaut Complex adds per landed biome, the
# extravehicular report, and with the second level of Research and Development, the
# surface sample.
stages.append(("E5 per Mun biome, report and sample", v(["eva", "sample"], MUN["L"])))
stages.append(("E5 per Minmus biome, report and sample", v(["eva", "sample"], MINMUS["L"])))

cum = 0.0
for name, s in stages:
    print(f"{name:42} Normal {s:8.1f}   Hard {s * HARD:8.1f}")
early = e0 + e1 + e2 + e3
print(f"{'E0 to E3 total':42} Normal {early:8.1f}   Hard {early * HARD:8.1f}")
all_land = 17 * mun_l + 9 * min_l
print(f"{'E0 to E4, every landed biome':42} Normal {early + all_land:8.1f}   Hard {(early + all_land) * HARD:8.1f}")
