use adventofcode2020::prelude::*;
use std::collections::HashMap;

fn part1(numbers: &[i64]) -> u64 {
    let (diff1, diff3): (u64, u64) = numbers.windows(2).fold((0, 0), |mut state, win| {
        match win[1] - win[0] {
            1 => state.0 += 1,
            3 => state.1 += 1,
            _ => {}
        };

        state
    });

    diff1 * diff3
}

fn part2(numbers: &[i64], n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(result) = memo.get(&n) {
        return *result;
    }

    let mut variants = 1;
    for i in n..numbers.len() {
        let mut j = i + 2;
        while j < numbers.len() && numbers[j] - numbers[i] <= 3 {
            variants += part2(numbers, j, memo);
            j += 1;
        }
    }

    memo.insert(n, variants);

    variants
}

fn main() -> Result<()> {
    let mut numbers: Vec<i64> = read_file("data/10.txt")?;

    numbers.sort();

    // add outlet joltage
    numbers.insert(0, 0);
    // add device joltage
    numbers.push(*numbers.last().unwrap() + 3);

    let part1 = part1(&numbers);

    println!("{}", part1);

    let part2 = part2(&numbers, 0, &mut HashMap::default());

    println!("{}", part2);

    Ok(())
}
