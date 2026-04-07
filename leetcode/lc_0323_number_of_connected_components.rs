// LeetCode 323. Number of Connected Components in an Undirected Graph (Premium)
// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.621us
//   ai_solution  avg: 0.318us
//   Ratio: ai_solution 1.95x faster

/// Given n nodes (0-indexed) and undirected edges, return the number of
/// connected components.
///
/// Example 1: n=5, edges=[[0,1],[1,2],[3,4]] -> 2
/// Example 2: n=5, edges=[[0,1],[1,2],[2,3],[3,4]] -> 1
///
/// Constraints: 1<=n<=2000, 1<=edges.length<=5000

/// # my_solution: DFS
/// Time: O(n + m)
/// Space: O(n + m) for adjacency list and visited
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count connected components using DFS.
        pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
            let n = n as usize;
            let mut g = vec![vec![]; n];
            for e in &edges {
                let (a, b) = (e[0] as usize, e[1] as usize);
                g[a].push(b);
                g[b].push(a);
            }

            let mut vis = vec![false; n];
            let mut count = 0;

            for i in 0..n {
                if !vis[i] {
                    count += 1;
                    dfs(i, &g, &mut vis);
                }
            }

            count
        }
    }

    fn dfs(node: usize, g: &[Vec<usize>], vis: &mut [bool]) {
        vis[node] = true;
        for &next in &g[node] {
            if !vis[next] {
                dfs(next, g, vis);
            }
        }
    }
}

/// # ai_solution: Union-Find with path compression and union by size
/// Time: O(n + m * alpha(n)) ≈ O(n + m)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count connected components using Union-Find.
        pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
            let mut n = n as usize;
            let mut parent: Vec<usize> = (0..n).collect();
            let mut size = vec![1usize; n];

            for e in &edges {
                let (a, b) = (e[0] as usize, e[1] as usize);
                let pa = find(&mut parent, a);
                let pb = find(&mut parent, b);
                if pa != pb {
                    // Union by size
                    if size[pa] >= size[pb] {
                        parent[pb] = pa;
                        size[pa] += size[pb];
                    } else {
                        parent[pa] = pb;
                        size[pb] += size[pa];
                    }
                    n -= 1;
                }
            }

            n as i32
        }
    }

    /// Find with path compression.
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(i32, Vec<Vec<i32>>) -> i32) {
        assert_eq!(
            solve(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]),
            2
        );
        assert_eq!(
            solve(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]),
            1
        );
        // All isolated
        assert_eq!(solve(3, vec![]), 3);
        // Single node
        assert_eq!(solve(1, vec![]), 1);
        // Two nodes connected
        assert_eq!(solve(2, vec![vec![0, 1]]), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::count_components);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::count_components);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
    println!("Input: n={n}, edges={edges:?}");
    println!(
        "Output (my): {}",
        my_solution::Solution::count_components(n, edges.clone())
    );
    println!(
        "Output (ai): {}",
        ai_solution::Solution::count_components(n, edges.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::count_components(
            black_box(n), black_box(edges.clone()),
        ));
    }
    let my_dur = start.elapsed();
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::count_components(
            black_box(n), black_box(edges.clone()),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
