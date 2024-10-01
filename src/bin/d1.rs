use std::io;
use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()>{
    let mut lines = io::stdin().lines();
    let num_tests: u64 = lines.next().context("no input")??.parse()?;

    for t in 1..=num_tests {
        let line = lines.next().context("line")??;
        let (n, g) = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect_tuple()
            .context("collect_tuple")?;

        let mut speeds: Vec<usize> = lines
            .by_ref()
            .take(n)
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        assert_eq!(speeds.len(), n);

        if let Some((index, dist)) = solve(n, g, &mut speeds) {
            println!("Case #{t}: {index} {dist}");
        } else {
            println!("Case #{t}: FAIL");
        }
    }

    Ok(())
}

fn solve(n: usize, g: usize, speeds: &mut Vec<usize>) -> Option<(usize, usize)> {
    speeds.sort_by(|a, b| a.cmp(b));
    
    // println!("[{n}] speeds: {:?}", speeds.iter().enumerate().collect::<Vec<_>>());

    match speeds.binary_search(&g) {
        Ok(index) => {
            Some((n - index, 0))
        },
        Err(index) => {
            if index == n {
                Some((1, g - speeds[index-1]))
            } else if index == 0 {
                Some((n, speeds[index] - g))
            } else {
                let left_dist = g - speeds[index-1];
                let right_dist = speeds[index] - g;
                if left_dist < right_dist {
                    Some((n - index + 1, left_dist))
                } else {
                    Some((n - index, right_dist))
                }
            }
        }
    }
}