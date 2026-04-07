// LeetCode 152: Maximum Product Subarray [Medium]
// https://leetcode.com/problems/maximum-product-subarray/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.451us
//   AI solution avg: 0.302us
//   Ratio: AI 1.49x faster
//
// Given an integer array nums, find a subarray that has the largest product.
//
// Constraints: 1 <= nums.length <= 2*10^4, -10 <= nums[i] <= 10
//   The product of any subarray fits in a 32-bit integer
//
// Run:  cargo run --example lc_0152_max_product_subarray
// Test: cargo test --example lc_0152_max_product_subarray
// Bench: cargo test --example lc_0152_max_product_subarray bench -- --nocapture

/// My solution — track current min/max products (negatives can flip).
///
/// Time: O(n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum product of a subarray.
        pub fn max_product(nums: Vec<i32>) -> i32 {
            let mut global_max = nums[0];
            let mut cur_max = nums[0];
            let mut cur_min = nums[0];

            for &num in &nums[1..] {
                // A negative number swaps max and min
                if num < 0 {
                    std::mem::swap(&mut cur_max, &mut cur_min);
                }
                cur_max = num.max(cur_max * num);
                cur_min = num.min(cur_min * num);
                global_max = global_max.max(cur_max);
            }
            global_max
        }
    }
}

/// AI solution — same approach, expressed differently with tuple update.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum product subarray using tuple state.
        pub fn max_product(nums: Vec<i32>) -> i32 {
            let mut best = nums[0];
            // (max_ending_here, min_ending_here)
            let mut state = (nums[0], nums[0]);

            for &num in &nums[1..] {
                let candidates = [num, state.0 * num, state.1 * num];
                state = (
                    *candidates.iter().max().expect("non-empty"),
                    *candidates.iter().min().expect("non-empty"),
                );
                best = best.max(state.0);
            }
            best
        }
    }
}

fn main() {
    let my = my_solution::Solution::max_product(vec![2, 3, -2, 4]);
    let ai = ai_solution::Solution::max_product(vec![2, 3, -2, 4]);
    println!("max product: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_product {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::max_product(vec![2, 3, -2, 4]),
                        6,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::max_product(vec![-2, 0, -1]),
                        0,
                    );
                }

                #[test]
                fn test_single_negative() {
                    assert_eq!(<$solution>::max_product(vec![-2]), -2);
                }

                #[test]
                fn test_two_negatives() {
                    assert_eq!(
                        <$solution>::max_product(vec![-2, -3]),
                        6,
                    );
                }

                #[test]
                fn test_zeros_and_negatives() {
                    assert_eq!(
                        <$solution>::max_product(vec![-2, 0, -3, 4]),
                        4,
                    );
                }

                #[test]
                fn test_all_positive() {
                    assert_eq!(
                        <$solution>::max_product(vec![1, 2, 3, 4]),
                        24,
                    );
                }
            }
        };
    }

    test_max_product!(my, crate::my_solution::Solution);
    test_max_product!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..100)
            .map(|i| match i % 7 {
                0 => 0,  // zeros break product chains, avoiding overflow
                3 => -2,
                _ => 2,
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::max_product(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::max_product(
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
