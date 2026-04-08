// LeetCode 402: Remove K Digits [Medium]
// https://leetcode.com/problems/remove-k-digits/
// Benchmark: 100000 iterations
//   My solution (monotone stack, byte ops): avg 135.99us
//   AI solution (monotone stack, char ops): avg 163.01us
//   -> My solution is 1.20x faster
//
// Given string num (non-negative integer) and integer k, return the
// smallest possible integer after removing k digits from num.
//
// Examples:
//   num="1432219", k=3 -> "1219"
//   num="10200", k=1   -> "200"
//   num="10", k=2      -> "0"
//
// Constraints:
//   - 1 <= k <= num.length <= 10^5
//   - num consists of only digits
//   - num has no leading zeros except "0" itself
//
// Run:  cargo run --example lc_0402_remove_k_digits
// Test: cargo test --example lc_0402_remove_k_digits
// Bench: cargo test --example lc_0402_remove_k_digits bench -- --nocapture

/// My solution module — monotone stack (greedy).
///
/// Maintain a stack of chars representing the result. For each new digit,
/// pop from the stack while the top is greater than the current digit
/// and we still have removals left (k > 0). After processing, truncate
/// to `num.len() - k` digits and strip leading zeros.
///
/// Time:  O(n)
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the smallest number after removing k digits.
        ///
        /// # Arguments
        ///
        /// * `num` - String of digits
        /// * `k` - Number of digits to remove
        ///
        /// # Returns
        ///
        /// Smallest resulting number as a string
        pub fn remove_kdigits(num: String, k: i32) -> String {
            let remain = num.len() - k as usize;
            let mut stack: Vec<u8> = Vec::with_capacity(num.len());
            let mut k = k as usize;

            for &b in num.as_bytes() {
                // Pop larger digits from stack while we have removals left
                while k > 0 && !stack.is_empty() && *stack.last().unwrap() > b
                {
                    stack.pop();
                    k -= 1;
                }
                stack.push(b);
            }

            // Take only the first `remain` digits, strip leading zeros
            let result: String = stack[..remain]
                .iter()
                .map(|&b| b as char)
                .skip_while(|&c| c == '0')
                .collect();

            if result.is_empty() { "0".to_string() } else { result }
        }
    }
}

/// AI solution module — same greedy, different string building.
///
/// Uses a Vec<char> stack and builds the result by joining into String
/// directly. Functionally identical but uses char operations.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the smallest number after removing k digits.
        ///
        /// # Arguments
        ///
        /// * `num` - String of digits
        /// * `k` - Number of digits to remove
        ///
        /// # Returns
        ///
        /// Smallest resulting number as a string
        pub fn remove_kdigits(num: String, k: i32) -> String {
            let remain = num.len() - k as usize;
            let mut stack = Vec::with_capacity(num.len());
            let mut removals = k as usize;

            for ch in num.chars() {
                while removals > 0
                    && !stack.is_empty()
                    && *stack.last().unwrap() > ch
                {
                    stack.pop();
                    removals -= 1;
                }
                stack.push(ch);
            }

            // Truncate to `remain` length
            stack.truncate(remain);

            // Find first non-zero position
            let start = stack.iter().position(|&c| c != '0').unwrap_or(remain);
            if start == remain {
                "0".to_string()
            } else {
                stack[start..].iter().collect()
            }
        }
    }
}

fn main() {
    let cases = [
        ("1432219", 3, "1219"),
        ("10200", 1, "200"),
        ("10", 2, "0"),
    ];

    println!("--- My Solution ---");
    for (num, k, expected) in &cases {
        let result =
            my_solution::Solution::remove_kdigits(num.to_string(), *k);
        println!("  \"{num}\", k={k} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (num, k, expected) in &cases {
        let result =
            ai_solution::Solution::remove_kdigits(num.to_string(), *k);
        println!("  \"{num}\", k={k} -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_remove {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::remove_kdigits("1432219".to_string(), 3),
                        "1219"
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::remove_kdigits("10200".to_string(), 1),
                        "200"
                    );
                }

                #[test]
                fn test_example3() {
                    assert_eq!(
                        Solution::remove_kdigits("10".to_string(), 2),
                        "0"
                    );
                }

                #[test]
                fn test_remove_all() {
                    assert_eq!(
                        Solution::remove_kdigits("9".to_string(), 1),
                        "0"
                    );
                }

                #[test]
                fn test_ascending() {
                    // Already ascending — remove last k digits
                    assert_eq!(
                        Solution::remove_kdigits("12345".to_string(), 2),
                        "123"
                    );
                }

                #[test]
                fn test_leading_zeros() {
                    assert_eq!(
                        Solution::remove_kdigits("10001".to_string(), 1),
                        "1"
                    );
                }
            }
        };
    }

    test_remove!(my_tests, crate::my_solution::Solution);
    test_remove!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 200_000_u32;
        // Build a long number with mixed digits
        let num: String = (0..100_000)
            .map(|i| char::from(b'0' + (i % 10) as u8))
            .collect();
        let k = 50_000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::remove_kdigits(
                    std::hint::black_box(num.clone()),
                    std::hint::black_box(k),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::remove_kdigits(
                    std::hint::black_box(num.clone()),
                    std::hint::black_box(k),
                ),
            );
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
