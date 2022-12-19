use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn eval_round_1(round : String) -> i32 {
    let l_str = round.chars().nth(0).unwrap();
    let mut l : i32 = 0;
    match l_str {
        'A' => l = 0,
        'B' => l = 1,
        'C' => l = 2,
        _ => (),
    }

    let r_str = round.chars().nth(2).unwrap();
    let mut r : i32 = 0;
    match r_str {
        'X' => r = 0,
        'Y' => r = 1,
        'Z' => r = 2,
        _ => (),
    }

    let diff = (3 + 3*(( 3 + r - l)%3)) % 9; // gives 0 for loss, 3 for draw, 6 for win
    r + diff + 1 // since r is one less than the points
}

fn eval_round_2(round : String) -> i32 {
    let l_str = round.chars().nth(0).unwrap();
    let mut l : i32 = 0;
    match l_str {
        'A' => l = 0,
        'B' => l = 1,
        'C' => l = 2,
        _ => (),
    }

    let r_str = round.chars().nth(2).unwrap();
    let mut r : i32 = 0;
    match r_str {
        'X' => r = 0,
        'Y' => r = 1,
        'Z' => r = 2,
        _ => (),
    }

    let choice = (l + 2 + r)%3; // shift it depending on outcome condition
    3*r + choice + 1
}

pub fn part1(filename : &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            sum += eval_round_1(line.unwrap());
        }
        return sum;
    }
    0
}

pub fn part2(filename : &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            sum += eval_round_2(line.unwrap());
        }
        return sum;
    }
    0
}