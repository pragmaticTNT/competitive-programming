use std::io::stdin;
use std::{error::Error, result::Result as StdResult};
use std::cmp::{max, min};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

fn parse_pair(lines: &mut dyn Iterator<Item=String>) -> [i32; 2] {
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

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut intervals = Vec::new();

    for _ in 0..n {
        let interval = parse_pair(lines);
        intervals.push(interval)
    }
    // let mut result = 0;
    // for (i, u) in intervals.iter().enumerate() {
    //     for j in i..n {
    //         let v = intervals[j];
    //         result += if (u[0] < v[0] && u[1] > v[1]) || (u[0] > v[0] && u[1] < v[1]) {
    //             1
    //         } else {
    //             0
    //         }
    //     }
    // }
    let s = intervals.iter().fold(intervals[0][0], |acc, b| min(acc, b[0]));
    let t = intervals.iter().fold(intervals[0][1], |acc, b| max(acc, b[1]));
    Ok(recurse(&intervals, s, t))
}

fn recurse(intervals: &Vec<[usize,2]>, s: f64, t: f64) -> usize {
    if intervals.len() == 1 {
        return 0
    }

    let mid: f64 = (s+t)/2;
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut cross = Vec::new();
    for interval in intervals {
        if interval[1] <= mid {
            left.push(interval.clone());
        } else if interval[0] > mid {
            right.push(interval.clone());
        } else {
            cross.push(interval.clone());
        }
    }

    // conquer
    let mut result = 0;
    cross.sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });
    
    // recurse
    result += recurse(&left, s, mid) + recurse(&right, mid, t);
    result
}