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

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
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
            todo!()
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
