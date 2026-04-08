// LeetCode 136: Single Number [Easy]
// https://leetcode.com/problems/single-number/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.155us
//   AI solution avg: 0.134us
//   Ratio: AI 1.15x faster
//
// Given a non-empty array of integers nums, every element appears twice except
// for one. Find that single one. Must have O(n) time and O(1) extra space.
//
// Constraints: 1 <= nums.length <= 3*10^4, -3*10^4 <= nums[i] <= 3*10^4
//
// Run:  cargo run --example lc_0136_single_number
// Test: cargo test --example lc_0136_single_number
// Bench: cargo test --example lc_0136_single_number bench -- --nocapture

/// My solution — XOR all elements (a ^ a = 0, a ^ 0 = a).
///
/// Time: O(n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the element that appears only once.
        pub fn single_number(nums: Vec<i32>) -> i32 {
            // XOR cancels out duplicate pairs, leaving only the unique value
            nums.into_iter().fold(0, |acc, x| acc ^ x)
        }
    }
}

/// AI solution — same XOR approach (this is the canonical O(1) space solution).
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the element that appears only once using reduce.
        pub fn single_number(nums: Vec<i32>) -> i32 {
            nums.into_iter()
                .reduce(|a, b| a ^ b)
                .expect("nums is non-empty")
        }
    }
}

fn main() {
    let my = my_solution::Solution::single_number(vec![4, 1, 2, 1, 2]);
    let ai = ai_solution::Solution::single_number(vec![4, 1, 2, 1, 2]);
    println!("single number: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_single {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::single_number(vec![2, 2, 1]), 1);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::single_number(vec![4, 1, 2, 1, 2]), 4);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(<$solution>::single_number(vec![1]), 1);
                }

                #[test]
                fn test_negative() {
                    assert_eq!(<$solution>::single_number(vec![-1, -1, -2]), -2,);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(<$solution>::single_number(vec![0, 1, 0]), 1,);
                }
            }
        };
    }

    test_single!(my, crate::my_solution::Solution);
    test_single!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = {
            let mut v: Vec<i32> = (1..=500).flat_map(|x| [x, x]).collect();
            v.push(999);
            v
        };

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::single_number(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::single_number(
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
