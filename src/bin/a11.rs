use adventofcode2020::prelude::*;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Seat::Floor => f.write_char('.'),
            Seat::Empty => f.write_char('L'),
            Seat::Occupied => f.write_char('#'),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Row(Vec<Seat>);

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|seat| seat.fmt(f))?;
        f.write_char('\n')
    }
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Row(s.as_bytes().iter().try_fold(
            vec![],
            |mut state, b| -> Result<Vec<Seat>> {
                let seat = match b {
                    b'.' => Ok(Seat::Floor),
                    b'L' => Ok(Seat::Empty),
                    b'#' => Ok(Seat::Occupied),
                    _ => Err(Error::General(format!("Invalid layout '{}'", *b as char))),
                }?;
                state.push(seat);
                Ok(state)
            },
        )?))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Area(Vec<Row>);

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|row| row.fmt(f))
    }
}

impl Area {
    fn get(&self, x: i32, y: i32) -> Option<Seat> {
        let rows = &self.0;
        if y < 0 || y as usize >= rows.len() {
            return None;
        }

        let seats = &rows[y as usize].0;
        if x < 0 || x as usize >= seats.len() {
            return None;
        }

        return Some(seats[x as usize]);
    }

    fn count_occupied(&self) -> u64 {
        self.0
            .iter()
            .map(|row| row.0.iter().filter(|seat| **seat == Seat::Occupied).count() as u64)
            .sum()
    }
}

static DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn step(area: &Area) -> Area {
    let rows = &area.0;
    let mut result = Vec::with_capacity(rows.len());
    for y in 0..rows.len() {
        let seats = &rows[y as usize].0;

        let mut new_row = Vec::with_capacity(seats.len());

        for x in 0..seats.len() {
            let current = area.get(x as i32, y as i32).unwrap();
            let count = DIRECTIONS
                .iter()
                .map(|(dx, dy)| area.get(x as i32 + dx, y as i32 + dy))
                .map(|seat| match seat {
                    Some(Seat::Occupied) => 1,
                    _ => 0,
                })
                .sum();

            let update = match (&current, count) {
                (Seat::Empty, 0) => Seat::Occupied,
                (Seat::Occupied, 4..=8) => Seat::Empty,
                _ => current,
            };

            new_row.push(update);
        }
        result.push(Row(new_row));
    }
    Area(result)
}

fn part1(mut area: Area) -> u64 {
    loop {
        //println!("{}", area);
        let new_area = step(&area);

        if new_area == area {
            break;
        }

        area = new_area;
    }

    area.count_occupied()
}

fn main() -> Result<()> {
    let area = Area(read_file("data/11.txt")?);

    let num_occupied = part1(area.clone());

    println!("{}", num_occupied);

    Ok(())
}
