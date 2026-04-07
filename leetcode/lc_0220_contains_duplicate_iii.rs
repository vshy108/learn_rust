// LeetCode 220: Contains Duplicate III [Hard]
// https://leetcode.com/problems/contains-duplicate-iii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 3.702us
//   AI solution avg: 7.000us
//   Ratio: My 1.89x faster
//
// Given an integer array nums and two integers indexDiff and valueDiff,
// find a pair of indices (i, j) such that:
//   i != j, abs(i - j) <= indexDiff, abs(nums[i] - nums[j]) <= valueDiff
// Return true if such a pair exists, otherwise false.
//
// Constraints: 2 <= nums.length <= 10^5,
//              -10^9 <= nums[i] <= 10^9,
//              1 <= indexDiff <= nums.length,
//              0 <= valueDiff <= 10^9
//
// Run:  cargo run --example lc_0220_contains_duplicate_iii
// Test: cargo test --example lc_0220_contains_duplicate_iii
// Bench: cargo test --example lc_0220_contains_duplicate_iii bench -- --nocapture

/// My solution — BTreeSet sliding window.
///
/// Maintain a BTreeSet of (at most) indexDiff elements. For each new
/// element, query the BTreeSet for any value in [nums[i] - valueDiff,
/// nums[i] + valueDiff] using range. If found, return true. Otherwise
/// insert nums[i] and evict the oldest element when the window exceeds
/// indexDiff.
///
/// Time: O(n log(min(n, indexDiff))).
/// Space: O(min(n, indexDiff)).
mod my_solution {
    use std::collections::BTreeSet;

    pub struct Solution;

    impl Solution {
        /// Return true if a nearby almost-duplicate exists.
        pub fn contains_nearby_almost_duplicate(
            nums: Vec<i32>,
            index_diff: i32,
            value_diff: i32,
        ) -> bool {
            let index_diff = index_diff as usize;
            // Use i64 to avoid overflow when computing nums[i] +/- valueDiff
            let value_diff = value_diff as i64;
            let mut window: BTreeSet<i64> = BTreeSet::new();

            for i in 0..nums.len() {
                let val = nums[i] as i64;

                // Check if any value in the window is within [val - valueDiff, val + valueDiff]
                let lo = val - value_diff;
                let hi = val + value_diff;
                // range returns an iterator over values in [lo, hi]
                if window.range(lo..=hi).next().is_some() {
                    return true;
                }

                window.insert(val);

                // Evict the element that just fell out of the index window
                if i >= index_diff {
                    window.remove(&(nums[i - index_diff] as i64));
                }
            }
            false
        }
    }
}

/// AI solution — bucket sort approach.
///
/// Divide values into buckets of size (valueDiff + 1). Elements in the
/// same bucket are guaranteed to be within valueDiff of each other.
/// Elements in adjacent buckets might be. Maintain a HashMap of buckets
/// within the sliding index window.
///
/// Time: O(n).
/// Space: O(min(n, indexDiff)).
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return true if a nearby almost-duplicate exists.
        pub fn contains_nearby_almost_duplicate(
            nums: Vec<i32>,
            index_diff: i32,
            value_diff: i32,
        ) -> bool {
            let index_diff = index_diff as usize;
            let w = value_diff as i64 + 1; // bucket width

            // Map from bucket_id -> value currently in that bucket
            let mut buckets: HashMap<i64, i64> = HashMap::new();

            for i in 0..nums.len() {
                let val = nums[i] as i64;
                let bucket_id = Self::get_bucket(val, w);

                // Same bucket => guaranteed within valueDiff
                if buckets.contains_key(&bucket_id) {
                    return true;
                }

                // Adjacent bucket (left) — need to verify distance
                if let Some(&neighbor) = buckets.get(&(bucket_id - 1)) {
                    if (val - neighbor).abs() <= (w - 1) {
                        return true;
                    }
                }

                // Adjacent bucket (right) — need to verify distance
                if let Some(&neighbor) = buckets.get(&(bucket_id + 1)) {
                    if (val - neighbor).abs() <= (w - 1) {
                        return true;
                    }
                }

                buckets.insert(bucket_id, val);

                // Evict the element that fell out of the index window
                if i >= index_diff {
                    let old_val = nums[i - index_diff] as i64;
                    let old_bucket = Self::get_bucket(old_val, w);
                    buckets.remove(&old_bucket);
                }
            }
            false
        }

        /// Map a value to its bucket ID.
        ///
        /// For non-negative values: bucket = val / w.
        /// For negative values: shift to align negative numbers into
        /// correct buckets (e.g., -1 should go to bucket -1, not 0).
        fn get_bucket(val: i64, w: i64) -> i64 {
            if val >= 0 {
                val / w
            } else {
                // (val + 1) / w - 1 correctly maps negative values.
                // e.g., val=-1, w=3: (-1+1)/3 - 1 = -1 (bucket -1)
                //        val=-3, w=3: (-3+1)/3 - 1 = -1 - 1 = -2 (bucket -2) -- wrong
                // Actually use: (val + 1) / w - 1
                // val=-3, w=3: (-3+1)/3 = -2/3 = 0 (Rust truncates toward zero) => 0 - 1 = -1
                // val=-4, w=3: (-4+1)/3 = -3/3 = -1 => -1 - 1 = -2
                // This correctly groups -3,-2,-1 into bucket -1 and -6,-5,-4 into bucket -2
                (val + 1) / w - 1
            }
        }
    }
}

