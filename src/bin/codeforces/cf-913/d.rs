use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::fmt::Display;
use std::cmp::{max, min};

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

fn parse_pair(lines: &mut dyn Iterator<Item=String>) -> [usize; 2] {
    lines.next().ok_or("nm")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn try_k(intervals: &Vec<[usize; 2]>, k: usize) -> bool {
    let mut ub: i32 = k as i32;
    let mut lb: i32 = 0;
    for interval in intervals {
        if ub < interval[0] as i32 || lb > interval[1] as i32 {
            return false;
        }
        ub = min(ub, interval[1].try_into().unwrap()) + k as i32;
        lb = max(lb, interval[0].try_into().unwrap()) - k as i32;
    }
    true
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut intervals = Vec::new();
    for i in 0..n {
        let interval = parse_pair(lines);
        intervals.push(interval);
    }

    if try_k(&intervals, 0) {
        return Ok(0);
    }

    let mut k_ub: usize = 1;
    while !try_k(&intervals, k_ub) {
        k_ub *= 2;
    }
    
    let mut k_lb = k_ub/2;
    while k_ub > k_lb {
        let mid = (k_lb + k_ub)/2;
        if try_k(&intervals, mid) {
            k_ub = mid;
        } else {
            k_lb = mid + 1;
        }
    }

    Ok(k_ub)
}