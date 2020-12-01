use std::str::FromStr;

use super::{Puzzle, Result, Stage};

#[derive(Puzzle)]
#[puzzle(1, parse_inputs, process)]
pub struct Day1 {}

impl Day1 {
    fn parse_inputs(inputs: &[&str]) -> Result<Vec<u32>> {
        let mut numbers = Vec::new();
        for input in inputs {
            numbers.push(u32::from_str(*input)?);
        }
        Ok(numbers)
    }

    fn process(inputs: Vec<u32>, stage: Stage) -> Result<()> {
        'main: for i in &inputs {
            for j in &inputs {
                match stage {
                    Stage::First => {
                        if i + j == 2020 {
                            println!("Solution: {}", i * j);
                            break 'main;
                        }
                    }
                    Stage::Second => {
                        for k in &inputs {
                            if i + j + k == 2020 {
                                println!("Solution: {}", i * j * k);
                                break 'main;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
