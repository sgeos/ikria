//! Tests of the extractors on their fixtures, and of every refusal of a missing or
//! malformed field.

use std::path::Path;

use ikria_save::{
    BodyIndex, ContractList, ContractState, ExtractError, Facility, FacilityLevel, KerbalType,
    RosterStatus, Save, TechState, Trait, VesselSituation, VesselType,
};

fn fixture(name: &str) -> Save {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    Save::read(&path).expect("fixture parses")
}

/// Wraps a body of text in a `GAME` node and parses it.
fn game(body: &str) -> Save {
    Save::parse(&format!("GAME\n{{\n{body}\n}}\n")).expect("test save parses")
}

/// Wraps fields in a `SCENARIO` node with the given name, inside a `GAME` node.
fn with_scenario(name: &str, body: &str) -> Save {
    game(&format!("SCENARIO\n{{\nname = {name}\n{body}\n}}"))
}

fn assert_missing_field<T: core::fmt::Debug>(result: Result<T, ExtractError>, want: &str) {
    match result {
        Err(ExtractError::MissingField { field, .. }) => assert_eq!(field, want),
        other => panic!("expected missing `{want}`, got {other:?}"),
    }
}

fn assert_invalid_field<T: core::fmt::Debug>(result: Result<T, ExtractError>, want: &str) {
    match result {
        Err(ExtractError::InvalidField { field, .. }) => assert_eq!(field, want),
        other => panic!("expected invalid `{want}`, got {other:?}"),
    }
}

fn assert_missing_node<T: core::fmt::Debug>(result: Result<T, ExtractError>, want: &str) {
    match result {
        Err(ExtractError::MissingNode { node, .. }) => assert_eq!(node, want),
        other => panic!("expected missing `{want}`, got {other:?}"),
    }
}

// --- Agency ------------------------------------------------------------------------

#[test]
fn agency_totals_are_read_from_their_scenarios() {
    let agency = fixture("agency.sfs").agency().unwrap();
    assert_eq!(agency.funds.get(), 2_181_250.676_021_337_5);
    assert_eq!(agency.science.get(), 29.766_666_4);
    assert_eq!(agency.reputation.get(), -12.5);
}

#[test]
fn agency_refuses_a_missing_scenario_or_field_and_non_finite_numbers() {
    let save = with_scenario("Funding", "funds = 1");
    assert_missing_node(save.agency(), "SCENARIO name = ResearchAndDevelopment");
    let save = game(
        "SCENARIO\n{\nname = Funding\n}\nSCENARIO\n{\nname = ResearchAndDevelopment\nsci = 1\n}\nSCENARIO\n{\nname = Reputation\nrep = 1\n}",
    );
    assert_missing_field(save.agency(), "funds");
    for bad in ["NaN", "inf", "-infinity", "1,5", "lots", ""] {
        let save = game(&format!(
            "SCENARIO\n{{\nname = Funding\nfunds = {bad}\n}}\nSCENARIO\n{{\nname = ResearchAndDevelopment\nsci = 1\n}}\nSCENARIO\n{{\nname = Reputation\nrep = 1\n}}"
        ));
        assert_invalid_field(save.agency(), "funds");
    }
}

// --- Facilities ----------------------------------------------------------------------

#[test]
fn facility_levels_convert_from_fractions() {
    let facilities = fixture("facilities.sfs").facilities().unwrap();
    let got: Vec<(Facility, u8)> = facilities
        .iter()
        .map(|f| (f.facility, f.level.index()))
        .collect();
    assert_eq!(
        got,
        [
            (Facility::LaunchPad, 1),
            (Facility::Runway, 0),
            (Facility::VehicleAssemblyBuilding, 2),
            (Facility::SpaceplaneHangar, 0),
            (Facility::TrackingStation, 1),
            (Facility::AstronautComplex, 0),
            (Facility::MissionControl, 1),
            (Facility::ResearchAndDevelopment, 2),
            (Facility::Administration, 0),
            (Facility::FlagPole, 0),
        ],
        "the modified facility is ignored and a near fraction is accepted"
    );
}

