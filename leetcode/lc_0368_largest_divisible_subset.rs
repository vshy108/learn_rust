// @lc app=leetcode id=368 lang=rust
// #[Medium] - Largest Divisible Subset
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.221us
// ai_solution  | avg: 0.200us
// Performance: AI 1.10x faster

/// # 368. Largest Divisible Subset
///
/// Given a set of distinct positive integers, return the largest subset where every
/// pair (answer[i], answer[j]) satisfies answer[i] % answer[j] == 0 or vice versa.
///
/// Constraints:
/// - 1 <= nums.length <= 1000
/// - 1 <= nums[i] <= 2 * 10^9
/// - All integers are unique

/// ## My Solution — Sort + DP with backtracking O(n^2)
///
/// Sort, then dp[i] = length of largest divisible subset ending at nums[i].
/// Track the predecessor for reconstruction.
///
/// - Time: O(n^2)
/// - Space: O(n)
mod my_solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut dp = vec![1usize; n];
        let mut prev = vec![usize::MAX; n]; // predecessor index
        let mut best_idx = 0;

        for i in 1..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
            if dp[i] > dp[best_idx] {
                best_idx = i;
            }
        }

        // Reconstruct the subset by following prev pointers
        let mut result = Vec::with_capacity(dp[best_idx]);
        let mut idx = best_idx;
        while idx != usize::MAX {
            result.push(nums[idx]);
            idx = prev[idx];
        }
        result.reverse();
        result
    }
}

/// ## AI Solution — Sort + DP, reconstruct by scanning backwards
///
/// Same DP idea but reconstruct by scanning backwards using divisibility
/// and dp value matching, without storing predecessor array.
///
/// - Time: O(n^2)
/// - Space: O(n)
mod ai_solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut f = vec![1i32; n];
        let mut k = 0usize;

        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    f[i] = f[i].max(f[j] + 1);
                }
            }
            if f[k] < f[i] {
                k = i;
            }
        }

        let mut m = f[k];
        let mut ans = Vec::with_capacity(m as usize);
        // Walk backwards from k to collect the subset
        let mut i = k as i32;
        while m > 0 {
            let ui = i as usize;
            if nums[k] % nums[ui] == 0 && f[ui] == m {
                ans.push(nums[ui]);
                k = ui;
                m -= 1;
            }
            i -= 1;
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(Vec<i32>) -> Vec<i32>) {
        let res1 = f(vec![1, 2, 3]);
        // Valid answers: [1,2] or [1,3]
        assert!(res1.len() == 2);
        for i in 0..res1.len() {
            for j in 0..res1.len() {
                if i != j {
                    assert!(res1[i] % res1[j] == 0 || res1[j] % res1[i] == 0);
                }
            }
        }

        let res2 = f(vec![1, 2, 4, 8]);
        assert_eq!(res2, vec![1, 2, 4, 8]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::largest_divisible_subset);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::largest_divisible_subset);
    }
}

fn main() {
    let nums = vec![1, 2, 4, 8, 16, 32, 3, 9, 27];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::largest_divisible_subset(std::hint::black_box(
            nums.clone(),
        )));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::largest_divisible_subset(std::hint::black_box(
            nums.clone(),
        )));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::largest_divisible_subset(std::hint::black_box(
            nums.clone(),
        )));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::largest_divisible_subset(std::hint::black_box(
            nums.clone(),
        )));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 368 - Largest Divisible Subset");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
