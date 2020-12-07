use adventofcode2020::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

struct Rule {
    pub color: String,
    pub contents: Vec<(u32, String)>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.splitn(2, " bags contain ");
        let color = split
            .next()
            .ok_or_else(|| Error::General("Invalid rule".into()))?
            .to_owned();
        let contents = split
            .next()
            .ok_or_else(|| Error::General("Invalid rule".into()))?;

        let pattern: &Regex = regex!("^([0-9]+|no) (.*) bags?.?");

        let contents = contents.split(", ").try_fold(
            vec![],
            |mut state, c| -> Result<Vec<(u32, String)>> {
                let capture = pattern
                    .captures(c)
                    .ok_or_else(|| Error::General(format!("Invalid rule {}", &c)))?;
                let count = capture.get(1).unwrap().as_str();
                let count = if count == "no" {
                    0
                } else {
                    count.parse::<u32>()?
                };
                let nested_color = capture.get(2).unwrap().as_str().to_owned();

                if count > 0 {
                    state.push((count, nested_color));
                }
                Ok(state)
            },
        )?;

        Ok(Self { color, contents })
    }
}

fn dfs(
    rules_by_color: &HashMap<String, Vec<(u32, String)>>,
    current_color: &str,
    search_color: &str,
    depth: u32,
) -> Result<bool> {
    if current_color == search_color {
        return Ok(true);
    }

    if depth > 1000 {
        return Err(Error::General("Exceeded max depth".into()));
    }

    let rules = rules_by_color
        .get(current_color)
        .ok_or_else(|| Error::General(format!("Color {} not found in rules", &current_color)))?;

    rules
        .iter()
        .try_fold(false, |mut state, (_amount, color)| -> Result<bool> {
            state |= dfs(rules_by_color, &color, &search_color, depth + 1)?;
            Ok(state)
        })
}

fn main() -> Result<()> {
    let rules: Vec<Rule> = read_file("data/7.txt")?;

    let rules_by_color: HashMap<String, Vec<(u32, String)>> =
        rules.into_iter().map(|r| (r.color, r.contents)).collect();

    let part1 = rules_by_color
        .keys()
        .try_fold(0, |mut state, color| -> Result<u64> {
            if color != "shiny gold" {
                let found = dfs(&rules_by_color, &color, "shiny gold", 0)?;
                if found {
                    state += 1
                }
            }
            Ok(state)
        })?;

    println!("{}", part1);

    Ok(())
}
