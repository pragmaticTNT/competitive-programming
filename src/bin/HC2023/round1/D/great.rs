use competition::*;
use petgraph::graphmap::Nodes;
use std::cmp::{min, max};
const BIG_PRIME: u32 = 1_000_000_007;
//const BIG_PRIME: u32 = 107;

fn main() {
    run(solve).unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Node {
    max_val: u32, // in [1 to BIG_PRIME - 1]
    min_val: u32, // in [1 to BIG_PRIME - 1]
    negated: bool,
    max_index: usize,
    min_index: usize,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            max_val: 1,
            min_val: BIG_PRIME - 1,
            negated: false,
            min_index: 0,
            max_index: 0,
        }
    }
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let _: usize = lines.next().unwrap().parse()?;
    let days: Vec<u32> = lines.next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse())
                .collect::<std::result::Result<_, _>>()?;
    let q: usize = lines.next().unwrap().parse()?;
    let queries: Vec<_> = (0.. q).map(move |_| parse_pair(lines)).collect();  

    let mut days_heap = DaysHeap::new(days);
    //days_heap.show();

    let mut result = 0;
    for [li, ri] in queries{
        //println!("query: {}, {}", li, ri);
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
        let max_index = days_heap.data[0].unwrap().actual_max_index() + 1;
        // println!("maximum index: {}", max_index);
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
    data: Vec<Option<Node>>,
    number_of_days: usize,
}

impl DaysHeap {
    fn new(days: Vec<u32>) -> Self {
        let heap_length = days.len().next_power_of_two() - 1 + days.len();
        let mut data = vec![None; heap_length];

        for (i, &val) in days.iter().enumerate()  {
            let index = days.len().next_power_of_two() - 1 + i;
            debug_assert_eq!(index, data.len() - days.len() + i);
            data[index] = Some(Node {
                max_val: val,
                min_val: val,
                max_index: i,
                min_index: i,
                negated: false,
            })
        }

        let mut ret = DaysHeap {
            data,
            number_of_days: days.len(),
        };

        // Update internal nodes
        for i in (0.. days.len().next_power_of_two() - 1).rev() {
            let heap_i = HeapIndex(i + 1); 
            if ret.left_index(heap_i) >= days.len() {
                debug_assert!(ret.get(heap_i.left()).is_none());
                debug_assert!(ret.get(heap_i.right()).is_none());
                continue;
            }
            ret.data[i] = Some(ret.make_internal_node(heap_i))
        }

        ret
    }

    fn show(&self) {
        for i in 0..self.data.len() {
            let idx = HeapIndex(i+1);
            println!("{}, {}", self.left_index(idx), self.right_index(idx));
        }
    }

    /// Update node at heap_index (non-recursively)
    /// to respect it's childrens largest and smallest values.
    fn make_internal_node(&mut self, heap_index: HeapIndex) -> Node {
        let left = self.get(heap_index.left()).copied();
        let right = self.get(heap_index.right()).copied();

        match (left, right) {
            (Some(left_node), Some(right_node)) => {                 
                let mut new_node = Node::default();
                if left_node.actual_max_val() >= right_node.actual_max_val() {
                    new_node.max_val = left_node.actual_max_val();
                    new_node.max_index = left_node.actual_max_index();
                } else {
                    new_node.max_val = right_node.actual_max_val();
                    new_node.max_index = right_node.actual_max_index();
                }
                if left_node.actual_min_val() <= right_node.actual_min_val() {
                    new_node.min_val = left_node.actual_min_val();
                    new_node.min_index = left_node.actual_min_index();
                } else{
                    new_node.min_val = right_node.actual_min_val();
                    new_node.min_index = right_node.actual_min_index();
                }
                new_node
            }
            (Some(node), None) | (None, Some(node)) => {
                Node {
                    max_val: node.actual_max_val(),
                    max_index: node.actual_max_index(),
                    min_val: node.actual_min_val(),
                    min_index: node.actual_min_index(),
                    negated: false,
                }
            }
            (None, None) => {
                unreachable!()
            } // Unused node, nothing to update...
        }
    }

    fn leaf_values(&self) {
        let n = self.data.len();
        let mut values = vec![];
        print!("[");
        for i in (n - self.number_of_days)..n {
            if i != n - self.number_of_days {
                print!(", ");
            }
            let mut negated = self.data[i].unwrap().negated;

            let mut pi = HeapIndex(i + 1);
            while pi != HeapIndex(1) {
                pi = pi.parent();
                negated ^= self[pi].negated
            }
            let actual_val = if negated { BIG_PRIME - self.data[i].unwrap().max_val } else { self.data[i].unwrap().max_val };
            values.push(actual_val);

            print!("{actual_val}");
        }
        println!("]");
        println!("argmax: {}", values.into_iter().enumerate().max_by_key(|&(_, val)| val).unwrap().0); 
    }

    // fn check_invariants(&self, heap_index: HeapIndex) {
    //     let (left, right) = self.leaf_range(heap_index);
    //     let leaf_values = vec![];
    //     for leafi in left..= n - right {
    //         let heap_index_raw = self.data.len() - self.number_of_days + leafi + 1;
    //         let negated = self.data[heap_index]; 
    //     }
    // }

    fn leaf_range(&self, heap_index: HeapIndex) -> (usize, usize) {
        (self.left_index(heap_index), self.right_index(heap_index))
    }

