use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use super::{FnPuzzle, Result, Stage};

#[allow(dead_code)]
pub struct Passport {
    raw: String,
    fields: HashMap<String, String>,
}

impl Passport {
    fn new(raw: impl Into<String>) -> Passport {
        Passport {
            raw: raw.into(),
            fields: HashMap::new(),
        }
    }
}

impl Deref for Passport {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl DerefMut for Passport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}

fn parse_inputs(inputs: String) -> Result<Vec<Passport>> {
    let mut passports = Vec::new();
    for input in inputs.split("\n\n") {
        let raw: String = input.clone().into();
        let mut passport = Passport::new(raw.clone());
        let fields = raw.split(|c| c == ' ' || c == '\n');
        for field in fields {
            let mut field = field.split(":");
            let key = field.nth(0).expect("Malformed input").into();
            let value = field.nth(0).expect("Malformed input").into();
            passport.insert(key, value);
        }
        passports.push(passport);
    }
    Ok(passports)
}

#[puzzle(4, first, parse_inputs)]
pub fn first_stage(inputs: Vec<Passport>) -> Result<usize> {
    let mut count = 0;
    for passport in inputs {
        if passport.len() == 8 || passport.len() == 7 && !passport.contains_key("cid") {
            count += 1;
        }
    }

    Ok(count)
}

#[puzzle(4, second, parse_inputs)]
fn second_stage(inputs: Vec<Passport>) -> Result<usize> {
    let mut count = 0;
    'validation: for passport in inputs {
        if passport.len() != 8 && (passport.len() != 7 || passport.contains_key("cid")) {
            continue;
        }

        let byr = u32::from_str(passport["byr"].as_str());
        if byr.is_err() {
            continue;
        }
        let byr = byr.unwrap();
        if byr < 1920 || byr > 2002 {
            continue;
        }

        let iyr = u32::from_str(passport["iyr"].as_str());
        if iyr.is_err() {
            continue;
        }
        let iyr = iyr.unwrap();
        if iyr < 2010 || iyr > 2020 {
            continue;
        }

        let eyr = u32::from_str(passport["eyr"].as_str());
        if eyr.is_err() {
            continue;
        }
        let eyr = eyr.unwrap();
        if eyr < 2020 || eyr > 2030 {
            continue;
        }

        enum HeightType {
            Centimeters,
            Inches,
        }

        let hgt = passport["hgt"].as_str();
        let hgt_t: HeightType = if hgt.ends_with("cm") {
            HeightType::Centimeters
        } else if hgt.ends_with("in") {
            HeightType::Inches
        } else {
            continue;
        };
        let hgt = hgt.replace("cm", "").replace("in", "");
        let hgt = u32::from_str(&hgt);
        if hgt.is_err() {
            continue;
        }
        let hgt = hgt.unwrap();
        match hgt_t {
            HeightType::Centimeters => {
                if hgt < 150 || hgt > 193 {
                    continue;
                }
            }
            HeightType::Inches => {
                if hgt < 59 || hgt > 76 {
                    continue;
                }
            }
        }

        let hcl = passport["hcl"].as_str();
        if !hcl.starts_with("#") {
            continue;
        }
        let hcl = &hcl[1..];
        if hcl.len() != 6 {
            continue;
        }
        for character in hcl.chars() {
            if character < '0' {
                continue 'validation;
            }
            if character > '9' && character < 'a' {
                continue 'validation;
            }
            if character > 'f' {
                continue 'validation;
            }
        }

        let ecl = passport["ecl"].as_str();
        match ecl {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => continue,
        }

        let pid = passport["pid"].as_str();
        if pid.len() != 9 {
            continue;
        }
        let pid = u32::from_str(pid);
        if pid.is_err() {
            continue;
        }

        count += 1;
    }

    Ok(count)
}
