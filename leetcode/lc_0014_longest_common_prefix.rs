// LeetCode 14: Longest Common Prefix [Easy]
// https://leetcode.com/problems/longest-common-prefix/
//
// Write a function to find the longest common prefix string amongst an
// array of strings. If there is no common prefix, return an empty string "".
//
// Examples:
//   strs = ["flower","flow","flight"] -> "fl"
//   strs = ["dog","racecar","car"]    -> ""
//
// Constraints:
//   - 1 <= strs.length <= 200
//   - 0 <= strs[i].length <= 200
//   - strs[i] consists of only lowercase English letters if non-empty
//
// Run:  cargo run --example lc_0014_longest_common_prefix
// Test: cargo test --example lc_0014_longest_common_prefix
// Bench: cargo test --example lc_0014_longest_common_prefix bench -- --nocapture
//
// Benchmark (100_000 iterations, 2026-03-27):
//   My solution: 128.837959ms (avg 1.288µs)
//   AI solution: 157.168042ms (avg 1.571µs)
//   -> My solution is 1.22x faster
//   Reason: My solution's inner `starts_with` short-circuits as soon as the prefix mismatches
//   a string, which is cheap for short common prefixes; the AI's vertical scan iterates all
//   columns upfront with extra closure overhead from `take_while` + `all`.
//
// --- Rust concepts used ---
//
// Moving out of a Vec by index is forbidden:
//   `strs[0]` alone would try to move the String out of the Vec, which Rust forbids.
//   Use `.clone()` to get an owned copy, or work with references/slices.
//
// &strs[1..] to borrow a slice:
//   Iterating `&strs[1..]` yields `&String` references without transferring ownership.
//
// String::pop() for in-place shrinking:
//   Removes and returns the last Unicode char. Safer than `truncate(len - 1)` for
//   non-ASCII strings; avoids allocation unlike Ruby's `str[0...-1]`.
//
// str::starts_with(&str):
//   Ruby `start_with?` without the `?`. Takes a &str; &String auto-derefs to &str.
//
// as_bytes() for indexed byte access:
//   Returns &[u8] with O(1) indexing and no UTF-8 decoding overhead.
//   `.get(i)` returns Option<&u8> — None if index is out of bounds.

/// My solution module.
///
/// Time:  O(S) — S = total characters compared; worst case is all strings identical
/// Space: O(L) — L = length of first string; one clone for the mutable prefix
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest common prefix among an array of strings.
        ///
        /// # Arguments
        ///
        /// * `strs` - Array of strings
        ///
        /// # Returns
        ///
        /// The longest common prefix string
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            if strs.is_empty() {
                return "".to_string();
            };

            // `.clone()` is required because `strs[0]` is indexed by reference — Rust cannot
            // move a String out of a Vec by index (the Vec still owns all its elements).
            // Cloning gives `prefix` its own owned String that can be mutated independently.
            // A: need clone if want to assign vector item to another variable
            let mut prefix = strs[0].clone();

            // `&strs[1..]` borrows a slice of the Vec instead of trying to move out of it.
            // Without `&`, `strs[1..]` gives a bare `[String]` whose iterator would attempt
            // to move each String out of the Vec — Rust forbids moving out of a slice.
            // With `&`, the iterator yields `&String` references, leaving ownership in `strs`.
            for s in &strs[1..] {
                // Ruby `start_with?` -> Rust `starts_with` (no `?`; Rust uses plain names for bool predicates).
                // `prefix.as_str()` converts String -> &str because `starts_with` takes a &str argument.
                // Alternatively `&prefix` or `&prefix[..]` work too — all coerce to &str.
                // `s` is &String here; calling `.starts_with()` works because &String auto-derefs to &str.
                while !s.starts_with(prefix.as_str()) {
                    // Ruby `prefix[0...-1]` creates a new string without the last char and re-assigns.
                    // Rust `pop()` mutates the String in place — no allocation, no re-assignment needed.
                    // `pop()` is also Unicode-safe: it removes the last *char* (which may be multi-byte),
                    // unlike `truncate(len - 1)` which trims by bytes and would panic on multi-byte chars.
                    // The return value (Option<char>) is intentionally ignored here.
                    // prefix.truncate(prefix.len() - 1);  // byte-based alternative (ASCII-only safe)
                    prefix.pop();
                    if prefix.is_empty() {
                        return "".to_string();
                    }
                }
            }

            prefix
        }
    }
}

/// AI solution module.
///
/// Time:  O(S) — S = total characters across all strings; stops at first mismatch
/// Space: O(1) — works on byte slices with a single index; only allocates the result at the end
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest common prefix among an array of strings.
        ///
        /// # Arguments
        ///
        /// * `strs` - Array of strings
        ///
        /// # Returns
        ///
        /// The longest common prefix string
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            if strs.is_empty() {
                // `String::new()` creates an empty String with zero heap allocation (capacity 0).
                // Prefer it over `"".to_string()` which converts &str -> String unnecessarily.
                // A: most efficient
                return String::new();
            }
            // Vertical scanning: treat the first string as the candidate prefix and check
            // one column (character index) at a time across all other strings.
            // `as_bytes()` gives a &[u8] slice — O(1) index access with no UTF-8 overhead.
            // (Safe here: constraint guarantees only lowercase ASCII letters.)
            let first = strs[0].as_bytes();

            // For each byte position i in `first`, check that every other string has the
            // same byte at that position. `get(i)` returns None if the string is too short,
            // which correctly terminates the prefix at shorter strings.
            let prefix_len = (0..first.len())
                .take_while(|&i| strs[1..].iter().all(|s| s.as_bytes().get(i) == Some(&first[i])))
                .count();

            // Slice `strs[0]` to the matched length and allocate the result once.
            strs[0][..prefix_len].to_string()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<&str>, &str)> = vec![
        (vec!["flower", "flow", "flight"], "fl"),
        (vec!["dog", "racecar", "car"], ""),
    ];

    println!("--- My Solution ---");
    for (strs, expected) in &test_cases {
        let input: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        let result = my_solution::Solution::longest_common_prefix(input);
        println!("  {strs:?} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (strs, expected) in &test_cases {
        let input: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        let result = ai_solution::Solution::longest_common_prefix(input);
        println!("  {strs:?} -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_lcp {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn to_vec(strs: &[&str]) -> Vec<String> {
                    strs.iter().map(|s| s.to_string()).collect()
                }

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["flower", "flow", "flight"])),
                        "fl"
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["dog", "racecar", "car"])),
                        ""
                    );
                }

                #[test]
                fn test_single_string() {
                    assert_eq!(Solution::longest_common_prefix(to_vec(&["alone"])), "alone");
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["abc", "abc", "abc"])),
                        "abc"
                    );
                }

                #[test]
                fn test_empty_string_in_array() {
                    assert_eq!(Solution::longest_common_prefix(to_vec(&["", "abc"])), "");
                }

                #[test]
                fn test_single_char_prefix() {
                    assert_eq!(
                        Solution::longest_common_prefix(to_vec(&["ab", "ac", "ad"])),
                        "a"
                    );
                }
            }
        };
    }

    test_lcp!(my_tests, crate::my_solution::Solution);
    test_lcp!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<String>> = vec![
            vec!["flower", "flow", "flight"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec!["dog", "racecar", "car"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec!["abc", "abc", "abc"]
                .into_iter()
                .map(String::from)
                .collect(),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for strs in &inputs {
                std::hint::black_box(crate::my_solution::Solution::longest_common_prefix(
                    std::hint::black_box(strs.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for strs in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::longest_common_prefix(
                    std::hint::black_box(strs.clone()),
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
