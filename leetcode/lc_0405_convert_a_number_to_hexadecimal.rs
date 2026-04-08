// LeetCode 405: Convert a Number to Hexadecimal [Easy]
// https://leetcode.com/problems/convert-a-number-to-hexadecimal/
// Benchmark: 5000000 iterations
//   My solution (nibbles high to low):  avg 198ns
//   AI solution (nibbles low to high):  avg 242ns
//   -> My solution is 1.22x faster
//
// Given a 32-bit integer `num`, return its hexadecimal representation.
// For negative integers, two's complement is used.
// No leading zeros (except "0" itself), all letters lowercase.
// Cannot use built-in library methods.
//
// Examples:
//   num = 26  -> "1a"
//   num = -1  -> "ffffffff"
//   num = 0   -> "0"
//
// Constraints:
//   -2^31 <= num <= 2^31 - 1
//
// Run:  cargo run --example lc_0405_convert_a_number_to_hexadecimal
// Test: cargo test --example lc_0405_convert_a_number_to_hexadecimal
// Bench: cargo test --example lc_0405_convert_a_number_to_hexadecimal bench -- --nocapture

/// My solution module — extract nibbles from high to low.
///
/// Scan from the most-significant nibble (bits 28..31) down to 0..3.
/// Skip leading zeros until we've emitted the first non-zero nibble.
/// Treat `num` as `u32` so negative values use two's complement.
///
/// Time:  O(1) -- always 8 nibbles maximum
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert a 32-bit integer to its hexadecimal string.
        ///
        /// # Arguments
        ///
        /// * `num` - The integer to convert
        ///
        /// # Returns
        ///
        /// Lowercase hex string without leading zeros
        pub fn to_hex(num: i32) -> String {
            if num == 0 {
                return "0".to_string();
            }

            // Reinterpret as unsigned so negative values wrap correctly
            let bits = num as u32;
            let hex_chars = b"0123456789abcdef";
            let mut result = String::with_capacity(8);

            // Walk from the highest nibble (i=7 means bits 28..31) to the lowest
            for i in (0..8).rev() {
                let nibble = ((bits >> (4 * i)) & 0xF) as usize;
                // Skip leading zeros
                if result.is_empty() && nibble == 0 {
                    continue;
                }
                result.push(hex_chars[nibble] as char);
            }

            result
        }
    }
}

/// AI solution module — extract nibbles from low to high, then reverse.
///
/// Repeatedly mask off the lowest 4 bits, shift right by 4.
/// Uses unsigned right shift (>>> equivalent) by casting to u32.
/// Collects nibbles in reverse order, then reverses.
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert a 32-bit integer to its hexadecimal string.
        ///
        /// # Arguments
        ///
        /// * `num` - The integer to convert
        ///
        /// # Returns
        ///
        /// Lowercase hex string without leading zeros
        pub fn to_hex(num: i32) -> String {
            if num == 0 {
                return "0".to_string();
            }

            let hex_chars = b"0123456789abcdef";
            let mut bits = num as u32;
            let mut result = Vec::with_capacity(8);

            // Extract nibbles from least significant to most significant
            while bits != 0 {
                result.push(hex_chars[(bits & 0xF) as usize]);
                bits >>= 4;
            }

            // Reverse to get correct order
            result.reverse();
            // Safety: all bytes are valid ASCII hex chars
            String::from_utf8(result).expect("hex chars are valid UTF-8")
        }
    }
}

fn main() {
    let cases = [(26, "1a"), (-1, "ffffffff"), (0, "0"), (16, "10")];

    println!("--- My Solution ---");
    for (num, expected) in &cases {
        let result = my_solution::Solution::to_hex(*num);
        println!("  num={num} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (num, expected) in &cases {
        let result = ai_solution::Solution::to_hex(*num);
        println!("  num={num} -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_to_hex {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::to_hex(26), "1a");
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::to_hex(-1), "ffffffff");
                }

                #[test]
                fn test_zero() {
                    assert_eq!(Solution::to_hex(0), "0");
                }

                #[test]
                fn test_sixteen() {
                    assert_eq!(Solution::to_hex(16), "10");
                }

                #[test]
                fn test_positive_large() {
                    // 2^31 - 1 = 2147483647 = 0x7FFFFFFF
                    assert_eq!(Solution::to_hex(i32::MAX), "7fffffff");
                }

                #[test]
                fn test_negative_large() {
                    // -2^31 = 0x80000000
                    assert_eq!(Solution::to_hex(i32::MIN), "80000000");
                }

                #[test]
                fn test_negative_two() {
                    // -2 in two's complement = 0xFFFFFFFE
                    assert_eq!(Solution::to_hex(-2), "fffffffe");
                }
            }
        };
    }

    test_to_hex!(my_tests, crate::my_solution::Solution);
    test_to_hex!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 5_000_000_u32;
        let nums = [26, -1, 0, 16, i32::MAX, i32::MIN, -2, 255];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &nums {
                std::hint::black_box(
                    crate::my_solution::Solution::to_hex(
                        std::hint::black_box(n),
                    ),
                );
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &nums {
                std::hint::black_box(
                    crate::ai_solution::Solution::to_hex(
                        std::hint::black_box(n),
                    ),
                );
            }
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