#[test]
fn facility_level_conversion_round_trips_and_refuses_other_fractions() {
    for index in 0..FacilityLevel::LEVELS {
        let level = FacilityLevel::new(index).unwrap();
        let fraction = f64::from(index) / f64::from(FacilityLevel::LEVELS - 1);
        assert_eq!(FacilityLevel::from_fraction(fraction), Some(level));
        assert_eq!(level.displayed(), index + 1);
    }
    assert_eq!(FacilityLevel::new(FacilityLevel::LEVELS), None);
    for bad in [
        -0.5,
        0.25,
        0.75,
        1.5,
        0.5 + 2.0 * FacilityLevel::TOLERANCE,
        f64::NAN,
    ] {
        assert_eq!(FacilityLevel::from_fraction(bad), None, "{bad}");
    }
}

#[test]
fn facilities_refuse_a_missing_or_invalid_level() {
    let save = with_scenario("ScenarioUpgradeableFacilities", "SpaceCenter/Runway\n{\n}");
    assert_missing_field(save.facilities(), "lvl");
    let save = with_scenario(
        "ScenarioUpgradeableFacilities",
        "SpaceCenter/Runway\n{\nlvl = 0.25\n}",
    );
    match save.facilities() {
        Err(ExtractError::InvalidField {
            node, field, line, ..
        }) => {
            assert_eq!((node.as_str(), field), ("SpaceCenter/Runway", "lvl"));
            assert_eq!(line, 8);
        }
        other => panic!("unexpected {other:?}"),
    }
    assert_missing_node(
        game("").facilities(),
        "SCENARIO name = ScenarioUpgradeableFacilities",
    );
}

// --- Research ------------------------------------------------------------------------

#[test]
fn technologies_keep_their_parts_as_written() {
    let techs = fixture("research.sfs").technologies().unwrap();
    let ids: Vec<&str> = techs.iter().map(|t| t.id.as_str()).collect();
    assert_eq!(ids, ["start", "engineering101", "basicRocketry"]);
    assert!(techs.iter().all(|t| t.state == TechState::Available));
    assert_eq!(techs[0].cost, 0);
    assert_eq!(
        techs[0].parts,
        ["mk1pod.v2", "mk1pod.v2", "solidBooster.sm.v2"]
    );
    assert_eq!(techs[1].cost, 5);
    assert!(techs[2].parts.is_empty());
}

#[test]
fn technologies_refuse_missing_or_invalid_fields() {
    let rnd =
        |body: &str| with_scenario("ResearchAndDevelopment", &format!("Tech\n{{\n{body}\n}}"));
    assert_missing_field(rnd("state = Available\ncost = 1").technologies(), "id");
    assert_missing_field(rnd("id = a\ncost = 1").technologies(), "state");
    assert_missing_field(rnd("id = a\nstate = Available").technologies(), "cost");
    assert_invalid_field(
        rnd("id = \nstate = Available\ncost = 1").technologies(),
        "id",
    );
    assert_invalid_field(
        rnd("id = a\nstate = Researched\ncost = 1").technologies(),
        "state",
    );
    for bad in ["-1", "1.5", "4294967296", "x"] {
        assert_invalid_field(
            rnd(&format!("id = a\nstate = Unavailable\ncost = {bad}")).technologies(),
            "cost",
        );
    }
}

#[test]
fn science_subjects_are_read_with_every_number() {
    let subjects = fixture("research.sfs").science_subjects().unwrap();
    assert_eq!(subjects.len(), 2);
    let first = &subjects[0];
    assert_eq!(first.id, "recovery@KerbinFlew");
    assert_eq!(
        first.title,
        "Recovery of a vessel returned from a flight over Kerbin"
    );
    assert_eq!(
        (first.science.get(), first.cap.get()),
        (6.0, 6.0),
        "a subject collected to its cap"
    );
    assert_eq!(first.data_scale.get(), 1.0);
    assert_eq!(first.subject_value.get(), 5.0);
    assert_eq!(first.scientific_value.get(), 0.0);
    assert_eq!(first.apply_science_scale, Some(true));
    let second = &subjects[1];
    assert_eq!(second.science.get() * 2.0, second.cap.get());
    assert_eq!(second.apply_science_scale, None);
}

