use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::cmp::min;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

const MAX_VAL: i32 = 1_000_000;

fn parse_list(lines: &mut dyn Iterator<Item=String>) -> Vec<usize> {
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

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<i32> {
    let n: usize = lines.next().unwrap().parse().unwrap();
    // println!("{}", n);
    let array = parse_list(lines);
    let mut array: Vec<_> = array.into_iter().rev().collect();
    array.extend(&array.clone());
    // println!("Array: {:?}", array);

    let mut inc = Vec::with_capacity(2*n);
    let mut dec = Vec::with_capacity(2*n);
    inc.push(0);
    dec.push(0);

    for (i, w) in array.windows(2).enumerate(){
        // println!("{}, {:?}", i, w);
        inc.push(if w[0] <= w[1] {
            inc[i] + 1
        } else {
            0
        });
        dec.push(if w[0] >= w[1] {
            dec[i] + 1
        } else {
            0
        });

    }
    // println!("{:?}", inc);
    // println!("{:?}", dec);

    let mut result: i32 = MAX_VAL;
    for i in (n-1)..inc.len() {
        if dec[i] == n - 1 {
            result = min(result, (i+1-n).try_into().unwrap());
            result = min(result, (2*n-i+1).try_into().unwrap());
        }
        if inc[i] == n - 1 {
            result = min(result, (i+2-n).try_into().unwrap());
            result = min(result, (2*n-i).try_into().unwrap());
        }
    } 
    
    if result == MAX_VAL {
        Ok(-1)
    } else {
        Ok(result)
    }
}