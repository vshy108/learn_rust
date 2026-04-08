// @lc app=leetcode id=363 lang=rust
// #[Hard] - Max Sum of Rectangle No Larger Than K
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.227us
// ai_solution  | avg: 0.025us
// Performance: AI 9.00x faster

/// # 363. Max Sum of Rectangle No Larger Than K
///
/// Given an m x n matrix and integer k, return the max sum of a rectangle
/// such that its sum is no larger than k.
///
/// Constraints:
/// - 1 <= m, n <= 100
/// - -100 <= matrix[i][j] <= 100
/// - -10^5 <= k <= 10^5

/// ## My Solution — Enumerate row boundaries + sorted set via BTreeSet O(m^2 * n log n)
///
/// Fix top row i, expand bottom row j. Collapse columns into 1-D prefix sums.
/// Use a BTreeSet to find the minimum prefix sum >= (current_sum - k).
///
/// - Time: O(m^2 * n * log n)
/// - Space: O(n)
mod my_solution {
    use std::collections::BTreeSet;

    pub fn max_sum_submatrix(matrix: &[Vec<i32>], k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = i32::MIN;

        // Enumerate the top boundary
        for i in 0..m {
            // Column sums between row i and row j
            let mut col_sums = vec![0i32; n];
            for j in i..m {
                for h in 0..n {
                    col_sums[h] += matrix[j][h];
                }

                // Find max subarray sum <= k using prefix sums + BTreeSet
                let mut prefix = 0i64;
                let mut set = BTreeSet::new();
                set.insert(0i64);
                for &val in &col_sums {
                    prefix += val as i64;
                    // We want max (prefix - prev) such that prefix - prev <= k
                    // => prev >= prefix - k
                    if let Some(&prev) = set.range((prefix - k as i64)..).next() {
                        let candidate = (prefix - prev) as i32;
                        ans = ans.max(candidate);
                    }
                    set.insert(prefix);
                }
            }
        }
        ans
    }
}

/// ## AI Solution — Kadane's optimization with BTreeSet fallback
///
/// Same boundary enumeration. For each pair of rows, first try Kadane's to
/// get max subarray sum. If that sum <= k, use it directly (O(n)).
/// Otherwise fall back to the BTreeSet approach O(n log n).
///
/// - Time: O(m^2 * n) best case, O(m^2 * n log n) worst case
/// - Space: O(n)
mod ai_solution {
    use std::collections::BTreeSet;

    pub fn max_sum_submatrix(matrix: &[Vec<i32>], k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = i32::MIN;

        for i in 0..m {
            let mut col_sums = vec![0i32; n];
            for j in i..m {
                for h in 0..n {
                    col_sums[h] += matrix[j][h];
                }

                // Kadane's to check if we can short-circuit
                let kadane_max = kadane(&col_sums);
                if kadane_max <= k {
                    ans = ans.max(kadane_max);
                    if ans == k {
                        return k; // Can't do better
                    }
                    continue;
                }

                // Fall back to BTreeSet approach
                let mut prefix = 0i64;
                let mut set = BTreeSet::new();
                set.insert(0i64);
                for &val in &col_sums {
                    prefix += val as i64;
                    if let Some(&prev) = set.range((prefix - k as i64)..).next() {
                        let candidate = (prefix - prev) as i32;
                        ans = ans.max(candidate);
                        if ans == k {
                            return k;
                        }
                    }
                    set.insert(prefix);
                }
            }
        }
        ans
    }

    /// Standard Kadane's algorithm for max subarray sum.
    fn kadane(nums: &[i32]) -> i32 {
        let mut best = i32::MIN;
        let mut cur = 0i32;
        for &x in nums {
            cur = cur.max(0) + x;
            best = best.max(cur);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[Vec<i32>], i32) -> i32) {
        assert_eq!(f(&[vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
        assert_eq!(f(&[vec![2, 2, -1]], 3), 3);
        assert_eq!(f(&[vec![2, 2, -1]], 0), -1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::max_sum_submatrix);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::max_sum_submatrix);
    }
}

fn main() {
    let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
    let k = 2;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::max_sum_submatrix(
            std::hint::black_box(&matrix),
            k,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::max_sum_submatrix(
            std::hint::black_box(&matrix),
            k,
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::max_sum_submatrix(
            std::hint::black_box(&matrix),
            k,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::max_sum_submatrix(
            std::hint::black_box(&matrix),
            k,
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 363 - Max Sum of Rectangle No Larger Than K");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
