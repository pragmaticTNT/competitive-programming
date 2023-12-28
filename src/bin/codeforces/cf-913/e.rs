use std::io::stdin;
use std::{error::Error, result::Result as StdResult};

use std::collections::HashMap;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let tests: u64 = lines.next().ok_or("t").unwrap().parse().unwrap();

    let mut digit_sum: HashMap<u8, u8> = HashMap::new();
    for i in 0..10 {
        digit_sum.insert(i, (i+1)*(i+2)/2);
    }

    for _t in 0..tests {
        let solution = solve(&mut lines, &digit_sum).unwrap();
        println!("{}", solution);
    }
}

fn solve(lines: &mut dyn Iterator<Item=String>, digit_sum: &HashMap<u8, u8>) -> Result<usize> {
    let mut n: usize = lines.next().unwrap().parse().unwrap();
    let mut num_triples: usize = 1;
    while n > 0 {
        let last: u8 = (n % 10).try_into().unwrap();
        num_triples *= *(digit_sum.get(&last).unwrap()) as usize;
        n /= 10;
    }

    Ok(num_triples.into())
}