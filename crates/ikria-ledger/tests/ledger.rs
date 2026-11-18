//! The join of the enumeration with a hand-written save, covering collected, partially
//! collected, unmatched, repeated, cap-mismatch and multiplier-mismatch subjects, and the
//! refusals.

use ikria_ledger::{
    Body, CapCheck, Enumeration, Ledger, MultiplierCheck, ScaleError, ScienceScale, Situation,
    SubjectKind, science_scale,
};
use ikria_save::{NonNegative, Save, ScienceSubject};

const FIXTURE: &str = include_str!("fixtures/ledger.sfs");
const TOLERANCE: f64 = 1e-6;

fn enumeration() -> Enumeration {
    Enumeration::phase0().expect("the tables hold no defect")
}

fn save() -> Save {
    Save::parse(FIXTURE).expect("the fixture parses")
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= TOLERANCE
}

fn base(e: &Enumeration, id: &str) -> f64 {
    e.get(id)
        .unwrap_or_else(|| panic!("{id} is enumerated"))
        .base_value()
        .as_points()
}

#[test]
fn partially_collected_subject_has_its_remainder() {
    let e = enumeration();
    let ledger = Ledger::from_save(&e, &save()).expect("the subjects extract");
    let entry = ledger
        .entry("crewReport@KerbinSrfLandedLaunchPad")
        .expect("an entry");
    // The first of the repeated identifier is joined.
    assert!(close(entry.collected(), 0.9));
    assert!(close(entry.possible(), 5.0 * 0.3));
    assert!(close(entry.remaining(), entry.possible() - 0.9));
    assert_eq!(entry.cap_check(), CapCheck::Agrees);
    assert!(close(entry.excess(), 0.0));
}

#[test]
fn fully_collected_subject_agrees_within_single_precision() {
    let e = enumeration();
    let ledger = Ledger::from_save(&e, &save()).expect("the subjects extract");
    let entry = ledger
        .entry("temperatureScan@KerbinFlyingHigh")
        .expect("an entry");
    assert_eq!(entry.cap_check(), CapCheck::Agrees);
    assert_eq!(entry.multiplier_check(), MultiplierCheck::Agrees);
    // The single precision residue is clamped to exactly zero.
    assert!(entry.possible() - entry.collected() > 0.0);
    assert_eq!(entry.remaining(), 0.0);
    assert_eq!(entry.excess(), 0.0);
    assert!(close(entry.possible(), 8.0 * 0.9));
}

fn subject(id: &str, science: f64, cap: f64, multiplier: f64) -> ScienceSubject {
    let n = |v| NonNegative::new(v).expect("a value not negative");
    ScienceSubject {
        id: id.to_owned(),
        title: String::new(),
        science: n(science),
        cap: n(cap),
        data_scale: n(1.0),
        subject_value: n(multiplier),
        scientific_value: n(0.0),
        apply_science_scale: Some(true),
    }
}

#[test]
fn remaining_is_zero_only_within_the_tolerance() {
    let e = enumeration();
    let id = "crewReport@KerbinFlyingHigh";
    let possible = base(&e, id);
    let cases = [
        (possible - 4e-6, 0.0),
        (possible + 4e-6, 0.0),
        (possible - 0.01, 0.01),
        (0.0, possible),
    ];
    for (collected, expected) in cases {
        let subjects = [subject(id, collected, possible, 0.9)];
        let ledger = Ledger::build(&e, &subjects);
        let entry = ledger.entry(id).expect("an entry");
        assert!(close(entry.remaining(), expected), "{collected}");
        if expected == 0.0 {
            assert_eq!(entry.remaining(), 0.0, "{collected}");
            assert_eq!(entry.excess(), 0.0, "{collected}");
        }
    }
}

#[test]
fn cap_mismatches_are_reported_not_resolved() {
    let e = enumeration();
    let ledger = Ledger::from_save(&e, &save()).expect("the subjects extract");
    let goo = ledger.entry("mysteryGoo@MunInSpaceLow").expect("an entry");
    assert_eq!(
        goo.cap_check(),
        CapCheck::Differs {
            save: 50.0,
            enumerated: base(&e, "mysteryGoo@MunInSpaceLow"),
        }
    );
    assert!(close(goo.remaining(), goo.possible() - 20.0));

    // A save holding more than the enumeration allows has no remainder and an excess.
    let baro = ledger
        .entry("barometerScan@KerbinFlyingLow")
        .expect("an entry");
    assert!(matches!(baro.cap_check(), CapCheck::Differs { save, .. } if close(save, 9.0)));
    assert!(close(baro.remaining(), 0.0));
    assert!(close(baro.excess(), 9.0 - baro.possible()));

    let ids: Vec<&str> = ledger.cap_differences().map(|x| x.subject().id()).collect();
    // In the enumeration's order, where Kerbin precedes the Mun and features come last.
    assert_eq!(
        ids,
        [
            "barometerScan@KerbinFlyingLow",
            "mysteryGoo@MunInSpaceLow",
            "ROCScience_MunStone@MunSrfLanded"
        ]
    );
    assert_eq!(ledger.totals().cap_differences, 3);
}

