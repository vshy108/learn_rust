// LeetCode 393: UTF-8 Validation [Medium]
// https://leetcode.com/problems/utf-8-validation/
//
// Benchmark: 500,000 iterations (2026-04-08)
//   My solution (if-else shift):     avg 13.28us
//   AI solution (match + bit mask):  avg 13.13us
//   -> AI solution is 1.01x faster (effectively tied)
//
// Given an integer array `data` representing the data, return whether it is
// a valid UTF-8 encoding.
//
// UTF-8 rules:
//   1-byte:  0xxxxxxx
//   2-byte:  110xxxxx 10xxxxxx
//   3-byte:  1110xxxx 10xxxxxx 10xxxxxx
//   4-byte:  11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
//
// Only the least significant 8 bits of each integer are used.
//
// Examples:
//   [197,130,1]   -> true   (11000101 10000010 00000001: 2-byte + 1-byte)
//   [235,140,4]   -> false  (11101011 10001100 00000100: 3-byte but bad cont.)
//
// Constraints:
//   - 1 <= data.length <= 2 * 10^4
//   - 0 <= data[i] <= 255
//
// Run:  cargo run --example lc_0393_utf8_validation
// Test: cargo test --example lc_0393_utf8_validation
// Bench: cargo test --example lc_0393_utf8_validation bench -- --nocapture

/// My solution module — single pass with continuation counter.
///
/// Walk through each byte. If expecting continuation bytes (`cnt > 0`),
/// verify the byte starts with `10`. Otherwise, determine the character
/// length from the leading bits and set `cnt` accordingly.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Validate whether `data` represents valid UTF-8 encoding.
        ///
        /// # Arguments
        ///
        /// * `data` - Integer array where only the lowest 8 bits matter
        ///
        /// # Returns
        ///
        /// true if valid UTF-8
        pub fn valid_utf8(data: Vec<i32>) -> bool {
            let mut cnt = 0u32; // number of continuation bytes expected

            for v in &data {
                if cnt > 0 {
                    // Continuation byte must start with 10xxxxxx
                    if v >> 6 != 0b10 {
                        return false;
                    }
                    cnt -= 1;
                } else if v >> 7 == 0 {
                    // 0xxxxxxx — 1-byte character
                    cnt = 0;
                } else if v >> 5 == 0b110 {
                    // 110xxxxx — 2-byte character, expect 1 continuation
                    cnt = 1;
                } else if v >> 4 == 0b1110 {
                    // 1110xxxx — 3-byte character, expect 2 continuations
                    cnt = 2;
                } else if v >> 3 == 0b11110 {
                    // 11110xxx — 4-byte character, expect 3 continuations
                    cnt = 3;
                } else {
                    // Invalid leading byte (e.g. 10xxxxxx without header,
                    // or 5+ leading 1s which UTF-8 doesn't allow)
                    return false;
                }
            }
            // All characters must be complete — no pending continuations
            cnt == 0
        }
    }
}

/// AI solution module — match-based with explicit bit masks.
///
/// Same algorithm but uses Rust `match` with bit-masked ranges for
/// clearer pattern matching on the leading bits.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Validate whether `data` represents valid UTF-8 encoding.
        ///
        /// # Arguments
        ///
        /// * `data` - Integer array where only the lowest 8 bits matter
        ///
        /// # Returns
        ///
        /// true if valid UTF-8
        pub fn valid_utf8(data: Vec<i32>) -> bool {
            let mut cnt = 0u32;

            for v in &data {
                let byte = (*v & 0xFF) as u8; // mask to 8 bits
                if cnt > 0 {
                    // Must be 10xxxxxx
                    if byte & 0xC0 != 0x80 {
                        return false;
                    }
                    cnt -= 1;
                } else {
                    // Determine character length from leading bits
                    cnt = match byte {
                        0x00..=0x7F => 0,  // 0xxxxxxx
                        0xC0..=0xDF => 1,  // 110xxxxx
                        0xE0..=0xEF => 2,  // 1110xxxx
                        0xF0..=0xF7 => 3,  // 11110xxx
                        _ => return false, // invalid (10xx or 11111xxx)
                    };
                }
            }
            cnt == 0
        }
    }
}

fn main() {
    let cases = [(vec![197, 130, 1], true), (vec![235, 140, 4], false)];

    println!("--- My Solution ---");
    for (data, expected) in &cases {
        let result = my_solution::Solution::valid_utf8(data.clone());
        println!("  {data:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (data, expected) in &cases {
        let result = ai_solution::Solution::valid_utf8(data.clone());
        println!("  {data:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_utf8 {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_valid_two_plus_one() {
                    assert!(Solution::valid_utf8(vec![197, 130, 1]));
                }

                #[test]
                fn test_invalid_continuation() {
                    assert!(!Solution::valid_utf8(vec![235, 140, 4]));
                }

                #[test]
                fn test_single_byte() {
                    assert!(Solution::valid_utf8(vec![0]));
                }

                #[test]
                fn test_all_single_byte() {
                    assert!(Solution::valid_utf8(vec![65, 66, 67]));
                }

                #[test]
                fn test_incomplete_sequence() {
                    // 110xxxxx but no continuation
                    assert!(!Solution::valid_utf8(vec![192]));
                }

                #[test]
                fn test_four_byte() {
                    // 11110000 10000000 10000000 10000000
                    assert!(Solution::valid_utf8(vec![0xF0, 0x80, 0x80, 0x80]));
                }

                #[test]
                fn test_invalid_leading_byte() {
                    // 11111000 — 5 leading 1s, not valid UTF-8
                    assert!(!Solution::valid_utf8(vec![248]));
                }

                #[test]
                fn test_lone_continuation() {
                    // 10xxxxxx without a leading byte
                    assert!(!Solution::valid_utf8(vec![128]));
                }
            }
        };
    }

    test_utf8!(my_tests, crate::my_solution::Solution);
    test_utf8!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        // Build a long valid UTF-8 stream: mix of 1,2,3,4-byte chars
        let mut data = Vec::with_capacity(20_000);
        for i in 0..5_000 {
            match i % 4 {
                0 => data.push(0x41i32), // 1-byte 'A'
                1 => {
                    data.push(0xC3);
                    data.push(0xA9);
                } // 2-byte e-acute
                2 => {
                    data.push(0xE4);
                    data.push(0xB8);
                    data.push(0xAD);
                } // 3-byte CJK
                _ => {
                    data.push(0xF0);
                    data.push(0x9F);
                    data.push(0x98);
                    data.push(0x80);
                }
            }
        }

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::valid_utf8(
                std::hint::black_box(data.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::valid_utf8(
                std::hint::black_box(data.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