    fn update_recursive(&mut self, heap_index: HeapIndex, li: usize, ri: usize) {
        if self.get(heap_index).is_none() {
            return 
        }
        match (li <= self.left_index(heap_index), self.right_index(heap_index) <= ri) {
            (true, true) => {
                //println!("Modified: {}, {}", self.left_index(heap_index), self.right_index(heap_index));
                self[heap_index].negated = !self[heap_index].negated;
                return 
            },
            _ => {
                //eprintln!("{}-{} checking branches!", self.left_index(heap_index), self.right_index(heap_index));
                let left_child = heap_index.left();
                let right_child = heap_index.right();
                if self.right_index(left_child) >= li {
                    self.update_recursive(heap_index.left(), li, ri);
                }
                if self.left_index(right_child) <= ri {
                    self.update_recursive(heap_index.right(), li, ri);
                }
            }, 
        }
        self[heap_index] = Node {
            negated: self[heap_index].negated,
            .. self.make_internal_node(heap_index)
        };
        // self.make_internal_node(heap_index)
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
        result_day_index
        // min(result_day_index, self.number_of_days - 1)
    }

    fn right_index(&self, heap_index: HeapIndex) -> usize {
        let mut result_heap_index = heap_index;
        for _ in 0.. (self.total_depth() - heap_index.depth()) {
            result_heap_index = result_heap_index.right()
        }
        let day_index = self.heap_index_to_day_index(result_heap_index).unwrap();
        day_index
        // min(day_index, self.number_of_days - 1) 
    }

    fn get(&self, index: HeapIndex) -> Option<&Node> {
        self.data.get(index.0 - 1).and_then(|node| node.as_ref())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    fn parent(self) -> Self {
        HeapIndex(self.0 / 2)
    }

    fn depth(self) -> usize {
        self.0.ilog2() as usize
    }
}

impl std::ops::Index<HeapIndex> for DaysHeap {
    type Output = Node;

    fn index(&self, index: HeapIndex) -> &Node {
        self.data[index.0 - 1].as_ref().unwrap()
    }
}

impl std::ops::IndexMut<HeapIndex> for DaysHeap {
    fn index_mut(&mut self, index: HeapIndex) -> &mut Node {
        self.data[index.0 - 1].as_mut().unwrap()
    }
}

// struct SegTree {
//     data: Vec<Node>,
// }

// impl SegTree {
//     fn new() -> SegTree {
//         todo!()
//     }
//     fn pull(&mut self, node_idx: usize) {
//         let left = self.left_index(node_idx);
//         let right = self.right_index(node_idx);
//         self.data[node_idx].max_val = max(self.data[left].max_val, self.data[right].max_val);
//         self.data[node_idx].min_val = max(self.data[left].min_val, self.data[right].min_val);
//         self.data[node_idx].max_index = if self.data[left].max_val >= self.data[right].max_val {
//             self.data[left].max_index
//         } else {
//             self.data[right].max_index
//         };
//         self.data[node_idx].min_index = if self.data[left].min_val >= self.data[right].min_val {
//             self.data[left].min_index
//         } else {
//             self.data[right].min_index
//         };
//     }
//     fn give(&mut self, node_idx: usize) {
//         self.data[node_idx].negated ^= true;
//         (self.data[node_idx].max_val, self.data[node_idx].min_val) = (self.data[node_idx].min_val, self.data[node_idx].max_val);
//         (self.data[node_idx].max_index, self.data[node_idx].min_index) = (self.data[node_idx].min_index, self.data[node_idx].max_index);
//     }
//     fn push(&mut self, node_idx: usize) {
//         if self.data[node_idx].negated {
//             self.give(self.left_index(self.data[node_idx].left));
//             self.give(self.right_index(self.data[node_idx].right));
//             self.data[node_idx].negated = false;
//         }
//     }
//     fn modify(&mut self, node_idx: usize, a: usize, b: usize) {
//         if a <= self.data[node_idx].left && self.data[node_idx].right <= b {
//             self.give(node_idx);
//         } else {
//             self.push(node_idx);
//             if a < self.data[node_idx].mid {
//                 self.modify(self.left_index(node_idx), a, b);
//             }
//             if self.data[node_idx].mid < b {
//                 self.modify(self.right_index(node_idx), a, b);
//             }
//             self.pull(node_idx);
//         }
//     }
//     fn left_index(&self, node_idx: usize) -> usize {
//         node_idx*2 + 1
//     }
//     fn right_index(&self, node_idx: usize) -> usize {
//         node_idx*2 + 2
//     }
// }
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

// c++: *Seg
// rust: &'static mut Seg

// (assuming you never deallocate Seg)

// dyn = (pointer_to_object, pointer_to_vtable);

// struct SomeTree { opaque }

// impl SomeTree {
//     void visit(Visitor visitor) {

//     }
//     fn visit(&self, visitor: &dyn Visitor) {
//         ...
//     }
// }

// interface Visitor { same thing as in rust }
// trait Visit {
//     fn visit_expression(&self, e: Expr) {

//     }

//     fn visit_token(&self, e: Expr) {

//     }
// }

// int main() {}
//     some_tree.visit(interface Visitor {
//         void visit_expression(Expr e) {
//             // Do things
//         }
//     })
// }

// struct MyVisitor;

// impl Visitor for MyVisitor {
//     fn visit_expression(&self, e: Expr)
// }

// fn main() {
//     some_tree.visit(&MyVisitor);
// }