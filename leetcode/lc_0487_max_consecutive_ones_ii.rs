// LeetCode 487: Max Consecutive Ones II [Medium]
// https://leetcode.com/problems/max-consecutive-ones-ii/
//
// Given a binary array nums, return the maximum number of consecutive 1's
// in the array if you can flip at most one 0.
//
// Examples:
//   nums = [1,0,1,1,0] -> 4
//   nums = [1,0,1,1,0,1] -> 4
//
// Constraints:
//   1 <= nums.length <= 10^5
//   nums[i] is either 0 or 1
//
// Follow up: What if the input numbers come in one by one as an infinite
// stream?
//
// Run:  cargo run --example lc_0487_max_consecutive_ones_ii
// Test: cargo test --example lc_0487_max_consecutive_ones_ii
// Bench: cargo test --example lc_0487_max_consecutive_ones_ii bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 738ns
//   AI solution: avg 774ns
//   Ratio: 0.95x (similar sliding window)

/// Time: O(n), Space: O(1)
/// Sliding window: grow right side each step; when zeros > 1, shrink
/// left side by one step. Window never shrinks, only grows or stays.
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
            let mut l = 0usize;
            let mut cnt = 0; // count of zeros in the window
            for &x in &nums {
                cnt += x ^ 1; // increment if x == 0
                if cnt > 1 {
                    // shrink window from left by one position
                    cnt -= nums[l] ^ 1;
                    l += 1;
                }
            }
            (nums.len() - l) as i32
        }
    }
}

/// Time: O(n), Space: O(1)
/// Two-pointer with explicit tracking of previous-zero position.
/// Tracks the position of the last zero seen to efficiently reset
/// the left boundary of the window.
#[allow(dead_code)]
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
            let mut max_len = 0i32;
            let mut left = 0i32;
            // Position just after the last zero we flipped
            let mut zero_pos: i32 = -1;
            for (i, &x) in nums.iter().enumerate() {
                let i = i as i32;
                if x == 0 {
                    // Move left boundary past the previous zero
                    left = zero_pos + 1;
                    zero_pos = i;
                }
                max_len = max_len.max(i - left + 1);
            }
            max_len
        }
    }
}

fn main() {
    println!("LeetCode 487: Max Consecutive Ones II");
    let cases = vec![(vec![1, 0, 1, 1, 0], 4), (vec![1, 0, 1, 1, 0, 1], 4)];
    for (nums, expected) in &cases {
        let r1 = my_solution::Solution::find_max_consecutive_ones(nums.clone());
        let r2 = ai_solution::Solution::find_max_consecutive_ones(nums.clone());
        println!("  nums={nums:?} => my={r1}, ai={r2} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $nums:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(
                        my_solution::Solution::find_max_consecutive_ones($nums),
                        $expected
                    );
                }
                #[test]
                fn ai() {
                    assert_eq!(
                        ai_solution::Solution::find_max_consecutive_ones($nums),
                        $expected
                    );
                }
            }
        };
    }

    test_both!(example1, vec![1, 0, 1, 1, 0], 4);
    test_both!(example2, vec![1, 0, 1, 1, 0, 1], 4);
    test_both!(all_ones, vec![1, 1, 1, 1], 4);
    test_both!(all_zeros, vec![0, 0, 0], 1);
    test_both!(single_one, vec![1], 1);
    test_both!(single_zero, vec![0], 1);
    test_both!(alternating, vec![1, 0, 1, 0, 1], 3);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let nums: Vec<i32> = (0..1_000).map(|i| if i % 3 == 0 { 0 } else { 1 }).collect();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(nums.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution(
        "My solution",
        my_solution::Solution::find_max_consecutive_ones,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::find_max_consecutive_ones,
    );
}
