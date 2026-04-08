// LeetCode 133: Clone Graph [Medium]
// https://leetcode.com/problems/clone-graph/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.585us
//   AI solution avg: 0.452us
//   Ratio: AI 1.29x faster
//
// Given a reference to a node in a connected undirected graph, return a deep
// copy (clone) of the graph. Each node has a val and a list of neighbors.
//
// Constraints: 0 <= number of nodes <= 100, 1 <= Node.val <= 100
//   node values are unique, no self-loops or repeated edges, graph is connected
//
// Run:  cargo run --example lc_0133_clone_graph
// Test: cargo test --example lc_0133_clone_graph
// Bench: cargo test --example lc_0133_clone_graph bench -- --nocapture

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    pub fn new(val: i32) -> Self {
        GraphNode {
            val,
            neighbors: Vec::new(),
        }
    }
}

type GNode = Rc<RefCell<GraphNode>>;

/// Build a graph from an adjacency list (1-indexed).
fn build_graph(adj: &[Vec<usize>]) -> Option<GNode> {
    if adj.is_empty() {
        return None;
    }
    let nodes: Vec<GNode> = (0..adj.len())
        .map(|i| Rc::new(RefCell::new(GraphNode::new(i as i32 + 1))))
        .collect();
    for (i, neighbors) in adj.iter().enumerate() {
        let mut node = nodes[i].borrow_mut();
        for &n in neighbors {
            // adj uses 1-indexed neighbor values
            node.neighbors.push(Rc::clone(&nodes[n - 1]));
        }
    }
    Some(Rc::clone(&nodes[0]))
}

/// Collect adjacency list from a graph node.
fn to_adj_list(root: &Option<GNode>) -> Vec<Vec<i32>> {
    let Some(root) = root else { return vec![] };
    let mut visited: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut stack = vec![Rc::clone(root)];
    while let Some(node) = stack.pop() {
        let node_ref = node.borrow();
        if visited.contains_key(&node_ref.val) {
            continue;
        }
        let nbrs: Vec<i32> = node_ref.neighbors.iter().map(|n| n.borrow().val).collect();
        visited.insert(node_ref.val, nbrs);
        for n in &node_ref.neighbors {
            if !visited.contains_key(&n.borrow().val) {
                stack.push(Rc::clone(n));
            }
        }
    }
    let mut result: Vec<(i32, Vec<i32>)> = visited.into_iter().collect();
    result.sort_by_key(|(v, _)| *v);
    result.into_iter().map(|(_, ns)| ns).collect()
}

/// My solution — recursive DFS with visited map.
///
/// Time: O(V + E).
/// Space: O(V) for the visited map.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Deep clone the graph.
        pub fn clone_graph(node: Option<GNode>) -> Option<GNode> {
            let mut visited: HashMap<i32, GNode> = HashMap::new();
            Self::dfs(&node, &mut visited)
        }

        fn dfs(node: &Option<GNode>, visited: &mut HashMap<i32, GNode>) -> Option<GNode> {
            let node = node.as_ref()?;
            let node_ref = node.borrow();
            if let Some(cloned) = visited.get(&node_ref.val) {
                return Some(Rc::clone(cloned));
            }
            let clone = Rc::new(RefCell::new(GraphNode::new(node_ref.val)));
            visited.insert(node_ref.val, Rc::clone(&clone));
            for nbr in &node_ref.neighbors {
                if let Some(c) = Self::dfs(&Some(Rc::clone(nbr)), visited) {
                    clone.borrow_mut().neighbors.push(c);
                }
            }
            Some(clone)
        }
    }
}

/// AI solution — iterative BFS with visited map.
///
/// Time: O(V + E).
/// Space: O(V).
mod ai_solution {
    use super::*;
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Deep clone the graph using BFS.
        pub fn clone_graph(node: Option<GNode>) -> Option<GNode> {
            let node = node?;
            let mut visited: HashMap<i32, GNode> = HashMap::new();
            let root_clone = Rc::new(RefCell::new(GraphNode::new(node.borrow().val)));
            visited.insert(node.borrow().val, Rc::clone(&root_clone));

            let mut queue = VecDeque::new();
            queue.push_back(Rc::clone(&node));

            while let Some(cur) = queue.pop_front() {
                let cur_ref = cur.borrow();
                let cur_clone = Rc::clone(visited.get(&cur_ref.val).expect("visited"));
                for nbr in &cur_ref.neighbors {
                    let nbr_val = nbr.borrow().val;
                    let nbr_clone = visited
                        .entry(nbr_val)
                        .or_insert_with(|| {
                            let new = Rc::new(RefCell::new(GraphNode::new(nbr_val)));
                            queue.push_back(Rc::clone(nbr));
                            new
                        })
                        .clone();
                    cur_clone.borrow_mut().neighbors.push(nbr_clone);
                }
            }
            Some(root_clone)
        }
    }
}

fn main() {
    let graph = build_graph(&[vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
    let my = my_solution::Solution::clone_graph(graph.clone());
    let ai = ai_solution::Solution::clone_graph(graph);
    println!("cloned: my={:?}", to_adj_list(&my));
    println!("cloned: ai={:?}", to_adj_list(&ai));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_clone_graph {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let adj = vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]];
                    let graph = build_graph(&adj);
                    let cloned = <$solution>::clone_graph(graph.clone());
                    assert_eq!(to_adj_list(&cloned), to_adj_list(&graph));
                    // Ensure it's a deep copy (different Rc pointers)
                    if let (Some(a), Some(b)) = (&graph, &cloned) {
                        assert!(!Rc::ptr_eq(a, b));
                    }
                }

                #[test]
                fn test_single_node() {
                    let graph = build_graph(&[vec![]]);
                    let cloned = <$solution>::clone_graph(graph.clone());
                    assert_eq!(to_adj_list(&cloned), to_adj_list(&graph));
                }

                #[test]
                fn test_empty() {
                    let cloned = <$solution>::clone_graph(None);
                    assert!(cloned.is_none());
                }

                #[test]
                fn test_two_nodes() {
                    let graph = build_graph(&[vec![2], vec![1]]);
                    let cloned = <$solution>::clone_graph(graph.clone());
                    assert_eq!(to_adj_list(&cloned), to_adj_list(&graph));
                }
            }
        };
    }

    test_clone_graph!(my, crate::my_solution::Solution);
    test_clone_graph!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let adj = vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]];
        let graph = build_graph(&adj);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::clone_graph(
                std::hint::black_box(graph.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::clone_graph(
                std::hint::black_box(graph.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
