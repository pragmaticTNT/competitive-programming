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

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<String> {
    let text = lines.next().unwrap();
    let mut lowercase = Vec::with_capacity(text.len());
    let mut uppercase = Vec::with_capacity(text.len()); 
    let mut index = 0; 

    for &c in text.as_bytes(){
        match c {
            b'b' => { lowercase.pop(); },
            b'B' => { uppercase.pop(); },
            _ => {
                if c.is_ascii_uppercase() {
                    uppercase.push((c, index));
                } else {
                    lowercase.push((c, index));
                }
                index += 1;
            },
        }
    }

    let mut result = String::with_capacity(lowercase.len() + uppercase.len());
    let mut lowercase = lowercase.into_iter().peekable();
    let mut uppercase = uppercase.into_iter().peekable();
    while let (Some(lc), Some(uc)) = (lowercase.peek(), uppercase.peek()) {
        if lc.1 < uc.1 {
            result.push(lc.0 as char);
            lowercase.next();
        } else {
            result.push(uc.0 as char);
            uppercase.next();
        }
        
    }

    result.extend(lowercase.map(|c| c.0 as char));
    result.extend(uppercase.map(|c| c.0 as char));

    // lowercase.extend_from_slice(&uppercase);
    // lowercase.sort_by_key(|a| a.1);

    Ok(result)
}