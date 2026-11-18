//! Runs one Keleusma rate damper against three scripted ticks without the game.
//!
//! Run with `cargo run -p ikria-harness --example scripted_host`. Each output line is
//! an existing version 1 harness command message, suitable for inspecting the replay.

use ikria_harness::{
    control::RateDamper,
    harness::Harness,
    host::{ScriptedHost, Tick},
};
use ikria_records::{
    GameTime, HarnessMessage, Quaternion, Situation, Vector3, VesselId, VesselTelemetry, encode,
};
use keleusma::{Arena, vm::DEFAULT_ARENA_CAPACITY};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vessel = VesselId(7);
    let mut ticks = Vec::new();
    for (seconds, rate) in [(0.0, 0.25), (0.02, -0.125), (0.04, 0.0)] {
        ticks.push(Tick {
            ut: GameTime::new(seconds)?,
            vessel: VesselTelemetry {
                id: vessel,
                situation: Situation::Flying,
                body: "Kerbin".into(),
                position: Vector3::new(600_000.0, 0.0, 0.0)?,
                velocity: Vector3::new(0.0, 0.0, 0.0)?,
                rotation: Quaternion::new([0.0, 0.0, 0.0, 1.0])?,
                angular_velocity: Vector3::new(rate, 0.0, 0.0)?,
                mass_kg: 1_000.0,
                available_thrust_n: 0.0,
                altitude_surface_m: 100.0,
                altitude_sea_level_m: 170.0,
                dynamic_pressure_pa: 0.0,
                stage: 1,
                has_link: false,
                resources: vec![],
            },
        });
    }
    let count = ticks.len();
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let mut harness = Harness::new(ScriptedHost::new(ticks), vessel, RateDamper::new(&arena)?);
    for _ in 0..count {
        harness.step()?.ok_or("the script ended early")?;
    }
    if harness.step()?.is_some() {
        return Err("the script did not end".into());
    }
    for applied in harness.host().applied() {
        println!(
            "{}",
            encode(&HarnessMessage::Commands {
                ut: applied.ut,
                commands: vec![applied.command.clone()],
            })?
        );
    }
    Ok(())
}
