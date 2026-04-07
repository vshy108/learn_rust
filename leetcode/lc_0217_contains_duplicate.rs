// LeetCode 217: Contains Duplicate [Easy]
// https://leetcode.com/problems/contains-duplicate/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.493us
//   AI solution avg: 3.502us
//   Ratio: My 1.40x faster
//
// Given an integer array nums, return true if any value appears at
// least twice in the array, and return false if every element is distinct.
//
// Constraints: 1 <= nums.length <= 10^5, -10^9 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0217_contains_duplicate
// Test: cargo test --example lc_0217_contains_duplicate
// Bench: cargo test --example lc_0217_contains_duplicate bench -- --nocapture

/// My solution — sort the array, then check adjacent elements.
///
/// If any two adjacent elements are equal after sorting, there is a
/// duplicate. Uses in-place sort to avoid extra allocation.
///
/// Time: O(n log n).
/// Space: O(1) extra (in-place sort, ignoring sort internals).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if any value appears at least twice.
        pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
            nums.sort_unstable();
            // windows(2) yields overlapping pairs of adjacent elements
            nums.windows(2).any(|w| w[0] == w[1])
        }
    }
}

/// AI solution — insert elements into a HashSet; if an insert fails,
/// the element was already seen.
///
/// Time: O(n) average.
/// Space: O(n) for the HashSet.
mod ai_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Return true if any value appears at least twice.
        pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            let mut seen = HashSet::with_capacity(nums.len());
            // HashSet::insert returns false when the value already exists
            nums.into_iter().any(|num| !seen.insert(num))
        }
    }
}

fn main() {
    let cases: &[(&[i32], bool)] = &[
        (&[1, 2, 3, 1], true),
        (&[1, 2, 3, 4], false),
        (&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
    ];
    for (nums, expected) in cases {
        let nums_vec = nums.to_vec();
        let my = my_solution::Solution::contains_duplicate(nums_vec.clone());
        let ai = ai_solution::Solution::contains_duplicate(nums_vec);
        println!("{nums:?} => my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_contains_dup {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert!(<$solution>::contains_duplicate(vec![1, 2, 3, 1]));
                }

                #[test]
                fn test_example2() {
                    assert!(!<$solution>::contains_duplicate(vec![1, 2, 3, 4]));
                }

                #[test]
                fn test_example3() {
                    assert!(<$solution>::contains_duplicate(
                        vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
                    ));
                }

                #[test]
                fn test_single() {
                    assert!(!<$solution>::contains_duplicate(vec![42]));
                }

                #[test]
                fn test_two_same() {
                    assert!(<$solution>::contains_duplicate(vec![5, 5]));
                }

                #[test]
                fn test_two_different() {
                    assert!(!<$solution>::contains_duplicate(vec![1, 2]));
                }

                #[test]
                fn test_negatives() {
                    assert!(<$solution>::contains_duplicate(vec![-1, -2, -1]));
                }

                #[test]
                fn test_large_range_no_dup() {
                    let nums: Vec<i32> = (0..1000).collect();
                    assert!(!<$solution>::contains_duplicate(nums));
                }
            }
        };
    }

    test_contains_dup!(my, crate::my_solution::Solution);
    test_contains_dup!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a deterministic array with a duplicate near the end
        let mut nums: Vec<i32> = (0..500).collect();
        nums.push(0); // duplicate

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::contains_duplicate(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::contains_duplicate(
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
