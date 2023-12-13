use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io,
};
use competition::*;

fn main(){
    run(solve).unwrap()
    // let mut lines = io::stdin().lines();
    // let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    // for i in 1..=num_tests {
    //     let line = lines.next().context("line")??;
    //     let (r, c) = line
    //         .split_whitespace()
    //         .map(|s| s.parse().unwrap())
    //         .collect_tuple()
    //         .unwrap();
    //     let board = parse_grid(lines.by_ref().take(r).map(Result::unwrap));
    //     assert_eq!(board.cells.len(), r);
    //     for row in &board.cells {
    //         assert_eq!(row.len(), c);
    //     }

    //     // dbg!(board.dims());

    //     let ans = board.best_move();
    //     // let ans = if ans != 0 { "YES" } else { "NO" };
    //     println!("Case #{i}: {ans}");
    // }
    // debug_assert!(lines.next().is_none());
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let line = lines.next().ok_or("line")?;
    let (r, c) = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let board = parse_grid(lines.take(r));
    assert_eq!(board.cells.len(), r);
    for row in &board.cells {
        assert_eq!(row.len(), c);
    }

    //dbg!(&board);

    Ok(board.best_move())
}

fn parse_grid(lines: impl Iterator<Item = String>) -> Board {
    let cells = lines.map(parse_row).collect();
    Board { cells }
}

fn parse_row(line: String) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '.' => None,
            'B' => Some(Black),
            'W' => Some(White),
            _ => panic!("{c:?}"),
        })
        .collect()
}

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

type Cell = Option<Color>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

use itertools::Itertools;
use Color::*;

type Point = (isize, isize);

impl Board {
    /// Black to move. How many white stones can they capture?
    fn best_move(&self) -> usize {
        let (n_rows, n_cols) = self.dims();

        // Find all white groups
        let mut groups = vec![];
        let mut seen = HashMap::new(); // point -> group_idx
        for i in 0..n_rows {
            for j in 0..n_cols {
                let p = (i, j);
                if self.get(p) == Some(White) && !seen.contains_key(&p) {
                    let g = self.explore_group(p);
                    for &p2 in &g.white_stones {
                        seen.insert(p2, groups.len()); // groups.len() is the index
                    }
                    groups.push(g);
                }
            }
        }

        // Find the best black move
        let mut best = 0;
        for i in 0..n_rows {
            for j in 0..n_cols {
                let p = (i, j);
                if self.get(p).is_none() {
                    best = max(best, self.score(p, &groups, &seen));
                }
            }
        }
        best
    }

    /// If black plays here, how many white stones are captured?
    fn score(&self, p: Point, groups: &[Group], group_keys: &HashMap<Point, usize>) -> usize {
        let mut captured_groups = HashSet::new();
        for p2 in self.neighbors(p) {
            if let Some(&g) = group_keys.get(&p2) {
                if groups[g].liberties == 1 {
                    captured_groups.insert(g);
                }
            }
        }

        let mut score = 0;
        for g in captured_groups {
            score += groups[g].white_stones.len();
        }
        score
    }

    fn neighbors(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        let dims = self.dims();
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(di, dj)| (i + di, j + dj))
            .filter(move |&p| in_range(dims, p))
    }

    fn explore_group(&self, start: Point) -> Group {
        fn dfs(b: &Board, p: Point, seen: &mut HashSet<Point>, g: &mut Group) {
            let mut stack = vec![p];
            while let Some(p) = stack.pop() {
                if seen.contains(&p) {
                    //return;
                    continue;
                }
                seen.insert(p);

                match b.get(p) {
                    None => g.liberties += 1,
                    Some(White) => {
                        g.white_stones.insert(p);
                        for p2 in b.neighbors(p) {
                            stack.push(p2);
                            // if b.in_range(p2) { // this shouldn't be necessary
                            //     // dfs(b, p2, seen, g)
                            //     stack.push(p2);
                            // }
                        }
                    }
                    Some(Black) => (),
                }
            }
        }

        let mut seen = HashSet::new();
        let mut g = Group::default();
        dfs(self, start, &mut seen, &mut g);
        g
    }

    fn get(&self, (i, j): Point) -> Cell {
        assert!(self.in_range((i, j)));
        self.cells[i as usize][j as usize]
    }

    fn dims(&self) -> Point {
        let n_rows = self.cells.len() as isize;
        let n_cols = self.cells[0].len() as isize;
        (n_rows, n_cols)
    }

    fn in_range(&self, p: Point) -> bool {
        in_range(self.dims(), p)
    }
}

fn in_range((n_rows, n_cols): Point, (i, j): Point) -> bool {
    0 <= i && i < n_rows && 0 <= j && j < n_cols
}

#[derive(Debug, Default)]
struct Group {
    white_stones: HashSet<Point>,
    liberties: usize,
}