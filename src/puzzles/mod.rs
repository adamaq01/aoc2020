use std::{collections::HashMap, error::Error};

pub mod day1;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Puzzle {
    fn day() -> u8;
    fn run(inputs: &[&str]) -> Result<()>;
}

pub struct PuzzleRegistry {
    registry: HashMap<u8, fn(&[&str]) -> Result<()>>,
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

    pub fn run(&self, day: u8, inputs: &[&str]) -> Result<()> {
        if let Some(puzzle) = self.registry.get(&day) {
            puzzle(inputs)?;
        }

        Ok(())
    }
}
