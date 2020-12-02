use std::str::FromStr;

use super::{FnPuzzle, Result, Stage};

pub struct PasswordPolicy(char, u8, u8);

#[allow(dead_code)]
pub struct PasswordEntry {
    raw: String,
    policy: PasswordPolicy,
    password: String,
}

fn parse_inputs(inputs: &[&str]) -> Result<Vec<PasswordEntry>> {
    let mut entries = Vec::new();
    for input in inputs {
        let raw = input.clone().into();
        let mut input = input.split("-");
        let least = input
            .nth(0)
            .map(u8::from_str)
            .expect("Malformed input: Couldn't extract least")?;
        let mut input = input.nth(0).expect("Malformed input").split(" ");
        let most = input
            .nth(0)
            .map(u8::from_str)
            .expect("Malformed input: Couldn't extract most")?;
        let character = input
            .nth(0)
            .and_then(|s| s.chars().nth(0))
            .expect("Malformed input");
        let password = input
            .nth(0)
            .expect("Malformed input: Couldn't find password")
            .into();
        let policy = PasswordPolicy(character, least, most);
        let entry = PasswordEntry {
            raw,
            policy,
            password,
        };
        entries.push(entry);
    }
    Ok(entries)
}

#[puzzle(2, first, parse_inputs)]
pub fn first_stage(inputs: Vec<PasswordEntry>) -> Result<usize> {
    let mut count: usize = 0;
    for input in inputs {
        let policy = input.policy;
        let mut char_count: u8 = 0;
        for c in input.password.chars() {
            if c == policy.0 {
                char_count += 1;
            }
        }
        if char_count >= policy.1 && char_count <= policy.2 {
            count += 1;
        }
    }

    Ok(count)
}

#[puzzle(2, second, parse_inputs)]
fn second_stage(inputs: Vec<PasswordEntry>) -> Result<usize> {
    let mut count: usize = 0;
    for input in inputs {
        let policy = input.policy;
        let matches = input
            .password
            .char_indices()
            .filter(|(index, character)| {
                (*index == policy.1 as usize - 1 || *index == policy.2 as usize - 1)
                    && *character == policy.0
            })
            .count();
        if matches == 1 {
            count += 1;
        }
    }

    Ok(count)
}
