// LeetCode 74: Search a 2D Matrix [Medium]
// https://leetcode.com/problems/search-a-2d-matrix/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 8.900us
//   AI solution avg: 8.817us
//   Ratio: ~tied
//
// Each row is sorted left-to-right. First integer of each row is greater
// than the last integer of the previous row. Search for target.
//
// Example 1: matrix=[[1,3,5,7],[10,11,16,20],[23,30,34,60]], target=3 -> true
// Example 2: same matrix, target=13 -> false
//
// Constraints: m,n in [1,100], -10^4 <= matrix[i][j], target <= 10^4
//
// Run:  cargo run --example lc_0074_search_2d_matrix
// Test: cargo test --example lc_0074_search_2d_matrix
// Bench: cargo test --example lc_0074_search_2d_matrix bench -- --nocapture

/// My solution — two binary searches (row then column).
///
/// First binary search the rightmost column to find the correct row,
/// then binary search within that row.
///
/// Time: O(log m + log n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Search for target in a sorted 2D matrix.
        ///
        /// # Arguments
        /// * `matrix` - sorted 2D matrix
        /// * `target` - value to find
        ///
        /// # Returns
        /// true if target exists in matrix
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            let (m, n) = (matrix.len(), matrix[0].len());
            // Binary search for the row
            let (mut lo, mut hi) = (0usize, m);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if matrix[mid][n - 1] < target {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            if lo >= m {
                return false;
            }
            // Binary search within the row
            matrix[lo].binary_search(&target).is_ok()
        }
    }
}

/// AI solution — single binary search treating matrix as flat array.
///
/// Treat the m*n matrix as a single sorted array of length m*n.
/// Map flat index to (row, col) via divmod.
///
/// Time: O(log(m*n)) = O(log m + log n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Search for target in a sorted 2D matrix.
        ///
        /// # Arguments
        /// * `matrix` - sorted 2D matrix
        /// * `target` - value to find
        ///
        /// # Returns
        /// true if target exists in matrix
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            let (m, n) = (matrix.len(), matrix[0].len());
            let (mut lo, mut hi) = (0usize, m * n);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                let val = matrix[mid / n][mid % n];
                if val == target {
                    return true;
                } else if val < target {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            false
        }
    }
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    for target in [3, 13] {
        println!(
            "target={target} -> my:{}, ai:{}",
            my_solution::Solution::search_matrix(matrix.clone(), target),
            ai_solution::Solution::search_matrix(matrix.clone(), target),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_search_matrix {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_found() {
                    let m = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
                    assert!(<$solution>::search_matrix(m, 3));
                }

                #[test]
                fn test_not_found() {
                    let m = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
                    assert!(!<$solution>::search_matrix(m, 13));
                }

                #[test]
                fn test_single_found() {
                    assert!(<$solution>::search_matrix(vec![vec![1]], 1));
                }

                #[test]
                fn test_single_not_found() {
                    assert!(!<$solution>::search_matrix(vec![vec![1]], 2));
                }

                #[test]
                fn test_first_element() {
                    let m = vec![vec![1, 3], vec![5, 7]];
                    assert!(<$solution>::search_matrix(m, 1));
                }

                #[test]
                fn test_last_element() {
                    let m = vec![vec![1, 3], vec![5, 7]];
                    assert!(<$solution>::search_matrix(m, 7));
                }
            }
        };
    }

    test_search_matrix!(my, crate::my_solution::Solution);
    test_search_matrix!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let matrix: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..100).map(|c| r * 100 + c).collect())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::search_matrix(
                std::hint::black_box(matrix.clone()),
                std::hint::black_box(5050),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::search_matrix(
                std::hint::black_box(matrix.clone()),
                std::hint::black_box(5050),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
