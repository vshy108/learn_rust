// LeetCode 42: Trapping Rain Water [Hard]
// https://leetcode.com/problems/trapping-rain-water/
//
// Given n non-negative integers representing an elevation map where the
// width of each bar is 1, compute how much water it can trap after raining.
//
// Examples:
//   height = [0,1,0,2,1,0,1,3,2,1,2,1] -> 6
//   height = [4,2,0,3,2,5]               -> 9
//
// Constraints:
//   - n == height.length
//   - 1 <= n <= 2 * 10^4
//   - 0 <= height[i] <= 10^5
//
// Run:  cargo run --example lc_0042_trapping_rain_water
// Test: cargo test --example lc_0042_trapping_rain_water
// Bench: cargo test --example lc_0042_trapping_rain_water bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute how much water can be trapped after raining.
        ///
        /// # Arguments
        ///
        /// * `height` - A vector of non-negative integers representing
        ///   the elevation map
        ///
        /// # Returns
        ///
        /// The total amount of trapped water
        pub fn trap(height: Vec<i32>) -> i32 {
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
        /// Compute how much water can be trapped after raining.
        ///
        /// # Arguments
        ///
        /// * `height` - A vector of non-negative integers representing
        ///   the elevation map
        ///
        /// # Returns
        ///
        /// The total amount of trapped water
        pub fn trap(height: Vec<i32>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
    ];

    println!("--- My Solution ---");
    for (height, expected) in &test_cases {
        let result = my_solution::Solution::trap(height.clone());
        println!("  height={height:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (height, expected) in &test_cases {
        let result = ai_solution::Solution::trap(height.clone());
        println!("  height={height:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_trap {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
                }

                #[test]
                fn test_no_trap_ascending() {
                    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5]), 0);
                }

                #[test]
                fn test_no_trap_descending() {
                    assert_eq!(Solution::trap(vec![5, 4, 3, 2, 1]), 0);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::trap(vec![5]), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::trap(vec![1, 2]), 0);
                }

                #[test]
                fn test_flat() {
                    assert_eq!(Solution::trap(vec![3, 3, 3, 3]), 0);
                }
            }
        };
    }

    test_trap!(my_tests, crate::my_solution::Solution);
    test_trap!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            vec![4, 2, 0, 3, 2, 5],
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 1, 2, 1, 4, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for height in &inputs {
                std::hint::black_box(crate::my_solution::Solution::trap(std::hint::black_box(
                    height.clone(),
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for height in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::trap(std::hint::black_box(
                    height.clone(),
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
