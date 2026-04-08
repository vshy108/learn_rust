// LeetCode 65: Valid Number [Hard]
// https://leetcode.com/problems/valid-number/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.508us
//   AI solution avg: 0.509us
//   Ratio: ~tied
//
// Determine if a string is a valid number. A valid number can be an integer
// or a decimal, optionally followed by 'e'/'E' and an integer exponent.
//
// Rules: Integer = optional sign + digits. Decimal = optional sign + one of:
//   digits '.' digits, '.' digits, digits '.'
//
// Example: "2" -> true, "e" -> false, "." -> false, ".1" -> true
//
// Constraints: 1 <= s.length <= 20, s consists of [+-.eE0-9]
//
// Run:  cargo run --example lc_0065_valid_number
// Test: cargo test --example lc_0065_valid_number
// Bench: cargo test --example lc_0065_valid_number bench -- --nocapture

/// My solution — explicit state-machine parsing.
///
/// Walk through the string checking each part: optional sign, digits/decimal,
/// optional exponent with optional sign and digits.
///
/// Time: O(n) — single pass.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the string is a valid number.
        ///
        /// # Arguments
        /// * `s` - input string
        ///
        /// # Returns
        /// true if valid number
        pub fn is_number(s: String) -> bool {
            let b = s.as_bytes();
            let n = b.len();
            let mut i = 0;

            // Skip leading sign
            if i < n && (b[i] == b'+' || b[i] == b'-') {
                i += 1;
            }

            let mut has_digits = false;
            let mut has_dot = false;

            // Parse digits and optional dot
            while i < n && (b[i].is_ascii_digit() || b[i] == b'.') {
                if b[i] == b'.' {
                    if has_dot {
                        return false; // second dot
                    }
                    has_dot = true;
                } else {
                    has_digits = true;
                }
                i += 1;
            }

            if !has_digits {
                return false;
            }

            // Parse optional exponent
            if i < n && (b[i] == b'e' || b[i] == b'E') {
                i += 1;
                // Skip sign after e/E
                if i < n && (b[i] == b'+' || b[i] == b'-') {
                    i += 1;
                }
                // Must have at least one digit after e/E
                if i >= n || !b[i].is_ascii_digit() {
                    return false;
                }
                while i < n && b[i].is_ascii_digit() {
                    i += 1;
                }
            }

            i == n
        }
    }
}

/// AI solution — same approach (linear scan is optimal for this problem).
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if the string is a valid number.
        ///
        /// # Arguments
        /// * `s` - input string
        ///
        /// # Returns
        /// true if valid number
        pub fn is_number(s: String) -> bool {
            let b = s.as_bytes();
            let mut i = 0;
            let n = b.len();

            let scan_sign = |i: &mut usize| {
                if *i < n && (b[*i] == b'+' || b[*i] == b'-') {
                    *i += 1;
                }
            };
            let scan_digits = |i: &mut usize| -> bool {
                let start = *i;
                while *i < n && b[*i].is_ascii_digit() {
                    *i += 1;
                }
                *i > start
            };

            scan_sign(&mut i);

            let int_part = scan_digits(&mut i);
            let mut dec_part = false;
            if i < n && b[i] == b'.' {
                i += 1;
                dec_part = scan_digits(&mut i);
            }

            if !int_part && !dec_part {
                return false;
            }

            if i < n && (b[i] == b'e' || b[i] == b'E') {
                i += 1;
                scan_sign(&mut i);
                if !scan_digits(&mut i) {
                    return false;
                }
            }

            i == n
        }
    }
}

fn main() {
    let valid = [
        "2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "53.5e93",
    ];
    let invalid = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
    for s in &valid {
        println!(
            "\"{s}\" -> my:{}, ai:{}",
            my_solution::Solution::is_number(s.to_string()),
            ai_solution::Solution::is_number(s.to_string()),
        );
    }
    for s in &invalid {
        println!(
            "\"{s}\" -> my:{}, ai:{}",
            my_solution::Solution::is_number(s.to_string()),
            ai_solution::Solution::is_number(s.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_is_number {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_valid_integers() {
                    for s in ["2", "0089", "-123", "+456"] {
                        assert!(<$solution>::is_number(s.into()), "expected true for {s}");
                    }
                }

                #[test]
                fn test_valid_decimals() {
                    for s in ["-0.1", "+3.14", "4.", "-.9", ".1"] {
                        assert!(<$solution>::is_number(s.into()), "expected true for {s}");
                    }
                }

                #[test]
                fn test_valid_exponents() {
                    for s in ["2e10", "-90E3", "3e+7", "53.5e93", "1E-5"] {
                        assert!(<$solution>::is_number(s.into()), "expected true for {s}");
                    }
                }

                #[test]
                fn test_invalid() {
                    for s in ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", ".", ""] {
                        // skip empty string since constraint says len >= 1
                        if s.is_empty() {
                            continue;
                        }
                        assert!(!<$solution>::is_number(s.into()), "expected false for {s}");
                    }
                }
            }
        };
    }

    test_is_number!(my, crate::my_solution::Solution);
    test_is_number!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let cases: Vec<String> = [
            "2", "0089", "-0.1", "+3.14", "4.", "2e10", "abc", "1e", "99e2.5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for s in &cases {
                std::hint::black_box(crate::my_solution::Solution::is_number(
                    std::hint::black_box(s.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for s in &cases {
                std::hint::black_box(crate::ai_solution::Solution::is_number(
                    std::hint::black_box(s.clone()),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
