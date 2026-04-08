// LeetCode 427: Construct Quad Tree [Medium]
// https://leetcode.com/problems/construct-quad-tree/
//
// Given an n x n matrix grid of 0s and 1s, construct a Quad-Tree.
// Each node has: val (bool), is_leaf (bool), and four children
// (top_left, top_right, bottom_left, bottom_right).
// If all values in a sub-grid are the same, create a leaf node.
// Otherwise, split into four quadrants and recurse.
//
// Examples:
//   grid = [[0,1],[1,0]] -> [[0,1],[1,0],[1,1],[1,1],[1,0]]
//   grid = [[1,1,1,1,0,0,0,0],...] -> see LeetCode for full output
//
// Constraints:
//   n == grid.length == grid[i].length
//   n == 2^x where 0 <= x <= 6
//
// Run:  cargo run --example lc_0427_construct_quad_tree
// Test: cargo test --example lc_0427_construct_quad_tree
// Bench: cargo test --example lc_0427_construct_quad_tree bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 430ns
//   AI solution: avg 633ns
//   Ratio: 0.68x (prefix sum overhead)

/// Quad tree node definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuadNode {
    pub val: bool,
    pub is_leaf: bool,
    pub top_left: Option<Box<QuadNode>>,
    pub top_right: Option<Box<QuadNode>>,
    pub bottom_left: Option<Box<QuadNode>>,
    pub bottom_right: Option<Box<QuadNode>>,
}

impl QuadNode {
    pub fn new_leaf(val: bool) -> Self {
        QuadNode {
            val,
            is_leaf: true,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

/// Serialize a QuadTree to level-order [is_leaf, val] pairs for testing.
fn serialize(root: &Option<Box<QuadNode>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    if let Some(node) = root {
        queue.push_back(node.as_ref());
    }
    while let Some(node) = queue.pop_front() {
        result.push(vec![
            if node.is_leaf { 1 } else { 0 },
            if node.val { 1 } else { 0 },
        ]);
        if !node.is_leaf {
            if let Some(ref tl) = node.top_left {
                queue.push_back(tl);
            }
            if let Some(ref tr) = node.top_right {
                queue.push_back(tr);
            }
            if let Some(ref bl) = node.bottom_left {
                queue.push_back(bl);
            }
            if let Some(ref br) = node.bottom_right {
                queue.push_back(br);
            }
        }
    }
    result
}

/// My solution — DFS with subgrid scanning (translated from doocs Java).
///
/// For each subgrid, check if all values are the same. If so, create a
/// leaf node. Otherwise, split into four quadrants and recurse.
///
/// Time:  O(n^2 * log n)
/// Space: O(log n) — recursion depth
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Construct a QuadTree from a grid of 0s and 1s.
        ///
        /// # Arguments
        /// * `grid` - n x n grid where n is a power of 2
        ///
        /// # Returns
        /// Root of the constructed QuadTree.
        pub fn construct(grid: Vec<Vec<i32>>) -> Option<Box<QuadNode>> {
            let n = grid.len();
            Self::dfs(0, 0, n - 1, n - 1, &grid)
        }

        fn dfs(
            r1: usize,
            c1: usize,
            r2: usize,
            c2: usize,
            grid: &[Vec<i32>],
        ) -> Option<Box<QuadNode>> {
            let mut zero = false;
            let mut one = false;
            // Scan the entire subgrid for uniformity
            for row in grid.iter().take(r2 + 1).skip(r1) {
                for &val in row.iter().take(c2 + 1).skip(c1) {
                    if val == 0 {
                        zero = true;
                    } else {
                        one = true;
                    }
                }
            }
            let is_leaf = !(zero && one); // uniform if not both present
            let val = one; // val = true if any 1 exists (for leaf: the value)
            if is_leaf {
                return Some(Box::new(QuadNode::new_leaf(val)));
            }
            let mid_r = (r1 + r2) / 2;
            let mid_c = (c1 + c2) / 2;
            Some(Box::new(QuadNode {
                val,
                is_leaf: false,
                top_left: Self::dfs(r1, c1, mid_r, mid_c, grid),
                top_right: Self::dfs(r1, mid_c + 1, mid_r, c2, grid),
                bottom_left: Self::dfs(mid_r + 1, c1, r2, mid_c, grid),
                bottom_right: Self::dfs(mid_r + 1, mid_c + 1, r2, c2, grid),
            }))
        }
    }
}

/// AI solution — DFS with prefix sum optimization.
///
/// Precompute a 2D prefix sum to check subgrid uniformity in O(1)
/// instead of scanning every cell.
///
/// Time:  O(n^2) — prefix sum build, then O(1) per uniformity check
/// Space: O(n^2) — prefix sum array
mod ai_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Construct a QuadTree using prefix sums for fast uniformity checks.
        pub fn construct(grid: Vec<Vec<i32>>) -> Option<Box<QuadNode>> {
            let n = grid.len();
            // Build 2D prefix sum (1-indexed)
            let mut prefix = vec![vec![0i32; n + 1]; n + 1];
            for i in 0..n {
                for j in 0..n {
                    prefix[i + 1][j + 1] =
                        grid[i][j] + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
                }
            }
            Self::dfs(0, 0, n, &prefix)
        }

