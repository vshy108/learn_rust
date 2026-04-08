// LeetCode 280: Wiggle Sort [Medium] (Premium)
// https://leetcode.com/problems/wiggle-sort/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.121us
//   AI solution avg: 0.084us
//   Ratio: AI 1.44x faster
//
// Given an integer array nums, reorder it in-place such that:
//   nums[0] <= nums[1] >= nums[2] <= nums[3] ...
//
// Example 1: nums = [3,5,2,1,6,4] -> [3,5,1,6,2,4] (or other valid)
// Example 2: nums = [6,6,5,6,3,8] -> [6,6,5,6,3,8]
//
// Constraints:
//   - 1 <= nums.length <= 5 * 10^4
//   - 0 <= nums[i] <= 10^4
//   - Guaranteed valid answer exists
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0280.Wiggle%20Sort/README_EN.md
//
// Run:  cargo run --example lc_0280_wiggle_sort
// Test: cargo test --example lc_0280_wiggle_sort
// Bench: cargo test --example lc_0280_wiggle_sort bench -- --nocapture

/// Check whether an array satisfies the wiggle property:
///   nums[0] <= nums[1] >= nums[2] <= nums[3] ...
#[cfg(test)]
fn is_wiggle_sorted(nums: &[i32]) -> bool {
    for i in 1..nums.len() {
        if i % 2 == 1 && nums[i] < nums[i - 1] {
            return false;
        }
        if i % 2 == 0 && nums[i] > nums[i - 1] {
            return false;
        }
    }
    true
}

/// My solution — Single pass swap (from doocs).
///
/// Iterate from index 1. At odd indices we want a peak (nums[i] >= nums[i-1]);
/// at even indices we want a valley (nums[i] <= nums[i-1]). If the condition
/// is violated, swap adjacent elements. Each swap fixes the local property
/// without breaking previously established ordering.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn wiggle_sort(nums: &mut Vec<i32>) {
            for i in 1..nums.len() {
                // Odd index → peak: nums[i] should be >= nums[i-1]
                // Even index → valley: nums[i] should be <= nums[i-1]
                if (i % 2 == 1 && nums[i] < nums[i - 1]) || (i % 2 == 0 && nums[i] > nums[i - 1]) {
                    nums.swap(i, i - 1);
                }
            }
        }
    }
}

/// AI solution — Sort then swap adjacent pairs.
///
/// First sort the array. Then swap every pair of adjacent elements starting
/// from index 1: swap(1,2), swap(3,4), ... This creates peaks at odd indices.
///
/// Time:  O(n log n) due to sorting
/// Space: O(log n) for sort stack
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn wiggle_sort(nums: &mut Vec<i32>) {
            nums.sort_unstable();
            // Swap pairs: (1,2), (3,4), (5,6), ...
            // After sorting, swapping adjacent from index 1 creates
            // valley-peak-valley pattern
            let mut i = 1;
            while i + 1 < nums.len() {
                nums.swap(i, i + 1);
                i += 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_wiggle {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                let mut my_nums: Vec<i32> = $input.to_vec();
                my_solution::Solution::wiggle_sort(&mut my_nums);
                assert!(
                    is_wiggle_sorted(&my_nums),
                    "my_solution not wiggle: {:?}",
                    my_nums
                );
                // Verify same multiset of elements
                let mut sorted_orig: Vec<i32> = $input.to_vec();
                sorted_orig.sort_unstable();
                let mut sorted_my = my_nums.clone();
                sorted_my.sort_unstable();
                assert_eq!(sorted_my, sorted_orig, "my_solution changed elements");

                let mut ai_nums: Vec<i32> = $input.to_vec();
                ai_solution::Solution::wiggle_sort(&mut ai_nums);
                assert!(
                    is_wiggle_sorted(&ai_nums),
                    "ai_solution not wiggle: {:?}",
                    ai_nums
                );
                let mut sorted_ai = ai_nums;
                sorted_ai.sort_unstable();
                assert_eq!(sorted_ai, sorted_orig, "ai_solution changed elements");
            }
        };
    }

    // Example 1
    test_wiggle!(test_example_1, [3, 5, 2, 1, 6, 4]);
    // Example 2
    test_wiggle!(test_example_2, [6, 6, 5, 6, 3, 8]);
    // Single element
    test_wiggle!(test_single, [1]);
    // Two elements
    test_wiggle!(test_two, [2, 1]);
    // Already wiggle sorted
    test_wiggle!(test_already_wiggle, [1, 3, 2, 4, 1]);
    // Sorted ascending
    test_wiggle!(test_sorted_asc, [1, 2, 3, 4, 5]);
    // Sorted descending
    test_wiggle!(test_sorted_desc, [5, 4, 3, 2, 1]);
    // All same
    test_wiggle!(test_all_same, [3, 3, 3, 3]);
    // Two distinct values
    test_wiggle!(test_two_values, [1, 2, 1, 2, 1, 2]);
    // Large values
    test_wiggle!(test_large_values, [10000, 0, 10000, 0, 10000]);
    // Three elements
    test_wiggle!(test_three, [3, 1, 2]);
    // Duplicates
    test_wiggle!(test_duplicates, [1, 1, 2, 2, 3, 3]);
    // Already sorted with pairs
    test_wiggle!(test_pairs, [1, 1, 2, 2]);
}

fn main() {
    println!("LeetCode 280: Wiggle Sort");
    let mut my_nums = vec![3, 5, 2, 1, 6, 4];
    let mut ai_nums = my_nums.clone();
    my_solution::Solution::wiggle_sort(&mut my_nums);
    ai_solution::Solution::wiggle_sort(&mut ai_nums);
    println!("Input:  [3, 5, 2, 1, 6, 4]");
    println!("My:     {:?}", my_nums);
    println!("AI:     {:?}", ai_nums);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let base: Vec<i32> = (0..100).collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            let mut nums = base.clone();
            my_solution::Solution::wiggle_sort(&mut nums);
            let mut nums = base.clone();
            ai_solution::Solution::wiggle_sort(&mut nums);
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            std::hint::black_box(my_solution::Solution::wiggle_sort(&mut nums));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            std::hint::black_box(ai_solution::Solution::wiggle_sort(&mut nums));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
