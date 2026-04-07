// LeetCode 154: Find Minimum in Rotated Sorted Array II [Hard]
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.252us
//   AI solution avg: 0.333us
//   Ratio: AI 3.75x faster
//
// Same as 153, but the array may contain duplicates.
// Return the minimum element.
//
// Constraints: n == nums.length, 1 <= n <= 5000, -5000 <= nums[i] <= 5000
//   nums may contain duplicates, was sorted ascending then rotated
//
// Run:  cargo run --example lc_0154_find_min_rotated_ii
// Test: cargo test --example lc_0154_find_min_rotated_ii
// Bench: cargo test --example lc_0154_find_min_rotated_ii bench -- --nocapture

/// My solution — binary search with duplicate handling (shrink hi when equal).
///
/// Time: O(log n) average, O(n) worst case when all elements are equal.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the minimum in a rotated sorted array with duplicates.
        pub fn find_min(nums: Vec<i32>) -> i32 {
            let (mut lo, mut hi) = (0, nums.len() - 1);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] > nums[hi] {
                    lo = mid + 1;
                } else if nums[mid] < nums[hi] {
                    hi = mid;
                } else {
                    // nums[mid] == nums[hi] — can't determine which half,
                    // so shrink by one (safe since mid < hi)
                    hi -= 1;
                }
            }
            nums[lo]
        }
    }
}

/// AI solution — same approach (canonical for this problem).
///
/// Time: O(log n) average, O(n) worst case.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum — handles duplicates by decrementing hi.
        pub fn find_min(nums: Vec<i32>) -> i32 {
            let (mut lo, mut hi) = (0usize, nums.len() - 1);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                match nums[mid].cmp(&nums[hi]) {
                    std::cmp::Ordering::Greater => lo = mid + 1,
                    std::cmp::Ordering::Less => hi = mid,
                    std::cmp::Ordering::Equal => hi -= 1,
                }
            }
            nums[lo]
        }
    }
}

fn main() {
    let my = my_solution::Solution::find_min(vec![2, 2, 2, 0, 1]);
    let ai = ai_solution::Solution::find_min(vec![2, 2, 2, 0, 1]);
    println!("find min II: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_find_min {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::find_min(vec![1, 3, 5]),
                        1,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::find_min(vec![2, 2, 2, 0, 1]),
                        0,
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        <$solution>::find_min(vec![3, 3, 3, 3]),
                        3,
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::find_min(vec![1]), 1);
                }

                #[test]
                fn test_duplicates_at_boundary() {
                    assert_eq!(
                        <$solution>::find_min(vec![3, 3, 1, 3]),
                        1,
                    );
                }

                #[test]
                fn test_not_rotated() {
                    assert_eq!(
                        <$solution>::find_min(vec![1, 1, 2, 2, 3]),
                        1,
                    );
                }
            }
        };
    }

    test_find_min!(my, crate::my_solution::Solution);
    test_find_min!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let mut nums = vec![1; 500];
        nums.extend(vec![2; 500]);
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
