//! The acceptance of roadmap task T9, that the enumeration's totals match the Phase 0
//! count, and the agreement of the ported rules with their tracked sources.
//!
//! Both sides of each comparison come from tracked files. The expected totals are read
//! from `record/verification/data/science_subjects.json`, the record of the count. The
//! script's tables are parsed from `tools/phase0/science_subjects.py`, and the biomes from
//! `record/verification/data/inventory.json`. A change to any of them that the port does
//! not follow fails here.

use ikria_ledger::catalog::{Body, Experiment, HARD_TENTHS, LOCATION_BIOMES, SURFACE_FEATURES};
use ikria_ledger::{Enumeration, Hundredths, Provenance, Situation, SubjectKind};
use serde_json::Value;

const RECORD: &str = include_str!("../../../record/verification/data/science_subjects.json");
const INVENTORY: &str = include_str!("../../../record/verification/data/inventory.json");
const SCRIPT: &str = include_str!("../../../tools/phase0/science_subjects.py");

fn enumeration() -> Enumeration {
    Enumeration::phase0().expect("the tables hold no defect")
}

fn record() -> Value {
    serde_json::from_str(RECORD).expect("the record is JSON")
}

fn uint(value: &Value, key: &str) -> u64 {
    value[key]
        .as_u64()
        .unwrap_or_else(|| panic!("`{key}` is an unsigned integer"))
}

/// A decimal of the record, such as 1841.7, as an exact count of tenths.
fn tenths(value: &Value, key: &str) -> u64 {
    let number = value[key]
        .as_f64()
        .unwrap_or_else(|| panic!("`{key}` is a number"));
    let scaled = (number * 10.0).round();
    assert!(
        (number * 10.0 - scaled).abs() < 1e-6,
        "`{key}` has one decimal at most"
    );
    // The record's values are below 2^53 and not negative.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let out = scaled as u64;
    out
}

fn len_u64(n: usize) -> u64 {
    u64::try_from(n).expect("a count fits in 64 bits")
}

#[test]
fn per_body_counts_and_values_match_the_record() {
    let e = enumeration();
    let record = record();
    let bodies = record["bodies"].as_object().expect("bodies is an object");
    assert_eq!(bodies.len(), Body::ALL.len());
    for body in Body::ALL {
        let expected = &bodies[body.name()];
        let got = e.totals_where(|s| s.kind() == SubjectKind::Body && s.body() == body);
        assert_eq!(
            len_u64(got.subjects),
            uint(expected, "subjects"),
            "{body} subjects"
        );
        assert_eq!(
            got.hard.round_to_tenths(),
            tenths(expected, "hard"),
            "{body} hard"
        );
        assert_eq!(
            got.base.round_to_tenths(),
            tenths(expected, "normal"),
            "{body} normal"
        );
    }
}

#[test]
fn body_and_location_totals_match_the_record_exactly() {
    let e = enumeration();
    let record = record();
    let body = e.totals_where(|s| s.kind() == SubjectKind::Body);
    assert_eq!(len_u64(body.subjects), uint(&record, "body_subjects"));
    assert_eq!(body.hard.round_to_points(), uint(&record, "body_hard"));
    assert_eq!(body.base.round_to_points(), uint(&record, "body_normal"));

    let location = e.totals_where(|s| s.kind() == SubjectKind::SpaceCenterLocation);
    assert_eq!(
        len_u64(location.subjects),
        uint(&record, "ksc_location_subjects")
    );
    assert_eq!(location.hard.round_to_points(), uint(&record, "ksc_hard"));
    assert_eq!(location.base.round_to_points(), uint(&record, "ksc_normal"));
    assert_eq!(
        body.base.round_to_points() + location.base.round_to_points(),
        uint(&record, "total_normal_without_features")
    );
}

#[test]
fn the_acceptance_totals_match_the_corrected_phase0_record() {
    let e = enumeration();
    let record = record();
    // The subject count matches exactly, 3,104.
    assert_eq!(len_u64(e.len()), uint(&record, "total_subjects"));

    let features = e.totals_where(|s| s.kind() == SubjectKind::SurfaceFeature);
    assert_eq!(
        len_u64(features.subjects),
        uint(&record, "surface_feature_subjects")
    );
    let recorded_features = uint(&record, "surface_feature_hard_if_unscaled");
    assert_eq!(features.hard.round_to_points(), recorded_features);

    // The total is the sum of the three rounded parts, as the script forms it.
    let parts = [
        SubjectKind::Body,
        SubjectKind::SpaceCenterLocation,
        SubjectKind::SurfaceFeature,
    ]
    .map(|k| e.totals_where(|s| s.kind() == k).hard.round_to_points());
    let total: u64 = parts.iter().sum();
    // The record's Hard total, corrected on 2026-11-18 to 217,827, is reproduced.
    assert_eq!(total, uint(&record, "total_hard"));
}

