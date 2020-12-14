use adventofcode2020::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            and_mask: 0,
            or_mask: 0,
        }
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }
}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "((x & {:b}) | {:b})",
            self.and_mask, self.or_mask
        ))
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Instruction {
    SetMask(Mask),
    Assign(u64, u64),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::SetMask(mask) => f.write_fmt(format_args!("mask = {}", mask)),
            Instruction::Assign(address, value) => {
                f.write_fmt(format_args!("mem[{}] = {}", address, value))
            }
        }
    }
}

fn compute_mask(s: &str) -> Result<Mask> {
    s.as_bytes()
        .iter()
        .rev()
        .enumerate()
        .try_fold(Mask::default(), |mut mask, (i, x)| {
            match x {
                b'X' => {
                    //or_mask = 0
                    mask.and_mask |= 1 << i;
                }
                b'1' => {
                    mask.or_mask |= 1 << i;
                }
                b'0' => {
                    // and_mask = 0
                }
                _ => {
                    return Err(Error::General(format!(
                        "Could not parse mask char {}",
                        *x as char
                    )));
                }
            };
            Ok(mask)
        })
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let pattern: &Regex = regex!("^(?:mask *= *([01X]+))|(?:mem\\[([0-9]+)\\] *= *([0-9]+))$");

        let captures = pattern
            .captures(s)
            .ok_or_else(|| Error::General(format!("Invalid instruction {}", s)))?;

        let insn = match (captures.get(1), captures.get(2), captures.get(3)) {
            (Some(mask), None, None) => Instruction::SetMask(compute_mask(mask.as_str())?),
            (None, Some(address), Some(value)) => {
                Instruction::Assign(address.as_str().parse()?, value.as_str().parse()?)
            }
            _ => {
                return Err(Error::General(format!("Invalid pattern {:?}", captures)));
            }
        };

        Ok(insn)
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = read_file("data/14.txt")?;

    println!("{}", compute_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")?);

    #[derive(Default)]
    struct State {
        mask: Mask,
        memory: HashMap<u64, u64>,
    }

    let state = instructions
        .iter()
        .fold(State::default(), |mut state, insn| {
            match insn {
                Instruction::SetMask(mask) => {
                    state.mask = *mask;
                }
                Instruction::Assign(addres, value) => {
                    state.memory.insert(*addres, state.mask.apply(*value));
                }
            };
            state
        });

    let part1: u64 = state.memory.values().sum();

    println!("{}", part1);

    Ok(())
}
