// @lc app=leetcode id=377 lang=rust
// #[Medium] - Combination Sum IV
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.217us
// ai_solution  | avg: 0.200us
// Performance: AI 1.09x faster

/// # 377. Combination Sum IV
///
/// Given distinct integers nums and a target, return the number of possible
/// combinations (order matters) that add up to target.
///
/// Constraints:
/// - 1 <= nums.length <= 200
/// - 1 <= nums[i] <= 1000
/// - All elements are unique
/// - 1 <= target <= 1000

/// ## My Solution — Bottom-up DP O(n * target)
///
/// dp[i] = number of ordered combinations that sum to i.
/// dp[0] = 1. For each i, add dp[i - x] for all x in nums where i >= x.
///
/// - Time: O(n * target)
/// - Space: O(target)
mod my_solution {
    pub fn combination_sum4(nums: &[i32], target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0i32; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &x in nums {
                let x = x as usize;
                if i >= x {
                    dp[i] += dp[i - x];
                }
            }
        }
        dp[target]
    }
}

/// ## AI Solution — Top-down memoized recursion
///
/// Same recurrence with caching.
///
/// - Time: O(n * target)
/// - Space: O(target)
mod ai_solution {
    pub fn combination_sum4(nums: &[i32], target: i32) -> i32 {
        let target = target as usize;
        let mut memo = vec![-1i32; target + 1];
        memo[0] = 1;
        solve(nums, target, &mut memo)
    }

    fn solve(nums: &[i32], remain: usize, memo: &mut [i32]) -> i32 {
        if memo[remain] >= 0 {
            return memo[remain];
        }
        let mut count = 0;
        for &x in nums {
            let x = x as usize;
            if remain >= x {
                count += solve(nums, remain - x, memo);
            }
        }
        memo[remain] = count;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[i32], i32) -> i32) {
        assert_eq!(f(&[1, 2, 3], 4), 7);
        assert_eq!(f(&[9], 3), 0);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::combination_sum4);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::combination_sum4);
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let target = 32;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::combination_sum4(
            std::hint::black_box(&nums),
            target,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::combination_sum4(
            std::hint::black_box(&nums),
            target,
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::combination_sum4(
            std::hint::black_box(&nums),
            target,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::combination_sum4(
            std::hint::black_box(&nums),
            target,
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

    println!("LC 377 - Combination Sum IV");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
