// LeetCode 258: Add Digits [Easy]
// https://leetcode.com/problems/add-digits/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.995us
//   AI solution avg: 2.682us
//   Ratio: My 2.70x faster
//
// Given an integer num, repeatedly add all its digits until the result
// has only one digit, and return it.
//
// Example 1: num = 38 -> 2  (38 -> 11 -> 2)
// Example 2: num = 0  -> 0
//
// Constraints:
//   - 0 <= num <= 2^31 - 1
//
// Follow up: Could you do it without any loop/recursion in O(1) runtime?
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0258.Add%20Digits/README_EN.md
//
// Run:  cargo run --example lc_0258_add_digits
// Test: cargo test --example lc_0258_add_digits
// Bench: cargo test --example lc_0258_add_digits bench -- --nocapture

/// My solution — digital root formula O(1) (from doocs Rust).
///
/// The digital root of a number n is:
///   0           if n == 0
///   (n-1) % 9 + 1   otherwise
/// This is a well-known number theory result.
///
/// Time: O(1)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the digital root of `num`.
        pub fn add_digits(num: i32) -> i32 {
            if num == 0 { 0 } else { (num - 1) % 9 + 1 }
        }
    }
}

/// AI solution — iterative simulation.
///
/// Repeatedly sum the digits until a single digit remains.
/// Simple and direct, useful for understanding the problem.
///
/// Time: O(log n) amortized — each iteration reduces the number
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the digital root of `num`.
        pub fn add_digits(mut num: i32) -> i32 {
            while num >= 10 {
                let mut sum = 0;
                while num > 0 {
                    sum += num % 10;
                    num /= 10;
                }
                num = sum;
            }
            num
        }
    }
}

fn main() {
    println!("my 38: {}", my_solution::Solution::add_digits(38));
    println!("ai 38: {}", ai_solution::Solution::add_digits(38));
}

#[cfg(test)]
mod tests {
    macro_rules! test_add_digits {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(Sol::add_digits(38), 2);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::add_digits(0), 0);
                }

                #[test]
                fn test_single_digit() {
                    for d in 0..=9 {
                        assert_eq!(Sol::add_digits(d), d);
                    }
                }

                #[test]
                fn test_powers_of_10() {
                    assert_eq!(Sol::add_digits(10), 1);
                    assert_eq!(Sol::add_digits(100), 1);
                    assert_eq!(Sol::add_digits(1000), 1);
                }

                #[test]
                fn test_nines() {
                    assert_eq!(Sol::add_digits(9), 9);
                    assert_eq!(Sol::add_digits(18), 9);
                    assert_eq!(Sol::add_digits(99), 9);
                    assert_eq!(Sol::add_digits(999), 9);
                }

                #[test]
                fn test_large() {
                    // 2^31 - 1 = 2147483647, digit sum = 2+1+4+7+4+8+3+6+4+7 = 46 -> 10 -> 1
                    assert_eq!(Sol::add_digits(i32::MAX), 1);
                }

                #[test]
                fn test_misc() {
                    assert_eq!(Sol::add_digits(123), 6);
                    assert_eq!(Sol::add_digits(199), 1);
                }
            }
        };
    }

    test_add_digits!(my, crate::my_solution::Solution);
    test_add_digits!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..1000).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &nums {
                std::hint::black_box(crate::my_solution::Solution::add_digits(n));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &nums {
                std::hint::black_box(crate::ai_solution::Solution::add_digits(n));
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
