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
// --- Benchmark (100_000 iterations, 2026-04-06) ---
//   My solution: 114ms (avg 1.14us)
//   AI solution: 268ms (avg 2.68us)
//   -> My solution is 2.35x faster
//   Reason: My solution modifies result in-place (one allocation). AI solution
//     creates separate prefix/suffix Vecs then zips them (3 allocations +
//     extra iterator overhead).
//
// --- Rust concepts used ---
//   - vec![1; n]: initialize Vec with n copies of 1. Ruby: Array.new(n, 1).
//   - result[i] = val: direct index assignment (overwrite existing element).
//     NOT result.insert(i, val) — that inserts a new element, shifting right.
//   - result[i] *= suffix: augmented assignment works on Vec elements.
//   - (0..n).rev(): reverse any iterator. Ruby: (0...n).reverse_each.
//   - .scan(init, |acc, x| ...): stateful iterator transform — yields a value
//     each step while mutating an accumulator. No direct Ruby equivalent.
//   - .zip(): pairs two iterators element-wise into tuples. Ruby: .zip().
//   - .collect::<Vec<_>>(): turbofish to specify collection target type.
//
// Run:  cargo run --example lc_0238_product_of_array_except_self
// Test: cargo test --example lc_0238_product_of_array_except_self
// Bench: cargo test --example lc_0238_product_of_array_except_self bench -- --nocapture

/// My solution module.
///
/// Two-pass prefix/suffix product. First pass fills result with prefix
/// products (product of all elements before i). Second pass multiplies
/// in suffix products (product of all elements after i) in reverse.
///
/// Time:  O(n) — two passes over the array
/// Space: O(1) — output array doesn't count; only prefix/suffix scalars
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
            let n = nums.len();
            let mut result = vec![1; n];
            let mut prefix = 1;
            for i in 0..n {
                // Ruby: result[i] = prefix  (overwrite element at index i)
                // Rust: result[i] = prefix  (same syntax — direct index assignment)
                // NOT result.insert(i, prefix) — that inserts a NEW element,
                // shifting everything right and growing the Vec
                result[i] = prefix;
                prefix *= nums[i];
            }

            let mut suffix = 1;

            // Ruby: (0...n).reverse_each { |i| ... }
            // Rust: (0..n).rev() — .rev() reverses any iterator
            // .into_iter() is redundant on Range (it's already an iterator)
            for i in (0..n).rev() {
                // Ruby: result[i] *= suffix
                // Rust: result[i] *= suffix (same — augmented assignment works)
                result[i] *= suffix;
                suffix *= nums[i];
            }

            result
        }
    }
}

/// AI solution module.
///
/// Same two-pass prefix/suffix approach but uses iterator adapters
/// (`scan` + `zip`) instead of manual index loops. `scan` accumulates
/// a running product while yielding the *previous* accumulator value,
/// which is exactly the prefix/suffix product needed.
///
/// Time:  O(n) — two passes over the array
/// Space: O(1) — output array doesn't count; only iterator state
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
            // Build prefix products using scan:
            // scan(initial_state, |state, element| { ... }) yields the
            // *previous* state before updating, giving us the product
            // of all elements before index i.
            let prefix: Vec<i32> = nums
                .iter()
                .scan(1, |acc, &x| {
                    let prev = *acc;
                    *acc *= x;
                    Some(prev)
                })
                .collect();

            // Build suffix products in reverse, then zip with prefix.
            // .rev() on slice iter gives us right-to-left traversal.
            // We collect suffix products reversed, then reverse back.
            let suffix: Vec<i32> = nums
                .iter()
                .rev()
                .scan(1, |acc, &x| {
                    let prev = *acc;
                    *acc *= x;
                    Some(prev)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

            // Multiply prefix and suffix products element-wise
            // .zip() pairs elements from two iterators into tuples
            prefix
                .iter()
                .zip(suffix.iter())
                .map(|(&p, &s)| p * s)
                .collect()
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
                    assert_eq!(Solution::product_except_self(vec![3, 5]), vec![5, 3]);
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
                    assert_eq!(Solution::product_except_self(vec![0, 0, 3]), vec![0, 0, 0]);
                }

                #[test]
                fn test_single_zero() {
                    assert_eq!(Solution::product_except_self(vec![2, 0, 4]), vec![0, 8, 0]);
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
        let inputs: Vec<Vec<i32>> =
            // Note: constraint says -30 <= nums[i] <= 30 and product fits i32,
            // but (1..=20) would cause 20! overflow. Use small values.
            vec![vec![1, 2, 3, 4], vec![-1, 1, 0, -3, 3], vec![2, 3, 1, 4, 2, 1, 3, 2]];

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
