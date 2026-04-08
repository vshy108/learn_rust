// LeetCode 162: Find Peak Element [Medium]
// https://leetcode.com/problems/find-peak-element/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.231us
//   AI solution avg: 0.124us
//   Ratio: AI 1.86x faster
//
// A peak element is strictly greater than its neighbors. Given a 0-indexed
// integer array nums, find a peak element and return its index. If the array
// contains multiple peaks, return the index to any of them.
// Must be O(log n) time. nums[-1] = nums[n] = -infinity.
//
// Constraints: 1 <= nums.length <= 1000, -2^31 <= nums[i] <= 2^31-1
//   nums[i] != nums[i+1] for all valid i
//
// Run:  cargo run --example lc_0162_find_peak_element
// Test: cargo test --example lc_0162_find_peak_element
// Bench: cargo test --example lc_0162_find_peak_element bench -- --nocapture

/// My solution — binary search: move toward the higher neighbor.
///
/// Time: O(log n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the index of any peak element.
        pub fn find_peak_element(nums: Vec<i32>) -> i32 {
            let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid as usize] < nums[(mid + 1) as usize] {
                    // Peak must be to the right (ascending slope)
                    lo = mid + 1;
                } else {
                    // Peak is at mid or to the left
                    hi = mid;
                }
            }
            lo
        }
    }
}

/// AI solution — same binary search (canonical O(log n) approach).
///
/// Time: O(log n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return peak index using unsigned binary search.
        pub fn find_peak_element(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n == 1 {
                return 0;
            }
            // Check edges first
            if nums[0] > nums[1] {
                return 0;
            }
            if nums[n - 1] > nums[n - 2] {
                return (n - 1) as i32;
            }

            let (mut lo, mut hi) = (1usize, n - 2);
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
                    return mid as i32;
                }
                if nums[mid] < nums[mid + 1] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            lo as i32
        }
    }
}

fn main() {
    let my = my_solution::Solution::find_peak_element(vec![1, 2, 3, 1]);
    let ai = ai_solution::Solution::find_peak_element(vec![1, 2, 3, 1]);
    println!("peak: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_peak {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                /// Verify the returned index is indeed a peak.
                fn is_peak(nums: &[i32], idx: i32) -> bool {
                    let i = idx as usize;
                    let left_ok = i == 0 || nums[i] > nums[i - 1];
                    let right_ok = i == nums.len() - 1 || nums[i] > nums[i + 1];
                    left_ok && right_ok
                }

                #[test]
                fn test_example1() {
                    let nums = vec![1, 2, 3, 1];
                    let idx = <$solution>::find_peak_element(nums.clone());
                    assert!(is_peak(&nums, idx));
                }

                #[test]
                fn test_example2() {
                    let nums = vec![1, 2, 1, 3, 5, 6, 4];
                    let idx = <$solution>::find_peak_element(nums.clone());
                    assert!(is_peak(&nums, idx));
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::find_peak_element(vec![1]), 0,);
                }

                #[test]
                fn test_ascending() {
                    let nums = vec![1, 2, 3, 4, 5];
                    let idx = <$solution>::find_peak_element(nums.clone());
                    assert!(is_peak(&nums, idx)); // last element is peak
                }

                #[test]
                fn test_descending() {
                    let nums = vec![5, 4, 3, 2, 1];
                    let idx = <$solution>::find_peak_element(nums.clone());
                    assert!(is_peak(&nums, idx)); // first element is peak
                }
            }
        };
    }

    test_peak!(my, crate::my_solution::Solution);
    test_peak!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..1000)
            .map(|i| if i < 500 { i } else { 1000 - i })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_peak_element(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_peak_element(
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
