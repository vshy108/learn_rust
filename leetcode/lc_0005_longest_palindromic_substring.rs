// LeetCode 5: Longest Palindromic Substring [Medium]
// https://leetcode.com/problems/longest-palindromic-substring/
//
// Given a string `s`, return the longest palindromic substring in `s`.
//
// A "palindrome" reads the same forwards and backwards.
// A "substring" is a contiguous sequence of characters within the string.
//
// Examples:
//   "babad" -> "bab"  (or "aba" — both are valid)
//   "cbbd"  -> "bb"
//
// Constraints:
//   - 1 <= s.len() <= 1000
//   - s consists of only digits and English letters
//
// Run:  cargo run --example lc_0005_longest_palindromic_substring
// Test: cargo test --example lc_0005_longest_palindromic_substring
// Bench: cargo test --example lc_0005_longest_palindromic_substring bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-26) ---
//   My solution (Manacher's, Vec<char>):       498.57ms avg (4.985us/iter)
//   AI solution (expand-around-center, bytes):  56.18ms avg (561ns/iter)
//   AI solution is 8.87x faster
//
// --- Rust concepts used ---
//
// Iterator chaining (once, chain, flat_map, collect):
//   Building a transformed string by composing iterator adapters.
//   `std::iter::once` wraps a single value as an iterator.
//   `chain` concatenates iterators end-to-end.
//   `flat_map` maps each element to an array and flattens.
//
// Vec<char> for O(1) indexing:
//   `String` doesn't support `s[i]` indexing (compile error).
//   `Vec<char>` gives O(1) random access by position.
//
// usize arithmetic and underflow:
//   `usize` is unsigned — subtracting past 0 wraps to `usize::MAX`
//   (panics in debug). Guard arithmetic with conditionals or use
//   `i32` when negative values are possible.
//
// Closures capturing by reference:
//   `let expand = |left, right| { ... }` captures `b` and `n`
//   by reference. The closure can read but not modify them.
//
// Byte slicing for ASCII strings:
//   `s.as_bytes()` gives `&[u8]` with O(1) indexing.
//   `s[start..end]` byte-slices a String (safe for ASCII-only input).
//   `.to_string()` converts the `&str` slice back to owned String.
//
// Pattern destructuring with &:
//   `(center_index, &max_len)` destructures `&usize` into owned
//   `usize` (works because usize is Copy).

