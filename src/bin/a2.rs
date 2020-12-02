use adventofcode2020::prelude::*;
use std::io::ErrorKind;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Policy {
    character: u8,
    min: usize,
    max: usize,
}

#[derive(Debug, Clone)]
struct Line {
    policy: Policy,
    password: String,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let pattern: &Regex = regex!("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)");
        let captures = pattern
            .captures(s)
            .ok_or_else(|| std::io::Error::new(ErrorKind::InvalidData, "Pattern did not match"))?;

        Ok(Line {
            policy: Policy {
                min: captures.get(1).unwrap().as_str().parse()?,
                max: captures.get(2).unwrap().as_str().parse()?,
                character: captures.get(3).unwrap().as_str().as_bytes()[0],
            },
            password: captures.get(4).unwrap().as_str().to_owned(),
        })
    }
}

fn main() -> Result<()> {
    let data = read_file::<_, Line>("data/2.txt")?;

    dbg!(data);

    Ok(())
}
