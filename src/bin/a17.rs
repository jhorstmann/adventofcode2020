use adventofcode2020::prelude::*;



fn main() -> Result<()> {
    let lines : Vec<String> = read_file("data/17.txt")?;

    // slice has width and height 8, leave another 8 in each direction for simulation
    const off : usize =  8;
    const len: usize = 8 +2*off;

    assert_eq!(lines.len(),8);
    let mut slice: Vec<Vec<bool>> = Vec::default();
    (0..off).for_each(|_| slice.push(vec![false; len]));
    lines.iter().for_each(|l| {
        assert_eq!(l.len(), 8);
        let mut row = Vec::with_capacity(len);
        (0..off).for_each(|_| row.push(false));
        l.as_bytes().iter().for_each(|b| row.push(*b == b'#')).collect();
        (0..off).for_each(|_| row.push(false));
        slice.push(row);
    });
    (0..off).for_each(|_| slice.push(vec![false; len]));


    let empty_slice : Vec<Vec<bool>> = (0..len).map(|_| {
        vec![false; len]
    }).collect();

    let mut cube: Vec<Vec<Vec<bool>>> = Vec::with_capacity(off);
    (0..off).for_each(|i| { cube.push(empty_slice.clone()); });
    cube.push(slice);
    (0..off).for_each(|i| { cube.push(empty_slice.clone()); });


    let mut directions :Vec<(i32,i32,i32)> = Vec::with_capacity(26);
    (-1..=1).for_each(|z| (-1..=1).for_each(|y| (-1..=1).for_each(|x| if x!=0 || y!= 0 || z!= 0{ directions.push((x,y,z))})));

    dbg!(&directions);

    (0..6).for_each(|_| {


    });

    Ok(())
}