fn main() {
    let cases: &[(Vec<i32>, i32, i32, bool)] = &[
        (vec![1, 2, 3, 1], 3, 0, true),
        (vec![1, 5, 9, 1, 5, 9], 2, 3, false),
        (vec![1, 0, 1, 1], 1, 2, true),
    ];
    for (nums, index_diff, value_diff, expected) in cases {
        let my = my_solution::Solution::contains_nearby_almost_duplicate(
            nums.clone(),
            *index_diff,
            *value_diff,
        );
        let ai = ai_solution::Solution::contains_nearby_almost_duplicate(
            nums.clone(),
            *index_diff,
            *value_diff,
        );
        println!(
            "{nums:?}, indexDiff={index_diff}, valueDiff={value_diff} => \
             my={my}, ai={ai} (expected {expected})"
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_dup3 {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert!(<$solution>::contains_nearby_almost_duplicate(
                        vec![1, 2, 3, 1],
                        3,
                        0
                    ));
                }

                #[test]
                fn test_example2() {
                    assert!(!<$solution>::contains_nearby_almost_duplicate(
                        vec![1, 5, 9, 1, 5, 9],
                        2,
                        3
                    ));
                }

                #[test]
                fn test_example3() {
                    assert!(<$solution>::contains_nearby_almost_duplicate(
                        vec![1, 0, 1, 1],
                        1,
                        2
                    ));
                }

                #[test]
                fn test_negative_values() {
                    // -1 and -1 are within indexDiff=1 and valueDiff=0
                    assert!(<$solution>::contains_nearby_almost_duplicate(
                        vec![-1, -1],
                        1,
                        0
                    ));
                }

                #[test]
                fn test_large_value_diff() {
                    // Any two elements are within valueDiff=10^9 of each other
                    assert!(<$solution>::contains_nearby_almost_duplicate(
                        vec![1, 2],
                        1,
                        1_000_000_000
                    ));
                }

                #[test]
                fn test_no_match_index() {
                    // Elements differ by 0, but indexDiff=1, and they are 3 apart
                    assert!(!<$solution>::contains_nearby_almost_duplicate(
                        vec![1, 2, 3, 1],
                        1,
                        0
                    ));
                }

                #[test]
                fn test_value_diff_zero() {
                    // Must find exact duplicate within indexDiff
                    assert!(!<$solution>::contains_nearby_almost_duplicate(
                        vec![1, 2, 1],
                        1,
                        0
                    ));
                }

                #[test]
                fn test_mixed_signs() {
                    // -3 and -2 are adjacent, differ by 1 <= valueDiff=1
                    assert!(<$solution>::contains_nearby_almost_duplicate(
                        vec![-3, -2, 5, 8],
                        1,
                        1
                    ));
                }
            }
        };
    }

    test_dup3!(my, crate::my_solution::Solution);
    test_dup3!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Deterministic array — no duplicates within window to force full scan
        let nums: Vec<i32> = (0..200).map(|i| i * 100).collect();
        let index_diff = 5;
        let value_diff = 10;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::contains_nearby_almost_duplicate(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(index_diff),
                    std::hint::black_box(value_diff),
                ),
            );
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::contains_nearby_almost_duplicate(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(index_diff),
                    std::hint::black_box(value_diff),
                ),
            );
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