#[test]
fn science_subjects_refuse_missing_or_invalid_numbers() {
    let full = [
        ("id", "a@b"),
        ("title", "T"),
        ("sci", "1"),
        ("cap", "2"),
        ("dsc", "1"),
        ("sbv", "1"),
        ("scv", "0.5"),
    ];
    let subject = |skip: &str, replace: Option<(&str, &str)>| {
        let mut body = String::from("Science\n{\n");
        for (key, value) in full {
            if key == skip {
                continue;
            }
            let value = match replace {
                Some((k, v)) if k == key => v,
                _ => value,
            };
            body.push_str(&format!("{key} = {value}\n"));
        }
        if let Some(("asc", v)) = replace {
            body.push_str(&format!("asc = {v}\n"));
        }
        body.push('}');
        with_scenario("ResearchAndDevelopment", &body)
    };
    assert!(subject("", None).science_subjects().is_ok());
    for (key, _) in full {
        assert_missing_field(subject(key, None).science_subjects(), key);
    }
    for key in ["sci", "cap", "dsc", "sbv", "scv"] {
        assert_invalid_field(subject("", Some((key, "-1"))).science_subjects(), key);
        assert_invalid_field(subject("", Some((key, "NaN"))).science_subjects(), key);
    }
    assert_invalid_field(subject("", Some(("asc", "yes"))).science_subjects(), "asc");
    assert_eq!(
        subject("", Some(("asc", "FALSE")))
            .science_subjects()
            .unwrap()[0]
            .apply_science_scale,
        Some(false)
    );
}

// --- Roster --------------------------------------------------------------------------

#[test]
fn the_roster_is_read_with_optional_experience() {
    let roster = fixture("roster.sfs").roster().unwrap();
    let names: Vec<&str> = roster.iter().map(|k| k.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "Jebediah Kerman",
            "Bill Kerman",
            "Bob Kerman",
            "Valentina Kerman"
        ]
    );
    assert!(roster.iter().all(|k| k.kerbal_type == KerbalType::Crew));
    let traits: Vec<&Trait> = roster.iter().map(|k| &k.kerbal_trait).collect();
    assert_eq!(
        traits,
        [
            &Trait::Pilot,
            &Trait::Engineer,
            &Trait::Scientist,
            &Trait::Pilot
        ]
    );
    let statuses: Vec<RosterStatus> = roster.iter().map(|k| k.status).collect();
    assert_eq!(
        statuses,
        [
            RosterStatus::Missing,
            RosterStatus::Available,
            RosterStatus::Assigned,
            RosterStatus::Assigned
        ]
    );
    let jeb = &roster[0];
    assert_eq!(jeb.career_flights, Some(3));
    assert_eq!(jeb.extra_xp.map(|x| x.get()), Some(0.0));
    assert_eq!(jeb.experience, None);
    assert_eq!(roster[1].career_flights, None);
    assert_eq!(roster[1].extra_xp, None);
    assert_eq!(roster[2].extra_xp.map(|x| x.get()), Some(1.5));
    let val = &roster[3];
    assert_eq!(val.experience.map(|x| x.get()), Some(8.0));
    assert_eq!(val.experience_level, Some(2));
}

#[test]
fn the_roster_keeps_an_unknown_trait_and_refuses_unknown_types_and_states() {
    let kerbal = |body: &str| game(&format!("ROSTER\n{{\nKERBAL\n{{\n{body}\n}}\n}}"));
    let ok = kerbal("name = Bill Kerman\ntype = Tourist\ntrait = Botanist\nstate = Dead");
    let read = ok.roster().unwrap();
    assert_eq!(read[0].kerbal_trait, Trait::Other("Botanist".into()));
    assert_eq!(read[0].kerbal_type, KerbalType::Tourist);
    assert_eq!(read[0].status, RosterStatus::Dead);
    assert_missing_field(
        kerbal("type = Crew\ntrait = Pilot\nstate = Available").roster(),
        "name",
    );
    assert_missing_field(
        kerbal("name = Bill Kerman\ntype = Crew\nstate = Available").roster(),
        "trait",
    );
    assert_invalid_field(
        kerbal("name = Bill Kerman\ntype = Hero\ntrait = Pilot\nstate = Available").roster(),
        "type",
    );
    assert_invalid_field(
        kerbal("name = Bill Kerman\ntype = Crew\ntrait = Pilot\nstate = Resting").roster(),
        "state",
    );
    assert_invalid_field(
        kerbal("name = Bill Kerman\ntype = Crew\ntrait = Pilot\nstate = Available\nexperienceLevel = -1")
            .roster(),
        "experienceLevel",
    );
    assert_invalid_field(
        kerbal("name = Bill Kerman\ntype = Crew\ntrait = Pilot\nstate = Available\nCAREER_LOG\n{\nflight = many\n}")
            .roster(),
        "flight",
    );
    assert_missing_node(game("").roster(), "ROSTER");
}

// --- Vessels -------------------------------------------------------------------------

