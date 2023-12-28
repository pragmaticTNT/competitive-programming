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

const ROWS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const COLS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<String> {
    let rook = lines.next().unwrap();
    let mut rook = rook.chars(); 
    let row = rook.next().unwrap();
    let col = rook.next().unwrap();
    let mut result = String::new();

    for char in ROWS {
        if char != row {
            result.push(char);
            result.push(col);
            result.push('\n');
        }
    }

    for char in COLS {
        if char != col {
            result.push(row);
            result.push(char);
            result.push('\n');
        }
    }

    Ok(result)
}