#[test]
fn multiplier_mismatches_are_reported_apart_from_caps() {
    let e = enumeration();
    let ledger = Ledger::from_save(&e, &save()).expect("the subjects extract");
    // The Mun goo has a wrong cap in the save but the right multiplier.
    let goo = ledger.entry("mysteryGoo@MunInSpaceLow").expect("an entry");
    assert_eq!(goo.multiplier_check(), MultiplierCheck::Agrees);
    // A save that applied the Mun's landed multiplier to a surface feature would show
    // that the enumeration's assumed multiplier of one is wrong.
    let feature = ledger
        .entry("ROCScience_MunStone@MunSrfLanded")
        .expect("an entry");
    assert!(feature.subject().multiplier_is_assumed());
    assert_eq!(
        feature.multiplier_check(),
        MultiplierCheck::Differs {
            save: 4.0,
            enumerated: 1.0
        }
    );
    assert!(close(feature.excess(), 72.0 - feature.possible()));
    let ids: Vec<&str> = ledger
        .multiplier_differences()
        .map(|x| x.subject().id())
        .collect();
    assert_eq!(
        ids,
        [
            "barometerScan@KerbinFlyingLow",
            "ROCScience_MunStone@MunSrfLanded"
        ]
    );
    assert_eq!(ledger.totals().multiplier_differences, 2);
    let none = Ledger::build(&e, &[]);
    assert!(
        none.entries()
            .iter()
            .all(|x| x.multiplier_check() == MultiplierCheck::NotInSave)
    );
}

#[test]
fn the_account_holds_all_the_science_of_the_save() {
    let e = enumeration();
    let save = save();
    let subjects = save.science_subjects().expect("the subjects extract");
    let ledger = Ledger::build(&e, &subjects);
    let account = ledger.save_account();
    let whole: f64 = subjects.iter().map(|s| s.science.get()).sum();
    assert_eq!(account.records, subjects.len());
    assert_eq!(account.matched_records, ledger.totals().in_save);
    assert!(close(account.total(), whole));
    assert!(close(account.matched, ledger.totals().collected));
    assert!(close(account.unmatched, ledger.unmatched_collected()));
    assert!(close(account.repeated, 1.5));
    assert!(close(ledger.repeated_collected(), 1.5));

    let empty = Ledger::build(&e, &[]).save_account();
    assert_eq!(empty.records, 0);
    assert!(close(empty.total(), 0.0));
    assert!(empty.repeated.is_sign_positive() && empty.unmatched.is_sign_positive());
}

#[test]
fn unmatched_and_repeated_subjects_are_kept_with_their_science() {
    let e = enumeration();
    let save = save();
    let subjects = save.science_subjects().expect("the subjects extract");
    let ledger = Ledger::build(&e, &subjects);
    let unmatched: Vec<(&str, f64)> = ledger
        .unmatched()
        .iter()
        .map(|u| (u.id.as_str(), u.collected))
        .collect();
    assert_eq!(
        unmatched,
        [
            ("asteroidSample@KerbinInSpaceLow_PotatoroidAAA-001", 30.0),
            ("recovery@KerbinFlew", 6.0),
            ("crewreport@KerbinSrfLandedLaunchPad", 0.25),
            ("crewReport@KerbinSrfLanded<unnamedlocation1>", 0.5),
        ]
    );
    assert!(close(ledger.unmatched_collected(), 36.75));
    let repeated: Vec<&str> = ledger.repeated().iter().map(|r| r.id.as_str()).collect();
    assert_eq!(repeated, ["crewReport@KerbinSrfLandedLaunchPad"]);

    // Every save subject is accounted for exactly once.
    let joined = ledger
        .entries()
        .iter()
        .filter(|x| x.save().is_some())
        .count();
    assert_eq!(
        joined + ledger.unmatched().len() + ledger.repeated().len(),
        subjects.len()
    );
}

#[test]
fn totals_follow_from_the_entries() {
    let e = enumeration();
    let ledger = Ledger::from_save(&e, &save()).expect("the subjects extract");
    let totals = ledger.totals();
    let possible: f64 = e
        .subjects()
        .iter()
        .map(|s| s.base_value().as_points())
        .sum();
    assert_eq!(totals.subjects, e.len());
    assert_eq!(totals.in_save, 5);
    assert!(close(totals.possible, possible));
    assert!(close(
        totals.collected,
        0.9 + 7.199_999_81 + 20.0 + 9.0 + 72.0
    ));
    // The excesses of the barometer and the feature do not offset any other remainder,
    // and the temperature scan's residue is clamped to zero.
    let expected_remaining = possible
        - 0.9
        - base(&e, "temperatureScan@KerbinFlyingHigh")
        - 20.0
        - base(&e, "barometerScan@KerbinFlyingLow")
        - base(&e, "ROCScience_MunStone@MunSrfLanded");
    assert!(close(totals.remaining, expected_remaining));

    let (p, c, r) = totals.points(ScienceScale::HARD);
    assert!(close(p, totals.possible * 0.6));
    assert!(close(c, totals.collected * 0.6));
    assert!(close(r, totals.remaining * 0.6));
}

