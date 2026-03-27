// LeetCode 11: Container With Most Water [Medium]
// https://leetcode.com/problems/container-with-most-water/
//
// You are given an integer array `height` of length n. There are n vertical
// lines drawn such that the two endpoints of the i-th line are (i, 0) and
// (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such that
// the container contains the most water. Return the maximum amount of water
// a container can store. You may not slant the container.
//
// Examples:
//   height = [1,8,6,2,5,4,8,3,7] -> 49
//   height = [1,1]                -> 1
//
// Constraints:
//   - n == height.length
//   - 2 <= n <= 10^5
//   - 0 <= height[i] <= 10^4
//
// Run:  cargo run --example lc_0011_container_with_most_water
// Test: cargo test --example lc_0011_container_with_most_water
// Bench: cargo test --example lc_0011_container_with_most_water bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find two lines forming a container with the most water.
        ///
        /// # Arguments
        ///
        /// * `height` - Array of line heights
        ///
        /// # Returns
        ///
        /// Maximum area of water the container can hold
        pub fn max_area(height: Vec<i32>) -> i32 {
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
        /// Find two lines forming a container with the most water.
        ///
        /// # Arguments
        ///
        /// * `height` - Array of line heights
        ///
        /// # Returns
        ///
        /// Maximum area of water the container can hold
        pub fn max_area(height: Vec<i32>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49),
        (vec![1, 1], 1),
    ];

    println!("--- My Solution ---");
    for (height, expected) in &test_cases {
        let result = my_solution::Solution::max_area(height.clone());
        println!("  {height:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (height, expected) in &test_cases {
        let result = ai_solution::Solution::max_area(height.clone());
        println!("  {height:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_area {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]),
                        49
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::max_area(vec![1, 1]), 1);
                }

                #[test]
                fn test_decreasing() {
                    assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::max_area(vec![5, 5, 5, 5]), 15);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::max_area(vec![3, 7]), 3);
                }
            }
        };
    }

    test_max_area!(my_tests, crate::my_solution::Solution);
    test_max_area!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
            vec![1, 1],
            vec![4, 3, 2, 1, 4],
            vec![5, 5, 5, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for h in &inputs {
                std::hint::black_box(crate::my_solution::Solution::max_area(
                    std::hint::black_box(h.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for h in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::max_area(
                    std::hint::black_box(h.clone()),
                ));
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
