// LeetCode 261: Graph Valid Tree [Medium] (Premium)
// https://leetcode.com/problems/graph-valid-tree/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 5.467us
//   AI solution avg: 10.839us
//   Ratio: My 1.98x faster
//
// Given n nodes labeled 0..n-1 and a list of undirected edges, determine
// if these edges form a valid tree.
// A valid tree: connected, no cycles, exactly n-1 edges.
//
// Example 1: n=5, edges=[[0,1],[0,2],[0,3],[1,4]] -> true
// Example 2: n=5, edges=[[0,1],[1,2],[2,3],[1,3],[1,4]] -> false
//
// Constraints:
//   - 1 <= n <= 2000
//   - 0 <= edges.length <= 5000
//   - edges[i].length == 2, 0 <= a_i, b_i < n
//   - No self-loops or repeated edges
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0261.Graph%20Valid%20Tree/README_EN.md
//
// Run:  cargo run --example lc_0261_graph_valid_tree
// Test: cargo test --example lc_0261_graph_valid_tree
// Bench: cargo test --example lc_0261_graph_valid_tree bench -- --nocapture

/// My solution — Union-Find (from doocs Python).
///
/// Initialize each node as its own parent. For each edge, find roots
/// of both endpoints. If same root -> cycle -> not a tree. Otherwise
/// union them and decrement component count. At the end, exactly 1
/// component means it's a tree.
///
/// Time: O(n * alpha(n)) ~ O(n) with path compression
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if n nodes + edges form a valid tree.
        pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
            let n = n as usize;
            let mut parent: Vec<usize> = (0..n).collect();
            let mut components = n;

            // Find with path compression
            fn find(parent: &mut [usize], x: usize) -> usize {
                if parent[x] != x {
                    parent[x] = find(parent, parent[x]);
                }
                parent[x]
            }

            for e in &edges {
                let (a, b) = (e[0] as usize, e[1] as usize);
                let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
                if pa == pb {
                    // Cycle detected
                    return false;
                }
                parent[pa] = pb;
                components -= 1;
            }
            components == 1
        }
    }
}

/// AI solution — DFS connectivity check (from doocs Solution 2).
///
/// A graph is a tree iff it has exactly n-1 edges AND is connected.
/// Check edge count first (quick reject), then DFS from node 0 to
/// verify all nodes are reachable.
///
/// Time: O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if n nodes + edges form a valid tree.
        pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
            let n = n as usize;
            // A tree with n nodes has exactly n-1 edges
            if edges.len() != n - 1 {
                return false;
            }
            // Build adjacency list
            let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
            for e in &edges {
                let (a, b) = (e[0] as usize, e[1] as usize);
                g[a].push(b);
                g[b].push(a);
            }
            // DFS from 0, count reachable nodes
            let mut vis = vec![false; n];
            let mut stack = vec![0usize];
            vis[0] = true;
            let mut count = 0usize;
            while let Some(node) = stack.pop() {
                count += 1;
                for &neighbor in &g[node] {
                    if !vis[neighbor] {
                        vis[neighbor] = true;
                        stack.push(neighbor);
                    }
                }
            }
            count == n
        }
    }
}

fn main() {
    println!(
        "my: {}",
        my_solution::Solution::valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]])
    );
    println!(
        "ai: {}",
        ai_solution::Solution::valid_tree(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_graph_valid_tree {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1_tree() {
                    assert!(Sol::valid_tree(
                        5,
                        vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]
                    ));
                }

                #[test]
                fn test_example_2_cycle() {
                    assert!(!Sol::valid_tree(
                        5,
                        vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]
                    ));
                }

                #[test]
                fn test_single_node() {
                    assert!(Sol::valid_tree(1, vec![]));
                }

                #[test]
                fn test_two_nodes_connected() {
                    assert!(Sol::valid_tree(2, vec![vec![0, 1]]));
                }

                #[test]
                fn test_two_nodes_disconnected() {
                    assert!(!Sol::valid_tree(2, vec![]));
                }

                #[test]
                fn test_line() {
                    assert!(Sol::valid_tree(
                        4,
                        vec![vec![0, 1], vec![1, 2], vec![2, 3]]
                    ));
                }

                #[test]
                fn test_too_many_edges() {
                    // n=3 but 3 edges -> cycle
                    assert!(!Sol::valid_tree(
                        3,
                        vec![vec![0, 1], vec![1, 2], vec![0, 2]]
                    ));
                }

                #[test]
                fn test_disconnected_forest() {
                    // Two separate components, no cycle but not connected
                    assert!(!Sol::valid_tree(
                        4,
                        vec![vec![0, 1], vec![2, 3]]
                    ));
                }

                #[test]
                fn test_star() {
                    assert!(Sol::valid_tree(
                        5,
                        vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]]
                    ));
                }
            }
        };
    }

    test_graph_valid_tree!(my, crate::my_solution::Solution);
    test_graph_valid_tree!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a line graph of 100 nodes (valid tree)
        let edges: Vec<Vec<i32>> = (0..99).map(|i| vec![i, i + 1]).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::valid_tree(100, edges.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::valid_tree(100, edges.clone()));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
