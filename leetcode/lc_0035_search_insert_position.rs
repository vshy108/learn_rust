// LeetCode 35: Search Insert Position [Easy]
// https://leetcode.com/problems/search-insert-position/
//
// Given a sorted array of distinct integers and a target value, return the
// index if the target is found. If not, return the index where it would be
// if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Examples:
//   nums = [1,3,5,6], target = 5 -> 2
//   nums = [1,3,5,6], target = 2 -> 1
//   nums = [1,3,5,6], target = 7 -> 4
//
// Constraints:
//   - 1 <= nums.length <= 10^4
//   - -10^4 <= nums[i] <= 10^4
//   - nums contains distinct values sorted in ascending order
//   - -10^4 <= target <= 10^4
//
// Run:  cargo run --example lc_0035_search_insert_position
// Test: cargo test --example lc_0035_search_insert_position
// Bench: cargo test --example lc_0035_search_insert_position bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-03-27) ---
//   My solution: 106.50ms (avg 1.064us)
//   AI solution: 117.84ms (avg 1.178us)
//   -> My solution is 1.11x faster
//   Reason: Both use the same algorithm; partition_point adds closure indirection
//           and generic monomorphization overhead in debug builds. Near-identical
//           in optimized (release) builds.
/// My solution module.
///
/// Time:  O(log n) -- binary search halves the range each iteration
/// Space: O(1) -- only two index variables
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the index of `target` in `nums`, or where it would be inserted.
        ///
        /// # Arguments
        ///
        /// * `nums` - A sorted vector of distinct integers
        /// * `target` - The target value to find or insert
        ///
        /// # Returns
        ///
        /// The index of `target`, or the insertion position
        pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
            let mut left: usize = 0;
            // Exclusive upper bound: right = len (not len - 1) avoids the usize
            // underflow that happens with `right = mid - 1` when mid is 0.
            // Loop uses `left < right` (not <=) and converges when they meet.
            let mut right: usize = nums.len();

            while left < right {
                let mid = left + (right - left) / 2;

                if nums[mid] < target {
                    // mid + 1 won't overflow: mid = left + (right - left) / 2,
                    // so mid <= right - 1, meaning mid + 1 <= right <= nums.len().
                    left = mid + 1;
                } else {
                    // Set right = mid (not mid - 1) — no subtraction, no underflow.
                    // This is the "lower bound" binary search pattern idiomatic in Rust.
                    right = mid;
                }
            }

            // When left == right, it's either the index where target was found,
            // or the insertion point where target would go.
            left as i32
        }
    }
}

/// AI solution module.
///
/// Uses `slice::partition_point()` -- a stdlib method that performs the
/// exact same exclusive-upper-bound binary search internally. It returns
/// the first index where the predicate returns false, which for a sorted
/// slice and `|&x| x < target` gives the insertion point.
///
/// Time:  O(log n) -- binary search inside partition_point
/// Space: O(1) -- no extra allocation
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the index of `target` in `nums`, or where it would be inserted.
        ///
        /// # Arguments
        ///
        /// * `nums` - A sorted vector of distinct integers
        /// * `target` - The target value to find or insert
        ///
        /// # Returns
        ///
        /// The index of `target`, or the insertion position
        pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
            // partition_point returns the first index where the predicate is false.
            // For sorted data, `|&x| x < target` finds the lower bound -- exactly
            // the insertion point. This is the same algorithm as my_solution but
            // using a single stdlib call.
            nums.partition_point(|&x| x < target) as i32
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![1, 3, 5, 6], 5, 2),
        (vec![1, 3, 5, 6], 2, 1),
        (vec![1, 3, 5, 6], 7, 4),
    ];

    println!("--- My Solution ---");
    for (nums, target, expected) in &test_cases {
        let result = my_solution::Solution::search_insert(nums.clone(), *target);
        println!("  nums={nums:?}, target={target} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, target, expected) in &test_cases {
        let result = ai_solution::Solution::search_insert(nums.clone(), *target);
        println!("  nums={nums:?}, target={target} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_search_insert {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_target_found() {
                    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
                }

                #[test]
                fn test_insert_middle() {
                    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
                }

                #[test]
                fn test_insert_end() {
                    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
                }

                #[test]
                fn test_insert_beginning() {
                    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
                }

                #[test]
                fn test_single_element_found() {
                    assert_eq!(Solution::search_insert(vec![1], 1), 0);
                }

                #[test]
                fn test_single_element_before() {
                    assert_eq!(Solution::search_insert(vec![3], 1), 0);
                }

                #[test]
                fn test_single_element_after() {
                    assert_eq!(Solution::search_insert(vec![3], 5), 1);
                }
            }
        };
    }

    test_search_insert!(my_tests, crate::my_solution::Solution);
    test_search_insert!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 3, 5, 6], 5),
            (vec![1, 3, 5, 6], 2),
            (vec![1, 3, 5, 6], 7),
            (vec![1, 3, 5, 6], 0),
            ((1..10000).collect(), 5000),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, target) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::search_insert(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*target),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, target) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::search_insert(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*target),
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
