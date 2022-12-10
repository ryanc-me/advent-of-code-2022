use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;


const INPUT_FILE: &'static str = "../input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
 
fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten();

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    let mut b: HashSet<u8> = HashSet::new();
    let mut buffers: [HashSet<u8>; 3] = [
        HashSet::new(), HashSet::new(), HashSet::new(),
    ];

    for (i, line) in lines.enumerate() {
        let n = i % 3;
        let length = line.len();
        let a: &mut HashSet<u8> = &mut buffers[n];
        a.clear();
        b.clear();
        a.extend(line.chars().map(|x| to_code(&x)).take(length / 2));
        b.extend(line.chars().map(|x| to_code(&x)).skip(length / 2));
        part1 += a.intersection(&b).sum::<u8>() as u32;
        a.extend(&b);

        if n == 2 {
            let temp: HashSet<u8> = buffers[0].intersection(&buffers[1]).cloned().collect();
            part2 += temp.intersection(&buffers[2]).sum::<u8>() as u32;
        }
    }

    println!("Part #1: {:?}", part1);
    println!("Part #2: {:?}", part2);

    Ok(())
}

fn to_code(c: &char) -> u8 {
    match *c as u8 {
        c if c > 96 => { c - 96 }
        c => { c - 38 }
    }
}
