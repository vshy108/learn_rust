// LeetCode 179: Largest Number [Medium]
// https://leetcode.com/problems/largest-number/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 61.739us
//   AI solution avg: 86.148us
//   Ratio: My 1.40x faster
//
// Given a list of non-negative integers nums, arrange them such that
// they form the largest number and return it as a string.
// E.g. [10, 2] -> "210", [3, 30, 34, 5, 9] -> "9534330".
//
// Constraints: 1 <= nums.length <= 100, 0 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0179_largest_number
// Test: cargo test --example lc_0179_largest_number
// Bench: cargo test --example lc_0179_largest_number bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the largest number formed by arranging nums.
        pub fn largest_number(nums: Vec<i32>) -> String {
            let mut strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
            strs.sort_by(|a, b| {
                let ab = format!("{a}{b}");
                let ba = format!("{b}{a}");
                ba.cmp(&ab)
            });
            if strs[0] == "0" {
                return "0".to_string();
            }
            strs.join("")
        }
    }
}

/// AI solution — custom sort: compare a+b vs b+a as strings.
///
/// If "ab" > "ba" then a should come before b.
/// Handle edge case: if result starts with '0', return "0".
///
/// Time: O(n log n * k) where k is max digit count.
/// Space: O(n * k) for the string representations.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Custom comparator sort: compare concatenated orderings.
        pub fn largest_number(nums: Vec<i32>) -> String {
            let mut strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
            // Sort descending by comparing "a+b" vs "b+a"
            strs.sort_unstable_by(|a, b| {
                let ab = format!("{a}{b}");
                let ba = format!("{b}{a}");
                ba.cmp(&ab) // reverse order for "largest first"
            });

            // Edge case: all zeros
            if strs[0] == "0" {
                return "0".to_string();
            }

            strs.join("")
        }
    }
}

fn main() {
    let my = my_solution::Solution::largest_number(vec![3, 30, 34, 5, 9]);
    let ai = ai_solution::Solution::largest_number(vec![3, 30, 34, 5, 9]);
    println!("largest: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_ln {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::largest_number(vec![10, 2]), "210",);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::largest_number(vec![3, 30, 34, 5, 9]),
                        "9534330",
                    );
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(<$solution>::largest_number(vec![0, 0]), "0",);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::largest_number(vec![1]), "1",);
                }

                #[test]
                fn test_same_prefix() {
                    assert_eq!(<$solution>::largest_number(vec![12, 121]), "12121",);
                }
            }
        };
    }

    test_ln!(my, crate::my_solution::Solution);
    test_ln!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..100).map(|i| i * 7 + 3).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::largest_number(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::largest_number(
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
