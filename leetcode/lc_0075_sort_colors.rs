// LeetCode 75: Sort Colors [Medium]
// https://leetcode.com/problems/sort-colors/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.321us
//   AI solution avg: 0.561us
//   Ratio: My 1.74x faster
//
// Given an array with n objects colored red (0), white (1), or blue (2),
// sort them in-place so that same colors are adjacent (0s, 1s, 2s).
// Must solve without using the library sort function.
//
// Constraints: n == nums.length, 1 <= n <= 300, nums[i] is 0, 1, or 2.
//
// Run:  cargo run --example lc_0075_sort_colors
// Test: cargo test --example lc_0075_sort_colors
// Bench: cargo test --example lc_0075_sort_colors bench -- --nocapture

/// My solution — counting sort (two-pass).
///
/// Count occurrences, then overwrite array.
///
/// Time: O(n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Sort an array of 0s, 1s, 2s in-place.
        ///
        /// # Arguments
        /// * `nums` - mutable array of colors (0, 1, 2)
        pub fn sort_colors(nums: &mut Vec<i32>) {
            let mut counts = [0usize; 3];
            for &n in nums.iter() {
                counts[n as usize] += 1;
            }
            let mut idx = 0;
            for color in 0..3 {
                for _ in 0..counts[color] {
                    nums[idx] = color as i32;
                    idx += 1;
                }
            }
        }
    }
}

/// AI solution — Dutch National Flag algorithm (one-pass, three pointers).
///
/// Maintain three regions: [0..lo) = 0, [lo..mid) = 1, (hi..end] = 2.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Sort an array of 0s, 1s, 2s in-place using Dutch National Flag.
        ///
        /// # Arguments
        /// * `nums` - mutable array of colors (0, 1, 2)
        pub fn sort_colors(nums: &mut Vec<i32>) {
            if nums.is_empty() {
                return;
            }
            let mut lo = 0usize;
            let mut mid = 0usize;
            let mut hi = nums.len() - 1;

            while mid <= hi {
                match nums[mid] {
                    0 => {
                        nums.swap(lo, mid);
                        lo += 1;
                        mid += 1;
                    }
                    1 => {
                        mid += 1;
                    }
                    2 => {
                        nums.swap(mid, hi);
                        // hi could underflow if it's 0
                        if hi == 0 {
                            break;
                        }
                        hi -= 1;
                    }
                    _ => unreachable!("only 0, 1, 2 expected"),
                }
            }
        }
    }
}

fn main() {
    let mut v1 = vec![2, 0, 2, 1, 1, 0];
    let mut v2 = v1.clone();
    my_solution::Solution::sort_colors(&mut v1);
    ai_solution::Solution::sort_colors(&mut v2);
    println!("my: {v1:?}");
    println!("ai: {v2:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_sort_colors {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let mut nums = vec![2, 0, 2, 1, 1, 0];
                    <$solution>::sort_colors(&mut nums);
                    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
                }

                #[test]
                fn test_example2() {
                    let mut nums = vec![2, 0, 1];
                    <$solution>::sort_colors(&mut nums);
                    assert_eq!(nums, vec![0, 1, 2]);
                }

                #[test]
                fn test_all_same() {
                    let mut nums = vec![1, 1, 1];
                    <$solution>::sort_colors(&mut nums);
                    assert_eq!(nums, vec![1, 1, 1]);
                }

                #[test]
                fn test_already_sorted() {
                    let mut nums = vec![0, 0, 1, 2, 2];
                    <$solution>::sort_colors(&mut nums);
                    assert_eq!(nums, vec![0, 0, 1, 2, 2]);
                }

                #[test]
                fn test_reverse_sorted() {
                    let mut nums = vec![2, 2, 1, 0, 0];
                    <$solution>::sort_colors(&mut nums);
                    assert_eq!(nums, vec![0, 0, 1, 2, 2]);
                }

                #[test]
                fn test_single() {
                    let mut nums = vec![0];
                    <$solution>::sort_colors(&mut nums);
                    assert_eq!(nums, vec![0]);
                }
            }
        };
    }

    test_sort_colors!(my, crate::my_solution::Solution);
    test_sort_colors!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let input = vec![2, 0, 1, 2, 0, 1, 1, 0, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 2, 1];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut nums = input.clone();
            std::hint::black_box(crate::my_solution::Solution::sort_colors(
                std::hint::black_box(&mut nums),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut nums = input.clone();
            std::hint::black_box(crate::ai_solution::Solution::sort_colors(
                std::hint::black_box(&mut nums),
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
