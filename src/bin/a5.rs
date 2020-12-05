use adventofcode2020::prelude::*;

fn walk(input: &[u8]) -> i32 {
    let final_range = input
        .iter()
        .fold((0, (1 << input.len())), |(min, max), dir| match dir {
            b'F' | b'L' => (min, max - (max - min) / 2),
            b'B' | b'R' => (min + (max - min) / 2, max),
            _ => panic!("Unsupported direction"),
        });

    assert_eq!(final_range.0 + 1, final_range.1);

    final_range.0
}

fn process(input: &[u8]) -> i32 {
    let (row, col) = input.split_at(7);

    walk(row) * (1 << col.len()) + walk(col)
}

fn main() -> Result<()> {
    let data = read_file::<_, String>("data/5.txt")?;

    let part1 = data.iter().map(|line| process(line.as_bytes())).max();

    println!("{:?}", part1);

    let mut seats = data
        .iter()
        .map(|line| process(line.as_bytes()))
        .collect::<Vec<i32>>();
    seats.sort();

    let part2 = seats.windows(2).find_map(|w| {
        if w[1] - w[0] == 2 {
            Some(w[0] + 1)
        } else {
            None
        }
    });

    println!("{:?}", part2);

    Ok(())
}
