use std::io::stdin;
use std::{error::Error, result::Result as StdResult};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let tests: u64 = lines.next().ok_or("t").unwrap().parse().unwrap();

    for _t in 0..tests {
        let solution = solve(&mut lines).unwrap();
        println!("{}", solution);
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e'
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<String> {
    let n: usize = lines.next().unwrap().parse().unwrap();
    let word = lines.next().unwrap();
    let word: Vec<char> = word.chars().collect();
    let mut result = String::new();

    for (i,c) in word.iter().enumerate() {
        if i > 0 && !is_vowel(*c) {
            if (i > 0 && !is_vowel(word[i-1])) || (i < (word.len()-1) && is_vowel(word[i+1])) {
                result.push('.')
            }
        }
        result.push(*c)
    }

    Ok(result)
}