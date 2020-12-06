use adventofcode2020::prelude::*;

fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/6.txt")?;

    let part1: u64 = lines
        .split(|l| l.is_empty())
        .into_iter()
        .map(|ls| ls.iter().flat_map(|l| l.as_bytes().iter()))
        .map(|bs| {
            let mut set = 0_u64;
            bs.for_each(|c| set |= 1 << (c - b'a'));
            set.count_ones() as u64
        })
        .sum();

    println!("{}", part1);

    let part2: u64 = lines
        .split(|l| l.is_empty())
        .into_iter()
        .map(|ls| {
            ls.iter().fold(u64::MAX, |state, s| {
                let set = s
                    .as_bytes()
                    .iter()
                    .fold(0, |set, c| set | (1 << (c - b'a')));
                state & set
            })
        })
        .map(|mask| mask.count_ones() as u64)
        .sum();

    println!("{}", part2);

    Ok(())
}
