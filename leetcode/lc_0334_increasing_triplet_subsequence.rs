// @lc app=leetcode id=334 lang=rust
// #[Medium] - Increasing Triplet Subsequence
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.004us
// ai_solution  | avg: 0.069us
// Performance: My 19.10x faster

/// # 334. Increasing Triplet Subsequence
///
/// Given an integer array `nums`, return `true` if there exists a triple
/// (i, j, k) with i < j < k and nums[i] < nums[j] < nums[k].
///
/// Follow-up: O(n) time, O(1) space.
///
/// Constraints:
/// - 1 <= nums.length <= 5 * 10^5
/// - -2^31 <= nums[i] <= 2^31 - 1

/// ## My Solution — Two-variable greedy
///
/// Track `first` (smallest so far) and `second` (smallest value larger
/// than some previous `first`). If we find a value > `second`, return true.
///
/// - Time: O(n)
/// - Space: O(1)
mod my_solution {
    pub fn increasing_triplet(nums: &[i32]) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;
        for &num in nums {
            if num <= first {
                first = num;
            } else if num <= second {
                // num > first and <= second, update second
                second = num;
            } else {
                // num > second > first => found triplet
                return true;
            }
        }
        false
    }
}

/// ## AI Solution — Prefix min + suffix max
///
/// Build arrays of prefix minimums and suffix maximums.
/// For index j, check if prefix_min[j] < nums[j] < suffix_max[j].
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    pub fn increasing_triplet(nums: &[i32]) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        // prefix_min[i] = min(nums[0..=i])
        let mut prefix_min = vec![0i32; n];
        prefix_min[0] = nums[0];
        for i in 1..n {
            prefix_min[i] = prefix_min[i - 1].min(nums[i]);
        }
        // suffix_max[i] = max(nums[i..n])
        let mut suffix_max = vec![0i32; n];
        suffix_max[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            suffix_max[i] = suffix_max[i + 1].max(nums[i]);
        }
        // Check middle element
        for j in 1..n - 1 {
            if prefix_min[j - 1] < nums[j] && nums[j] < suffix_max[j + 1] {
                return true;
            }
        }
        false
    }
}

fn run_tests(f: fn(&[i32]) -> bool) {
    assert!(f(&[1, 2, 3, 4, 5]));
    assert!(!f(&[5, 4, 3, 2, 1]));
    assert!(f(&[2, 1, 5, 0, 4, 6]));
    assert!(!f(&[1, 1, 1]));
    assert!(f(&[20, 100, 10, 12, 5, 13])); // 10 < 12 < 13
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::increasing_triplet);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::increasing_triplet);
    }
}

fn main() {
    let nums = vec![2, 1, 5, 0, 4, 6];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::increasing_triplet(
            std::hint::black_box(&nums),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::increasing_triplet(
            std::hint::black_box(&nums),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::increasing_triplet(
            std::hint::black_box(&nums),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::increasing_triplet(
            std::hint::black_box(&nums),
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

    println!("LC 334 - Increasing Triplet Subsequence");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
