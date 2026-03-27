// LeetCode 27: Remove Element [Easy]
// https://leetcode.com/problems/remove-element/
//
// Given an integer array nums and an integer val, remove all occurrences
// of val in nums in-place. The order of the elements may be changed.
// Then return the number of elements in nums which are not equal to val.
//
// Consider the number of elements in nums which are not equal to val be k,
// to get accepted, you need to do the following things:
//   - Change the array nums such that the first k elements of nums contain
//     the elements which are not equal to val.
//   - The remaining elements of nums are not important as well as the size
//     of nums.
//   - Return k.
//
// Examples:
//   nums = [3,2,2,3], val = 3         -> 2, nums = [2,2,_,_]
//   nums = [0,1,2,2,3,0,4,2], val = 2 -> 5, nums = [0,1,4,0,3,_,_,_]
//
// Constraints:
//   - 0 <= nums.length <= 100
//   - 0 <= nums[i] <= 50
//   - 0 <= val <= 100
//
// Run:  cargo run --example lc_0027_remove_element
// Test: cargo test --example lc_0027_remove_element
// Bench: cargo test --example lc_0027_remove_element bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Remove all occurrences of `val` from `nums` in-place.
        ///
        /// # Arguments
        ///
        /// * `nums` - A mutable reference to an integer vector
        /// * `val` - The value to remove
        ///
        /// # Returns
        ///
        /// The number of elements remaining (k). The first k elements
        /// of `nums` will contain only values != val.
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
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
        /// Remove all occurrences of `val` from `nums` in-place.
        ///
        /// # Arguments
        ///
        /// * `nums` - A mutable reference to an integer vector
        /// * `val` - The value to remove
        ///
        /// # Returns
        ///
        /// The number of elements remaining (k). The first k elements
        /// of `nums` will contain only values != val.
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![3, 2, 2, 3], 3, 2),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5),
    ];

    println!("--- My Solution ---");
    for (nums, val, expected_k) in &test_cases {
        let mut nums = nums.clone();
        let k = my_solution::Solution::remove_element(&mut nums, *val);
        println!(
            "  k={k}, nums={:?} (expected k={expected_k})",
            &nums[..k as usize]
        );
    }

    println!("--- AI Solution ---");
    for (nums, val, expected_k) in &test_cases {
        let mut nums = nums.clone();
        let k = ai_solution::Solution::remove_element(&mut nums, *val);
        println!(
            "  k={k}, nums={:?} (expected k={expected_k})",
            &nums[..k as usize]
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_remove_element {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn run(mut nums: Vec<i32>, val: i32, expected_k: i32) {
                    let k = Solution::remove_element(&mut nums, val);
                    assert_eq!(k, expected_k);
                    // First k elements must not contain val.
                    for &n in &nums[..k as usize] {
                        assert_ne!(n, val);
                    }
                }

                #[test]
                fn test_basic_remove_3() {
                    run(vec![3, 2, 2, 3], 3, 2);
                }

                #[test]
                fn test_remove_2_from_mixed() {
                    run(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5);
                }

                #[test]
                fn test_empty_array() {
                    run(vec![], 1, 0);
                }

                #[test]
                fn test_no_matches() {
                    run(vec![1, 2, 3], 4, 3);
                }

                #[test]
                fn test_all_matches() {
                    run(vec![7, 7, 7], 7, 0);
                }

                #[test]
                fn test_single_element_match() {
                    run(vec![1], 1, 0);
                }

                #[test]
                fn test_single_element_no_match() {
                    run(vec![1], 2, 1);
                }
            }
        };
    }

    test_remove_element!(my_tests, crate::my_solution::Solution);
    test_remove_element!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![3, 2, 2, 3], 3),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            (vec![7, 7, 7, 7, 7], 7),
            (vec![1, 2, 3, 4, 5], 6),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, val) in &inputs {
                let mut nums = nums.clone();
                std::hint::black_box(crate::my_solution::Solution::remove_element(
                    std::hint::black_box(&mut nums),
                    std::hint::black_box(*val),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, val) in &inputs {
                let mut nums = nums.clone();
                std::hint::black_box(crate::ai_solution::Solution::remove_element(
                    std::hint::black_box(&mut nums),
                    std::hint::black_box(*val),
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
