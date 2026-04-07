// LeetCode 233: Number of Digit One [Hard]
// https://leetcode.com/problems/number-of-digit-one/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.638us
//   AI solution avg: 1.042us
//   Ratio: AI 1.57x faster
//
// Given an integer n, count the total number of digit 1 appearing in
// all non-negative integers less than or equal to n.
//
// Constraints: 0 <= n <= 10^9
//
// Run:  cargo run --example lc_0233_number_of_digit_one
// Test: cargo test --example lc_0233_number_of_digit_one
// Bench: cargo test --example lc_0233_number_of_digit_one bench -- --nocapture

/// My solution — digit-by-digit counting.
///
/// For each digit position (ones, tens, hundreds, ...), count how many
/// times the digit 1 appears in that position across 0..=n.
///
/// For a position with multiplier `m` (1, 10, 100, ...):
///   - `high` = n / (m * 10)  — full cycles above this position
///   - `cur`  = (n / m) % 10    — the digit at this position
///   - `low`  = n % m           — digits below this position
///
/// Contribution for this position:
///   if cur == 0 → high * m
///   if cur == 1 → high * m + low + 1
///   if cur >= 2 → (high + 1) * m
///
/// Time: O(log n) — one iteration per digit.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count total occurrences of digit 1 in 0..=n.
        pub fn count_digit_one(n: i32) -> i32 {
            if n <= 0 {
                return 0;
            }
            let n = n as i64;
            let mut count: i64 = 0;
            let mut m: i64 = 1; // current digit multiplier

            while m <= n {
                let high = n / (m * 10);
                let cur = (n / m) % 10;
                let low = n % m;

                count += match cur {
                    0 => high * m,
                    1 => high * m + low + 1,
                    _ => (high + 1) * m,
                };

                m *= 10;
            }

            count as i32
        }
    }
}

/// AI solution — same mathematical approach, different loop structure.
///
/// Uses divmod to split n at each digit position. The logic is identical
/// but expressed with a running quotient/remainder.
///
/// Time: O(log n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count total occurrences of digit 1 in 0..=n.
        pub fn count_digit_one(n: i32) -> i32 {
            if n <= 0 {
                return 0;
            }
            let n = n as i64;
            let mut count: i64 = 0;
            // `factor` represents the current digit position value
            let mut factor: i64 = 1;

            while factor <= n {
                // `divider` is the full cycle length for this digit
                let divider = factor * 10;
                // Number of complete cycles * positions per cycle
                count += (n / divider) * factor;
                // Partial cycle contribution from the remainder
                count += (n % divider - factor + 1).clamp(0, factor);

                factor *= 10;
            }

            count as i32
        }
    }
}

fn main() {
    let cases = [(13, 6), (0, 0), (1, 1), (100, 21)];
    for (n, expected) in &cases {
        let my = my_solution::Solution::count_digit_one(*n);
        let ai = ai_solution::Solution::count_digit_one(*n);
        println!("n={n}: my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_digit_one {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::count_digit_one(13), 6);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(<$solution>::count_digit_one(0), 0);
                }

                #[test]
                fn test_one() {
                    assert_eq!(<$solution>::count_digit_one(1), 1);
                }

                #[test]
                fn test_nine() {
                    // 1 appears once in [1..9]
                    assert_eq!(<$solution>::count_digit_one(9), 1);
                }

                #[test]
                fn test_ten() {
                    // 1, 10 → 2 ones (1 has one '1', 10 has one '1')
                    assert_eq!(<$solution>::count_digit_one(10), 2);
                }

                #[test]
                fn test_hundred() {
                    // Known: f(100) = 21
                    assert_eq!(<$solution>::count_digit_one(100), 21);
                }

                #[test]
                fn test_large() {
                    // f(1_000_000_000) is a well-known value
                    assert_eq!(
                        <$solution>::count_digit_one(1_000_000_000),
                        900_000_001,
                    );
                }
            }
        };
    }

    test_digit_one!(my, crate::my_solution::Solution);
    test_digit_one!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let test_values: Vec<i32> = (0..100).map(|i| i * 10_000_000).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &test_values {
                std::hint::black_box(crate::my_solution::Solution::count_digit_one(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &test_values {
                std::hint::black_box(crate::ai_solution::Solution::count_digit_one(
                    std::hint::black_box(n),
                ));
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
