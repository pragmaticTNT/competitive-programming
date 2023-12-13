use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::fmt::Display;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

pub fn parse_pair(lines: &mut dyn Iterator<Item=String>) -> [usize; 2] {
    lines.next().ok_or("nm")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn run<R: Display>(runner: fn(&mut dyn Iterator<Item=String>) -> Result<R>) -> Result<()> {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let tests: u64 = lines.next().ok_or("t")?.parse()?;

    for t in 0..tests {
        let solution = runner(&mut lines)?;
        println!("Case #{}: {}", t + 1, solution);
    }

    Ok(())
}