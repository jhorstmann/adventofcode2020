use adventofcode2020::prelude::*;

fn gcd(m: u64, n: u64) -> u64 {
    if n > 0 {
        gcd(n, m % n)
    } else {
        m
    }
}

fn lcm(m: u64, n: u64) -> u64 {
    m * n / gcd(m, n)
}

fn ceildiv(m: u64, n: u64) -> u64 {
    (m + n - 1) / n
}

fn round_to_multiple(m: u64, n: u64) -> u64 {
    let r = (m + n - 1);
    r - r % n
}

fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/13_example.txt")?;
    let time: u64 = lines[0].parse()?;
    let busses: Vec<u64> = lines[1]
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    let bus = busses
        .iter()
        .min_by_key(|i| round_to_multiple(time, **i))
        .unwrap();
    let wait = round_to_multiple(time, *bus) - time;
    println!("{}", bus * wait);

    let busses: Vec<(usize, u64)> = lines[1]
        .split(',')
        .map(|s| {
            if s == "x" {
                0
            } else {
                s.parse::<u64>().unwrap()
            }
        })
        .enumerate()
        .filter(|(i, bus)| *bus > 0)
        .collect();

    dbg!(&busses);

    Ok(())
}
