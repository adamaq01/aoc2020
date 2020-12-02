use std::str::FromStr;

use super::{FnPuzzle, Result, Stage};

fn parse_inputs(inputs: &[&str]) -> Result<Vec<u32>> {
    let mut numbers = Vec::new();
    for input in inputs {
        numbers.push(u32::from_str(*input)?);
    }
    Ok(numbers)
}

#[puzzle(1, first, parse_inputs)]
pub fn first_stage(inputs: Vec<u32>) -> Result<usize> {
    for i in &inputs {
        for j in &inputs {
            if i + j == 2020 {
                return Ok((i * j) as usize);
            }
        }
    }

    Err("Couldn't find solution".into())
}

#[puzzle(1, second, parse_inputs)]
pub fn second_stage(inputs: Vec<u32>) -> Result<usize> {
    for i in &inputs {
        for j in &inputs {
            for k in &inputs {
                if i + j + k == 2020 {
                    return Ok((i * j * k) as usize);
                }
            }
        }
    }

    Err("Couldn't find solution".into())
}
