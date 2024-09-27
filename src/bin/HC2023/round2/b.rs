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
    let mut lowercase = Vec::new();
    let mut uppercase = Vec::new(); 
    let mut index = 0; 

    for c in text.chars(){
        if c == 'b' {
            lowercase.pop();
        } else if c == 'B' {
            uppercase.pop();
        } else {
            if c.is_uppercase() {
                uppercase.push((c, index));
            } else {
                lowercase.push((c, index));
            }
            index += 1;
        }
        // match c {
        //     'b' => lowercase.pop(),
        //     'B' => uppercase.pop(),
        //     _ => {
        //         if c.is_uppercase() {
        //             uppercase.push((c, index));
        //         } else {
        //             lowercase.push((c, index));
        //         }
        //         index += 1;
        //         None // <- why doesn't this work? 
        //     };
        // };
    }

    lowercase.extend(&uppercase);
    lowercase.sort_by(|a, b| { a.1.cmp(&b.1)});

    Ok(lowercase.iter().map(|(c, i)| c).collect())
}