/// My solution module — Manacher's algorithm.
///
/// Time:  O(n) — each position is visited at most twice (once by the
///   for-loop, once by the while expansion). The mirror trick ensures
///   total expansion work is bounded by n.
///
/// Space: O(n) — the transformed array `t` (2n+3 chars) and the
///   palindrome radius array `p` (2n+3 entries).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest palindromic substring in `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to search
        ///
        /// # Returns
        ///
        /// The longest palindromic substring
        pub fn longest_palindrome(s: String) -> String {
            if s.len() < 2 {
                return s;
            };

            // Build Manacher's transformed string: "babad" -> "^#b#a#b#a#d#$"
            // `^` and `$` are unique sentinels that never match each other,
            // so the while loop naturally stops at the boundaries without
            // needing explicit bounds checks.
            // `#` between every character unifies odd/even-length palindromes
            // into a single case (all palindromes become odd-length in `t`).
            //
            // once('^')              — single start sentinel
            // flat_map(|c| ['#', c]) — for each char, emit '#' then the char
            // chain(['#', '$'])      — trailing '#' and end sentinel
            //
            // Collect into Vec<char> because String doesn't support indexing
            // (s[i] is a compile error). Vec<char> gives O(1) random access.
            //
            // `chain` concatenates iterators end-to-end (like linking train
            // cars). For s = "babad", the pipeline builds:
            //   once('^')                              → ['^']
            //   chain(s.chars().flat_map(|c| ['#',c])) → ['^','#','b','#','a','#','b','#','a','#','d']
            //   chain(['#','$'])                        → ['^','#','b','#','a','#','b','#','a','#','d','#','$']
            //   collect()                               → Vec<char>
            //
            // `once('^')` wraps a single value as an iterator (needed because
            //   `chain` joins two iterators — a bare char isn't one).
            // `flat_map(|c| ['#', c])` emits two items per char: '#' then
            //   the char itself. `flat_map` flattens the [char; 2] arrays
            //   into a single stream.
            // `['#', '$']` is a [char; 2] array — arrays implement
            //   IntoIterator, so `chain` accepts them directly.
            let t: Vec<char> = std::iter::once('^')
                .chain(s.chars().flat_map(|c| ['#', c]))
                .chain(['#', '$'])
                .collect();
            let n = t.len();
            // p[i] = radius of the longest palindrome centered at t[i].
            // `vec![0usize; n]` creates n zeros — same as Ruby `Array.new(n, 0)`.
            // The `usize` suffix is needed because p[i] will be used as an
            // index offset (e.g. `t[i + 1 + p[i]]`), and Rust requires
            // `usize` for array indexing. Without it, Rust infers `i32`,
            // which can't be used as an index without casting.
            // A: dynamic array all 0
            let mut p = vec![0usize; n];
            let mut center = 0usize;
            let mut right = 0usize;

            for i in 1..n - 1 {
                // Only use the mirror trick when i is inside the known
                // palindrome boundary. When i >= right, center/mirror
                // are stale — skip to avoid `2 * center - i` underflowing
                // (usize can't go negative).
                if i < right {
                    let mirror = 2 * center - i;
                    p[i] = (right - i).min(p[mirror]);
                }

                // Expand around center i. Direct indexing t[...] is safe
                // because `^` and `$` sentinels never match each other,
                // so the loop always stops before going out of bounds.
                //
                // Gotcha: do NOT use `t.get()` here. `.get()` returns
                // `Option<&char>`, and if both sides go out of bounds,
                // `None == None` evaluates to `true` — the loop would
                // keep expanding infinitely instead of stopping.
                while t[i + 1 + p[i]] == t[i - 1 - p[i]] {
                    p[i] += 1;
                }

                if i + p[i] > right {
                    center = i;
                    right = i + p[i];
                }
            }

            // Find the index with the largest p[i] value and its value.
            // Ruby `p.max` returns just the max value, and `p.index(max_len)`
            // finds its position — two separate passes.
            // Rust `.iter().enumerate().max_by_key()` gets both in one pass.
            //
            // `.iter()` yields `&usize` references (Vec has no `.max()` method
            //   — you must go through an iterator first).
            // `.enumerate()` wraps each item as `(index, &value)`.
            // `.max_by_key(|&(_, &v)| v)` compares by value, returns the
            //   tuple with the largest `v`.
            // `.unwrap()` is safe here because p is never empty (n >= 3
            //   due to the `^`, `#`, `$` sentinels).
            //
            // Destructuring `(center_index, &max_len)`:
            //   `center_index` binds the usize index directly.
            //   `&max_len` pattern-matches the `&usize` reference into an
            //   owned `usize` (works because usize is Copy). Without the `&`,
            //   `max_len` would be `&usize` and you'd need `*max_len`.
            // A: index and the max value
            let (center_index, &max_len) = p.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();

            // Convert from transformed index back to original string index.
            // Each original char at position `k` sits at `2*k + 2` in `t`,
            // so dividing by 2 reverses the mapping.
            let start = (center_index - max_len) / 2;
            // Ruby `s[start, max_len]` takes a substring by (offset, length).
            // Rust equivalent: `&s[start..start + max_len]` on a &str slice.
            // This is byte slicing — safe here because the problem guarantees
            // only ASCII digits and English letters (1 byte per char).
            // `.to_string()` converts the `&str` slice back to an owned String.
            // A: can slice String with Range
            s[start..start + max_len].to_string()
        }
    }
}

