use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::collections::HashSet;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn parse_vector(lines: &mut dyn Iterator<Item=String>) -> Vec<i64> {
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

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<&str> {
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut juice = parse_vector(lines);
    for i in 0..n {
        if i % 2 == 1 {
            juice[i] = -juice[i];
        }
    }

    let mut curr = 0;
    let mut pos: HashSet<i64> = HashSet::new();
    pos.insert(0);
    for cup in juice {
        curr += cup;
        if pos.contains(&curr) {
            return Ok("YES")
        }
        pos.insert(curr);
    }
    Ok("NO")
}