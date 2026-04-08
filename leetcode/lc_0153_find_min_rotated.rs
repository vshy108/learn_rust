// LeetCode 153: Find Minimum in Rotated Sorted Array [Medium]
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.154us
//   AI solution avg: 0.121us
//   Ratio: AI 1.27x faster
//
// Given a sorted rotated array of unique elements, return the minimum element.
// Must be O(log n) time.
//
// Constraints: n == nums.length, 1 <= n <= 5000, -5000 <= nums[i] <= 5000
//   All values are unique, nums was sorted ascending then rotated 1..n times
//
// Run:  cargo run --example lc_0153_find_min_rotated
// Test: cargo test --example lc_0153_find_min_rotated
// Bench: cargo test --example lc_0153_find_min_rotated bench -- --nocapture

/// My solution — binary search comparing mid with right boundary.
///
/// Time: O(log n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the minimum element in the rotated sorted array.
        pub fn find_min(nums: Vec<i32>) -> i32 {
            let (mut lo, mut hi) = (0, nums.len() - 1);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] > nums[hi] {
                    // Minimum is in the right half (rotation pivot is there)
                    lo = mid + 1;
                } else {
                    // Minimum is at mid or in the left half
                    hi = mid;
                }
            }
            nums[lo]
        }
    }
}

/// AI solution — same binary search (this is the canonical approach).
///
/// Time: O(log n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum using binary search with left boundary comparison.
        pub fn find_min(nums: Vec<i32>) -> i32 {
            let (mut lo, mut hi) = (0usize, nums.len() - 1);
            // If not rotated at all
            if nums[lo] <= nums[hi] {
                return nums[lo];
            }
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                // If mid > mid+1, then mid+1 is the minimum
                // clippy::manual_arithmetic_check: `mid + 1 <= hi` is equivalent
                // to `mid < hi` (for unsigned types). The simpler form avoids a
                // potential overflow if mid + 1 wrapped (though unlikely here).
                if mid < hi && nums[mid] > nums[mid + 1] {
                    return nums[mid + 1];
                }
                // If mid < mid-1, then mid is the minimum
                if mid > lo && nums[mid - 1] > nums[mid] {
                    return nums[mid];
                }
                if nums[mid] > nums[lo] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            nums[lo]
        }
    }
}

fn main() {
    let my = my_solution::Solution::find_min(vec![3, 4, 5, 1, 2]);
    let ai = ai_solution::Solution::find_min(vec![3, 4, 5, 1, 2]);
    println!("find min: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_find_min {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::find_min(vec![3, 4, 5, 1, 2]), 1,);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0,);
                }

                #[test]
                fn test_not_rotated() {
                    assert_eq!(<$solution>::find_min(vec![11, 13, 15, 17]), 11,);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::find_min(vec![1]), 1);
                }

                #[test]
                fn test_two_rotated() {
                    assert_eq!(<$solution>::find_min(vec![2, 1]), 1);
                }
            }
        };
    }

    test_find_min!(my, crate::my_solution::Solution);
    test_find_min!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let mut nums: Vec<i32> = (0..1000).collect();
        // Rotate by 300
        nums.rotate_left(300);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_min(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_min(
                std::hint::black_box(nums.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
