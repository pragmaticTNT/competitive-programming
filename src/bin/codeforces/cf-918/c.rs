use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::cmp::max;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn parse_vector(lines: &mut dyn Iterator<Item=String>) -> Vec<usize> {
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
    let cubes = parse_vector(lines);
    let result: u64 = cubes.iter().fold(0, |acc, b| acc + *b as u64);
    let mut try_sqrt = (result as f64).sqrt() as i64;
    try_sqrt = max(0, try_sqrt-2);
    let mut try_sqrt = try_sqrt as u64;
    
    while try_sqrt*try_sqrt < result {
        try_sqrt += 1;
        if try_sqrt*try_sqrt == result {
            return Ok("YES")
        }
    }
    Ok("NO")
}