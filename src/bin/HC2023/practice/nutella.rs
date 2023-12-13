use std::collections::{VecDeque, HashSet};
use std::io::stdin;
use std::{error::Error, result::Result as StdResult};

use petgraph::algo::is_bipartite_undirected;

type Result<T> = StdResult<T, Box<dyn Error>>;

fn parse_pair(lines: &mut impl Iterator<Item=String>) -> [usize; 2] {
    lines.next().ok_or("nm")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[derive(Debug, Default)]
struct Node {
    i: usize, // For debugging
    special: bool,
    visited: bool,
    color: bool,
}

#[derive(Debug, Clone)]
struct MergedNode {
    containing: Vec<usize>,
    special: bool,
}

fn main() -> Result<()> {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let tests: u64 = lines.next().ok_or("t")?.parse()?;

    for t in 0..tests {
        let mut graph = petgraph::Graph::new_undirected();
        let [n, m] = parse_pair(&mut lines);
        let nodes: Vec<_> = (0.. n).map(|i| graph.add_node(Node{ i: i + 1, .. Default::default() })).collect();

        for _edge in 0.. m {
            let [a, b] = parse_pair(&mut lines);

            graph.add_edge(nodes[a - 1], nodes[b - 1], ());
        }

        let q = lines.next().ok_or("q")?.parse()?;
        let queries: Vec<_> = (0.. q).map(|_| parse_pair(&mut lines)).collect();

        if is_bipartite_undirected(&graph, nodes[0]) {
            println!("Case #{}: {}", t + 1, -q);
            continue;
        }

        // Pre-process graph

        // BFS: Label at least one node in each odd cycle as special
        let mut to_visit = VecDeque::new();
        graph[nodes[0]].visited = true;
        to_visit.push_back(nodes[0]);
        while let Some(node) = to_visit.pop_front() {
            let mut nbhd = graph.neighbors(node).detach();
            while let Some((_, nbh)) = nbhd.next(&graph) {
                if graph[nbh].visited {
                    if graph[nbh].color == graph[node].color {
                        graph[nbh].special = true;
                    }
                } else {
                    graph[nbh].visited = true;
                    graph[nbh].color = !graph[node].color;
                    to_visit.push_back(nbh);
                }
            }
        }
        eprintln!("Graph post coloring: {graph:#?}");

        // Collapse graph nodes
        let mut bridges: Vec<_> = bridges(&graph).map(|edge_ref| (edge_ref.source(), edge_ref.target(), edge_ref.id())).collect();
        bridges.sort_by_key(|e| usize::MAX - e.2.index()); // TODO: This is terrible.
        eprintln!("Bridges: {:?}", bridges);
        for &(_, _, edge) in &bridges {
            graph.remove_edge(edge);
        }

        let mut union_find = UnionFind::new(n);
        for edge in graph.raw_edges() {
            union_find.union(edge.source().index(), edge.target().index());
            // println!("Union find post union {} {}: {union_find:#?}", edge.source().index(), edge.target().index());
        }

        let mut merged_graph = petgraph::Graph::new_undirected();
        let mut merged_nodes = vec![petgraph::graph::NodeIndex::new(!0); n];
        for i in 0.. n {
            if union_find.find(i) == i {
                merged_nodes[i] = merged_graph.add_node(MergedNode {
                    special: false,
                    containing: vec![],
                });
            }
        }

        // Turn new_nodes into a complete map of original node index to merged node index
        // and mark merged nodes as special if any of the original nodes are special.
        for i in 0.. n {
            merged_nodes[i] = merged_nodes[union_find.find(i)];
            merged_graph[merged_nodes[i]].containing.push(i + 1);

            if graph[NodeIndex::new(i)].special {
                merged_graph[merged_nodes[i]].special = true;
            }
        }

        eprintln!("merged nodes: {merged_nodes:#?}");

        for (a, b, _) in bridges {
            merged_graph.update_edge(
                merged_nodes[a.index()],
                merged_nodes[b.index()],
                ()
            );
        }

        eprintln!("Merged graph: {merged_graph:#?}");
        drop(graph);
        drop(nodes);

        let mut tally = 0;
        for [start, end] in queries {
            eprintln!("{:?} Query {} {}", std::time::Instant::now(), start, end);
            let start = start - 1;
            let end = end - 1;
            // DFS - Find path from start to end

            let mut path = vec![(merged_nodes[start], 0)];
            let mut visited = vec![false; merged_graph.node_count()];
            visited[merged_nodes[start].index()] = true;
            while path.last().unwrap().0 != merged_nodes[end] {
                // println!("Path: {path:?}");
                let &(node, child_idx) = path.last().unwrap();
                let nbhd = merged_graph.neighbors(node).collect::<Vec<_>>();

                if nbhd.len() == child_idx {
                    path.pop();
                    continue;
                }

                let child = nbhd[child_idx];
                path.last_mut().unwrap().1 += 1;
                if !visited[child.index()] {
                    visited[child.index()] = true;
                    path.push((nbhd[child_idx], 0));
                }
            }

            // BFS from path towards other nodes, looking for a special node
            // This is too slow. You need to do some preprocessing
            let mut bfs_queue: VecDeque<_> = path.into_iter().map(|(node, _)| (node, 0)).collect();
            let mut visited = vec![false; merged_graph.node_count()];
            for node in &bfs_queue {
                visited[node.0.index()] = true;
            }

            let mut found_path = false;
            while let Some((next, depth)) = bfs_queue.pop_front() {
                if merged_graph[next].special {
                    eprintln!("Cost for query {depth}");
                    tally += depth;
                    found_path = true;
                    break
                } else {
                    for nbh in merged_graph.neighbors(next) {
                        if !visited[nbh.index()] {
                            visited[nbh.index()] = true;
                            bfs_queue.push_back((nbh, depth + 1));
                        }
                    }
                }
            }
            assert!(found_path);
        }

        println!("Case #{}: {}", t + 1, tally);
    }

    Ok(())
}


use petgraph::graph::NodeIndex;
use petgraph::unionfind::UnionFind;
// Credit: https://github.com/petgraph/petgraph/pull/473/files
use petgraph::visit::{IntoEdgeReferences, IntoNeighbors, IntoNodeIdentifiers, NodeIndexable, EdgeRef};

/// Find all [bridges](https://en.wikipedia.org/wiki/Bridge_(graph_theory)) in a simple undirected graph.
///
/// Returns the vector of pairs `(G::NodeID, G:: NodeID)`,
/// representing the edges of the input graph that are bridges.
/// The order of the vertices in the pair and the order of the edges themselves are arbitrary.
///
/// # Examples
///
/// ```
/// use petgraph::algo::bridges;
/// use petgraph::graph::UnGraph;
///
/// // Create the following graph:
/// // 0----1    4
/// //      | __/|
/// // 5----2/---3
///
/// let mut g = UnGraph::new_undirected();
/// let n0 = g.add_node(());
/// let n1 = g.add_node(());
/// let n2 = g.add_node(());
/// let n3 = g.add_node(());
/// let n4 = g.add_node(());
/// let n5 = g.add_node(());
/// g.add_edge(n0, n1, ());
/// g.add_edge(n1, n2, ());
/// g.add_edge(n2, n3, ());
/// g.add_edge(n3, n4, ());
/// g.add_edge(n2, n4, ());
/// g.add_edge(n5, n2, ());
///
/// // The bridges in this graph are the undirected edges {2, 5}, {1, 2}, {0, 1}.
/// assert_eq!(bridges(&g), vec![(n2, n5), (n1, n2), (n0, n1)]);
/// ```
pub fn bridges<G>(graph: G) -> impl Iterator<Item=G::EdgeRef>
where
    G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable + IntoEdgeReferences,
{
    let mut clock: usize = 0usize;
    // If and when a node was visited by the dfs
    let mut visit_time = vec![None; graph.node_bound()];
    // Lowest time on a node that is the target of a back-edge from the subtree rooted
    // at the indexed node.
    let mut earliest_backedge = vec![usize::MAX; graph.node_bound()];

    for start in 0..graph.node_bound() {
        // If node hasn't been visited yet, make it the root of a new dfs-tree in the forest.
        if visit_time[start].is_none() {
            visit_time[start] = Some(clock);
            clock += 1;

            // Perform a DFS starting at start
            let start = graph.from_index(start);
            let mut stack: Vec<(G::NodeId, G::Neighbors)> = vec![(start, graph.neighbors(start))];
            
            while let Some((stack_frame, rest_of_stack)) = stack.split_last_mut() {
                let &mut (node, ref mut neighbors) = stack_frame; 
                let parent = rest_of_stack.last().map(|&(n, _)| n);
        
                let node_index = graph.to_index(node);
        
                if let Some(child) = neighbors.next() {
                    // Pre-order DFS
                    if parent != Some(child) {
                        let child_index = graph.to_index(child);
        
                        if let Some(time) = visit_time[child_index] {
                            earliest_backedge[node_index] = earliest_backedge[node_index].min(time);
                        } else {
                            visit_time[child_index] = Some(clock);
                            clock += 1;
                            stack.push((child, graph.neighbors(child)));
                        }
                    }
                }
                else {
                    // Post-order DFS
                    if let Some(parent) = parent {
                        let parent_index = graph.to_index(parent);
                        earliest_backedge[parent_index] = earliest_backedge[parent_index].min(earliest_backedge[node_index]);
                    }
                    stack.pop();
                }
            }
        }
    }

    graph.edge_references().filter(move |edge| {
        let source_index = graph.to_index(edge.source());
        let target_index = graph.to_index(edge.target());

        // All nodes have been visited by the time we return, so unwraps are safe.
        // The node with the lower visit time is the "parent" in the dfs-forest created above.
        let (parent, node) = if visit_time[source_index].unwrap() < visit_time[target_index].unwrap() {
            (source_index, target_index)
        } else {
            (target_index, source_index)
        };

        // If there's no back-edge to before parent, then this the only way from parent to here
        // is directly from parent, so it's a bridge edge.
        earliest_backedge[node] > visit_time[parent].unwrap()
    })
}

// fn bridges_dfs_helper<G>(
//     graph: G,
//     v: usize,
//     p: Option<usize>,
//     bridges: &mut Vec<(G::NodeId, G::NodeId)>,
//     visited: &mut Vec<bool>,
// ) where
//     G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable,
// {
//     // TODO: Rename tin/fup
//     let mut tin = vec![0usize; graph.node_bound()];
//     let mut fup = vec![0usize; graph.node_bound()];
//     let mut stack = vec![(v, p)];
//     let mut time = 0usize;

//     while let Some((node, parent)) = stack.pop() { // O(|E|) times
//         if !visited[node] {
//             tin[node] = time; // discovery time
//             fup[node] = time; // lowest time
//             visited[node] = true;
//             time += 1;
//         }

//         let mut backtrack = true;
//         for n in graph.neighbors(graph.from_index(node)) { // This runs 1.. |neigbours| times
//             let to = graph.to_index(n);
//             if Some(to) == parent { // And you do it on average |E|/2 times.
//                 continue;
//             }
//             if visited[to] {
//                 fup[node] = fup[node].min(tin[to]);
//             } else {
//                 stack.push((node, parent));
//                 stack.push((to, Some(node)));
//                 backtrack = false;
//                 break;
//             }
//         }

//         if backtrack {
//             if let Some(parent) = parent {
//                 fup[parent] = fup[parent].min(fup[node]);
//                 if fup[node] > tin[parent] {
//                     bridges.push((graph.from_index(node), graph.from_index(parent)));
//                 }
//             }
//         }
//     }
// }




// fn bridges_dfs_helper_2<G>(
//     graph: G,
//     v: G::NodeId,
//     p: Option<usize>,
//     bridges: &mut Vec<(G::NodeId, G::NodeId)>,
//     visited: &mut Vec<bool>,
// ) where
//     G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable,
// {
//     // TODO: Rename tin/fup
//     let mut tin = vec![0usize; graph.node_bound()];
//     let mut fup = vec![0usize; graph.node_bound()];
//     let mut stack = vec![Box::new(std::iter::once((v, None))) as Box<dyn Iterator<Item=(G::NodeId, Option<G::NodeId>)>>];
//     let mut time = 0usize;

//     while let Some(mut stack_frame) = stack.last_mut() {
//         if let Some((node, parent)) = stack_frame.next() {
//             if !visited[node] {
//                 tin[node] = time; // discovery time
//                 fup[node] = time; // lowest time
//                 visited[node] = true;
//                 time += 1;
//             }
    
//             let mut backtrack = true;
//             for n in graph.neighbors(graph.from_index(node)) { // This runs 1.. |neigbours| times
//                 let to = graph.to_index(n);
//                 if Some(to) == parent { // And you do it on average |E|/2 times.
//                     continue;
//                 }
//                 if visited[to] {
//                     fup[node] = fup[node].min(tin[to]);
//                 } else {
//                     stack.push((to, Some(node)));
//                     backtrack = false;
//                     break;
//                 }
//             }
    
//             if backtrack {
//                 if let Some(parent) = parent {
//                     fup[parent] = fup[parent].min(fup[node]);
//                     if fup[node] > tin[parent] {
//                         bridges.push((graph.from_index(node), graph.from_index(parent)));
//                     }
//                 }
//             }
//         }
//     }
// }

// fn bridges_dfs_helper<G>(
//     graph: G,
//     v: G::NodeId,
//     bridges: &mut Vec<(G::EdgeId)>,
//     visited: &mut Vec<bool>,
// ) where
//     G: IntoNodeIdentifiers + IntoEdgeReferences + NodeIndexable,
// {
//     let mut clock: usize = 1usize;
//     let mut time = vec![0usize; graph.node_bound()];
//     let mut earliest_backedge = vec![usize::MAX; graph.node_bound()];
//     let mut stack: Vec<(G::NodeId, Option<G::EdgeId>, G::Edges)> = vec![(v, graph.edges(v))];

//     visited[graph.to_index(v)] = true;

//     while let Some((stack_frame, rest_of_stack)) = stack.split_last_mut() {
//         let &mut (node, ref mut edges) = stack_frame; 
//         let parent = rest_of_stack.last().map(|&(n, _)| n);

//         let node_index = graph.to_index(node);

//         if let Some(edge) = edges.next() {
//             let child = edge.target();
//             if parent != Some(child) {
//                 let child_index = graph.to_index(child);

//                 if visited[child_index] {
//                     earliest_backedge[node_index] = earliest_backedge[node_index].min(time[child_index]);
//                 } else {
//                     time[child_index] = clock;
//                     visited[child_index] = true;
//                     clock += 1;
//                     stack.push((child, graph.edges(child)));
//                 }
//             }
//         }
//         else {
//             if let Some(parent) = parent {
//                 let parent_index = graph.to_index(parent);
//                 earliest_backedge[parent_index] = earliest_backedge[parent_index].min(earliest_backedge[node_index]);
//             }
//             stack.pop();
//         }
//     }
// }

// trait Visitor {
//     fn pre_order(node, parent, grandparent);
//     fn post_order(node, parent);
// }

// struct Dfs {
//     stack: []
//     visited: []
// }

// impl Dfs {
//     fn run_it(visitor: &Visitor) {
//         Does the structural to call pre_order and post_order
//     }
// }
