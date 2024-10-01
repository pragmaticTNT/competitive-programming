use std::io;
use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()>{
    let mut lines = io::stdin().lines();
    let num_tests: u64 = lines.next().context("no input")??.parse()?;

    for t in 1..=num_tests {
        let line = lines.next().context("line")??;
        let (n, p) = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect_tuple()
            .context("collect_tuple")?;

        if let Some(solution) = solve(n, p) {
            println!("Case #{t}: {solution}");
        } else {
            println!("Case #{t}: FAIL");
        }
    }

    Ok(())
}

fn solve(n: f64, p: f64) -> Option<f64> {
    let q = (p/100.0).powf((n-1 as f64)/(n as f64));
    Some(q * 100. - p)
}