// LeetCode 166: Fraction to Recurring Decimal [Medium]
// https://leetcode.com/problems/fraction-to-recurring-decimal/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 6.567us
//   AI solution avg: 6.503us
//   Ratio: ~tied
//
// Given two integers numerator and denominator, return the fraction as a
// string. If the fractional part is repeating, enclose it in parentheses.
// E.g. 1/3 = "0.(3)", 2/1 = "2", 4/333 = "0.(012)".
//
// Constraints: -2^31 <= numerator, denominator <= 2^31-1, denominator != 0
//
// Run:  cargo run --example lc_0166_fraction_to_recurring_decimal
// Test: cargo test --example lc_0166_fraction_to_recurring_decimal
// Bench: cargo test --example lc_0166_fraction_to_recurring_decimal bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the string representation of numerator/denominator.
        pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
            use std::collections::HashMap;
            let mut num = numerator as i64;
            let den = denominator as i64;
            let negative = (num < 0) ^ (den < 0) && num != 0;
            num = num.abs();
            let den = den.abs();

            let integer_part = num / den;
            let mut remainder = num % den;
            let mut result = if negative {
                format!("-{integer_part}")
            } else {
                format!("{integer_part}")
            };

            if remainder == 0 {
                return result;
            }
            result.push('.');
            let mut seen: HashMap<i64, usize> = HashMap::new();
            while remainder != 0 {
                if let Some(&pos) = seen.get(&remainder) {
                    result.insert(pos, '(');
                    result.push(')');
                    return result;
                }
                seen.insert(remainder, result.len());
                remainder *= 10;
                result.push_str(&(remainder / den).to_string());
                remainder %= den;
            }
            result
        }
    }
}

/// AI solution — long division with remainder tracking via HashMap.
///
/// Track remainders to detect cycles. When a remainder repeats, the
/// digits between the first occurrence and now form the repeating block.
///
/// Time: O(d) where d is the number of distinct remainders (bounded by denominator).
/// Space: O(d) for the remainder map.
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the fraction string, with repeating parts in parentheses.
        pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
            // Use i64 to avoid overflow with i32::MIN
            let mut num = numerator as i64;
            let den = denominator as i64;

            // Handle sign
            let negative = (num < 0) ^ (den < 0) && num != 0;
            num = num.abs();
            let den = den.abs();

            let integer_part = num / den;
            let mut remainder = num % den;

            let mut result = if negative {
                format!("-{integer_part}")
            } else {
                format!("{integer_part}")
            };

            if remainder == 0 {
                return result;
            }

            result.push('.');
            // Map remainder -> position in the decimal string
            let mut seen: HashMap<i64, usize> = HashMap::new();

            while remainder != 0 {
                if let Some(&pos) = seen.get(&remainder) {
                    // Insert '(' at pos, append ')'
                    result.insert(pos, '(');
                    result.push(')');
                    return result;
                }
                seen.insert(remainder, result.len());
                remainder *= 10;
                result.push_str(&(remainder / den).to_string());
                remainder %= den;
            }
            result
        }
    }
}

fn main() {
    let my = my_solution::Solution::fraction_to_decimal(4, 333);
    let ai = ai_solution::Solution::fraction_to_decimal(4, 333);
    println!("fraction: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_frac {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_exact() {
                    assert_eq!(<$solution>::fraction_to_decimal(1, 2), "0.5");
                }

                #[test]
                fn test_integer() {
                    assert_eq!(<$solution>::fraction_to_decimal(2, 1), "2");
                }

                #[test]
                fn test_repeating() {
                    assert_eq!(<$solution>::fraction_to_decimal(4, 333), "0.(012)");
                }

                #[test]
                fn test_third() {
                    assert_eq!(<$solution>::fraction_to_decimal(1, 3), "0.(3)");
                }

                #[test]
                fn test_negative() {
                    assert_eq!(<$solution>::fraction_to_decimal(-1, 2), "-0.5");
                }

                #[test]
                fn test_zero_numerator() {
                    assert_eq!(<$solution>::fraction_to_decimal(0, 5), "0");
                }

                #[test]
                fn test_negative_denominator() {
                    assert_eq!(<$solution>::fraction_to_decimal(1, -2), "-0.5");
                }
            }
        };
    }

    test_frac!(my, crate::my_solution::Solution);
    test_frac!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::fraction_to_decimal(
                std::hint::black_box(355),
                std::hint::black_box(113),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::fraction_to_decimal(
                std::hint::black_box(355),
                std::hint::black_box(113),
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
