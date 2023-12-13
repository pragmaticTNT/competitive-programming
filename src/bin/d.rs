use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::fmt::Display;

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

fn try_k(intervals: &Vec<(usize, usize)>, k: usize) -> bool {
    true
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let n: usize = lines.next().unwrap().parse().unwrap();
    let intervals = Vec::new();
    for i in 0..n {
        let (left, right) = lines.next().unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .next_tuple();
        intervals.push((left, right));
    }

    let mut k = 0;
    while !try_k(&intervals, k) {
        if k == 0 {
            k += 1;
        } else {
            k *= 2;
        }
    }

    Ok(k)
}