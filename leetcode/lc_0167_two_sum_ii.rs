// LeetCode 167: Two Sum II - Input Array Is Sorted [Medium]
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 5.331us
//   AI solution avg: 1.065us
//   Ratio: AI 5.00x faster
//
// Given a 1-indexed sorted array numbers, find two numbers that add up to
// target. Return [index1, index2] (1-indexed). Exactly one solution exists.
// May not use the same element twice. Must use O(1) extra space.
//
// Constraints: 2 <= numbers.length <= 3*10^4, -1000 <= numbers[i] <= 1000
//   numbers is sorted in non-decreasing order.
//
// Run:  cargo run --example lc_0167_two_sum_ii
// Test: cargo test --example lc_0167_two_sum_ii
// Bench: cargo test --example lc_0167_two_sum_ii bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return 1-indexed pair of indices whose values sum to target.
        pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
            // Binary search for each element's complement — O(n log n)
            for i in 0..numbers.len() {
                let complement = target - numbers[i];
                // Binary search in numbers[i+1..]
                if let Ok(j) = numbers[i + 1..].binary_search(&complement) {
                    return vec![(i + 1) as i32, (i + 1 + j + 1) as i32];
                }
            }
            vec![]
        }
    }
}

/// AI solution — two-pointer approach on sorted array.
///
/// Left pointer at start, right pointer at end. If sum < target, move
/// left right; if sum > target, move right left.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Two-pointer approach; returns 1-indexed result.
        pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
            let (mut lo, mut hi) = (0usize, numbers.len() - 1);
            while lo < hi {
                let sum = numbers[lo] + numbers[hi];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        return vec![(lo + 1) as i32, (hi + 1) as i32];
                    }
                    std::cmp::Ordering::Less => lo += 1,
                    std::cmp::Ordering::Greater => hi -= 1,
                }
            }
            // Guaranteed to have a solution, so this is unreachable
            vec![]
        }
    }
}

fn main() {
    let my = my_solution::Solution::two_sum(vec![2, 7, 11, 15], 9);
    let ai = ai_solution::Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("two_sum: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_ts {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::two_sum(vec![2, 7, 11, 15], 9),
                        vec![1, 2],
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::two_sum(vec![2, 3, 4], 6),
                        vec![1, 3],
                    );
                }

                #[test]
                fn test_negatives() {
                    assert_eq!(
                        <$solution>::two_sum(vec![-1, 0], -1),
                        vec![1, 2],
                    );
                }

                #[test]
                fn test_larger() {
                    assert_eq!(
                        <$solution>::two_sum(vec![1, 2, 3, 4, 5, 6], 11),
                        vec![5, 6],
                    );
                }
            }
        };
    }

    test_ts!(my, crate::my_solution::Solution);
    test_ts!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (1..=1000).collect();
        let target = 1999; // last two

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::two_sum(
                std::hint::black_box(nums.clone()),
                std::hint::black_box(target),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::two_sum(
                std::hint::black_box(nums.clone()),
                std::hint::black_box(target),
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
