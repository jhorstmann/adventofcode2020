use adventofcode2020::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Line(Vec<bool>);

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Line(s.bytes().map(|b| b == b'#').collect()))
    }
}

fn count_trees(map: &Vec<Line>, dx: usize, dy: usize) -> u64 {
    let mut y = 0;
    let mut x = 0;
    let mut count = 0;
    while y < map.len() {
        let trees = &map[y].0;
        if trees[x % trees.len()] {
            count += 1;
        }

        x += dx;
        y += dy;
    }

    count
}

fn main() -> Result<()> {
    let map = read_file::<_, Line>("data/3.txt")?;

    let part1 = count_trees(&map, 3, 1);
    println!("{}", part1);

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = slopes
        .iter()
        .map(|(dx, dy)| count_trees(&map, *dx, *dy))
        .product::<u64>();
    println!("{}", part2);

    Ok(())
}
