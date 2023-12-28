use std::io::stdin;
use std::{error::Error, result::Result as StdResult};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn parse_vector(lines: &mut dyn Iterator<Item=String>) -> Vec<usize> {
    lines.next().ok_or("nm")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
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
    let n: usize = lines.next().unwrap().parse().unwrap();
    let sign: String = lines.next().unwrap();
    let child = parse_vector(lines);
    let sign: Vec<bool> = sign.chars().map(|c| 
        if c == '0'{
            false
        } else {
            true
        }).collect();
    
    
    todo!()
}