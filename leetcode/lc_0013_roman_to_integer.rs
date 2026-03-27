// LeetCode 13: Roman to Integer [Easy]
// https://leetcode.com/problems/roman-to-integer/
//
// Roman numerals: I=1, V=5, X=10, L=50, C=100, D=500, M=1000.
// Subtraction rule: when a smaller value precedes a larger one, subtract it.
//   IV=4, IX=9, XL=40, XC=90, CD=400, CM=900.
//
// Given a roman numeral, convert it to an integer.
//
// Examples:
//   s = "III"     -> 3
//   s = "LVIII"   -> 58
//   s = "MCMXCIV" -> 1994
//
// Constraints:
//   - 1 <= s.length <= 15
//   - s contains only ('I', 'V', 'X', 'L', 'C', 'D', 'M')
//   - s is a valid roman numeral in the range [1, 3999]
//
// Run:  cargo run --example lc_0013_roman_to_integer
// Test: cargo test --example lc_0013_roman_to_integer
// Bench: cargo test --example lc_0013_roman_to_integer bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert a Roman numeral string to an integer.
        ///
        /// # Arguments
        ///
        /// * `s` - A valid Roman numeral string
        ///
        /// # Returns
        ///
        /// The integer value
        pub fn roman_to_int(s: String) -> i32 {
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
        /// Convert a Roman numeral string to an integer.
        ///
        /// # Arguments
        ///
        /// * `s` - A valid Roman numeral string
        ///
        /// # Returns
        ///
        /// The integer value
        pub fn roman_to_int(s: String) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("III", 3),
        ("LVIII", 58),
        ("MCMXCIV", 1994),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::roman_to_int(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::roman_to_int(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_roman_to_int {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_iii() {
                    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
                }

                #[test]
                fn test_lviii() {
                    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
                }

                #[test]
                fn test_mcmxciv() {
                    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
                }

                #[test]
                fn test_iv() {
                    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
                }

                #[test]
                fn test_ix() {
                    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
                }

                #[test]
                fn test_single() {
                    assert_eq!(Solution::roman_to_int("M".to_string()), 1000);
                }

                #[test]
                fn test_3999() {
                    assert_eq!(Solution::roman_to_int("MMMCMXCIX".to_string()), 3999);
                }
            }
        };
    }

    test_roman_to_int!(my_tests, crate::my_solution::Solution);
    test_roman_to_int!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec!["III", "LVIII", "MCMXCIV", "IV", "IX", "MMMCMXCIX"];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::roman_to_int(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::roman_to_int(
                    std::hint::black_box(s.to_string()),
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