#[test]
fn grouped_totals_partition_the_whole() {
    let e = enumeration();
    let ledger = Ledger::from_save(&e, &save()).expect("the subjects extract");
    let whole = ledger.totals();
    let groups = [
        ledger
            .by_body()
            .into_iter()
            .map(|(_, t)| t)
            .collect::<Vec<_>>(),
        ledger
            .by_situation()
            .into_iter()
            .map(|(_, t)| t)
            .collect::<Vec<_>>(),
        ledger
            .by_experiment()
            .into_iter()
            .map(|(_, t)| t)
            .collect::<Vec<_>>(),
        ledger
            .by_kind()
            .into_iter()
            .map(|(_, t)| t)
            .collect::<Vec<_>>(),
    ];
    for group in groups {
        assert_eq!(
            group.iter().map(|t| t.subjects).sum::<usize>(),
            whole.subjects
        );
        assert_eq!(
            group.iter().map(|t| t.in_save).sum::<usize>(),
            whole.in_save
        );
        assert!(close(
            group.iter().map(|t| t.possible).sum(),
            whole.possible
        ));
        assert!(close(
            group.iter().map(|t| t.collected).sum(),
            whole.collected
        ));
        assert!(close(
            group.iter().map(|t| t.remaining).sum(),
            whole.remaining
        ));
    }
    let mun = ledger
        .by_body()
        .into_iter()
        .find(|(b, _)| *b == Body::Mun)
        .map(|(_, t)| t)
        .expect("the Mun");
    assert!(close(mun.collected, 20.0 + 72.0));
    let high = ledger
        .by_situation()
        .into_iter()
        .find(|(s, _)| *s == Situation::FlyingHigh)
        .map(|(_, t)| t)
        .expect("flying high");
    assert!(close(high.collected, 7.199_999_81));
    let location = ledger
        .by_kind()
        .into_iter()
        .find(|(k, _)| *k == SubjectKind::SpaceCenterLocation)
        .map(|(_, t)| t)
        .expect("locations");
    assert!(close(location.collected, 0.9));
    assert_eq!(ledger.by_experiment().len(), 13 + 34);
}

#[test]
fn a_save_without_subjects_leaves_everything_remaining() {
    let e = enumeration();
    let ledger = Ledger::build(&e, &[]);
    let totals = ledger.totals();
    assert_eq!(totals.in_save, 0);
    assert!(close(totals.collected, 0.0));
    assert!(close(totals.remaining, totals.possible));
    assert!(ledger.unmatched().is_empty());
    assert!(ledger.repeated().is_empty());
    assert_eq!(ledger.cap_differences().count(), 0);
    assert!(
        ledger
            .entries()
            .iter()
            .all(|x| x.cap_check() == CapCheck::NotInSave)
    );
}

#[test]
fn the_science_scale_is_read_from_the_save() {
    assert_eq!(science_scale(&save()), Ok(ScienceScale::HARD));
}

fn game(body: &str) -> Save {
    Save::parse(&format!("GAME\n{{\n{body}\n}}\n")).expect("the text parses")
}

#[test]
fn a_missing_or_invalid_science_scale_is_refused() {
    assert_eq!(
        science_scale(&game("")),
        Err(ScaleError::Missing("PARAMETERS"))
    );
    assert_eq!(
        science_scale(&game("PARAMETERS\n{\n}")),
        Err(ScaleError::Missing("PARAMETERS/CAREER"))
    );
    assert_eq!(
        science_scale(&game("PARAMETERS\n{\nCAREER\n{\n}\n}")),
        Err(ScaleError::Missing(
            "PARAMETERS/CAREER/ScienceGainMultiplier"
        ))
    );
    for bad in ["abc", "0", "-0.6", "NaN", "inf", ""] {
        let text = format!("PARAMETERS\n{{\nCAREER\n{{\nScienceGainMultiplier = {bad}\n}}\n}}");
        assert_eq!(
            science_scale(&game(&text)),
            Err(ScaleError::Invalid(bad.to_owned())),
            "{bad}"
        );
    }
}

#[test]
fn a_save_without_research_is_refused() {
    let e = enumeration();
    assert!(Ledger::from_save(&e, &game("")).is_err());
}

#[test]
fn a_malformed_subject_is_refused_by_the_reader() {
    let e = enumeration();
    let text = "SCENARIO\n{\nname = ResearchAndDevelopment\nScience\n{\nid = crewReport@KerbinFlyingHigh\ntitle = x\ndsc = 1\nscv = 0\nsbv = 1\nsci = -1\ncap = 5\n}\n}";
    assert!(Ledger::from_save(&e, &game(text)).is_err());
}
