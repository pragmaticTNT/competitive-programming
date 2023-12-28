use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::fmt::Display;

use std::collections::HashMap;
use std::cmp::max;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

pub fn run<R: Display>(runner: fn(&mut dyn Iterator<Item=String>) -> Result<R>) -> Result<()> {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let tests: u64 = lines.next().ok_or("t")?.parse()?;

    for _t in 0..tests {
        let solution = runner(&mut lines)?;
        println!("{}", solution);
    }

    Ok(())
}

fn main() {
    run(solve).unwrap();
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let s_length: i32 = lines.next().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap();
    let mut frequency = HashMap::new();

    for c in s.chars() {
        frequency.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }

    let majority: i32 = frequency.values().fold(0, |a, b| a.max(*b));
    let parity: i32 = if s_length % 2 == 1 { 1 } else { 0 };
    // println!("n: {}, majority: {}", s_length, majority);
    Ok(max(parity, 2*majority - s_length) as usize)
}