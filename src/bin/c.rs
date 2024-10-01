use std::io;

use anyhow::Context;
use itertools::Itertools;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() -> anyhow::Result<()>{
    let mut lines = io::stdin().lines();
    let num_tests: u64 = lines.next().context("no input")??.parse()?;

    for t in 1..=num_tests {
        let line = lines.next().context("line")??;
        let n = line.parse()?;
        let ants: Vec<(i64, i64)> = lines
            .by_ref()
            .take(n)
            .map(|line| 
                line.unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().expect("parse"))
                    .collect_tuple()
                    .context("collect_tuple")
                    .unwrap()
            )
            .collect();

        if let Some(solution) = solve(n, ants) {
            println!("Case #{t}: {solution}");
        } else {
            println!("Case #{t}: FAIL");
        }
    }

    Ok(())
}

fn solve(n: usize, ants: Vec<(i64, i64)>) -> Option<usize> {
    let ants_index: Vec<usize> = if n <= 25 {
        (0..n).collect()
    } else {
        let mut rng = thread_rng();
        let sqrtn: usize = 3*((n as f64).ln() as usize) + 1;
        (0..n).collect::<Vec<usize>>().choose_multiple(&mut rng, sqrtn).cloned().collect::<Vec<usize>>()
    };

    let mut lines = Vec::new();
    // for (p1, p2) in ants_subset.iter().combinations(2) {
    //     let cy = p2.0 - p1.0;
    //     let cx = p2.1 - p1.1;
    //     let b = p1.1*cy - p2.0*cx;
    //     lines.push([cx, cy, b]);
    // }

    for i in 0..ants_index.len() {
        for j in i+1..ants_index.len() {
            let (px, py) = ants[ants_index[i]];
            let (qx, qy) = ants[ants_index[j]];
            let cy = qx - px;
            let cx = qy - py;
            let b = py*cy - px*cx;
            lines.push([cx, cy, b]);
        }
    }
    // println!("Lines: {:?}", lines);

    let mut max_on_line = 0;
    for line in lines {
        let on_line = ants
            .iter()
            .filter(|(x, y)| y*line[1] - x*line[0] == line[2])
            .count();

        if on_line > max_on_line {
            max_on_line = on_line;
        }
    }

    // println!("[{n}] max_on_line: {max_on_line}");
    if max_on_line > n/2 {
        Some(n - max_on_line)
    } else {
        Some(n)
    }

}