use std::io;
use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()>{
    let mut lines = io::stdin().lines();
    let num_tests: u64 = lines.next().context("no input")??.parse()?;

    for t in 1..=num_tests {
        let line = lines.next().context("line")??;
        let (n, k) = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect_tuple()
            .context("collect_tuple")?;

        let speeds: Vec<usize> = lines
            .by_ref()
            .take(n)
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        assert_eq!(speeds.len(), n);

        if let Some(solution) = solve(n, k, &speeds) {
            let answer = if solution { "YES" } else { "NO" };
            println!("Case #{t}: {answer}");
        } else {
            println!("Case #{t}: FAIL");
        }
    }

    Ok(())
}

fn solve(n: usize, k: usize, speeds: &Vec<usize>) -> Option<bool> {
    let fast = speeds.iter().min()?;
    if n == 1 {
        Some(k >= *fast)
    } else {
        Some(k >= 2*fast*(n-2) + fast)
    }
}