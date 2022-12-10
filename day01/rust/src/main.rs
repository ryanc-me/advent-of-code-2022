use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::BinaryHeap;


const INPUT_FILE: &'static str = "../input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
 
fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().flatten();

    let mut buffer = 0;
    let mut calories = BinaryHeap::<u32>::new();

    loop {
        match lines.next() {
            Some(line) if !line.is_empty() => {
                let num: u32 = line.parse()?;
                buffer += num;
            }
            res => {
                calories.push(buffer);
                buffer = 0;

                if res.is_none() {
                    break
                }
            }
        }
    }

    let part1 = &calories.peek().expect("No calories found in input file!");
    let part2: u32 = (&calories).iter().take(3).sum();

    println!("Part #1: {:?}", part1);
    println!("Part #2: {:?}", part2);

    Ok(())
}
