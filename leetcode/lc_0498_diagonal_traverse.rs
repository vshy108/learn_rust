// LeetCode 498: Diagonal Traverse [Medium]
// https://leetcode.com/problems/diagonal-traverse/
//
// Given an m x n matrix, return all elements in diagonal order.
// The traversal alternates direction: up-right, then down-left, etc.
//
// Examples:
//   [[1,2,3],[4,5,6],[7,8,9]] -> [1,2,4,7,5,3,6,8,9]
//   [[1,2],[3,4]]             -> [1,2,3,4]
//
// Constraints:
//   m == mat.len(), n == mat[i].len()
//   1 <= m, n <= 10^4
//   1 <= m * n <= 10^4
//   -10^5 <= mat[i][j] <= 10^5
//
// Run:  cargo run --example lc_0498_diagonal_traverse
// Test: cargo test --example lc_0498_diagonal_traverse
// Bench: cargo test --example lc_0498_diagonal_traverse bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 231ns
//   AI solution: avg 232ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — simulate diagonal walk.
///
/// Time:  O(m * n)
/// Space: O(1) — output excluded

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
            let (m, n) = (mat.len(), mat[0].len());
            let mut result = Vec::with_capacity(m * n);
            for d in 0..(m + n - 1) {
                if d % 2 == 0 {
                    let r = d.min(m - 1);
                    let c = d - r;
                    let (mut r, mut c) = (r as i32, c as i32);
                    while r >= 0 && (c as usize) < n {
                        result.push(mat[r as usize][c as usize]);
                        r -= 1;
                        c += 1;
                    }
                } else {
                    let c = d.min(n - 1);
                    let r = d - c;
                    let (mut r, mut c) = (r as i32, c as i32);
                    while (r as usize) < m && c >= 0 {
                        result.push(mat[r as usize][c as usize]);
                        r += 1;
                        c -= 1;
                    }
                }
            }
            result
        }
    }
}

/// AI solution module — for each diagonal d (0..m+n-1), collect elements.
/// Even diagonals go up-right; odd diagonals go down-left.
///
/// Time:  O(m * n)
/// Space: O(1) extra
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
            let m = mat.len();
            let n = mat[0].len();
            let mut result = Vec::with_capacity(m * n);

            for d in 0..(m + n - 1) {
                if d % 2 == 0 {
                    // Up-right: row decreases, col increases
                    let r = d.min(m - 1);
                    let c = d - r;
                    let mut r = r as i32;
                    let mut c = c as i32;
                    while r >= 0 && (c as usize) < n {
                        result.push(mat[r as usize][c as usize]);
                        r -= 1;
                        c += 1;
                    }
                } else {
                    // Down-left: row increases, col decreases
                    let c = d.min(n - 1);
                    let r = d - c;
                    let mut r = r as i32;
                    let mut c = c as i32;
                    while (r as usize) < m && c >= 0 {
                        result.push(mat[r as usize][c as usize]);
                        r += 1;
                        c -= 1;
                    }
                }
            }

            result
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_3x3() {
            let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            assert_eq!(
                Solution::find_diagonal_order(mat),
                vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
            );
        }
        #[test]
        fn test_2x2() {
            let mat = vec![vec![1, 2], vec![3, 4]];
            assert_eq!(Solution::find_diagonal_order(mat), vec![1, 2, 3, 4]);
        }
        #[test]
        fn test_1x1() {
            assert_eq!(Solution::find_diagonal_order(vec![vec![1]]), vec![1]);
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
    F: Fn(Vec<Vec<i32>>) -> Vec<i32>,
{
    let iterations = 500_000;
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(mat.clone())));
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
    bench_solution("My solution", my_solution::Solution::find_diagonal_order);
    bench_solution("AI solution", ai_solution::Solution::find_diagonal_order);
}

fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", ai_solution::Solution::find_diagonal_order(mat));
}
