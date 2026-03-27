// LeetCode 7: Reverse Integer [Medium]
// https://leetcode.com/problems/reverse-integer/
//
// Given a signed 32-bit integer x, return x with its digits reversed.
// If reversing x causes the value to go outside the signed 32-bit
// integer range [-2^31, 2^31 - 1], then return 0.
//
// Assume the environment does not allow you to store 64-bit integers
// (signed or unsigned).
//
// Examples:
//   x = 123   -> 321
//   x = -123  -> -321
//   x = 120   -> 21
//
// Constraints:
//   - -2^31 <= x <= 2^31 - 1
//
// Run:  cargo run --example lc_0007_reverse_integer
// Test: cargo test --example lc_0007_reverse_integer
// Bench: cargo test --example lc_0007_reverse_integer bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-26) ---
//   My solution (abs + sign + manual check):           17.79ms avg (177ns/iter)
//   AI solution (sign-preserving + checked arith):     41.05ms avg (410ns/iter)
//   My solution is 2.31x faster
//   Reason: checked_mul/checked_add each emit an overflow-check instruction + branch per digit;
//   plain manual arithmetic lets the compiler reduce the hot loop to branchless operations.
//
// --- Rust concepts used ---
//
// i32::MIN.abs() panics:
//   i32::MIN = -2_147_483_648 but i32::MAX = 2_147_483_647.
//   `.abs()` tries to produce 2_147_483_648 which overflows i32.
//   Guard with `if x == i32::MIN` before calling `.abs()`.
//   Ruby integers are unbounded so this never happens there.
//
// Remainder preserves sign in Rust:
//   `-123 % 10 == -3` (not 7). This lets you process negative
//   numbers directly without converting to absolute value.
//
// checked_mul / checked_add:
//   Return `Option<i32>` — `Some(val)` on success, `None` on overflow.
//   Chained via `.and_then()` for concise overflow detection.
//   Avoids manual boundary math like `result > (MAX - digit) / 10`.
//
// match with early return:
//   `match expr { Some(v) => v, None => return 0 }` is idiomatic
//   for unwrapping-or-bailing. Alternative: `expr.unwrap_or(return 0)`
//   is NOT valid — `return` is a statement, not an expression.

/// My solution module — abs + sign approach with manual overflow check.
///
/// Time:  O(d) — d is the number of digits (at most 10 for i32)
/// Space: O(1) — only scalar variables
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse digits of a 32-bit signed integer.
        ///
        /// # Arguments
        ///
        /// * `x` - The signed 32-bit integer to reverse
        ///
        /// # Returns
        ///
        /// The reversed integer, or 0 if it overflows i32
        pub fn reverse(x: i32) -> i32 {
            // A: reverse of i32::MIN is larger than i32::MIN
            // A: absolute value of i32::MIN is larger than i32::MAX
            // A: hence early return here is good choice
            if x == i32::MIN {
                return 0;
            }
            let mut result = 0;
            let sign = if x < 0 { -1 } else { 1 };
            let mut x = x.abs();

            let max_int = i32::MAX;

            while x > 0 {
                let digit = x % 10;
                x /= 10;

                // A: negative reverse is -8_463_847_412
                // it is less than -2_147_483_648
                // hence no need to check that case
                if result > (max_int - digit) / 10 {
                    return 0;
                }

                result = result * 10 + digit
            }

            result * sign
        }
    }
}

/// AI solution module — sign-preserving with checked arithmetic.
///
/// Instead of converting to absolute value and tracking sign separately,
/// this works directly with the original (possibly negative) number.
/// Rust's `%` preserves sign: `-123 % 10 == -3`, so digits carry the
/// sign naturally. Uses `checked_mul` / `checked_add` to detect overflow
/// without manual boundary math.
///
/// No special case for `i32::MIN` needed — checked ops handle it.
///
/// Time:  O(d) — d is the number of digits (at most 10 for i32)
/// Space: O(1) — only scalar variables
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse digits of a 32-bit signed integer.
        ///
        /// Uses `checked_mul` and `checked_add` to safely detect
        /// overflow without manual boundary comparisons. Works with
        /// negative values directly — no `.abs()` needed.
        ///
        /// # Arguments
        ///
        /// * `x` - The signed 32-bit integer to reverse
        ///
        /// # Returns
        ///
        /// The reversed integer, or 0 if it overflows i32
        pub fn reverse(x: i32) -> i32 {
            let mut x = x;
            let mut result: i32 = 0;

            while x != 0 {
                // Rust's `%` preserves sign: -123 % 10 == -3.
                // So `digit` is negative when `x` is negative,
                // and `result` accumulates the correct sign naturally.
                let digit = x % 10;
                x /= 10;

                // `checked_mul` returns None if result * 10 overflows i32.
                // `checked_add` returns None if adding digit overflows.
                // On any overflow, return 0.
                result = match result.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                    Some(val) => val,
                    None => return 0,
                };
            }

            result
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32)> = vec![(123, 321), (-123, -321), (120, 21)];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let result = my_solution::Solution::reverse(*input);
        println!("  {input} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let result = ai_solution::Solution::reverse(*input);
        println!("  {input} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_reverse {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_positive() {
                    assert_eq!(Solution::reverse(123), 321);
                }

                #[test]
                fn test_negative() {
                    assert_eq!(Solution::reverse(-123), -321);
                }

                #[test]
                fn test_trailing_zero() {
                    assert_eq!(Solution::reverse(120), 21);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(Solution::reverse(0), 0);
                }

                #[test]
                fn test_single_digit() {
                    assert_eq!(Solution::reverse(5), 5);
                    assert_eq!(Solution::reverse(-8), -8);
                }

                #[test]
                fn test_overflow_positive() {
                    // 2_147_483_647 reversed = 7_463_847_412 > i32::MAX
                    assert_eq!(Solution::reverse(i32::MAX), 0);
                }

                #[test]
                fn test_overflow_negative() {
                    // -2_147_483_648 reversed overflows
                    assert_eq!(Solution::reverse(i32::MIN), 0);
                }

                #[test]
                fn test_near_overflow() {
                    // 1_534_236_469 reversed = 9_646_324_351 > i32::MAX
                    assert_eq!(Solution::reverse(1_534_236_469), 0);
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_reverse!(my_tests, crate::my_solution::Solution);
    test_reverse!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0007_reverse_integer bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![123, -123, 120, 0, i32::MAX, i32::MIN, 1_534_236_469];

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &x in &inputs {
                std::hint::black_box(crate::my_solution::Solution::reverse(std::hint::black_box(
                    x,
                )));
            }
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &x in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::reverse(std::hint::black_box(
                    x,
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
