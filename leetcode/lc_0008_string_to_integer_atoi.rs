// LeetCode 8: String to Integer (atoi) [Medium]
// https://leetcode.com/problems/string-to-integer-atoi/
//
// Implement myAtoi(string s), which converts a string to a 32-bit signed
// integer.
//
// Algorithm:
//   1. Whitespace: Ignore any leading whitespace (" ").
//   2. Signedness: Determine the sign by checking if the next character is
//      '-' or '+', assuming positivity if neither present.
//   3. Conversion: Read the integer by skipping leading zeros until a
//      non-digit character is encountered or the end of the string is
//      reached. If no digits were read, then the result is 0.
//   4. Rounding: If the integer is out of the 32-bit signed integer range
//      [-2^31, 2^31 - 1], then clamp it to remain in the range.
//
// Examples:
//   s = "42"            -> 42
//   s = "   -042"       -> -42
//   s = "1337c0d3"      -> 1337
//   s = "0-1"           -> 0
//   s = "words and 987" -> 0
//
// Constraints:
//   - 0 <= s.length <= 200
//   - s consists of English letters (lower/upper), digits (0-9),
//     ' ', '+', '-', and '.'
//
// Run:  cargo run --example lc_0008_string_to_integer_atoi
// Test: cargo test --example lc_0008_string_to_integer_atoi
// Bench: cargo test --example lc_0008_string_to_integer_atoi bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-26) ---
//   My solution (index-based byte scanning):           87.5ms avg (875ns/iter)
//   AI solution (iterator + trim + peekable + fold):   283.1ms avg (2.831us/iter)
//   My solution is 3.23x faster
//   Reason: Direct byte-index scanning is a tight pointer-advance loop; the iterator adaptor chain
//   (trim → peekable → fold) adds indirection and state that the compiler cannot fully optimise away.
//
// --- Rust concepts used ---
//
// String can't be indexed by usize:
//   `s[i]` on a `String`/`&str` doesn't compile because UTF-8
//   chars can be 1-4 bytes. Use `s.as_bytes()[i]` for byte access
//   or `s.chars().nth(i)` for char access (O(n) though).
//
// Byte literals (b'x'):
//   `b' '`, `b'0'`, `b'-'` are `u8` values (the ASCII byte).
//   Use with `as_bytes()` indexing: `s[i] == b' '`.
//   Ruby equivalent: `' '.ord` (32), `'0'.ord` (48).
//
// is_ascii_digit():
//   Method on `u8` and `char`. Equivalent to `>= b'0' && <= b'9'`.
//   Ruby equivalent: `c.match?(/\d/)` or `c.between?('0', '9')`.
//
// No ternary operator:
//   Ruby `x ? a : b` becomes `if x { a } else { b }` in Rust.
//   The `if` is an expression and returns a value.
//
// trim_start():
//   Returns `&str` with leading whitespace removed. Zero-cost —
//   just adjusts the slice pointer, no allocation.
//
// peekable() iterator:
//   Wraps an iterator to add `.peek()` — look at the next
//   element without consuming it. Useful for conditional advance.
//
// take_while() + fold():
//   `take_while(|c| pred)` yields elements while predicate holds.
//   `fold(init, |acc, x| ...)` accumulates like Ruby's `inject`.
//
// i64 for intermediate overflow detection:
//   Accumulate in i64, then clamp to i32 range at the end.
//   Avoids manual boundary checks like `result > (MAX - digit) / 10`.
//
// .clamp(min, max):
//   Returns the value clamped to [min, max]. Built-in on all
//   numeric types. Like `[min, [val, max].min].max` in Ruby.

/// My solution module — index-based byte scanning.
///
/// Time:  O(n) — single pass over the string bytes
/// Space: O(1) — only scalar variables (as_bytes borrows, no copy)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert string to 32-bit signed integer (atoi).
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to parse
        ///
        /// # Returns
        ///
        /// The parsed integer, clamped to [i32::MIN, i32::MAX]
        pub fn my_atoi(s: String) -> i32 {
            let mut i = 0;
            // Rust strings can't be indexed by `s[i]` because UTF-8
            // characters can be multi-byte. `as_bytes()` gives `&[u8]`
            // with O(1) indexing — safe here since the input is ASCII.
            // Compare with byte literals: `b' '`, `b'+'`, `b'-'`, `b'0'`.
            let s = s.as_bytes();
            let n = s.len();
            let mut result = 0;
            let mut sign = 1;
            let max_int = i32::MAX;
            let min_int = i32::MIN;
            // Step 1: skip leading whitespace
            // A: byte of space b' '
            while i < n && s[i] == b' ' {
                i += 1;
            }

            // Step 2: check for sign character
            if i < n && (s[i] == b'+' || s[i] == b'-') {
                if s[i] == b'-' {
                    sign = -1
                }
                i += 1;
            }

            // Step 3: read digits
            // `b'0'..=b'9'` are byte literals for '0' through '9'.
            // `s[i].is_ascii_digit()` is equivalent to `s[i] >= b'0' && s[i] <= b'9'`.
            // Ruby's `s[i].ord - '0'.ord` becomes `(s[i] - b'0') as i32`.
            while i < n && s[i].is_ascii_digit() {
                let digit = (s[i] - b'0') as i32;

                if result > (max_int - digit) / 10 {
                    // Ruby: `sign == 1 ? max_int : min_int`
                    // Rust has no ternary `?:` — use `if` expression instead.
                    return if sign == 1 { max_int } else { min_int };
                }

                result = result * 10 + digit;
                i += 1
            }

            result * sign
        }
    }
}

