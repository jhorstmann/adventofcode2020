use adventofcode2020::prelude::*;
use regex_automata::{RegexBuilder, DFA};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Rule {
    id: u64,
    pattern: Pattern,
}

#[derive(Debug, Clone)]
enum Pattern {
    Char(char),
    Sequence(Vec<u64>),
    Alternatives(Vec<Pattern>),
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(": ");
        let id = split
            .next()
            .ok_or_else(|| Error::General("Missing id".into()))?
            .parse()?;
        let pattern = split
            .next()
            .ok_or_else(|| Error::General("Missing pattern".into()))?;
        let pattern = Pattern::from_str(pattern)?;
        Ok(Self { id, pattern })
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(first_char) = s
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .and_then(|s| {
                let mut chars = s.chars();
                chars.next()
            })
        {
            Ok(Pattern::Char(first_char))
        } else {
            let mut alternatives: Vec<Vec<u64>> = s.split(" | ").try_fold(
                vec![],
                |mut alternatives, s| -> Result<Vec<Vec<u64>>> {
                    let sequence = s.split_ascii_whitespace().try_fold(
                        vec![],
                        |mut sequence, item| -> Result<Vec<u64>> {
                            sequence.push(item.parse::<u64>()?);
                            Ok(sequence)
                        },
                    )?;
                    alternatives.push(sequence);

                    Ok(alternatives)
                },
            )?;

            if alternatives.len() > 1 {
                Ok(Pattern::Alternatives(
                    alternatives
                        .into_iter()
                        .map(|seq| Pattern::Sequence(seq))
                        .collect(),
                ))
            } else {
                Ok(Pattern::Sequence(std::mem::take(&mut alternatives[0])))
            }
        }
    }
}

fn append_pattern(
    pattern: &Pattern,
    rules: &HashMap<u64, Pattern>,
    output: &mut String,
) -> Result<()> {
    match pattern {
        Pattern::Char(ch) => {
            output.push(*ch);
        }
        Pattern::Sequence(seq) => {
            output.push('(');
            seq.iter()
                .try_for_each(|item| build_regex_recursive(*item, rules, output))?;
            output.push(')');
        }
        Pattern::Alternatives(seq) => {
            output.push('(');
            seq.iter().try_for_each(|item| -> Result<()> {
                append_pattern(item, rules, output)?;
                output.push('|');
                Ok(())
            })?;
            // remove last pipe
            output.pop();
            output.push(')');
        }
    };

    Ok(())
}

fn build_regex_recursive(
    rule: u64,
    rules: &HashMap<u64, Pattern>,
    output: &mut String,
) -> Result<()> {
    match rules.get(&rule) {
        None => Err(Error::General(format!("Could not find rule {}", rule))),
        Some(pattern) => append_pattern(pattern, rules, output),
    }
}

fn build_regex(rules: &HashMap<u64, Pattern>) -> Result<regex_automata::Regex> {
    let mut rx = String::new();
    build_regex_recursive(0, rules, &mut rx)?;

    dbg!(&rx);

    RegexBuilder::new()
        .anchored(true)
        .build(&rx)
        .map_err(|e| Error::General(format!("Could not build regex: {}", e)))
}

fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/19.txt")?;
    let mut split = lines.split(|l| l.is_empty());
    let rules = split
        .next()
        .ok_or_else(|| Error::General("Missing patterns".into()))?;
    let messages = split
        .next()
        .ok_or_else(|| Error::General("Missing messages".into()))?;

    let rules: HashMap<u64, Pattern> = rules.into_iter().try_fold(
        HashMap::default(),
        |mut map, line| -> Result<HashMap<u64, Pattern>> {
            let rule = Rule::from_str(line)?;
            map.insert(rule.id, rule.pattern);
            Ok(map)
        },
    )?;

    dbg!(&rules);

    let regex = build_regex(&rules)?;

    let part1 = messages
        .iter()
        .filter(|msg| match regex.forward().find(msg.as_bytes()) {
            None => false,
            Some(len) => len == msg.len(),
        })
        .count();

    println!("{}", part1);

    Ok(())
}
