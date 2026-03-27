// LeetCode 219: Contains Duplicate II [Easy]
// https://leetcode.com/problems/contains-duplicate-ii/
//
// Given an integer array nums and an integer k, return true if there are
// two distinct indices i and j in the array such that nums[i] == nums[j]
// and abs(i - j) <= k.
//
// Examples:
//   nums = [1,2,3,1],       k = 3 -> true
//   nums = [1,0,1,1],       k = 1 -> true
//   nums = [1,2,3,1,2,3],   k = 2 -> false
//
// Constraints:
//   - 1 <= nums.length <= 10^5
//   - -10^9 <= nums[i] <= 10^9
//   - 0 <= k <= 10^5
//
// Run:  cargo run --example lc_0219_contains_duplicate_ii
// Test: cargo test --example lc_0219_contains_duplicate_ii
// Bench: cargo test --example lc_0219_contains_duplicate_ii bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return `true` if there exist two distinct indices `i` and `j`
        /// such that `nums[i] == nums[j]` and `abs(i - j) <= k`.
        ///
        /// # Arguments
        ///
        /// * `nums` - The input integer array
        /// * `k` - Maximum allowed index distance
        ///
        /// # Returns
        ///
        /// `true` if a nearby duplicate exists, `false` otherwise
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return `true` if there exist two distinct indices `i` and `j`
        /// such that `nums[i] == nums[j]` and `abs(i - j) <= k`.
        ///
        /// # Arguments
        ///
        /// * `nums` - The input integer array
        /// * `k` - Maximum allowed index distance
        ///
        /// # Returns
        ///
        /// `true` if a nearby duplicate exists, `false` otherwise
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, bool)> = vec![
        (vec![1, 2, 3, 1], 3, true),
        (vec![1, 0, 1, 1], 1, true),
        (vec![1, 2, 3, 1, 2, 3], 2, false),
    ];

    println!("--- My Solution ---");
    for (nums, k, expected) in &test_cases {
        let result = my_solution::Solution::contains_nearby_duplicate(nums.clone(), *k);
        println!("  nums={nums:?}, k={k} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, k, expected) in &test_cases {
        let result = ai_solution::Solution::contains_nearby_duplicate(nums.clone(), *k);
        println!("  nums={nums:?}, k={k} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_contains_nearby_dup {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_duplicate_within_k() {
                    assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
                }

                #[test]
                fn test_duplicate_at_distance_1() {
                    assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
                }

                #[test]
                fn test_duplicate_outside_k() {
                    assert!(!Solution::contains_nearby_duplicate(
                        vec![1, 2, 3, 1, 2, 3],
                        2
                    ));
                }

                #[test]
                fn test_no_duplicates() {
                    assert!(!Solution::contains_nearby_duplicate(vec![1, 2, 3, 4], 2));
                }

                #[test]
                fn test_single_element() {
                    assert!(!Solution::contains_nearby_duplicate(vec![1], 1));
                }

                #[test]
                fn test_all_same_k_zero() {
                    assert!(!Solution::contains_nearby_duplicate(vec![1, 1, 1], 0));
                }

                #[test]
                fn test_duplicate_at_exact_k() {
                    // indices 0 and 2, distance = 2, k = 2 -> true
                    assert!(Solution::contains_nearby_duplicate(vec![1, 2, 1], 2));
                }
            }
        };
    }

    test_contains_nearby_dup!(my_tests, crate::my_solution::Solution);
    test_contains_nearby_dup!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 2, 3, 1], 3),
            (vec![1, 0, 1, 1], 1),
            (vec![1, 2, 3, 1, 2, 3], 2),
            ((0..100).collect(), 50),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, k) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::contains_nearby_duplicate(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*k),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, k) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::contains_nearby_duplicate(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*k),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
