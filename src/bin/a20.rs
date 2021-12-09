use std::fmt::{Debug, Display, Formatter, Write};
use adventofcode2020::prelude::*;

struct Tile {
    id: u64,
    data: Vec<Vec<u8>>,
    border_masks: [u64; 4],
}

fn to_mask(bytes: impl Iterator<Item=u8>) -> u64 {
    bytes.fold(0_u64, |a, x| {
        (a << 1) | ((x == b'#') as u64)
    })
}

impl Tile {
    fn new(id: u64, data: Vec<Vec<u8>>) -> Self {
        let top = to_mask(data[0].iter().copied());
        let bottom = to_mask(data[data.len()-1].iter().copied());
        let left = to_mask(data.iter().map(|row| row[0]));
        let right = to_mask(data.iter().map(|row| row[row.len()-1]));

        Self {
            id,
            data,
            border_masks: [top, right, bottom, left]
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Tile {:04}:\n", self.id))?;
        for row in self.data.iter() {
            for byte in row.iter() {
                f.write_char(*byte as char)?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))?;
        f.write_str("Borders: [\n")?;
        for mask in self.border_masks.iter() {
            f.write_fmt(format_args!("  {:010b}\n", mask))?;
        }
        f.write_str("]")?;

        Ok(())
    }
}

pub fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/20.txt")?;

    let tiles = lines.split(|line| line.is_empty()).map(|tile| {
        if let Some((header, data)) = tile.split_first() {
            let id = header["Tile ".len()..header.len()-1].parse::<u64>()?;
            Ok(Tile::new(id, data.iter().map(|s| s.clone().into_bytes()).collect()))
        } else {
            Err(Error::General("Empty tile".into()))
        }
    }).collect::<Result<Vec<_>>>()?;

    let corner_tiles =  tiles.iter().filter(|tile| {
        let count_outer_borders = tile.border_masks.iter().filter(|mask| {
            !tiles.iter().any(|other_tile| !std::ptr::eq(*tile, other_tile) && (other_tile.border_masks.contains(*mask)) ^ other_tile.border_masks.contains(&(mask.reverse_bits() >> 54)))
        }).count();
        count_outer_borders == 2
    }).collect::<Vec<_>>();

    dbg!(&corner_tiles);

    // eprintln!("{:010b}", u64::from_str_radix("1101010000", 2).unwrap().reverse_bits() >> 54);

    let part1 = corner_tiles.iter().map(|tile| tile.id).product::<u64>();

    println!("Part1: {}", part1);

    Ok(())
}