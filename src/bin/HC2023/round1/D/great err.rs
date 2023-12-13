use competition::*;
use std::cmp::{min, max};
//const BIG_PRIME: u32 = 1_000_000_007;
const BIG_PRIME: u32 = 107;

fn main() {
    run(solve).unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Node {
    max_val: u32,
    min_val: u32,
    negated: bool,
    max_index: usize,
    min_index: usize,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            max_val: 0,
            min_val: u32::MAX,
            negated: false,
            min_index: 0,
            max_index: 0,
        }
    }
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let n: usize = lines.next().unwrap().parse()?;
    let days: Vec<u32> = lines.next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse())
                .collect::<std::result::Result<_, _>>()?;
    let q: usize = lines.next().unwrap().parse()?;
    let queries: Vec<_> = (0.. q).map(move |_| parse_pair(lines)).collect();  

    let mut days_heap = DaysHeap::new(days);

    let mut result = 0;
    for [li, ri] in queries{
        days_heap.update_recursive(HeapIndex(1), li-1, ri-1);
        // {
        //     eprintln!("Query ({li}, {ri})");
        //     for (i, node) in days_heap.data.iter().enumerate() {
        //         let heap_index = HeapIndex(i + 1);
        //         eprintln!("\tNode ({}-{} {}) min_idx: {}, max_idx: {}", 
        //             days_heap.left_index(heap_index), 
        //             days_heap.right_index(heap_index), 
        //             node.negated, 
        //             node.min_index, 
        //             node.max_index);
        //     }
        // }
        let max_index = days_heap.data[0].actual_max_index() + 1;
        //eprintln!("maximum index: {}", max_index);
        result += max_index;
    }
    Ok(result)
}

impl Node {
    fn actual_max_val(self) -> u32 {
        if self.negated { BIG_PRIME - self.min_val } else { self.max_val }
    }

    fn actual_min_val(self) -> u32 {
        if self.negated { BIG_PRIME - self.max_val } else { self.min_val }
    }

    fn actual_max_index(self) -> usize {
        if self.negated { self.min_index } else { self.max_index }
    }

    fn actual_min_index(self) -> usize {
        if self.negated { self.max_index } else { self.min_index }
    }
}


struct DaysHeap {
    data: Vec<Node>,
    number_of_days: usize,
}

impl DaysHeap {
    fn new(days: Vec<u32>) -> Self {
        let heap_length = days.len().next_power_of_two() - 1 + days.len();
        let mut data: Vec<Node> = vec![Default::default(); heap_length];

        for (i, &val) in days.iter().enumerate()  {
            let index = days.len().next_power_of_two() - 1 + i;
            data[index].max_val = val;
            data[index].min_val = val;
            data[index].max_index = i;
            data[index].min_index = i;
        }


        let mut ret = DaysHeap {
            data,
            number_of_days: days.len(),
        };

        // Update internal nodes
        for i in (0.. days.len().next_power_of_two() - 1).rev() {
            ret.update_min_max(HeapIndex(i+1));
        }

        ret
    }

    /// Update node at heap_index (non-recursively)
    /// to respect it's childrens largest and smallest values.
    fn update_min_max(&mut self, heap_index: HeapIndex) {
        let left = self.get(heap_index.left()).copied();
        let right = self.get(heap_index.right()).copied();

        match (left, right) {
            (Some(left_node), Some(right_node)) => { 
                if left_node.actual_max_val() >= right_node.actual_max_val() {
                    self[heap_index].max_val = left_node.actual_max_val();
                    self[heap_index].max_index = left_node.actual_max_index();
                } else {
                    self[heap_index].max_val = right_node.actual_max_val();
                    self[heap_index].max_index = right_node.actual_max_index();
                }

                if left_node.actual_min_val() <= right_node.actual_min_val() {
                    self[heap_index].min_val = left_node.actual_min_val();
                    self[heap_index].min_index = left_node.actual_min_index();
                } else {
                    self[heap_index].min_val = right_node.actual_min_val();
                    self[heap_index].min_index = right_node.actual_min_index();
                }
            }
            (Some(node), None) | (None, Some(node)) => {
                self[heap_index].max_val = node.actual_max_val();
                self[heap_index].max_index = node.max_index;
                self[heap_index].min_val = node.actual_min_val();
                self[heap_index].min_index = node.min_index;
            }
            (None, None) => { } // Unused node, nothing to update...
        }
    }