#[test]
fn identifiers_are_unique_and_well_formed() {
    let e = enumeration();
    for subject in e.subjects() {
        let id = subject.id();
        assert!(!id.contains(' '), "{id}");
        let (experiment, rest) = id.split_once('@').expect("one @");
        assert_eq!(experiment, subject.source().id());
        let rest = rest
            .strip_prefix(subject.body().name())
            .expect("the body follows the @");
        let biome = rest
            .strip_prefix(subject.situation().id())
            .expect("the situation follows the body");
        match subject.biome() {
            None => assert!(biome.is_empty(), "{id}"),
            Some(b) => assert_eq!(biome, b.name.replace(' ', ""), "{id}"),
        }
        assert_eq!(e.get(id), Some(subject));
    }
}

#[test]
fn known_identifiers_have_the_game_form() {
    let e = enumeration();
    // Observed in the subject identifiers of a real save on 2026-11-18.
    for id in [
        "crewReport@KerbinSrfLandedLaunchPad",
        "crewReport@KerbinFlyingHigh",
        "crewReport@KerbinFlyingLowSouthernIceShelf",
        "evaReport@KerbinInSpaceLowGrasslands",
        "evaReport@KerbinSrfSplashedShores",
        "barometerScan@KerbinSrfSplashedWater",
        "barometerScan@KerbinFlyingLow",
        "mysteryGoo@KerbinSrfLandedR&D",
        "surfaceSample@KerbinSrfLandedAstronautComplex",
        "temperatureScan@KerbinFlyingLowNorthernIceShelf",
        "mobileMaterialsLab@KerbinInSpaceHigh",
    ] {
        let subject = e.get(id).unwrap_or_else(|| panic!("{id} is enumerated"));
        assert_ne!(
            subject.biome().map(|b| b.provenance),
            Some(Provenance::Placeholder)
        );
    }
    // Constructed under the rules, for bodies without biomes, multi-word biomes and
    // surface features.
    let sun = e
        .get("crewReport@SunInSpaceLow")
        .expect("no biome at the Sun");
    assert_eq!(sun.hard_value(), Hundredths::new(5 * 110 * 6));
    assert!(e.get("magnetometer@JoolInSpaceHigh").is_some());
    assert!(
        e.get("surfaceSample@LaytheSrfSplashedTheSagenSea")
            .is_some()
    );
    assert!(
        e.get("seismicScan@IkeSrfLandedSouthEasternMountainRange")
            .is_some()
    );
    let feature = e
        .get("ROCScience_DunaEjectaOnIke@IkeSrfLanded")
        .expect("a feature named for Duna stands on Ike");
    assert_eq!(feature.hard_value(), Hundredths::new(30 * 10 * 6));
}

#[test]
fn identifiers_the_rules_exclude_are_absent() {
    let e = enumeration();
    for id in [
        // Splashed exists only at Kerbin, Eve and Laythe.
        "crewReport@DunaSrfSplashedHighlands",
        // Atmosphere analysis needs an atmosphere.
        "atmosphereAnalysis@MunSrfLandedHighlands",
        // Crew reports do not depend on the biome when flying high.
        "crewReport@KerbinFlyingHighShores",
        // Surface samples are landed or splashed only.
        "surfaceSample@KerbinFlyingLowShores",
        // Matching is exact, so case matters.
        "crewreport@KerbinSrfLandedLaunchPad",
        // Recovery, asteroid samples and deployed experiments are left out.
        "recovery@KerbinFlew",
        "deployedSeismicSensor@MunSrfLanded",
        "",
    ] {
        assert!(e.get(id).is_none(), "{id}");
    }
}

#[test]
fn location_subjects_cover_every_location_once_per_landed_biome_experiment() {
    let e = enumeration();
    let experiments = Experiment::ALL
        .iter()
        .filter(|x| x.allows(Situation::SrfLanded) && x.biome_dependent(Situation::SrfLanded))
        .count();
    assert_eq!(experiments, 10);
    for location in &LOCATION_BIOMES {
        let n = e
            .subjects()
            .iter()
            .filter(|s| {
                s.kind() == SubjectKind::SpaceCenterLocation
                    && s.biome().map(|b| b.name) == Some(location.name)
            })
            .count();
        assert_eq!(n, experiments, "{}", location.name);
    }
    let placeholders = e.subjects().iter().filter(|s| s.is_placeholder()).count();
    let unnamed = LOCATION_BIOMES
        .iter()
        .filter(|l| l.provenance == Provenance::Placeholder)
        .count();
    assert_eq!(placeholders, unnamed * experiments);
}

// --- Agreement with the script's tables ---------------------------------------------

/// The text of a dictionary literal in the script, from `NAME = {` to its closing line.
fn literal(name: &str) -> &'static str {
    let start = SCRIPT
        .find(&format!("\n{name} = {{"))
        .unwrap_or_else(|| panic!("the script defines {name}"));
    let body = &SCRIPT[start..];
    let end = body.find("\n}").expect("the literal closes");
    &body[..end]
}

