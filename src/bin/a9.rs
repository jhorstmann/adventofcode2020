use adventofcode2020::prelude::*;
use std::collections::HashSet;

fn part1(numbers: &[i64], preamble_len: usize) -> i64 {
    assert!(preamble_len < numbers.len());

    let mut set: HashSet<i64> = numbers[0..preamble_len].iter().cloned().collect();

    let mut i = preamble_len;

    loop {
        let n = numbers[i];
        let m = set.iter().find(|i| **i < n && set.contains(&(n - **i)));
        match m {
            None => return n,
            _ => {}
        }

        set.remove(&numbers[i - preamble_len]);
        set.insert(n);
        i += 1;
    }
}

fn part2(numbers: &[i64], search: i64) -> Option<(usize, usize)> {
    let sum: Vec<i64> = numbers
        .iter()
        .scan(0, |state, i| {
            *state += *i;
            Some(*state)
        })
        .collect();

    assert_eq!(numbers.len(), sum.len());

    for i in 1..numbers.len() {
        for j in i + 2..(numbers.len()) {
            if sum[j] - sum[i - 1] == search {
                return Some((i, j));
            }
        }
    }

    return None;
}

fn main() -> Result<()> {
    let numbers: Vec<i64> = read_file("data/9.txt")?;
    let preamble_len = 25;

    let part1 = part1(&numbers, preamble_len);

    println!("{}", part1);

    assert_eq!(257342611, part1);

    if let Some((i, j)) = part2(&numbers, part1) {
        println!("{}, {}", i, j);
        let min = numbers[i..j].iter().min().unwrap();
        let max = numbers[i..j].iter().max().unwrap();
        println!("{}, {}", min, max);
        let part2 = min + max;
        println!("{}", part2);

        assert_eq!(35602097, part2);
    }

    Ok(())
}
