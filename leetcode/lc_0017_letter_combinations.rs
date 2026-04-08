// LeetCode 17: Letter Combinations of a Phone Number [Medium]
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
//
// Given a string containing digits from 2-9 inclusive, return all possible
// letter combinations that the number could represent (like telephone buttons).
// Return the answer in any order.
//
// Mapping: 2->"abc", 3->"def", 4->"ghi", 5->"jkl",
//          6->"mno", 7->"pqrs", 8->"tuv", 9->"wxyz"
//
// Example 1: digits = "23" -> ["ad","ae","af","bd","be","bf","cd","ce","cf"]
// Example 2: digits = "2"  -> ["a","b","c"]
// Example 3: digits = ""   -> []
//
// Constraints:
//   0 <= digits.length <= 4
//   digits[i] is a digit in the range ['2', '9']
//
// Run:  cargo run --example lc_0017_letter_combinations
// Test: cargo test --example lc_0017_letter_combinations
// Bench: cargo test --example lc_0017_letter_combinations bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-07) ---
//   My solution: 28.004us avg (2.80s total)
//   AI solution: 32.901us avg (3.29s total)
//   My solution is 1.17x faster
//   Reason: Backtracking reuses a single String buffer (push/pop) and only
//   clones when a full combination is ready. The iterative approach allocates
//   a fresh Vec of Strings at every digit level, cloning all partial results
//   each time — more allocations for the same output.

/// Digit-to-letters mapping as a const array indexed by digit value.
/// Index 0 and 1 are empty (no letters for 0 and 1).
const MAPPING: [&str; 10] = [
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

/// My solution — recursive backtracking.
///
/// Build each combination character by character using recursion. At each
/// level, iterate over the letters for the current digit, push a char,
/// recurse to the next digit, then pop (backtrack).
///
/// Time: O(4^n * n) — at most 4 choices per digit, n digits, and each
///   complete combination is an n-length string that gets cloned into results.
/// Space: O(n) — recursion depth is n (the current combination buffer).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all letter combinations for the given digit string.
        ///
        /// # Arguments
        /// * `digits` - string of digits '2'-'9' (length 0..=4)
        ///
        /// # Returns
        /// All possible letter combinations.
        pub fn letter_combinations(digits: String) -> Vec<String> {
            if digits.is_empty() {
                return Vec::new();
            }

            let mut result = Vec::new();
            let mut current = String::with_capacity(digits.len());
            // Convert digits to byte slice for fast indexing.
            let digits = digits.as_bytes();

            Self::backtrack(digits, 0, &mut current, &mut result);
            result
        }

        /// Recursive helper that builds combinations one character at a time.
        fn backtrack(digits: &[u8], index: usize, current: &mut String, result: &mut Vec<String>) {
            if index == digits.len() {
                // Built a full-length combination — save it.
                result.push(current.clone());
                return;
            }

            // Convert ASCII digit byte to usize index: b'2' - b'0' = 2.
            let digit = (digits[index] - b'0') as usize;
            let letters = crate::MAPPING[digit];

            for ch in letters.chars() {
                current.push(ch);
                Self::backtrack(digits, index + 1, current, result);
                current.pop();
            }
        }
    }
}

/// AI-optimized solution — iterative Cartesian product.
///
/// Instead of recursion, build combinations iteratively: start with [""],
/// then for each digit, expand every existing partial combination by
/// appending each letter for that digit. Avoids recursion overhead and
/// produces the result in a single pass through the digits.
///
/// Time: O(4^n * n) — same asymptotic as backtracking.
/// Space: O(4^n * n) — stores all combinations (same output size).
///   However, avoids the recursion stack and does fewer allocations by
///   pre-computing capacity.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all letter combinations for the given digit string.
        ///
        /// # Arguments
        /// * `digits` - string of digits '2'-'9' (length 0..=4)
        ///
        /// # Returns
        /// All possible letter combinations.
        pub fn letter_combinations(digits: String) -> Vec<String> {
            if digits.is_empty() {
                return Vec::new();
            }

            // Calculate total combinations to pre-allocate.
            // Each digit contributes 3 or 4 letters (multiply).
            let total: usize = digits
                .bytes()
                .map(|b| crate::MAPPING[(b - b'0') as usize].len())
                .product();

            // Start with a single empty string as the seed.
            let mut result = Vec::with_capacity(total);
            result.push(String::with_capacity(digits.len()));

            for b in digits.bytes() {
                let letters = crate::MAPPING[(b - b'0') as usize];

                // For each existing partial, append each letter for this digit.
                // Collect into a new vec — avoids borrow conflicts with result.
                let mut next = Vec::with_capacity(result.len() * letters.len());

                for combo in &result {
                    for ch in letters.bytes() {
                        let mut new_combo = combo.clone();
                        new_combo.push(ch as char);
                        next.push(new_combo);
                    }
                }

                result = next;
            }

            result
        }
    }
}

fn main() {
    let digits = "23".to_string();

    println!("--- My Solution ---");
    let result = my_solution::Solution::letter_combinations(digits.clone());
    println!("letter_combinations(\"23\") = {result:?}");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::letter_combinations(digits);
    println!("letter_combinations(\"23\") = {result:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_letter_combinations {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                /// Sort results for order-independent comparison.
                fn sorted(mut v: Vec<String>) -> Vec<String> {
                    v.sort_unstable();
                    v
                }

                #[test]
                fn test_two_digits() {
                    let result = Solution::letter_combinations("23".into());
                    assert_eq!(
                        sorted(result),
                        sorted(
                            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                                .into_iter()
                                .map(String::from)
                                .collect()
                        )
                    );
                }

                #[test]
                fn test_single_digit() {
                    let result = Solution::letter_combinations("2".into());
                    assert_eq!(sorted(result), vec!["a", "b", "c"]);
                }

                #[test]
                fn test_empty() {
                    let result = Solution::letter_combinations("".into());
                    assert!(result.is_empty());
                }

                #[test]
                fn test_four_letter_digit() {
                    // Digit 7 maps to "pqrs" (4 letters).
                    let result = Solution::letter_combinations("7".into());
                    assert_eq!(sorted(result), vec!["p", "q", "r", "s"]);
                }

                #[test]
                fn test_max_length() {
                    // 4 digits, each with 4 letters = 4^4 = 256 combinations.
                    let result = Solution::letter_combinations("7979".into());
                    assert_eq!(result.len(), 256);
                    // Each combination should be 4 chars long.
                    assert!(result.iter().all(|s| s.len() == 4));
                }

                #[test]
                fn test_three_digits() {
                    let result = Solution::letter_combinations("234".into());
                    // 3 * 3 * 3 = 27 combinations.
                    assert_eq!(result.len(), 27);
                    assert!(result.iter().all(|s| s.len() == 3));
                }
            }
        };
    }

    test_letter_combinations!(my_tests, crate::my_solution::Solution);
    test_letter_combinations!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let digits = "7979".to_string(); // max: 4^4 = 256 combos

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::letter_combinations(
                std::hint::black_box(digits.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::letter_combinations(
                std::hint::black_box(digits.clone()),
            ));
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