/// The entries `"key": (a, b, ...)` of a dictionary literal, in order.
fn tuples(text: &str) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('"') {
        let after = &rest[open + 1..];
        let close = after.find('"').expect("a closing quote");
        let key = after[..close].to_owned();
        let after = &after[close + 1..];
        let paren = after.find('(').expect("a tuple");
        let end = after.find(')').expect("a closing parenthesis");
        let fields = after[paren + 1..end]
            .split(',')
            .map(|f| f.trim().to_owned())
            .collect();
        out.push((key, fields));
        rest = &after[end + 1..];
    }
    out
}

/// A multiplier literal, such as `0.3`, `11` or `None`, as tenths.
fn multiplier_tenths(text: &str) -> Option<u16> {
    if text == "None" {
        return None;
    }
    let (whole, fraction) = text.split_once('.').unwrap_or((text, "0"));
    assert_eq!(fraction.len(), 1, "one decimal at most in {text}");
    let whole: u16 = whole.parse().expect("an integer part");
    let fraction: u16 = fraction.parse().expect("a decimal digit");
    Some(whole * 10 + fraction)
}

fn script_constant(name: &str) -> &str {
    let line = SCRIPT
        .lines()
        .find(|l| l.starts_with(&format!("{name} = ")))
        .unwrap_or_else(|| panic!("the script defines {name}"));
    let value = &line[name.len() + 3..];
    value.split('#').next().expect("a value").trim()
}

#[test]
fn experiments_agree_with_the_script() {
    let rows = tuples(literal("EXPS"));
    assert_eq!(rows.len(), Experiment::ALL.len());
    for ((key, fields), experiment) in rows.iter().zip(Experiment::ALL) {
        assert_eq!(key, experiment.id());
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0], experiment.cap().to_string(), "{key} cap");
        assert_eq!(fields[1], experiment.situation_mask().to_string(), "{key}");
        assert_eq!(fields[2], experiment.biome_mask().to_string(), "{key}");
        let atmosphere = if experiment.requires_atmosphere() {
            "True"
        } else {
            "False"
        };
        assert_eq!(fields[3], atmosphere, "{key}");
    }
}

#[test]
fn multipliers_agree_with_the_script() {
    let rows = tuples(literal("MULT"));
    assert_eq!(rows.len(), Body::ALL.len());
    for ((key, fields), body) in rows.iter().zip(Body::ALL) {
        assert_eq!(key, body.name());
        let expected: Vec<Option<u16>> = fields.iter().map(|f| multiplier_tenths(f)).collect();
        let got: Vec<Option<u16>> = body
            .multipliers()
            .iter()
            .map(|m| m.map(|m| m.tenths()))
            .collect();
        assert_eq!(got, expected, "{key}");
    }
}

#[test]
fn constants_agree_with_the_script() {
    assert_eq!(
        multiplier_tenths(script_constant("HARD")),
        Some(HARD_TENTHS)
    );
    assert_eq!(
        script_constant("KSC_BIOMES"),
        LOCATION_BIOMES.len().to_string()
    );
    // The script's situation order is the mask bit order.
    let sits = script_constant("LANDED, SPLASHED, FLOW, FHIGH, SLOW, SHIGH");
    let bits: Vec<String> = Situation::ALL.iter().map(|s| s.bit().to_string()).collect();
    assert_eq!(sits, bits.join(", "));
}

#[test]
fn surface_features_agree_with_the_script() {
    // The script's expression is a sum of cap times count terms.
    let expression = script_constant("ROC_CAPS_TOTAL");
    let mut script_count = 0_u64;
    let mut script_caps = 0_u64;
    for term in expression.split('+') {
        let (cap, count) = term.split_once('*').expect("cap * count");
        let cap: u64 = cap.trim().parse().expect("a cap");
        let count: u64 = count.trim().parse().expect("a count");
        script_count += count;
        script_caps += cap * count;
    }
    let caps: u64 = SURFACE_FEATURES.iter().map(|f| u64::from(f.cap)).sum();
    assert_eq!(script_count, len_u64(SURFACE_FEATURES.len()));
    assert_eq!(script_caps, caps);
}

#[test]
fn biomes_and_atmospheres_agree_with_the_inventory() {
    // Python wrote the Sun's infinite sphere of influence as `Infinity`, which strict
    // JSON lacks. No field compared here holds it, so it becomes null before parsing.
    let text = INVENTORY.replace(": Infinity", ": null");
    let inventory: Value = serde_json::from_str(&text).expect("the inventory is JSON");
    let bodies = inventory.as_array().expect("a list of bodies");
    assert_eq!(bodies.len(), Body::ALL.len());
    for body in Body::ALL {
        let entry = bodies
            .iter()
            .find(|b| b["name"] == body.name())
            .unwrap_or_else(|| panic!("{body} is in the inventory"));
        let biomes: Vec<&str> = entry["biomes"]
            .as_array()
            .expect("biomes is a list")
            .iter()
            .map(|b| b.as_str().expect("a biome name"))
            .collect();
        assert_eq!(biomes, body.biomes(), "{body} biomes");
        assert_eq!(
            entry["has_atmosphere"].as_bool(),
            Some(body.has_atmosphere()),
            "{body} atmosphere"
        );
    }
}
