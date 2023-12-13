use competition::*;
use std::collections::{HashMap, HashSet};

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
        let (nr, nc) = self.dims();
        let mut storage = UnionFind{ point_to_node: HashMap::new()};
        let mut white_stones = Vec::new();

        // Initialize all white pieces to nodes in the Union Find
        for r in 0..nr {
            for c in 0..nc {
                if self.get((r,c)) == Some(Color::White) {
                    white_stones.push((r,c));
                    storage.add((r,c));
                    for nbour in self.neighbors((r,c)) {
                        if self.get(nbour) == None {
                            storage.get_mut((r,c)).unwrap().liberties.insert(nbour);
                        }
                    }
                }
            }
        }

        // Loop through all white nodes and group them with their nbours
        for stone in white_stones.iter() {
            // eprintln!(">>> Checking stone: {:?}", *stone);
            for nbour in [(-1, 0), (0, -1)].into_iter()
                .map(move |(di, dj)| (stone.0 + di, stone.1 + dj))
                .filter(move |&p| in_range(self.dims(), p)){
                if storage.contains(nbour) {
                    storage.merge(nbour, *stone);
                    // eprintln!("> leader after merging with {:?}: {:?}", nbour, storage.get_leader(*stone));
                    // eprintln!("> liberties of leader: {}", storage.get(storage.get_leader(*stone)).unwrap().liberties.len());
                }
            }
        } 
        // eprintln!("Number of stones: {}", white_stones.len());

        let mut groups: HashMap<&Point, usize> = HashMap::new();
        for stone in white_stones {
            let leader = storage.get_leader(stone);
            //eprintln!("Leader: {:?} ({:?})", leader, storage.get(leader).unwrap().liberties);;
            let leader_node = storage.get(leader).unwrap();
            if leader_node.liberties.len() == 1 {
                let liberty = leader_node.liberties.iter().next().unwrap();

                let entry = groups.entry(liberty).or_insert(0);
                *entry += 1;
            }
        }
        // eprintln!("Final groups: {:?}", groups);

        groups.values().copied().max().unwrap_or(0)
    }

    fn neighbors(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        let dims = self.dims();
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(di, dj)| (i + di, j + dj))
            .filter(move |&p| in_range(dims, p))
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

#[derive(PartialEq, Eq, Debug)]
struct Node {
    leader: Point,
    liberties: HashSet<Point>,
}

#[derive(Debug)]
struct UnionFind {
    point_to_node: HashMap<Point,Node>,
}

impl UnionFind {
    fn add(&mut self, point: Point) { 
        if !self.point_to_node.contains_key(&point) {
            let new_node = Node { leader: point, liberties: HashSet::new() };
            self.point_to_node.insert(point, new_node);
        }
    } // add point with leader: point

    fn merge(&mut self, n1: Point, n2: Point) -> bool {
        let n1_leader = self.get_leader(n1);
        let n2_leader = self.get_leader(n2);
        if n1_leader != n2_leader {
            self.get_mut(n2_leader).unwrap().leader = n1;
            self.update_path(n2, n1_leader);
            // eprintln!("N1: {:?}, N2: {:?}", n1, n2);
            // eprintln!("L1: {:?}, L2: {:?}", n1_leader, n2_leader);
            // eprintln!("Liberties: {}", self.get(n1_leader).unwrap().liberties.len());
            true
        } else {
            false
        }
    } // make p1 the leader of p2 and compress path
    
    fn contains(&self, point: Point) -> bool { self.point_to_node.contains_key(&point) }
    fn get(&self, point: Point) -> Option<&Node> { self.point_to_node.get(&point) }
    fn get_mut(&mut self, point: Point) -> Option<&mut Node> { self.point_to_node.get_mut(&point) } // returns point associated with node
    fn get_leader(&self, point: Point) -> Point {
        let mut leader = self.get(point).unwrap().leader;
        while self.get(leader).unwrap().leader != leader {
            leader = self.get(leader).unwrap().leader;
        } 
        leader 
    } // return point's leader (recurse if necessary)

    fn update_path(&mut self, point: Point, leader: Point) {
        let mut point = point;
        while point != leader {
            let new_liberties = self.get(point).unwrap().liberties.clone();
            self.get_mut(leader).unwrap().liberties.extend(new_liberties);

            let temp_leader = self.get(point).unwrap().leader;
            self.get_mut(point).unwrap().leader = leader;
            point = temp_leader;
        }
        //eprintln!("Point: {:?}, Leader: {:?}, Libs: {:?}", point, leader, self.get(point).unwrap().liberties);
    }// path compression
}

fn main() {
    run(solve).unwrap()
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

        // dbg!(board.dims());

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