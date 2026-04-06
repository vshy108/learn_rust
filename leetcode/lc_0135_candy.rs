// LeetCode 135: Candy [Hard]
// https://leetcode.com/problems/candy/
//
// There are n children standing in a line. Each child is assigned a rating
// value given in the integer array ratings. You are giving candies to these
// children subjected to the following requirements:
//   - Each child must have at least one candy.
//   - Children with a higher rating than their neighbors get more candies
//     than their neighbors.
//
// Return the minimum number of candies you need to distribute.
//
// Examples:
//   ratings = [1,0,2] -> 5  (candies: [2,1,2])
//   ratings = [1,2,2] -> 4  (candies: [1,2,1])
//
// Constraints:
//   - n == ratings.length
//   - 1 <= n <= 2 * 10^4
//   - 0 <= ratings[i] <= 2 * 10^4
//
// Run:  cargo run --example lc_0135_candy
// Test: cargo test --example lc_0135_candy
// Bench: cargo test --example lc_0135_candy bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-04-03) ---
// My solution: avg 1.307us
// AI solution: avg 520ns
// Ratio: AI solution is 2.51x faster
// Reason: AI uses O(1) space with slope-counting (no array allocation or
//         iteration for sum), avoiding the two-pass + sum overhead of my_solution.
//
// --- Rust concepts used ---
// - saturating_sub(): prevents usize underflow (unsigned can't go negative)
// - Slope-counting greedy: single-pass O(1) space alternative to two-pass array
//   (Previously covered: vec![val; n] in lc_0006, (0..n).rev() in lc_0042,
//    .max() on i32 in lc_0003, into_iter/sum in lc_0134/lc_0005)

/// My solution module.
///
/// Two-pass greedy: left-to-right ensures right-higher neighbors get more,
/// right-to-left ensures left-higher neighbors get more, taking the max.
///
/// Time:  O(n) - two linear passes + one sum pass
/// Space: O(n) - auxiliary candies array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the minimum candies needed to satisfy the rating constraints.
        ///
        /// # Arguments
        ///
        /// * `ratings` - Rating values for each child
        ///
        /// # Returns
        ///
        /// Minimum total number of candies
        pub fn candy(ratings: Vec<i32>) -> i32 {
            let n = ratings.len();
            if n == 0 {
                return 0;
            };
            // Vec, not array: n is only known at runtime; arrays require compile-time size
            let mut candies = vec![1; n];
            for i in 1..n {
                if ratings[i] > ratings[i - 1] {
                    candies[i] = candies[i - 1] + 1;
                }
            }
            // Rust ranges only go forward; use .rev() to iterate backwards.
            // n.saturating_sub(2) prevents usize underflow when n <= 1
            for i in (0..n.saturating_sub(1)).rev() {
                if ratings[i] > ratings[i + 1] {
                    candies[i] = candies[i].max(candies[i + 1] + 1);
                }
            }

            candies.into_iter().sum()
        }
    }
}

/// AI solution module.
///
/// Single-pass O(1) space approach. Tracks ascending and descending slopes.
/// When a descending slope ends, retroactively adjusts the peak if needed.
/// No auxiliary array required.
///
/// Time:  O(n) - single pass
/// Space: O(1) - only counter variables
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the minimum candies needed to satisfy the rating constraints.
        ///
        /// Uses slope-counting: tracks consecutive ascending (`up`) and
        /// descending (`down`) runs. At each slope change, the accumulated
        /// candy count for that mountain shape is added to the total.
        ///
        /// # Arguments
        ///
        /// * `ratings` - Rating values for each child
        ///
        /// # Returns
        ///
        /// Minimum total number of candies
        pub fn candy(ratings: Vec<i32>) -> i32 {
            let n = ratings.len();
            if n <= 1 {
                return n as i32;
            }

            let mut total = 1_i32; // first child always gets 1 candy
            let mut up = 0_i32; // length of current ascending slope
            let mut down = 0_i32; // length of current descending slope
            let mut peak = 0_i32; // height at the last peak

            for i in 1..n {
                if ratings[i] > ratings[i - 1] {
                    // Ascending: extend the upward slope
                    up += 1;
                    down = 0;
                    peak = up;
                    // This child gets (up + 1) candies, but we add
                    // incrementally: +1 more than previous
                    total += up + 1;
                } else if ratings[i] < ratings[i - 1] {
                    // Descending: extend the downward slope
                    down += 1;
                    up = 0;
                    // If descending slope reaches the peak height,
                    // the peak itself must be raised by 1
                    let peak_bump = if down > peak { 1 } else { 0 };
                    total += down + peak_bump;
                } else {
                    // Equal: reset both slopes, child gets 1 candy
                    up = 0;
                    down = 0;
                    peak = 0;
                    total += 1;
                }
            }

            total
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![1, 0, 2], 5), (vec![1, 2, 2], 4)];

    println!("--- My Solution ---");
    for (ratings, expected) in &test_cases {
        let result = my_solution::Solution::candy(ratings.clone());
        println!("  {ratings:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (ratings, expected) in &test_cases {
        let result = ai_solution::Solution::candy(ratings.clone());
        println!("  {ratings:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_candy {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_valley() {
                    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
                }

                #[test]
                fn test_equal_neighbors() {
                    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
                }

                #[test]
                fn test_single_child() {
                    assert_eq!(Solution::candy(vec![0]), 1);
                }

                #[test]
                fn test_ascending() {
                    // candies: [1, 2, 3]
                    assert_eq!(Solution::candy(vec![1, 2, 3]), 6);
                }

                #[test]
                fn test_descending() {
                    // candies: [3, 2, 1]
                    assert_eq!(Solution::candy(vec![3, 2, 1]), 6);
                }

                #[test]
                fn test_all_equal() {
                    // Each gets 1 candy (equal neighbors don't require more)
                    assert_eq!(Solution::candy(vec![1, 1, 1, 1]), 4);
                }

                #[test]
                fn test_mountain() {
                    // candies: [1, 2, 3, 2, 1]
                    assert_eq!(Solution::candy(vec![1, 3, 5, 3, 1]), 9);
                }
            }
        };
    }

    test_candy!(my_tests, crate::my_solution::Solution);
    test_candy!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 0, 2],
            vec![1, 2, 2],
            vec![1, 3, 5, 3, 1],
            vec![1, 2, 3, 4, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for ratings in &inputs {
                std::hint::black_box(crate::my_solution::Solution::candy(std::hint::black_box(
                    ratings.clone(),
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for ratings in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::candy(std::hint::black_box(
                    ratings.clone(),
                )));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