/// AI solution module — iterator-based with `skip_while`, `take_while`.
///
/// Uses Rust iterator adapters instead of manual index tracking.
/// `trim_start()` handles whitespace, then a peekable iterator
/// processes sign and digits. Uses `saturating_mul` / `saturating_add`
/// to clamp at i32 bounds without manual boundary math.
///
/// Time:  O(n) — single pass over the string
/// Space: O(1) — only scalar variables (no byte slice copy needed)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert string to 32-bit signed integer (atoi).
        ///
        /// Uses `trim_start()` for whitespace, `peekable()` iterator
        /// for lookahead on the sign character, and `saturating_mul` /
        /// `saturating_add` to auto-clamp at i32 bounds.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to parse
        ///
        /// # Returns
        ///
        /// The parsed integer, clamped to [i32::MIN, i32::MAX]
        pub fn my_atoi(s: String) -> i32 {
            // `trim_start()` returns a `&str` with leading whitespace
            // removed — no allocation, just moves the start pointer.
            let s = s.trim_start();

            // `peekable()` wraps an iterator so we can look at the
            // next element without consuming it (via `.peek()`).
            let mut chars = s.chars().peekable();

            // Determine sign: consume '+' or '-' if present.
            // `peek()` returns `Option<&char>` — match on it
            // without advancing the iterator.
            let sign: i64 = match chars.peek() {
                Some('-') => {
                    chars.next();
                    -1
                }
                Some('+') => {
                    chars.next();
                    1
                }
                _ => 1,
            };

            // Accumulate digits into i64 to detect overflow before
            // clamping. Stop at the first non-digit character.
            // `take_while` consumes chars while the predicate holds.
            // `fold` accumulates a running total — like Ruby's `inject`.
            // Clamp intermediate to i32::MAX+1 (not i32::MAX) so that
            // negative overflow to i32::MIN (-2^31) is preserved after
            // multiplying by sign. If we clamped to i32::MAX, then
            // sign * 2_147_483_647 = -2_147_483_647 != i32::MIN.
            let upper = i32::MAX as i64 + 1;
            let result: i64 = chars
                .take_while(|c| c.is_ascii_digit())
                .fold(0i64, |acc, c| {
                    (acc * 10 + (c as i64 - '0' as i64)).min(upper)
                });

            // Apply sign and clamp to i32 range.
            (sign * result).clamp(i32::MIN as i64, i32::MAX as i64) as i32
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("42", 42),
        ("   -042", -42),
        ("1337c0d3", 1337),
        ("0-1", 0),
        ("words and 987", 0),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let result = my_solution::Solution::my_atoi(input.to_string());
        println!("  \"{input}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let result = ai_solution::Solution::my_atoi(input.to_string());
        println!("  \"{input}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_atoi {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_simple_positive() {
                    assert_eq!(Solution::my_atoi("42".to_string()), 42);
                }

                #[test]
                fn test_leading_whitespace_negative() {
                    assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
                }

                #[test]
                fn test_stops_at_non_digit() {
                    assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
                }

                #[test]
                fn test_sign_after_digit() {
                    assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
                }

                #[test]
                fn test_no_digits() {
                    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
                }

                #[test]
                fn test_empty_string() {
                    assert_eq!(Solution::my_atoi("".to_string()), 0);
                }

                #[test]
                fn test_only_whitespace() {
                    assert_eq!(Solution::my_atoi("   ".to_string()), 0);
                }

                #[test]
                fn test_plus_sign() {
                    assert_eq!(Solution::my_atoi("+1".to_string()), 1);
                }

                #[test]
                fn test_overflow_positive() {
                    // Larger than i32::MAX -> clamp to 2_147_483_647
                    assert_eq!(Solution::my_atoi("91283472332".to_string()), i32::MAX);
                }

                #[test]
                fn test_overflow_negative() {
                    // Smaller than i32::MIN -> clamp to -2_147_483_648
                    assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
                }

                #[test]
                fn test_leading_zeros() {
                    assert_eq!(
                        Solution::my_atoi("0000000000012345678".to_string()),
                        12345678
                    );
                }

                #[test]
                fn test_only_sign() {
                    assert_eq!(Solution::my_atoi("+".to_string()), 0);
                    assert_eq!(Solution::my_atoi("-".to_string()), 0);
                }

                #[test]
                fn test_dot_stops_parsing() {
                    assert_eq!(Solution::my_atoi("3.14159".to_string()), 3);
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_atoi!(my_tests, crate::my_solution::Solution);
    test_atoi!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0008_string_to_integer_atoi bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec![
            "42",
            "   -042",
            "1337c0d3",
            "0-1",
            "words and 987",
            "91283472332",
            "-91283472332",
            "  +0000000012345678",
        ];

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::my_atoi(std::hint::black_box(
                    s.to_string(),
                )));
            }
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::my_atoi(std::hint::black_box(
                    s.to_string(),
                )));
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
