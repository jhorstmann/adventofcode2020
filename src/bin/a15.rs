use adventofcode2020::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn solve_part1(starting: &[u64], total_rounds: usize) -> u64 {
    let mut rounds: HashMap<u64, usize> = HashMap::new();
    starting.iter().enumerate().for_each(|(i, n)| {
        rounds.insert(*n, i + 1);
    });
    (rounds.len()..total_rounds)
        .scan(*starting.last().unwrap(), |last, i| {
            *last = match rounds.entry(*last) {
                Entry::Occupied(mut entry) => {
                    let r = entry.insert(i);
                    let new_n = i - r;
                    new_n as u64
                }
                Entry::Vacant(entry) => {
                    entry.insert(i);
                    0
                }
            };
            Some(*last)
        })
        .last()
        .unwrap()
}

fn main() -> Result<()> {
    let input = "1,2,16,19,18,0";
    let _example = "0,3,6";

    let starting: Vec<u64> = input
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let part1 = solve_part1(&starting, 2020);

    println!("{}", part1);

    // TODO: optimize, runs in 3s in release build
    let part2 = solve_part1(&starting, 30000000);

    println!("{}", part2);

    Ok(())
}
