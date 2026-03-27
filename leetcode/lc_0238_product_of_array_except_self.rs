// LeetCode 238: Product of Array Except Self [Medium]
// https://leetcode.com/problems/product-of-array-except-self/
//
// Given an integer array nums, return an array answer such that answer[i]
// is equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a
// 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using
// the division operation.
//
// Examples:
//   nums = [1,2,3,4]     -> [24,12,8,6]
//   nums = [-1,1,0,-3,3] -> [0,0,9,0,0]
//
// Constraints:
//   - 2 <= nums.length <= 10^5
//   - -30 <= nums[i] <= 30
//   - The product of any prefix or suffix of nums fits in a 32-bit integer
//
// Follow up: Can you solve it in O(1) extra space complexity?
//   (The output array does not count as extra space.)
//
// Run:  cargo run --example lc_0238_product_of_array_except_self
// Test: cargo test --example lc_0238_product_of_array_except_self
// Bench: cargo test --example lc_0238_product_of_array_except_self bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return an array where each element is the product of all other
        /// elements in `nums`. Must run in O(n) without division.
        ///
        /// # Arguments
        ///
        /// * `nums` - The input integer array
        ///
        /// # Returns
        ///
        /// Vec where `result[i]` = product of all nums except `nums[i]`
        pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
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
        /// Return an array where each element is the product of all other
        /// elements in `nums`. Must run in O(n) without division.
        ///
        /// # Arguments
        ///
        /// * `nums` - The input integer array
        ///
        /// # Returns
        ///
        /// Vec where `result[i]` = product of all nums except `nums[i]`
        pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
        (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::product_except_self(nums.clone());
        println!("  nums={nums:?} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::product_except_self(nums.clone());
        println!("  nums={nums:?} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_product_except_self {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic() {
                    assert_eq!(
                        Solution::product_except_self(vec![1, 2, 3, 4]),
                        vec![24, 12, 8, 6]
                    );
                }

                #[test]
                fn test_with_zero() {
                    assert_eq!(
                        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
                        vec![0, 0, 9, 0, 0]
                    );
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(
                        Solution::product_except_self(vec![3, 5]),
                        vec![5, 3]
                    );
                }

                #[test]
                fn test_all_ones() {
                    assert_eq!(
                        Solution::product_except_self(vec![1, 1, 1, 1]),
                        vec![1, 1, 1, 1]
                    );
                }

                #[test]
                fn test_negative_numbers() {
                    assert_eq!(
                        Solution::product_except_self(vec![-1, -2, -3]),
                        vec![6, 3, 2]
                    );
                }

                #[test]
                fn test_two_zeros() {
                    assert_eq!(
                        Solution::product_except_self(vec![0, 0, 3]),
                        vec![0, 0, 0]
                    );
                }

                #[test]
                fn test_single_zero() {
                    assert_eq!(
                        Solution::product_except_self(vec![2, 0, 4]),
                        vec![0, 8, 0]
                    );
                }
            }
        };
    }

    test_product_except_self!(my_tests, crate::my_solution::Solution);
    test_product_except_self!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 2, 3, 4],
            vec![-1, 1, 0, -3, 3],
            (1..=20).collect(),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::product_except_self(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::product_except_self(
                    std::hint::black_box(nums.clone()),
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
