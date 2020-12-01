use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::collections::{HashSet, HashMap};
use std::path::Path;

fn read_numbers(file: &str) -> Result<Vec<i64>, Error> {
    let path = Path::new(file);

    let io = File::open(path)?;
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let numbers = read_numbers("data/1.txt")?;
    let set: HashSet<i64> = numbers.iter().cloned().collect();

    let sums : HashMap<i64, (i64,i64)> = numbers.iter().enumerate().flat_map(|(i, n1)| {
        numbers.iter().enumerate().filter_map(move |(j, n2)| if i != j {
            Some((n1+n2, (*n1, *n2)))
        } else {
            None
        })
    }).collect();

    set.iter().filter_map(|i| {
        let other = 2020 - *i;
        if set.contains(&other) {
            Some((*i, other))
        } else {
            None
        }
    }).map(|(i, j)| i*j).take(1).for_each(|i| println!("{}", i));

    set.iter().filter_map(|i| {
        let other = 2020 - *i;
        if let Some((n1, n2)) = sums.get(&other) {
            Some((*i, *n1, *n2))
        } else {
            None
        }
    }).map(|(i, j, k)| i*j*k).take(1).for_each(|i| println!("{}", i));

    Ok(())
}
