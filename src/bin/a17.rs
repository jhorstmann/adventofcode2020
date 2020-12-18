use adventofcode2020::prelude::*;
use std::collections::HashSet;

fn part1(lines: &[String]) -> usize {
    let mut cube: HashSet<(i32, i32, i32)> = HashSet::default();

    lines.iter().enumerate().for_each(|(y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, b)| {
            if *b == b'#' {
                cube.insert((x as i32, y as i32, 0));
            }
        })
    });

    let mut directions: Vec<(i32, i32, i32)> = Vec::with_capacity(26);
    (-1..=1).for_each(|z| {
        (-1..=1).for_each(|y| {
            (-1..=1).for_each(|x| {
                if x != 0 || y != 0 || z != 0 {
                    directions.push((x, y, z))
                }
            })
        })
    });

    assert_eq!(26, directions.len());

    let min = -8;
    let max = 8 + 8 + 1;

    (0..6).for_each(|_| {
        let mut new_cube: HashSet<(i32, i32, i32)> = HashSet::default();
        (min..max).for_each(|z| {
            (min..max).for_each(|y| {
                (min..max).for_each(|x| {
                    let neighbors = directions
                        .iter()
                        .filter(|(dx, dy, dz)| cube.contains(&(x + dx, y + dy, z + dz)))
                        .count();
                    let new_active = if cube.contains(&(x, y, z)) {
                        if neighbors == 2 || neighbors == 3 {
                            true
                        } else {
                            false
                        }
                    } else {
                        if neighbors == 3 {
                            true
                        } else {
                            false
                        }
                    };
                    if new_active {
                        new_cube.insert((x, y, z));
                    }
                })
            })
        });
        /*
        (min..max).for_each(|z| {
            (min..max).for_each(|y| {
                (min..max).for_each(|x| {
                    if new_cube.contains(&(x, y, z)) {
                        print!("#");
                    } else {
                        print!(".")
                    }
                });
                println!();
            });
            println!();
        });

         */

        cube = new_cube;
    });

    cube.len()
}

fn part2(lines: &[String]) -> usize {
    let mut cube: HashSet<(i32, i32, i32, i32)> = HashSet::default();

    lines.iter().enumerate().for_each(|(y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, b)| {
            if *b == b'#' {
                cube.insert((x as i32, y as i32, 0, 0));
            }
        })
    });

    let mut directions: Vec<(i32, i32, i32, i32)> = Vec::with_capacity(80);
    (-1..=1).for_each(|w| {
        (-1..=1).for_each(|z| {
            (-1..=1).for_each(|y| {
                (-1..=1).for_each(|x| {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        directions.push((x, y, z, w))
                    }
                })
            })
        })
    });

    assert_eq!(80, directions.len());

    let min = -8;
    let max = 8 + 8 + 1;

    (0..6).for_each(|_| {
        let mut new_cube: HashSet<(i32, i32, i32, i32)> = HashSet::default();
        (min..max).for_each(|w| {
            (min..max).for_each(|z| {
                (min..max).for_each(|y| {
                    (min..max).for_each(|x| {
                        let neighbors = directions
                            .iter()
                            .filter(|(dx, dy, dz, dw)| {
                                cube.contains(&(x + dx, y + dy, z + dz, w + dw))
                            })
                            .count();
                        let new_active = if cube.contains(&(x, y, z, w)) {
                            if neighbors == 2 || neighbors == 3 {
                                true
                            } else {
                                false
                            }
                        } else {
                            if neighbors == 3 {
                                true
                            } else {
                                false
                            }
                        };
                        if new_active {
                            new_cube.insert((x, y, z, w));
                        }
                    })
                })
            })
        });

        cube = new_cube;
    });

    cube.len()
}

fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/17.txt")?;

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));

    Ok(())
}
