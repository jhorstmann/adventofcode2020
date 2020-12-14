#![feature(iterator_fold_self)]
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
    let r = m + n - 1;
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

    let busses: Vec<(u64, u64)> = lines[1]
        .split(',')
        .map(|s| {
            if s == "x" {
                0
            } else {
                s.parse::<u64>().unwrap()
            }
        })
        .enumerate()
        .map(|(i, bus)| (i as u64, bus))
        .filter(|(i, bus)| *bus > 0)
        .collect();

    dbg!(&busses);
    /*
       let part2 = busses.into_iter().skip(1).fold_first(|mut state, bus| {
           let num = state.0 * bus.1 + bus.0 * state.1;
           let den = state.1 * bus.1;
           let g = gcd(num, den);
           dbg!(num, den);
           (num/g, den/g)
       }).unwrap();

       dbg!(&part2);
       dbg!(gcd(part2.0, part2.1));
       dbg!(gcd(3162341, (1+2+5+7+8)));


       let test = (0..u32::MAX as u64).step_by(7).find(|i| (i+4) % 59 == 0 && (i+6) % 31 == 0 && (i+7) % 19 == 0 && (i+1) % 13 == 0 );
       dbg!(test);
       let test = (0..u32::MAX as u64).step_by(7).find(|i| (i) % 59 == 59-4 && (i) % 31 == 31-6 && (i) % 19 == 19-7 && (i) % 13 == 13-1 );
       dbg!(test);

       (0..1000 as u64).step_by(7).filter(|i| (i+1) % 13 == 0 ).for_each(|i| {
           dbg!(i);
       });

       //let test = (0..1000000000000_u64).step_by(19).find(|i| (i+19) % 787 == 0 && (i+50) % 571 == 0 && (i+9) % 41 == 0 && (i+13) % 37 == 0 && (i+48) % 29 == 0 && (i+42) % 23 == 0 && (i+67) % 17 == 0 && (i+32) % 13 == 0 );

       //na::VectorN::from_vec(vec![0, 1, 4, 6, 7]);

       Ok(())

    */
}
