// LeetCode 12: Integer to Roman [Medium]
// https://leetcode.com/problems/integer-to-roman/
//
// Seven different symbols represent Roman numerals with the following values:
//   I=1, V=5, X=10, L=50, C=100, D=500, M=1000
//
// Subtractive forms: IV=4, IX=9, XL=40, XC=90, CD=400, CM=900.
// Given an integer, convert it to a Roman numeral.
//
// Examples:
//   num = 3749 -> "MMMDCCXLIX"
//   num = 58   -> "LVIII"
//   num = 1994 -> "MCMXCIV"
//
// Constraints:
//   - 1 <= num <= 3999
//
// Run:  cargo run --example lc_0012_integer_to_roman
// Test: cargo test --example lc_0012_integer_to_roman
// Bench: cargo test --example lc_0012_integer_to_roman bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert an integer to a Roman numeral string.
        ///
        /// # Arguments
        ///
        /// * `num` - Integer in range [1, 3999]
        ///
        /// # Returns
        ///
        /// Roman numeral representation as a String
        pub fn int_to_roman(num: i32) -> String {
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
        /// Convert an integer to a Roman numeral string.
        ///
        /// # Arguments
        ///
        /// * `num` - Integer in range [1, 3999]
        ///
        /// # Returns
        ///
        /// Roman numeral representation as a String
        pub fn int_to_roman(num: i32) -> String {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, &str)> = vec![
        (3749, "MMMDCCXLIX"),
        (58, "LVIII"),
        (1994, "MCMXCIV"),
    ];

    println!("--- My Solution ---");
    for (num, expected) in &test_cases {
        let result = my_solution::Solution::int_to_roman(*num);
        println!("  {num} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (num, expected) in &test_cases {
        let result = ai_solution::Solution::int_to_roman(*num);
        println!("  {num} -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_int_to_roman {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_3749() {
                    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
                }

                #[test]
                fn test_58() {
                    assert_eq!(Solution::int_to_roman(58), "LVIII");
                }

                #[test]
                fn test_1994() {
                    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
                }

                #[test]
                fn test_1() {
                    assert_eq!(Solution::int_to_roman(1), "I");
                }

                #[test]
                fn test_3999() {
                    assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
                }

                #[test]
                fn test_9() {
                    assert_eq!(Solution::int_to_roman(9), "IX");
                }

                #[test]
                fn test_40() {
                    assert_eq!(Solution::int_to_roman(40), "XL");
                }
            }
        };
    }

    test_int_to_roman!(my_tests, crate::my_solution::Solution);
    test_int_to_roman!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![3749, 58, 1994, 1, 3999, 9, 40, 400, 2023];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::int_to_roman(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::int_to_roman(
                    std::hint::black_box(n),
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