/// AI solution module — expand-around-center approach.
///
/// Instead of Manacher's O(n) algorithm with a transformed string,
/// this uses a simpler expand-around-center technique that works
/// directly on the byte slice. For each index, expand outward for
/// both odd- and even-length palindromes, tracking the best.
///
/// Despite being O(n^2) worst-case, the constant factor is much
/// lower: no allocation for the transformed string, no sentinel
/// setup, and byte comparisons are cheaper than Vec<char> indexing.
///
/// Time:  O(n^2) — for each of n centers, expansion is O(n) worst-case
/// Space: O(1)   — only a few scalar variables (no extra arrays)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the longest palindromic substring in `s`.
        ///
        /// For each index `i`, try expanding a palindrome centered at:
        ///   1. `(i, i)`   — odd-length  (e.g. "aba")
        ///   2. `(i, i+1)` — even-length (e.g. "abba")
        /// Track the start and length of the longest found.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string to search
        ///
        /// # Returns
        ///
        /// The longest palindromic substring
        ///
        /// # Complexity
        ///
        /// Time:  O(n^2) — for each of n centers, expansion is O(n)
        /// Space: O(1)   — only scalar tracking variables
        pub fn longest_palindrome(s: String) -> String {
            let b = s.as_bytes();
            let n = b.len();
            if n < 2 {
                return s;
            }

            let mut best_start = 0usize;
            let mut best_len = 1usize;

            // Expand from (left, right) outward while characters match.
            // Returns (start, length) of the palindrome found.
            // Uses i32 for left so it can safely go to -1 without
            // usize underflow.
            let expand = |mut left: i32, mut right: usize| -> (usize, usize) {
                while left >= 0 && right < n && b[left as usize] == b[right] {
                    left -= 1;
                    right += 1;
                }
                // left+1 is the actual start (left went one past),
                // right-left-1 is the length.
                ((left + 1) as usize, right - (left + 1) as usize)
            };

            for i in 0..n {
                // Odd-length palindrome centered at i
                let (start, len) = expand(i as i32, i);
                if len > best_len {
                    best_start = start;
                    best_len = len;
                }

                // Even-length palindrome centered between i and i+1
                if i + 1 < n {
                    let (start, len) = expand(i as i32, i + 1);
                    if len > best_len {
                        best_start = start;
                        best_len = len;
                    }
                }
            }

            s[best_start..best_start + best_len].to_string()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &[&str])> = vec![
        ("babad", &["bab", "aba"]),
        ("cbbd", &["bb"]),
        ("a", &["a"]),
        ("ac", &["a", "c"]),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let result = my_solution::Solution::longest_palindrome(input.to_string());
        println!("  \"{input}\" -> \"{result}\" (expected one of {expected:?})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let result = ai_solution::Solution::longest_palindrome(input.to_string());
        println!("  \"{input}\" -> \"{result}\" (expected one of {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                /// Helper: check the result is a valid palindrome
                /// substring of `s` with the expected length.
                fn is_palindrome(s: &str) -> bool {
                    let bytes = s.as_bytes();
                    let len = bytes.len();
                    for i in 0..len / 2 {
                        if bytes[i] != bytes[len - 1 - i] {
                            return false;
                        }
                    }
                    true
                }

                /// Helper: verify the result is a palindromic substring
                /// of `input` that matches one of the `expected` values.
                fn assert_valid(input: &str, expected: &[&str]) {
                    let result = Solution::longest_palindrome(input.to_string());
                    assert!(
                        expected.contains(&result.as_str()),
                        "input=\"{input}\", got \"{result}\", \
                         expected one of {expected:?}"
                    );
                    assert!(is_palindrome(&result), "\"{result}\" is not a palindrome");
                    assert!(
                        input.contains(&result),
                        "\"{result}\" is not a substring of \"{input}\""
                    );
                }

                #[test]
                fn test_basic_odd() {
                    // "babad" -> "bab" or "aba"
                    assert_valid("babad", &["bab", "aba"]);
                }

                #[test]
                fn test_basic_even() {
                    // "cbbd" -> "bb"
                    assert_valid("cbbd", &["bb"]);
                }

                #[test]
                fn test_single_char() {
                    assert_valid("a", &["a"]);
                }

                #[test]
                fn test_two_different_chars() {
                    // "ac" -> "a" or "c" (each single char is a palindrome)
                    assert_valid("ac", &["a", "c"]);
                }

                #[test]
                fn test_two_same_chars() {
                    assert_valid("bb", &["bb"]);
                }

                #[test]
                fn test_entire_string_palindrome() {
                    assert_valid("racecar", &["racecar"]);
                }

                #[test]
                fn test_palindrome_at_end() {
                    // "aacabdkacaa" -> "aca" (multiple valid answers)
                    let result = Solution::longest_palindrome("aacabdkacaa".to_string());
                    assert!(is_palindrome(&result));
                    assert!("aacabdkacaa".contains(&result));
                    assert!(result.len() >= 3); // at least "aca"
                }

                #[test]
                fn test_all_same_chars() {
                    assert_valid("aaaa", &["aaaa"]);
                }

                #[test]
                fn test_longer_input() {
                    // "forgeeksskeegfor" -> "geeksskeeg"
                    assert_valid("forgeeksskeegfor", &["geeksskeeg"]);
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_palindrome!(my_tests, crate::my_solution::Solution);
    test_palindrome!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0005_longest_palindromic_substring bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Use a moderately sized string with a known palindrome to exercise
        // a realistic workload.
        let input = "forgeeksskeegfor".to_string();

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::longest_palindrome(
                std::hint::black_box(input.clone()),
            ));
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::longest_palindrome(
                std::hint::black_box(input.clone()),
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
