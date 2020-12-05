#![feature(str_strip)]

use adventofcode2020::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct PassportData(HashMap<String, String>);

impl PassportData {
    fn has_required_fields(&self) -> bool {
        let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        let _optional_fields = vec!["cid"];
        required_keys.iter().all(|k| self.0.contains_key(k))
    }

    fn is_valid(&self) -> bool {
        self.0.iter().all(|(k, v)| match k.as_str() {
            "byr" => v
                .parse::<i64>()
                .map(|i| i >= 1920 && i <= 2002)
                .unwrap_or(false),
            "iyr" => v
                .parse::<i64>()
                .map(|i| i >= 2010 && i <= 2020)
                .unwrap_or(false),
            "eyr" => v
                .parse::<i64>()
                .map(|i| i >= 2020 && i <= 2030)
                .unwrap_or(false),
            "hgt" => {
                let valid_cm = v
                    .strip_suffix("cm")
                    .and_then(|s| s.parse::<i64>().map(|i| i >= 150 && i <= 193).ok())
                    .unwrap_or(false);
                let valid_in = v
                    .strip_suffix("in")
                    .and_then(|s| s.parse::<i64>().map(|i| i >= 59 && i <= 76).ok())
                    .unwrap_or(false);

                valid_cm || valid_in
            }

            "hcl" => {
                v.len() == 7
                    && v.strip_prefix("#")
                        .map(|rest| {
                            rest.as_bytes()
                                .iter()
                                .all(|b| (*b >= b'0' && *b <= b'9') || (*b >= b'a' && *b <= b'f'))
                        })
                        .unwrap_or(false)
            }
            "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
            "pid" => v.len() == 9 && v.as_bytes().iter().all(|b| b.is_ascii_digit()),
            "cid" => true,
            _ => false,
        })
    }
}

impl FromStr for PassportData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex: &Regex = regex!("([a-z]+):([^ ]+)");
        let data: HashMap<String, String> = regex
            .captures_iter(s)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().to_owned(),
                    cap.get(2).unwrap().as_str().to_owned(),
                )
            })
            .collect();

        Ok(PassportData(data))
    }
}

fn main() -> Result<()> {
    let data = read_file::<_, PassportData>("data/4.txt")?;
    let data = data
        .split(|d| d.0.is_empty())
        .flat_map(|g| {
            if let Some((first, rest)) = g.split_first() {
                let mut acc = first.clone();
                rest.iter().for_each(|r| {
                    r.0.iter().for_each(|(k, v)| {
                        acc.0.insert(k.clone(), v.clone());
                    });
                });
                Some(acc)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let part1 = data.iter().filter(|d| d.has_required_fields()).count();

    println!("{}", part1);

    let part1 = data
        .iter()
        .filter(|d| d.has_required_fields() && d.is_valid())
        .count();

    println!("{}", part1);

    Ok(())
}
