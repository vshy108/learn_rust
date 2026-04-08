// LeetCode 414: Third Maximum Number [Easy]
// https://leetcode.com/problems/third-maximum-number/
// Benchmark: 1000000 iterations
//   My solution (O(n) track top-3 Option): avg 14.95us
//   AI solution (sort + dedup):            avg 8.16us
//   -> AI solution is 1.83x faster
//
// Return the third distinct maximum number. If not enough distinct values,
// return the overall maximum.
//
// Examples:
//   [3,2,1]   -> 1  (third distinct max)
//   [1,2]     -> 2  (no third max, return max)
//   [2,2,3,1] -> 1  (duplicates don't count)
//
// Constraints:
//   1 <= nums.length <= 10^4
//   -2^31 <= nums[i] <= 2^31 - 1
//
// Run:  cargo run --example lc_0414_third_maximum_number
// Test: cargo test --example lc_0414_third_maximum_number
// Bench: cargo test --example lc_0414_third_maximum_number bench -- --nocapture

/// My solution module — track top 3 distinct values with Option.
///
/// Maintain three `Option<i64>` values (m1 >= m2 >= m3).
/// Use Option<i64> to handle the full i32 range without sentinel issues.
/// Single pass: skip duplicates, shift values down.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the third distinct maximum, or the overall maximum.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Third distinct maximum if it exists, else the maximum
        pub fn third_max(nums: Vec<i32>) -> i32 {
            let mut m1: Option<i64> = None;
            let mut m2: Option<i64> = None;
            let mut m3: Option<i64> = None;

            for &num in &nums {
                let val = num as i64;
                // Skip duplicates of any tracked value
                if m1 == Some(val) || m2 == Some(val) || m3 == Some(val) {
                    continue;
                }

                if m1.is_none() || val > m1.expect("checked") {
                    m3 = m2;
                    m2 = m1;
                    m1 = Some(val);
                } else if m2.is_none() || val > m2.expect("checked") {
                    m3 = m2;
                    m2 = Some(val);
                } else if m3.is_none() || val > m3.expect("checked") {
                    m3 = Some(val);
                }
            }

            // Return m3 if it exists, else m1 (guaranteed to exist)
            (m3.unwrap_or(m1.expect("non-empty array"))) as i32
        }
    }
}

/// AI solution module — sort + dedup, pick third from end.
///
/// Sort the array, deduplicate, then check if we have >= 3 distinct values.
/// If yes, return the third-from-last. Otherwise return the last (max).
///
/// Time:  O(n log n)
/// Space: O(1) -- in-place sort and dedup
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the third distinct maximum, or the overall maximum.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Third distinct maximum if it exists, else the maximum
        pub fn third_max(mut nums: Vec<i32>) -> i32 {
            nums.sort_unstable();
            nums.dedup();

            let n = nums.len();
            if n >= 3 {
                nums[n - 3]
            } else {
                nums[n - 1]
            }
        }
    }
}

fn main() {
    let cases = [
        (vec![3, 2, 1], 1),
        (vec![1, 2], 2),
        (vec![2, 2, 3, 1], 1),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &cases {
        let result = my_solution::Solution::third_max(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &cases {
        let result = ai_solution::Solution::third_max(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_third_max {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::third_max(vec![1, 2]), 2);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
                }

                #[test]
                fn test_single() {
                    assert_eq!(Solution::third_max(vec![5]), 5);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::third_max(vec![1, 1, 1]), 1);
                }

                #[test]
                fn test_min_values() {
                    // Edge case: i32::MIN is a valid value
                    assert_eq!(
                        Solution::third_max(
                            vec![1, 2, i32::MIN],
                        ),
                        i32::MIN,
                    );
                }

                #[test]
                fn test_negative() {
                    assert_eq!(
                        Solution::third_max(vec![-1, -2, -3]),
                        -3,
                    );
                }
            }
        };
    }

    test_third_max!(my_tests, crate::my_solution::Solution);
    test_third_max!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 1_000_000_u32;
        let nums: Vec<i32> = (0..10_000).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::third_max(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::third_max(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
