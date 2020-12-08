use adventofcode2020::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split_ascii_whitespace();

        let op = split
            .next()
            .ok_or_else(|| Error::General("Could not parse instruction".into()))?;
        let arg = split
            .next()
            .ok_or_else(|| Error::General("Could not parse argument".into()))?;
        let arg = arg.parse::<i32>()?;

        match op {
            "nop" => Ok(Instruction::Nop(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            _ => Err(Error::General(format!("Invalid instruction {}", op))),
        }
    }
}

#[derive(Debug, Clone)]
struct CPU {
    code: Vec<Instruction>,
    ip: i32,
    acc: i64,
    counters: Vec<i32>,
    corrupted_insn: usize,
}

#[derive(Debug)]
enum CPUError {
    AccumulatorOverflow,
    InstructionPointerOverflow,
    InfiniteLoop(i64),
}

type CPUResult<T> = std::result::Result<T, CPUError>;

impl CPU {
    pub fn new(code: Vec<Instruction>) -> Self {
        let code_len = code.len();
        let counters = vec![0; code_len];
        Self {
            code,
            ip: 0,
            acc: 0,
            counters,
            corrupted_insn: usize::MAX,
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
        self.counters.iter_mut().for_each(|c| *c = 0);
        self.corrupted_insn = 0;
    }

    pub fn mark_corrupted(&mut self, ip: usize) {
        self.corrupted_insn = ip;
    }

    pub fn run(&mut self) -> CPUResult<i64> {
        loop {
            if self.ip < 0 || self.ip as usize > self.code.len() {
                return Err(CPUError::InstructionPointerOverflow);
            }

            let ip = self.ip as usize;

            if ip == self.code.len() {
                return Ok(self.acc);
            }

            if self.counters[ip] > 0 {
                return Err(CPUError::InfiniteLoop(self.acc));
            }

            self.counters[ip] += 1;

            let insn = &self.code[ip];

            let is_error = ip == self.corrupted_insn;

            match (insn, is_error) {
                (Instruction::Acc(a), _) => {
                    self.acc = self
                        .acc
                        .checked_add(*a as i64)
                        .ok_or(CPUError::AccumulatorOverflow)?;
                    self.ip += 1;
                }
                (Instruction::Nop(_), false) | (Instruction::Jmp(_), true) => {
                    self.ip += 1;
                }
                (Instruction::Jmp(a), false) | (Instruction::Nop(a), true) => {
                    self.ip = self
                        .ip
                        .checked_add(*a)
                        .ok_or(CPUError::InstructionPointerOverflow)?;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = read_file("data/8.txt")?;

    let mut cpu = CPU::new(instructions.clone());

    match cpu.run() {
        Ok(acc) | Err(CPUError::InfiniteLoop(acc)) => println!("{}", acc),
        Err(e) => return Err(Error::General(format!("{:?}", e))),
    }

    let mut terminating_mask = vec![0_u8; (instructions.len() + 1 + 7) / 8];
    terminating_mask[instructions.len() / 8] |= 1 << (instructions.len() % 8);
    loop {
        let mut updated = false;
        let mut count = 0;
        instructions.iter().enumerate().for_each(|(ip, insn)| {
            if terminating_mask[ip / 8] & (1 << (ip % 8)) == 0 {
                let target = match insn {
                    Instruction::Nop(_) | Instruction::Acc(_) => ip + 1,
                    Instruction::Jmp(a) => (ip as i64 + *a as i64) as usize,
                };

                if terminating_mask[target / 8] & (1 << (target % 8)) != 0 {
                    updated = true;
                    count += 1;
                    terminating_mask[ip / 8] |= 1 << (ip % 8);
                }
            }
        });

        if !updated {
            break;
        }
    }

    let mut cpu2 = CPU::new(instructions.clone());

    instructions
        .iter()
        .enumerate()
        .filter(|(ip, insn)| match insn {
            Instruction::Acc(_) => false,
            Instruction::Jmp(_) => {
                let target_if_nop = *ip + 1;
                terminating_mask[target_if_nop / 8] & (1 << (target_if_nop % 8)) != 0
            }
            Instruction::Nop(a) => {
                let target_if_jmp = (*ip as i64 + *a as i64) as usize;
                terminating_mask[target_if_jmp / 8] & (1 << (target_if_jmp % 8)) != 0
            }
        })
        .try_for_each(|(ip, _insn)| -> Result<()> {
            cpu2.reset();
            cpu2.mark_corrupted(ip);

            match cpu2.run() {
                Ok(acc) => {
                    println!("{}", acc);
                }
                Err(CPUError::InfiniteLoop(_)) => {
                    // still an infinite loop, ignore
                }
                Err(e) => return Err(Error::General(format!("{:?}", e))),
            }

            Ok(())
        })?;

    Ok(())
}
