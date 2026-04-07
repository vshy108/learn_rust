// LeetCode 268: Missing Number [Easy]
// https://leetcode.com/problems/missing-number/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 1.154us
//   AI solution avg: 1.753us
//   Ratio: My 1.52x faster
//
// Given an array nums containing n distinct numbers in [0, n], return
// the only number in the range that is missing.
//
// Example 1: nums = [3,0,1] -> 2
// Example 2: nums = [0,1]   -> 2
// Example 3: nums = [9,6,4,2,3,5,7,0,1] -> 8
//
// Constraints:
//   - n == nums.length
//   - 1 <= n <= 10^4
//   - 0 <= nums[i] <= n
//   - All numbers are unique
//
// Follow up: O(1) extra space, O(n) time.
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0268.Missing%20Number/README_EN.md
//
// Run:  cargo run --example lc_0268_missing_number
// Test: cargo test --example lc_0268_missing_number
// Bench: cargo test --example lc_0268_missing_number bench -- --nocapture

/// My solution — XOR (from doocs Rust Solution 1).
///
/// XOR all indices [0..n] with all values. Duplicates cancel, leaving
/// the missing number. Since x ^ x = 0 and x ^ 0 = x.
///
/// Time: O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the missing number in [0..n].
        pub fn missing_number(nums: Vec<i32>) -> i32 {
            let n = nums.len() as i32;
            let mut ans = n;
            for (i, &v) in nums.iter().enumerate() {
                ans ^= (i as i32) ^ v;
            }
            ans
        }
    }
}

/// AI solution — sum formula (from doocs Solution 2).
///
/// Sum of [0..n] = n*(n+1)/2. Subtract actual sum of array.
/// The difference is the missing number.
///
/// Time: O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the missing number in [0..n].
        pub fn missing_number(nums: Vec<i32>) -> i32 {
            let n = nums.len() as i32;
            let mut ans = n;
            for (i, &v) in nums.iter().enumerate() {
                ans += (i as i32) - v;
            }
            ans
        }
    }
}

fn main() {
    println!("my: {}", my_solution::Solution::missing_number(vec![3, 0, 1]));
    println!("ai: {}", ai_solution::Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
}

#[cfg(test)]
mod tests {
    macro_rules! test_missing_number {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(Sol::missing_number(vec![3, 0, 1]), 2);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::missing_number(vec![0, 1]), 2);
                }

                #[test]
                fn test_example_3() {
                    assert_eq!(Sol::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
                }

                #[test]
                fn test_missing_zero() {
                    assert_eq!(Sol::missing_number(vec![1]), 0);
                }

                #[test]
                fn test_missing_n() {
                    assert_eq!(Sol::missing_number(vec![0]), 1);
                }

                #[test]
                fn test_larger() {
                    // 0..=99 missing 50
                    let nums: Vec<i32> = (0..50).chain(51..=99).collect();
                    assert_eq!(Sol::missing_number(nums), 50);
                }
            }
        };
    }

    test_missing_number!(my, crate::my_solution::Solution);
    test_missing_number!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..5000).chain(5001..=10000).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::missing_number(nums.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::missing_number(nums.clone()));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
