// LeetCode 415: Add Strings [Easy]
// https://leetcode.com/problems/add-strings/
// Benchmark: 500000 iterations
//   My solution (two pointers, index-based): avg 3.21us
//   AI solution (iterator-based rev()):      avg 2.52us
//   -> AI solution is 1.27x faster
//
// Given two non-negative integers num1 and num2 represented as strings,
// return their sum as a string. Cannot use BigInteger or convert directly.
//
// Examples:
//   "11" + "123" -> "134"
//   "456" + "77" -> "533"
//   "0" + "0"    -> "0"
//
// Constraints:
//   1 <= num1.length, num2.length <= 10^4
//   num1, num2 consist only of digits, no leading zeros (except "0")
//
// Run:  cargo run --example lc_0415_add_strings
// Test: cargo test --example lc_0415_add_strings
// Bench: cargo test --example lc_0415_add_strings bench -- --nocapture

/// My solution module — two pointers from the end, build reversed.
///
/// Walk both strings from the least significant digit, accumulating
/// carry. Push each digit into a Vec<u8>, then reverse and convert.
///
/// Time:  O(max(m, n))
/// Space: O(max(m, n))
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Add two number strings and return the sum as a string.
        ///
        /// # Arguments
        ///
        /// * `num1` - First non-negative integer as string
        /// * `num2` - Second non-negative integer as string
        ///
        /// # Returns
        ///
        /// Sum as a string
        pub fn add_strings(num1: String, num2: String) -> String {
            let s1 = num1.as_bytes();
            let s2 = num2.as_bytes();
            let mut i = s1.len();
            let mut j = s2.len();
            let mut carry = 0_u8;
            let mut result = Vec::with_capacity(i.max(j) + 1);

            // Process digits from right to left
            while i > 0 || j > 0 || carry > 0 {
                let mut sum = carry;
                if i > 0 {
                    i -= 1;
                    sum += s1[i] - b'0';
                }
                if j > 0 {
                    j -= 1;
                    sum += s2[j] - b'0';
                }
                carry = sum / 10;
                result.push(b'0' + sum % 10);
            }

            result.reverse();
            // Safety: all bytes are ASCII digits
            String::from_utf8(result).expect("digits are valid UTF-8")
        }
    }
}

/// AI solution module — iterator-based approach with rev().
///
/// Zip reverse byte iterators, then use fold-like loop.
/// Slightly more idiomatic Rust with iterators.
///
/// Time:  O(max(m, n))
/// Space: O(max(m, n))
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Add two number strings and return the sum as a string.
        ///
        /// # Arguments
        ///
        /// * `num1` - First non-negative integer as string
        /// * `num2` - Second non-negative integer as string
        ///
        /// # Returns
        ///
        /// Sum as a string
        pub fn add_strings(num1: String, num2: String) -> String {
            let mut a = num1.bytes().rev();
            let mut b = num2.bytes().rev();
            let mut carry = 0_u8;
            let mut result = Vec::with_capacity(num1.len().max(num2.len()) + 1);

            loop {
                let da = a.next();
                let db = b.next();
                if da.is_none() && db.is_none() && carry == 0 {
                    break;
                }
                let sum = da.map_or(0, |c| c - b'0') + db.map_or(0, |c| c - b'0') + carry;
                carry = sum / 10;
                result.push(b'0' + sum % 10);
            }

            result.reverse();
            String::from_utf8(result).expect("digits are valid UTF-8")
        }
    }
}

fn main() {
    let cases = [("11", "123", "134"), ("456", "77", "533"), ("0", "0", "0")];

    println!("--- My Solution ---");
    for (a, b, expected) in &cases {
        let result = my_solution::Solution::add_strings(a.to_string(), b.to_string());
        println!("  \"{a}\" + \"{b}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (a, b, expected) in &cases {
        let result = ai_solution::Solution::add_strings(a.to_string(), b.to_string());
        println!("  \"{a}\" + \"{b}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_add {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::add_strings("11".to_string(), "123".to_string(),),
                        "134",
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::add_strings("456".to_string(), "77".to_string(),),
                        "533",
                    );
                }

                #[test]
                fn test_both_zero() {
                    assert_eq!(
                        Solution::add_strings("0".to_string(), "0".to_string(),),
                        "0",
                    );
                }

                #[test]
                fn test_carry_overflow() {
                    assert_eq!(
                        Solution::add_strings("999".to_string(), "1".to_string(),),
                        "1000",
                    );
                }

                #[test]
                fn test_different_lengths() {
                    assert_eq!(
                        Solution::add_strings("1".to_string(), "99999".to_string(),),
                        "100000",
                    );
                }

                #[test]
                fn test_large_numbers() {
                    assert_eq!(
                        Solution::add_strings("9999999999".to_string(), "1".to_string(),),
                        "10000000000",
                    );
                }
            }
        };
    }

    test_add!(my_tests, crate::my_solution::Solution);
    test_add!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        let num1 = "9".repeat(1000);
        let num2 = "9".repeat(1000);

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::add_strings(
                std::hint::black_box(num1.clone()),
                std::hint::black_box(num2.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::add_strings(
                std::hint::black_box(num1.clone()),
                std::hint::black_box(num2.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64 / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64 / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
