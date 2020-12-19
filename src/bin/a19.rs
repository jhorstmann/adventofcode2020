use adventofcode2020::prelude::*;
use regex_automata::{Regex, RegexBuilder, DFA};
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
    Alternatives(Vec<Vec<u64>>),
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

            if alternatives.len() == 1 {
                Ok(Pattern::Sequence(alternatives.pop().unwrap()))
            } else {
                Ok(Pattern::Alternatives(alternatives))
            }
        }
    }
}

fn append_sequence(seq: &[u64], rules: &HashMap<u64, Pattern>, output: &mut String) -> Result<()> {
    seq.iter()
        .try_for_each(|item| build_regex_recursive(*item, rules, output))?;

    Ok(())
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
            append_sequence(seq, rules, output)?;
        }
        Pattern::Alternatives(alternatives) => {
            output.push('(');
            alternatives.iter().try_for_each(|seq| -> Result<()> {
                append_sequence(seq, rules, output)?;
                output.push('|');
                Ok(())
            })?;
            // remove last pipe
            output.pop();
            output.push(')');

            // optimize a bit for readability
            if output.ends_with("(a|b)") || output.ends_with("(b|a)") {
                output.truncate(output.len() - 5);
                output.push('.');
            }
            if output.ends_with("(ba|aa)") {
                output.truncate(output.len() - 7);
                output.push_str(".a");
            }
            if output.ends_with("(ab|aa)") {
                output.truncate(output.len() - 7);
                output.push_str("a.");
            }
            if output.ends_with("(ab|bb)") {
                output.truncate(output.len() - 7);
                output.push_str(".b");
            }
            if output.ends_with("(bb|ba)") {
                output.truncate(output.len() - 7);
                output.push_str("b.");
            }
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

fn build_regex_str(rules: &HashMap<u64, Pattern>, start_rule: u64) -> Result<String> {
    let mut rx = String::new();
    build_regex_recursive(start_rule, rules, &mut rx)?;

    Ok(rx)
}

fn build_regex(rules: &HashMap<u64, Pattern>, start_rule: u64) -> Result<regex_automata::Regex> {
    let rx = build_regex_str(rules, start_rule)?;

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

    let mut rules: HashMap<u64, Pattern> = rules.into_iter().try_fold(
        HashMap::default(),
        |mut map, line| -> Result<HashMap<u64, Pattern>> {
            let rule = Rule::from_str(line)?;
            map.insert(rule.id, rule.pattern);
            Ok(map)
        },
    )?;

    let regex = build_regex(&rules, 0)?;

    let part1 = messages
        .iter()
        .filter(|msg| match regex.forward().find(msg.as_bytes()) {
            None => false,
            Some(len) => len == msg.len(),
        })
        .count();

    println!("{}", part1);

    let mut fortytwo = build_regex_str(&rules, 42)?;
    let mut thirtyone = build_regex_str(&rules, 31)?;

    let rx_fortytwo = RegexBuilder::new()
        .anchored(true)
        .allow_invalid_utf8(true)
        .build(&fortytwo)
        .map_err(|e| Error::General(format!("Could not build regex: {}", e)))?;
    let rx_thirtyone = RegexBuilder::new()
        .anchored(true)
        .allow_invalid_utf8(true)
        .build(&thirtyone)
        .map_err(|e| Error::General(format!("Could not build regex: {}", e)))?;

    fn match_nested(bytes: &[u8], left: &Regex, right: &Regex, level: u64) -> bool {
        if level > 0 && bytes.len() == 0 {
            return true;
        }
        if let Some(len1) = right.reverse().rfind(bytes) {
            if let Some(len2) = left.forward().find(&bytes[0..len1]) {
                match_nested(&bytes[len2..len1], left, right, level + 1)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn match_new_rules(bytes: &[u8], left: &Regex, right: &Regex) -> bool {
        let mut bytes = bytes;
        while let Some(len) = left.forward().find(bytes) {
            bytes = &bytes[len..];
            if match_nested(bytes, left, right, 0) {
                return true;
            }
        }
        return false;
    }

    let part2 = messages
        .iter()
        .filter(|msg| {
            let mut bytes = msg.as_bytes();
            match_new_rules(bytes, &rx_fortytwo, &rx_thirtyone)
        })
        .count();

    println!("{}", part2);

    Ok(())
}
