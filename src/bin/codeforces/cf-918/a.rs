use std::io::stdin;
use std::{error::Error, result::Result as StdResult};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn parse_trips(lines: &mut dyn Iterator<Item=String>) -> [usize; 3] {
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

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let words = parse_trips(lines);
    if words[0] == words[1] {
        Ok(words[2])
    } else if words[1] == words[2] {
        Ok(words[0])
    } else {
        Ok(words[1])
    }
}