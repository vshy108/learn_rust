// LeetCode 283: Move Zeroes [Easy]
// https://leetcode.com/problems/move-zeroes/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.102us
//   AI solution avg: 0.101us
//   Ratio: AI 1.02x faster
//
// Given an integer array nums, move all 0's to the end while maintaining
// the relative order of the non-zero elements. Do this in-place.
//
// Example 1: nums = [0,1,0,3,12] -> [1,3,12,0,0]
// Example 2: nums = [0]          -> [0]
//
// Constraints:
//   - 1 <= nums.length <= 10^4
//   - -2^31 <= nums[i] <= 2^31 - 1
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0283.Move%20Zeroes/README_EN.md
//
// Run:  cargo run --example lc_0283_move_zeroes
// Test: cargo test --example lc_0283_move_zeroes
// Bench: cargo test --example lc_0283_move_zeroes bench -- --nocapture

/// My solution — Two-pointer swap (from doocs Rust).
///
/// Pointer `k` tracks the insertion position for non-zero elements.
/// Iterate through array; whenever a non-zero is found, swap it to
/// position k and increment k. All elements before k are non-zero
/// in their original relative order.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut k = 0;
            for i in 0..nums.len() {
                if nums[i] != 0 {
                    nums.swap(i, k);
                    k += 1;
                }
            }
        }
    }
}

/// AI solution — Two-pass: copy non-zeros then fill zeros.
///
/// First pass writes all non-zero values to the front using a write
/// pointer. Second pass fills the remaining positions with zeros.
/// Avoids swaps — uses writes only, which can be faster when many
/// zeros are present (fewer total writes than swap-based approach).
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut k = 0;
            // Pass 1: copy all non-zero elements to the front
            for i in 0..nums.len() {
                if nums[i] != 0 {
                    nums[k] = nums[i];
                    k += 1;
                }
            }
            // Pass 2: fill remaining positions with zero
            for j in k..nums.len() {
                nums[j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_move_zeroes {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut my_nums = $input.to_vec();
                my_solution::Solution::move_zeroes(&mut my_nums);
                assert_eq!(my_nums, $expected.to_vec(), "my_solution failed");

                let mut ai_nums = $input.to_vec();
                ai_solution::Solution::move_zeroes(&mut ai_nums);
                assert_eq!(ai_nums, $expected.to_vec(), "ai_solution failed");
            }
        };
    }

    // Example 1
    test_move_zeroes!(test_example_1, [0, 1, 0, 3, 12], [1, 3, 12, 0, 0]);
    // Example 2
    test_move_zeroes!(test_example_2, [0], [0]);
    // No zeros
    test_move_zeroes!(test_no_zeros, [1, 2, 3], [1, 2, 3]);
    // All zeros
    test_move_zeroes!(test_all_zeros, [0, 0, 0], [0, 0, 0]);
    // Single non-zero
    test_move_zeroes!(test_single_nonzero, [1], [1]);
    // Zeros at front
    test_move_zeroes!(test_zeros_front, [0, 0, 1], [1, 0, 0]);
    // Zeros at end (already correct)
    test_move_zeroes!(test_zeros_end, [1, 2, 0], [1, 2, 0]);
    // Alternating
    test_move_zeroes!(test_alternating, [0, 1, 0, 2, 0, 3], [1, 2, 3, 0, 0, 0]);
    // Negative numbers
    test_move_zeroes!(test_negatives, [0, -1, 0, 3, 0], [-1, 3, 0, 0, 0]);
    // Two elements
    test_move_zeroes!(test_two_elem, [0, 1], [1, 0]);
    // Large single zero
    test_move_zeroes!(
        test_many_elements,
        [4, 0, 5, 0, 0, 6, 7],
        [4, 5, 6, 7, 0, 0, 0]
    );
}

fn main() {
    println!("LeetCode 283: Move Zeroes");
    let mut my_nums = vec![0, 1, 0, 3, 12];
    let mut ai_nums = my_nums.clone();
    my_solution::Solution::move_zeroes(&mut my_nums);
    ai_solution::Solution::move_zeroes(&mut ai_nums);
    println!("Input:  [0, 1, 0, 3, 12]");
    println!("My:     {:?}", my_nums);
    println!("AI:     {:?}", ai_nums);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // Mix of zeros and non-zeros
        let base: Vec<i32> = (0..100).map(|i| if i % 3 == 0 { 0 } else { i }).collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            let mut nums = base.clone();
            my_solution::Solution::move_zeroes(&mut nums);
            let mut nums = base.clone();
            ai_solution::Solution::move_zeroes(&mut nums);
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            std::hint::black_box(my_solution::Solution::move_zeroes(&mut nums));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            std::hint::black_box(ai_solution::Solution::move_zeroes(&mut nums));
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
