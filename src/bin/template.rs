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

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    

    todo!()
}