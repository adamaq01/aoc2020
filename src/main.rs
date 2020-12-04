use std::error::Error;

mod puzzles;

use clap::Clap;
use puzzles::{day1, day2, day3, day4, PuzzleRegistry, Stage};
use reqwest::blocking::Client;

#[macro_use]
extern crate aoc_proc_macros;

/// Run the algorithm to solve the AoC 2020 problem of the provided day and stage
#[derive(Clap)]
#[clap(version = "1.0", author = "Adamaq01 <adamthibert01@gmail.com>")]
struct Opts {
    /// Day number
    day: u8,
    /// The stage
    stage: Stage,
    /// The auth token (if not specified it will use the TOKEN environment variable)
    token: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let token = if let Some(token) = opts.token {
        token
    } else {
        dotenv::var("TOKEN")?
    };
    let client = reqwest::blocking::Client::new();
    let mut registry = PuzzleRegistry::new();

    // Register each day
    registry.register(day1::first_stage);
    registry.register(day1::second_stage);
    registry.register(day2::first_stage);
    registry.register(day2::second_stage);
    registry.register(day3::first_stage);
    registry.register(day3::second_stage);
    registry.register(day4::first_stage);
    registry.register(day4::second_stage);

    run(token, client, registry, opts.day, opts.stage)?;

    Ok(())
}

fn run(
    token: String,
    client: Client,
    mut registry: PuzzleRegistry,
    day: u8,
    stage: Stage,
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://adventofcode.com/2020/day/{}/input", day);
    let inputs = client
        .get(url.as_str())
        .header("cookie", format!("session={}", token))
        .send()?
        .text()?
        .trim()
        .into();

    registry.run(day, stage, inputs)?;

    Ok(())
}