        /// Sum of subgrid [r1..r1+size, c1..c1+size] using prefix sums.
        fn region_sum(r: usize, c: usize, size: usize, prefix: &[Vec<i32>]) -> i32 {
            let (r2, c2) = (r + size, c + size);
            prefix[r2][c2] - prefix[r][c2] - prefix[r2][c] + prefix[r][c]
        }

        fn dfs(r: usize, c: usize, size: usize, prefix: &[Vec<i32>]) -> Option<Box<QuadNode>> {
            let total = Self::region_sum(r, c, size, prefix);
            let area = (size * size) as i32;
            // All zeros or all ones -> leaf
            if total == 0 || total == area {
                return Some(Box::new(QuadNode::new_leaf(total > 0)));
            }
            let half = size / 2;
            Some(Box::new(QuadNode {
                val: true,
                is_leaf: false,
                top_left: Self::dfs(r, c, half, prefix),
                top_right: Self::dfs(r, c + half, half, prefix),
                bottom_left: Self::dfs(r + half, c, half, prefix),
                bottom_right: Self::dfs(r + half, c + half, half, prefix),
            }))
        }
    }
}

fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    let my_tree = my_solution::Solution::construct(grid.clone());
    let ai_tree = ai_solution::Solution::construct(grid);
    println!("my:  {:?}", serialize(&my_tree));
    println!("ai:  {:?}", serialize(&ai_tree));
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_2x2() {
            let grid = vec![vec![0, 1], vec![1, 0]];
            let tree = Solution::construct(grid);
            let ser = serialize(&tree);
            assert_eq!(
                ser,
                vec![vec![0, 1], vec![1, 0], vec![1, 1], vec![1, 1], vec![1, 0]]
            );
        }

        #[test]
        fn test_uniform_all_ones() {
            let grid = vec![vec![1, 1], vec![1, 1]];
            let tree = Solution::construct(grid);
            let ser = serialize(&tree);
            // Single leaf node with val=1
            assert_eq!(ser, vec![vec![1, 1]]);
        }

        #[test]
        fn test_single_cell() {
            let grid = vec![vec![0]];
            let tree = Solution::construct(grid);
            let ser = serialize(&tree);
            assert_eq!(ser, vec![vec![1, 0]]);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<Vec<i32>>) -> Option<Box<QuadNode>>,
{
    let iterations = 500_000;
    let grid = vec![vec![0, 1], vec![1, 0]];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(grid.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::construct);
    bench_solution("AI solution", ai_solution::Solution::construct);
}
