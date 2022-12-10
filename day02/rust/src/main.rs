use std::fs::File;
use std::io::{prelude::*, BufReader};


const INPUT_FILE: &'static str = "../input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
 
fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten();

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    for line in lines {
        // extract #0 (oponent move) and #2 (my move/result) as integers
        // from 1-3 (where 1 = rock, 2 = paper, etc)
        let mut chars = line.chars().step_by(2);
        let (op, me) = (
            chars.next().ok_or(format!("Malformed input: '{}'", &line))? as u8 - 64,
            chars.next().ok_or(format!("Malformed input: '{}'", &line))? as u8 - 87
        );

        // part 1
        part1 += me as u32;
        let result1 = get_result(me, op)?;
        if result1 == 2 {
            // draw
            part1 += 3;
        }
        else if result1 == 3 {
            // win
            part1 += 6;

        }

        // part 2
        let result2 = get_move(op, me)?;
        part2 += result2 as u32;
        if me == 2 {
            // draw
            part2 += 3;
        }
        else if me == 3 {
            // win
            part2 += 6;
        }
    }

    println!("Part #1: {:?}", part1);
    println!("Part #2: {:?}", part2);

    Ok(())
}

fn get_result(our_move: u8, their_move: u8) -> Result<u8> {
    //! Find the outcome of a Rock, Paper, Scissors round
    let mut result = our_move as i8 - their_move as i8 - 1;
    while result < 1 {
        result += 3;
    }

    Ok(result.try_into()?)
}

fn get_move(their_move: u8, desired_result: u8) -> Result<u8> {
    //! Find the move needed to get the desired round result
    let mut result = desired_result + their_move + 1;
    while result > 3 {
        result -= 3;
    }

    Ok(result)
}
