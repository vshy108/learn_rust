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
//
// Benchmark (100_000 iterations, 2026-03-27):
//   My solution: 215.3625ms  (avg 2.153µs)
//   AI solution: 83.230625ms (avg 832ns)
//   -> AI solution is 2.59x faster

/// My solution module.
///
/// Time:  O(1) — num is bounded [1, 3999]; the greedy loop iterates over at most 13 fixed symbols
/// Space: O(1) — `const` slice in read-only memory; result String is at most 15 chars
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
            // A sorted slice of (value, symbol) pairs is used instead of a HashMap for two reasons:
            // 1. Order matters: the algorithm must greedily subtract from largest to smallest,
            //    and HashMap gives no guaranteed iteration order.
            // 2. No allocation: a `const` slice lives in the binary's read-only data segment,
            //    whereas a HashMap would require a heap allocation on every call.
            const ROMAN_NUMERALS: &[(i32, &str)] = &[
                (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"),
                (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"),
                (5, "V"), (4, "IV"), (1, "I"),
            ];
            let mut num = num;
            let mut result: String = "".to_string();
            for (value, numeral) in ROMAN_NUMERALS {
                while num >= *value {
                    result += numeral;
                    num -= *value;
                }
            }
            result
        }
    }
}

/// AI solution module.
///
/// Time:  O(1) — constant: num is bounded [1, 3999]; four index lookups regardless of input
/// Space: O(1) — four `const` lookup tables live in the binary's read-only segment; no heap allocation
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
            // Pre-enumerate every possible Roman string for each decimal place.
            // Because num is bounded [1, 3999], thousands can only be 0..=3
            // and every other place 0..=9 — making this a true O(1) lookup.
            const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
            const HUNDREDS: [&str; 10] =
                ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
            const TENS: [&str; 10] =
                ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
            const ONES: [&str; 10] =
                ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

            let n = num as usize;
            // Concatenate the four place-value strings directly — no loop needed.
            format!(
                "{}{}{}{}",
                THOUSANDS[n / 1000],
                HUNDREDS[(n % 1000) / 100],
                TENS[(n % 100) / 10],
                ONES[n % 10],
            )
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
