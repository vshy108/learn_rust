// LeetCode 81: Search in Rotated Sorted Array II [Medium]
// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.371us
//   AI solution avg: 0.169us
//   Ratio: AI 2.19x faster
//
// Given a rotated sorted array that may contain duplicates, determine if
// target is in nums. Return true or false.
//
// Constraints: 1 <= nums.length <= 5000, -10^4 <= nums[i] <= 10^4
//
// Run:  cargo run --example lc_0081_search_rotated_sorted_ii
// Test: cargo test --example lc_0081_search_rotated_sorted_ii
// Bench: cargo test --example lc_0081_search_rotated_sorted_ii bench -- --nocapture

/// My solution — linear scan.
///
/// Time: O(n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if target exists in rotated sorted array with duplicates.
        ///
        /// # Arguments
        /// * `nums` - rotated sorted array (may have duplicates)
        /// * `target` - value to search for
        ///
        /// # Returns
        /// true if target is found
        pub fn search(nums: Vec<i32>, target: i32) -> bool {
            nums.contains(&target)
        }
    }
}

/// AI solution — modified binary search handling duplicates.
///
/// When nums[lo] == nums[mid] == nums[hi], we can't determine which
/// half is sorted, so shrink both ends. Worst case O(n).
///
/// Time: O(log n) average, O(n) worst case with many duplicates.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if target exists in rotated sorted array with duplicates.
        ///
        /// # Arguments
        /// * `nums` - rotated sorted array (may have duplicates)
        /// * `target` - value to search for
        ///
        /// # Returns
        /// true if target is found
        pub fn search(nums: Vec<i32>, target: i32) -> bool {
            let mut lo = 0usize;
            let mut hi = nums.len() - 1;

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] == target {
                    return true;
                }

                // Can't determine sorted half — shrink boundaries
                if nums[lo] == nums[mid] && nums[mid] == nums[hi] {
                    lo += 1;
                    if hi == 0 {
                        break;
                    }
                    hi -= 1;
                } else if nums[lo] <= nums[mid] {
                    // Left half is sorted
                    if nums[lo] <= target && target < nums[mid] {
                        if mid == 0 { break; }
                        hi = mid - 1;
                    } else {
                        lo = mid + 1;
                    }
                } else {
                    // Right half is sorted
                    if nums[mid] < target && target <= nums[hi] {
                        lo = mid + 1;
                    } else {
                        if mid == 0 { break; }
                        hi = mid - 1;
                    }
                }
            }
            false
        }
    }
}

fn main() {
    let cases = [
        (vec![2, 5, 6, 0, 0, 1, 2], 0),
        (vec![2, 5, 6, 0, 0, 1, 2], 3),
    ];
    for (nums, target) in cases {
        let my = my_solution::Solution::search(nums.clone(), target);
        let ai = ai_solution::Solution::search(nums, target);
        println!("target={target}: my={my}, ai={ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_search {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_found() {
                    assert!(<$solution>::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
                }

                #[test]
                fn test_not_found() {
                    assert!(!<$solution>::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
                }

                #[test]
                fn test_single_found() {
                    assert!(<$solution>::search(vec![1], 1));
                }

                #[test]
                fn test_single_not_found() {
                    assert!(!<$solution>::search(vec![1], 0));
                }

                #[test]
                fn test_all_same_found() {
                    assert!(<$solution>::search(vec![1, 1, 1, 1], 1));
                }

                #[test]
                fn test_all_same_not_found() {
                    assert!(!<$solution>::search(vec![1, 1, 1, 1], 2));
                }

                #[test]
                fn test_rotated_with_dups() {
                    assert!(<$solution>::search(vec![1, 0, 1, 1, 1], 0));
                }
            }
        };
    }

    test_search!(my, crate::my_solution::Solution);
    test_search!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = {
            let mut v: Vec<i32> = (0..100).collect();
            // Simulate rotation
            let pivot = 30;
            v.rotate_left(pivot);
            v
        };

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::search(
                std::hint::black_box(nums.clone()), 75,
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::search(
                std::hint::black_box(nums.clone()), 75,
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
