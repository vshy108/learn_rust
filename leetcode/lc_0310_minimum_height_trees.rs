// LeetCode 310: Minimum Height Trees [Medium]
// https://leetcode.com/problems/minimum-height-trees/
// Given a tree of n nodes (0 to n-1) and edges, return all root labels that
// produce minimum height trees. At most 2 such roots exist (center of tree).
//
// Benchmark: 100_000 iterations
// My solution (leaf peeling / topo sort): avg 0.984us
// AI solution (two BFS to find center):   avg 1.272us
// Winner: My solution (1.29x faster)

// ------ My Solution: Topological Sort (leaf peeling) ------
// Time: O(n) — each node processed once
// Space: O(n) — adjacency list, degree array, queue
/// Repeatedly remove leaf nodes (degree 1) from the tree.
/// The last remaining 1-2 nodes are the roots of minimum height trees.
mod my_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
            let n = n as usize;
            if n == 1 {
                return vec![0];
            }
            let mut adj = vec![vec![]; n];
            let mut degree = vec![0u32; n];
            for e in &edges {
                let (a, b) = (e[0] as usize, e[1] as usize);
                adj[a].push(b);
                adj[b].push(a);
                degree[a] += 1;
                degree[b] += 1;
            }

            // Start with all leaves (degree 1)
            let mut queue: VecDeque<usize> = (0..n).filter(|&i| degree[i] == 1).collect();
            let mut ans = Vec::new();

            while !queue.is_empty() {
                ans.clear();
                // Process all leaves at current level
                for _ in 0..queue.len() {
                    let a = queue.pop_front().expect("non-empty queue");
                    ans.push(a as i32);
                    for &b in &adj[a] {
                        degree[b] -= 1;
                        if degree[b] == 1 {
                            queue.push_back(b);
                        }
                    }
                }
            }
            ans
        }
    }
}

// ------ AI Solution: Two BFS to find tree center ------
// Time: O(n) — three BFS passes
// Space: O(n) — adjacency list and distance arrays
/// Find the diameter endpoints via two BFS passes, then find the center
/// of the diameter path. The center node(s) are the MHT root(s).
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
            let n = n as usize;
            if n == 1 {
                return vec![0];
            }
            let mut adj = vec![vec![]; n];
            for e in &edges {
                let (a, b) = (e[0] as usize, e[1] as usize);
                adj[a].push(b);
                adj[b].push(a);
            }

            // BFS from `start`, returns (farthest node, parent array)
            let bfs = |start: usize| -> (usize, Vec<i32>) {
                let mut dist = vec![-1i32; n];
                let mut parent = vec![-1i32; n];
                let mut queue = VecDeque::new();
                dist[start] = 0;
                queue.push_back(start);
                let mut farthest = start;
                while let Some(u) = queue.pop_front() {
                    for &v in &adj[u] {
                        if dist[v] == -1 {
                            dist[v] = dist[u] + 1;
                            parent[v] = u as i32;
                            queue.push_back(v);
                            if dist[v] > dist[farthest] {
                                farthest = v;
                            }
                        }
                    }
                }
                (farthest, parent)
            };

            // Step 1: BFS from node 0 to find one endpoint of diameter
            let (far1, _) = bfs(0);
            // Step 2: BFS from far1 to find the other endpoint
            let (far2, parent) = bfs(far1);

            // Step 3: Trace back the diameter path
            let mut path = Vec::new();
            let mut cur = far2 as i32;
            while cur != -1 {
                path.push(cur);
                cur = parent[cur as usize];
            }

            // The center of the path is the MHT root(s)
            let len = path.len();
            if len % 2 == 1 {
                vec![path[len / 2]]
            } else {
                let mut ans = vec![path[len / 2 - 1], path[len / 2]];
                ans.sort();
                ans
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn test_example1() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(4, edges.clone())),
            vec![1]
        );
        assert_eq!(sorted(AiSolution::find_min_height_trees(4, edges)), vec![1]);
    }

    #[test]
    fn test_example2() {
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(6, edges.clone())),
            vec![3, 4]
        );
        assert_eq!(
            sorted(AiSolution::find_min_height_trees(6, edges)),
            vec![3, 4]
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(MySolution::find_min_height_trees(1, vec![]), vec![0]);
        assert_eq!(AiSolution::find_min_height_trees(1, vec![]), vec![0]);
    }

    #[test]
    fn test_two_nodes() {
        let edges = vec![vec![0, 1]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(2, edges.clone())),
            vec![0, 1]
        );
        assert_eq!(
            sorted(AiSolution::find_min_height_trees(2, edges)),
            vec![0, 1]
        );
    }

    #[test]
    fn test_line_odd() {
        // 0-1-2-3-4: center is 2
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(5, edges.clone())),
            vec![2]
        );
        assert_eq!(sorted(AiSolution::find_min_height_trees(5, edges)), vec![2]);
    }

    #[test]
    fn test_line_even() {
        // 0-1-2-3: center is 1, 2
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(4, edges.clone())),
            vec![1, 2]
        );
        assert_eq!(
            sorted(AiSolution::find_min_height_trees(4, edges)),
            vec![1, 2]
        );
    }

    #[test]
    fn test_star() {
        // 0 connected to 1,2,3,4,5: center is 0
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(6, edges.clone())),
            vec![0]
        );
        assert_eq!(sorted(AiSolution::find_min_height_trees(6, edges)), vec![0]);
    }

    #[test]
    fn test_three_nodes_line() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(
            sorted(MySolution::find_min_height_trees(3, edges.clone())),
            vec![1]
        );
        assert_eq!(sorted(AiSolution::find_min_height_trees(3, edges)), vec![1]);
    }

    #[test]
    fn test_balanced_tree() {
        // Binary tree:    0
        //                / \
        //               1   2
        //              / \
        //             3   4
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4]];
        // Height from 0=2, from 1=2, from 2=3, from 3=3, from 4=3 -> root 0 or 1
        assert_eq!(
            sorted(MySolution::find_min_height_trees(5, edges.clone())),
            vec![0, 1]
        );
        assert_eq!(
            sorted(AiSolution::find_min_height_trees(5, edges)),
            vec![0, 1]
        );
    }
}

fn main() {
    println!("LeetCode 310: Minimum Height Trees");

    let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
    println!(
        "n=6: My={:?}, AI={:?}",
        my_solution::Solution::find_min_height_trees(6, edges.clone()),
        ai_solution::Solution::find_min_height_trees(6, edges.clone())
    );

    // Benchmark
    let iterations = 100_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::find_min_height_trees(
            6,
            std::hint::black_box(edges.clone()),
        ));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::find_min_height_trees(
            6,
            std::hint::black_box(edges.clone()),
        ));
    }
    let ai_time = start.elapsed();

    let my_avg = my_time.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_time.as_nanos() as f64 / iterations as f64 / 1000.0;

    println!("\nBenchmark ({iterations} iterations):");
    println!("  My solution:  {my_avg:.3}us avg");
    println!("  AI solution:  {ai_avg:.3}us avg");
    if my_avg < ai_avg {
        println!("  Winner: My solution ({:.2}x faster)", ai_avg / my_avg);
    } else {
        println!("  Winner: AI solution ({:.2}x faster)", my_avg / ai_avg);
    }
}
