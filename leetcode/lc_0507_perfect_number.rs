// LeetCode 507: Perfect Number [Easy]
// https://leetcode.com/problems/perfect-number/
//
// A perfect number is a positive integer equal to the sum of its positive
// divisors, excluding itself. Given an integer num, return true if it is
// a perfect number.
//
// Examples:
//   28 -> true   (28 = 1 + 2 + 4 + 7 + 14)
//   7  -> false
//
// Constraints:
//   1 <= num <= 10^8
//
// Run:  cargo run --example lc_0507_perfect_number
// Test: cargo test --example lc_0507_perfect_number
// Bench: cargo test --example lc_0507_perfect_number bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 7193ns
//   AI solution: avg 0.7ns
//   AI is ~10000x faster
//   Reason: AI uses a hardcoded lookup of all 5 perfect numbers <= 10^8,
//   reducing to a single `matches!` check vs O(sqrt(n)) divisor enumeration.

/// My solution — enumerate divisors up to sqrt(num).
///
/// For each i from 2 to sqrt(num), if i divides num, add both i and num/i.
/// Start sum at 1 (every number > 1 is divisible by 1).
///
/// Time:  O(sqrt(n))
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if num is a perfect number.
        ///
        /// # Arguments
        /// * `num` - positive integer to check
        ///
        /// # Returns
        /// Whether num equals the sum of its proper divisors
        pub fn check_perfect_number(num: i32) -> bool {
            if num <= 1 {
                return false;
            }
            let mut s = 1; // 1 is always a divisor
            let mut i = 2;
            while i * i <= num {
                if num % i == 0 {
                    s += i;
                    // Avoid double-counting when i == num/i
                    if i != num / i {
                        s += num / i;
                    }
                }
                i += 1;
            }
            s == num
        }
    }
}

/// AI solution — known perfect numbers lookup.
///
/// There are only 4 perfect numbers <= 10^8: 6, 28, 496, 8128, 33550336.
/// We can just check membership. This is O(1) but arguably "cheating".
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if num is a perfect number using a lookup table.
        ///
        /// # Arguments
        /// * `num` - positive integer to check
        ///
        /// # Returns
        /// Whether num is a perfect number
        pub fn check_perfect_number(num: i32) -> bool {
            // All perfect numbers <= 10^8
            matches!(num, 6 | 28 | 496 | 8128 | 33_550_336)
        }
    }
}

fn main() {
    println!(
        "my(28): {}",
        my_solution::Solution::check_perfect_number(28)
    );
    println!("ai(7): {}", ai_solution::Solution::check_perfect_number(7));
}

#[cfg(test)]
mod tests {
    macro_rules! test_perfect {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_28() {
                    assert!(<$solution>::check_perfect_number(28));
                }

                #[test]
                fn test_7() {
                    assert!(!<$solution>::check_perfect_number(7));
                }

                #[test]
                fn test_6() {
                    assert!(<$solution>::check_perfect_number(6));
                }

                #[test]
                fn test_496() {
                    assert!(<$solution>::check_perfect_number(496));
                }

                #[test]
                fn test_1() {
                    assert!(!<$solution>::check_perfect_number(1));
                }

                #[test]
                fn test_2() {
                    assert!(!<$solution>::check_perfect_number(2));
                }

                #[test]
                fn test_33550336() {
                    assert!(<$solution>::check_perfect_number(33_550_336));
                }
            }
        };
    }

    test_perfect!(my, crate::my_solution::Solution);
    test_perfect!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 500_000;
        let num = 33_550_336;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::check_perfect_number(
                std::hint::black_box(num),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::check_perfect_number(
                std::hint::black_box(num),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}ns avg",
            my_duration.as_secs_f64() * 1_000_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}ns avg",
            ai_duration.as_secs_f64() * 1_000_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
