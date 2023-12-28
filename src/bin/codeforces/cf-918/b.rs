use std::io::stdin;
use std::{error::Error, result::Result as StdResult};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

use std::collections::HashSet;

fn parse_pair(lines: &mut dyn Iterator<Item=String>) -> [usize; 2] {
    lines.next().ok_or("nm")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let tests: u64 = lines.next().ok_or("t").unwrap().parse().unwrap();

    for _t in 0..tests {
        let solution = solve(&mut lines).unwrap();
        println!("{}", solution);
    }
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<char> {
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();
    let line3 = lines.next().unwrap();
    
    for line in [&line1, &line2, &line3] {
        let mut visit = HashSet::from(['A', 'B', 'C']);
        for c in line.chars(){
            if c != '?' {
                visit.remove(&c);
            }
        }
        if visit.len() == 1 {
            let remainder = visit.iter().next().unwrap();
            return Ok(*remainder)
        }
    }
    unreachable!()
}