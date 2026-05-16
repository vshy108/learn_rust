// LeetCode 164: Maximum Gap [Hard]
// https://leetcode.com/problems/maximum-gap/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.264us
//   AI solution avg: 1.407us
//   Ratio: AI 1.61x faster
//
// Given an integer array nums, return the maximum difference between two
// successive elements in its sorted form. If the array contains fewer than
// two elements, return 0.
//
// Must run in O(n) time and O(n) space.
//
// Constraints: 1 <= nums.length <= 10^5, 0 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0164_maximum_gap
// Test: cargo test --example lc_0164_maximum_gap
// Bench: cargo test --example lc_0164_maximum_gap bench -- --nocapture

/// My solution — bucket / pigeonhole gap scan.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum gap between successive elements in sorted order.
        pub fn maximum_gap(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n < 2 {
                return 0;
            }

            let min_value = *nums.iter().min().expect("non-empty");
            let max_value = *nums.iter().max().expect("non-empty");
            if min_value == max_value {
                return 0;
            }

            let range = (max_value - min_value) as usize;
            let bucket_size = range.div_ceil(n - 1).max(1);
            let bucket_count = range / bucket_size + 1;
            let mut buckets: Vec<Option<(i32, i32)>> = vec![None; bucket_count];

            for num in nums {
                let idx = (num - min_value) as usize / bucket_size;
                buckets[idx] = Some(match buckets[idx] {
                    Some((low, high)) => (low.min(num), high.max(num)),
                    None => (num, num),
                });
            }

            let mut best_gap = 0;
            let mut previous_max = min_value;
            for bucket in buckets.into_iter().flatten() {
                let (bucket_min, bucket_max) = bucket;
                // FIX: Sorting passes the sample cases but violates the required O(n) time.
                // The pigeonhole bucket scan only compares gaps between non-empty buckets,
                // where the maximum sorted-adjacent gap must occur.
                best_gap = best_gap.max(bucket_min - previous_max);
                previous_max = bucket_max;
            }

            best_gap
        }
    }
}

/// AI solution — bucket / pigeonhole sort approach.
///
/// Place values into buckets of size = ceil((max-min)/(n-1)).
/// The max gap must span across buckets (not within), so we only
/// track each bucket's min and max.
///
/// Time: O(n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum gap using bucket sort.
        pub fn maximum_gap(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n < 2 {
                return 0;
            }

            let min_val = *nums.iter().min().expect("non-empty");
            let max_val = *nums.iter().max().expect("non-empty");
            if min_val == max_val {
                return 0;
            }

            // Bucket size: ensure at least 1
            let bucket_size = ((max_val - min_val) as usize / (n - 1)).max(1);
            let bucket_count = (max_val - min_val) as usize / bucket_size + 1;

            // Each bucket stores (min, max); None means empty
            let mut buckets: Vec<Option<(i32, i32)>> = vec![None; bucket_count];

            for &num in &nums {
                let idx = (num - min_val) as usize / bucket_size;
                buckets[idx] = Some(match buckets[idx] {
                    Some((lo, hi)) => (lo.min(num), hi.max(num)),
                    None => (num, num),
                });
            }

            // Walk non-empty buckets; gap = current_min - previous_max
            let mut max_gap = 0;
            let mut prev_max = min_val;
            // clippy::manual_flatten: `for item in iter { if let Some(x) = item { ... } }`
            // is exactly what .flatten() does — it skips None and unwraps Some.
            for (lo, hi) in buckets.iter().flatten() {
                max_gap = max_gap.max(lo - prev_max);
                prev_max = *hi;
            }
            max_gap
        }
    }
}

fn main() {
    let nums = vec![3, 6, 9, 1];
    let my = my_solution::Solution::maximum_gap(nums.clone());
    let ai = ai_solution::Solution::maximum_gap(nums);
    println!("max gap: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_gap {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::maximum_gap(vec![3, 6, 9, 1]), 3);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::maximum_gap(vec![10]), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(<$solution>::maximum_gap(vec![1, 100]), 99);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(<$solution>::maximum_gap(vec![5, 5, 5, 5]), 0);
                }

                #[test]
                fn test_sorted() {
                    assert_eq!(<$solution>::maximum_gap(vec![1, 3, 6, 10, 15]), 5);
                }
            }
        };
    }

    test_gap!(my, crate::my_solution::Solution);
    test_gap!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..500).map(|i| i * 3 + (i % 7)).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::maximum_gap(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::maximum_gap(
                std::hint::black_box(nums.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
