use adventofcode2020::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let numbers = read_file("data/1.txt")?;
    let set: HashSet<i64> = numbers.iter().cloned().collect();

    let sums: HashMap<i64, (i64, i64)> = numbers
        .iter()
        .enumerate()
        .flat_map(|(i, n1)| {
            numbers.iter().enumerate().filter_map(move |(j, n2)| {
                if i != j {
                    Some((n1 + n2, (*n1, *n2)))
                } else {
                    None
                }
            })
        })
        .collect();

    set.iter()
        .filter_map(|i| {
            let other = 2020 - *i;
            if set.contains(&other) {
                Some((*i, other))
            } else {
                None
            }
        })
        .map(|(i, j)| i * j)
        .take(1)
        .for_each(|i| println!("{}", i));

    set.iter()
        .filter_map(|i| {
            let other = 2020 - *i;
            if let Some((n1, n2)) = sums.get(&other) {
                Some((*i, *n1, *n2))
            } else {
                None
            }
        })
        .map(|(i, j, k)| i * j * k)
        .take(1)
        .for_each(|i| println!("{}", i));

    Ok(())
}