#[test]
fn vessels_include_a_kerbal_outside_a_vessel_and_a_flag() {
    let vessels = fixture("vessels.sfs").vessels().unwrap();
    let summary: Vec<(&str, VesselType, VesselSituation, Option<&str>)> = vessels
        .iter()
        .map(|v| {
            (
                v.name.as_str(),
                v.vessel_type,
                v.situation,
                v.body.stock_name(),
            )
        })
        .collect();
    assert_eq!(
        summary,
        [
            (
                "#autoLOC_501180",
                VesselType::Probe,
                VesselSituation::Orbiting,
                Some("Kerbin")
            ),
            (
                "Valentina Ship",
                VesselType::Ship,
                VesselSituation::Prelaunch,
                Some("Kerbin")
            ),
            (
                "Bob Kerman",
                VesselType::Eva,
                VesselSituation::Landed,
                Some("Mun")
            ),
            (
                "Mun Flag",
                VesselType::Flag,
                VesselSituation::Landed,
                Some("Mun")
            ),
            (
                "Debris",
                VesselType::Debris,
                VesselSituation::Landed,
                Some("Kerbin")
            ),
        ]
    );
    let eva: Vec<_> = vessels.iter().filter(|v| v.is_eva()).collect();
    assert_eq!(eva.len(), 1);
    assert_eq!(
        eva[0].crew,
        [eva[0].name.clone()],
        "an EVA vessel carries its kerbal"
    );
    let flags: Vec<_> = vessels.iter().filter(|v| v.is_flag()).collect();
    assert_eq!(flags.len(), 1);
    assert!(flags[0].crew.is_empty());
    assert_eq!(vessels[1].crew, ["Valentina Kerman"]);
    assert_eq!(vessels[1].part_count, 2);
    assert_eq!(vessels[1].landed_at.as_deref(), Some("LaunchPad"));
    assert_eq!(vessels[0].landed_at, None, "an empty landedAt is absent");
    assert_eq!(vessels[0].pid, "0000000000000000000000000000000a");
    assert_eq!(vessels[0].persistent_id, 1001);
}

#[test]
fn stock_body_names_cover_the_stock_system_only() {
    assert_eq!(BodyIndex(0).stock_name(), Some("Sun"));
    assert_eq!(BodyIndex(16).stock_name(), Some("Eeloo"));
    assert_eq!(BodyIndex(17).stock_name(), None);
    assert_eq!(BodyIndex(u32::MAX).stock_name(), None);
}

#[test]
fn vessels_refuse_missing_or_invalid_fields() {
    let full = "pid = 0a\npersistentId = 1\nname = V\ntype = Probe\nsit = FLYING";
    let vessel = |body: &str| {
        game(&format!(
            "FLIGHTSTATE\n{{\nVESSEL\n{{\n{body}\nORBIT\n{{\nREF = 1\n}}\n}}\n}}"
        ))
    };
    assert!(vessel(full).vessels().is_ok());
    for key in ["pid", "persistentId", "name", "type", "sit"] {
        let body: Vec<&str> = full.lines().filter(|l| !l.starts_with(key)).collect();
        assert_missing_field(vessel(&body.join("\n")).vessels(), key);
    }
    for (key, bad) in [
        ("pid", "not-hex"),
        ("pid", ""),
        ("persistentId", "-4"),
        ("name", ""),
        ("type", "Spaceship"),
        ("sit", "Orbiting"),
    ] {
        let body: Vec<String> = full
            .lines()
            .map(|l| {
                if l.starts_with(key) {
                    format!("{key} = {bad}")
                } else {
                    l.to_owned()
                }
            })
            .collect();
        assert_invalid_field(vessel(&body.join("\n")).vessels(), key);
    }
    let no_orbit = game(&format!("FLIGHTSTATE\n{{\nVESSEL\n{{\n{full}\n}}\n}}"));
    assert_missing_node(no_orbit.vessels(), "ORBIT");
    let no_ref = game(&format!(
        "FLIGHTSTATE\n{{\nVESSEL\n{{\n{full}\nORBIT\n{{\n}}\n}}\n}}"
    ));
    assert_missing_field(no_ref.vessels(), "REF");
    assert_missing_node(game("").vessels(), "FLIGHTSTATE");
    assert!(game("FLIGHTSTATE\n{\n}").vessels().unwrap().is_empty());
}

// --- Contracts -----------------------------------------------------------------------

