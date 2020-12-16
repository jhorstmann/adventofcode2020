use adventofcode2020::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FieldRule {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct Range(u64, u64);

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.splitn(2, "-");
        let min = split
            .next()
            .ok_or_else(|| Error::General(format!("Missing separator in [{}]", s)))?
            .parse()?;
        let max = split
            .next()
            .ok_or_else(|| Error::General(format!("Missing separator in [{}]", s)))?
            .parse()?;

        Ok(Self(min, max))
    }
}

impl FromStr for FieldRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.splitn(2, ": ");
        let name = split
            .next()
            .ok_or_else(|| Error::General(format!("Missing separator in [{}]", s)))?
            .to_owned();
        let ranges = split
            .next()
            .ok_or_else(|| Error::General(format!("Missing separator in [{}]", s)))?;

        let ranges =
            ranges
                .split(" or ")
                .try_fold(vec![], |mut vec, rule| -> Result<Vec<Range>> {
                    vec.push(Range::from_str(rule)?);

                    Ok(vec)
                })?;

        Ok(Self { name, ranges })
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<u64>,
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values = s
            .split(',')
            .try_fold(vec![], |mut vec, n| -> Result<Vec<u64>> {
                vec.push(n.parse()?);
                Ok(vec)
            })?;

        Ok(Self { values })
    }
}

fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/16_example.txt")?;
    let mut split = lines.split(|line| line.is_empty());
    let first = split
        .next()
        .ok_or_else(|| Error::General("Missing rules in input".into()))?;

    let fields = first
        .iter()
        .try_fold(vec![], |mut vec, line| -> Result<Vec<FieldRule>> {
            vec.push(FieldRule::from_str(line)?);

            Ok(vec)
        })?;

    let my_ticket = split
        .next()
        .ok_or_else(|| Error::General("Missing my ticket in input".into()))?;
    let my_ticket = Ticket::from_str(&my_ticket[1])?;

    let tickets = split
        .next()
        .ok_or_else(|| Error::General("Missing my ticket in input".into()))?;
    let tickets =
        tickets[1..]
            .into_iter()
            .try_fold(vec![], |mut vec, s| -> Result<Vec<Ticket>> {
                vec.push(Ticket::from_str(s)?);
                Ok(vec)
            })?;

    dbg!(&fields);
    dbg!(&my_ticket);
    dbg!(&tickets);

    Ok(())
}
