use std::{collections::HashMap, error::Error, str::FromStr};

pub mod day1;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Copy, Clone)]
pub enum Stage {
    First = 0,
    Second = 1,
}

impl FromStr for Stage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" | "first" => Ok(Stage::First),
            "1" | "second" => Ok(Stage::Second),
            _ => Err("Couldn't parse stage".into()),
        }
    }
}

pub trait Puzzle {
    fn day() -> u8;
    fn run(inputs: &[&str], stage: Stage) -> Result<()>;
}

pub struct PuzzleRegistry {
    registry: HashMap<u8, fn(&[&str], Stage) -> Result<()>>,
}

impl PuzzleRegistry {
    pub fn new() -> PuzzleRegistry {
        PuzzleRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn register<P: Puzzle>(&mut self) {
        self.registry.insert(P::day(), P::run);
    }

    pub fn run(&self, day: u8, inputs: &[&str], stage: Stage) -> Result<()> {
        if let Some(puzzle) = self.registry.get(&day) {
            puzzle(inputs, stage)?;
        }

        Ok(())
    }
}