#[test]
fn contracts_are_read_from_both_lists_with_their_values() {
    let contracts = fixture("contracts.sfs").contracts().unwrap();
    let summary: Vec<(&str, ContractState, ContractList)> = contracts
        .iter()
        .map(|c| (c.contract_type.as_str(), c.state, c.list))
        .collect();
    assert_eq!(
        summary,
        [
            (
                "ExplorationContract",
                ContractState::Offered,
                ContractList::Current
            ),
            (
                "CollectScience",
                ContractState::Active,
                ContractList::Current
            ),
            ("PartTest", ContractState::Generated, ContractList::Current),
            (
                "PlantFlag",
                ContractState::Completed,
                ContractList::Finished
            ),
        ]
    );
    let exploration = contracts[0].values.unwrap();
    assert_eq!(exploration.time_expiry.get(), 86_400.0);
    assert_eq!(exploration.funds_advance.get(), 672.0);
    assert_eq!(exploration.funds_completion.get(), 1_248.0);
    assert_eq!(exploration.funds_failure.get(), 672.0);
    assert_eq!(exploration.reputation_completion.get(), 1.0);
    assert_eq!(exploration.science_completion.get(), 0.0);
    assert_eq!(exploration.date_expire.get(), 370_625.962_302_731);
    let science = contracts[1].values.unwrap();
    assert_eq!(science.science_completion.get(), 5.0);
    assert_eq!(science.reputation_failure.get(), 2.857_143);
    assert_eq!(science.date_accepted.get(), 290_000.0);
    assert_eq!(
        science.date_deadline.get(),
        science.date_accepted.get() + science.time_deadline.get(),
        "the deadline falls its duration after acceptance in the fixture"
    );
    assert_eq!(contracts[2].values, None);
    assert_eq!(contracts[2].agent, None);
    let finished = contracts[3].values.unwrap();
    assert_eq!(finished.date_finished.get(), 1_500.0);
    assert_eq!(contracts[3].agent.as_deref(), Some("Kerbal Motion LLC"));
    assert_eq!(contracts[0].guid, "00000000-0000-4000-8000-000000000001");
}

#[test]
fn contracts_refuse_missing_or_invalid_fields() {
    let contract = |body: &str| {
        with_scenario(
            "ContractSystem",
            &format!("CONTRACTS\n{{\nCONTRACT\n{{\n{body}\n}}\n}}"),
        )
    };
    assert_missing_field(contract("type = T\nstate = Active").contracts(), "guid");
    assert_missing_field(contract("guid = g\nstate = Active").contracts(), "type");
    assert_missing_field(contract("guid = g\ntype = T").contracts(), "state");
    assert_invalid_field(
        contract("guid = g\ntype = T\nstate = Pending").contracts(),
        "state",
    );
    for bad in [
        "1,2,3",
        "1,2,3,4,5,6,7,8,9,10,11,12,13",
        "1,2,3,4,5,6,7,8,9,10,11,x",
        "1,2,3,4,5,6,7,8,9,10,11,NaN",
        "",
    ] {
        assert_invalid_field(
            contract(&format!(
                "guid = g\ntype = T\nstate = Active\nvalues = {bad}"
            ))
            .contracts(),
            "values",
        );
    }
    let spaced =
        contract("guid = g\ntype = T\nstate = Active\nvalues = 1, 2,3,4,5,6,7,8,9,10,11,12");
    assert_eq!(
        spaced.contracts().unwrap()[0]
            .values
            .unwrap()
            .time_deadline
            .get(),
        2.0
    );
    assert!(
        with_scenario("ContractSystem", "")
            .contracts()
            .unwrap()
            .is_empty()
    );
    assert_missing_node(game("").contracts(), "SCENARIO name = ContractSystem");
}

// --- Errors --------------------------------------------------------------------------

#[test]
fn extract_errors_name_their_line_and_shorten_long_values() {
    let long = "x".repeat(300);
    // The funds are read first, so the absent scenarios after them are never reached.
    let save = with_scenario("Funding", &format!("funds = {long}"));
    let error = save.agency().unwrap_err();
    let text = error.to_string();
    assert!(text.starts_with("line 6: "), "{text}");
    assert!(text.len() < 200, "message has {} bytes", text.len());
    let missing = with_scenario("Funding", "").agency().unwrap_err();
    assert!(missing.to_string().starts_with("line 3: "), "{missing}");
    let absent = game("").roster().unwrap_err();
    assert!(absent.to_string().contains("ROSTER"), "{absent}");
}