    fn update_recursive(&mut self, heap_index: HeapIndex, li: usize, ri: usize) {
        if self.get(heap_index).is_none() {
            return 
        }
        match (li <= self.left_index(heap_index), ri >= self.right_index(heap_index)) {
            (true, true) => {
                self[heap_index].negated = !self[heap_index].negated;
                return 
            },
            _ => {
                //eprintln!("{}-{} checking branches!", self.left_index(heap_index), self.right_index(heap_index));
                let left_child = heap_index.left();
                let right_child = heap_index.right();
                if self.right_index(left_child) >= li {
                    let ri_new = min(ri, self.right_index(left_child));
                    //eprintln!("==> {} LEFT branch with ({} {})", heap_index, li, ri_new);
                    self.update_recursive(heap_index.left(), li, ri_new);
                }
                if self.left_index(right_child) <= ri {
                    let li_new = max(li, self.left_index(right_child));
                    //eprintln!("==> {} RIGHT Branch with ({} {})", heap_index, li_new, ri);
                    self.update_recursive(heap_index.right(), li_new, ri);
                }
            }, 
        }
        self.update_min_max(heap_index)
    }

    fn total_depth(&self) -> usize {
        self.data.len().ilog2() as usize
    }

    fn heap_index_to_day_index(&self, heap_index: HeapIndex) -> Option<usize> {
        let start_final_row = 2usize.pow(self.total_depth() as u32);
        if start_final_row <= heap_index.0 {
            Some(heap_index.0 - start_final_row)
        } else {
            None
        }
    }
    /// Go from a node in the heap, to the index of the leftmost day that node represents
    fn left_index(&self, heap_index: HeapIndex) -> usize {
        let result_heap_index = HeapIndex(heap_index.0 * 2usize.pow((self.total_depth() - heap_index.depth()) as u32));
        let result_day_index = self.heap_index_to_day_index(result_heap_index).expect("Result heap index not on last row");
        min(result_day_index, self.number_of_days - 1)
    }

    fn right_index(&self, heap_index: HeapIndex) -> usize {
        let mut result_heap_index = heap_index;
        for _ in 0.. (self.total_depth() - heap_index.depth()) {
            result_heap_index = result_heap_index.right()
        }
        let day_index = self.heap_index_to_day_index(result_heap_index).unwrap();
        min(day_index, self.number_of_days - 1) 
    }

    fn get(&self, index: HeapIndex) -> Option<&Node> {
        self.data.get(index.0 - 1)
    }
}

#[derive(Clone, Copy, Debug)]
struct HeapIndex(usize);

impl std::fmt::Display for HeapIndex {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

impl HeapIndex {
    fn left(self) -> Self {
        HeapIndex(self.0 * 2)
    }

    fn right(self) -> Self {
        HeapIndex(self.0 * 2 + 1)
    }

    fn depth(self) -> usize {
        self.0.ilog2() as usize
    }
}

impl std::ops::Index<HeapIndex> for DaysHeap {
    type Output = Node;

    fn index(&self, index: HeapIndex) -> &Node {
        &self.data[index.0 - 1]
    }
}

impl std::ops::IndexMut<HeapIndex> for DaysHeap {
    fn index_mut(&mut self, index: HeapIndex) -> &mut Node {
        &mut self.data[index.0 - 1]
    }
}

//       1
//   2       3
// 4   5   6   7
//8 9 0 1 2 
// heap.len() = 7
// heap.len().next_power_of_two() = 8
// / 2 = 4
// - 1 = 3  


//      root
//   (0-1)     2-3
// 0    [1]   2   3

// 0..=2
// 1..=2
// is_negated on nodes 0-1, 2
// 2-3, root we *modify* min_val and max_val based on their children
// Don't touch 0 and 1

// 11
// 1 2 3 4 5 6 7 8 indices
// 1 2 5 5 4 1 6 2 original
//              (1,8)
//      (1,4)           (5,8)
//  (1,2)   (3,4)   (5,6)   (7,8)
// (1) (2) (3) (4) (5) (6) (7) (8)
//  ^