// LeetCode 383: Ransom Note [Easy]
// https://leetcode.com/problems/ransom-note/
//
// Given two strings ransomNote and magazine, return true if ransomNote can be
// constructed by using the letters from magazine and false otherwise.
// Each letter in magazine can only be used once in ransomNote.
//
// Examples:
//   ransomNote = "a",  magazine = "b"   -> false
//   ransomNote = "aa", magazine = "ab"  -> false
//   ransomNote = "aa", magazine = "aab" -> true
//
// Constraints:
//   - 1 <= ransomNote.length, magazine.length <= 10^5
//   - ransomNote and magazine consist of lowercase English letters
//
// Run:  cargo run --example lc_0383_ransom_note
// Test: cargo test --example lc_0383_ransom_note
// Bench: cargo test --example lc_0383_ransom_note bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-08) ---
// My solution:  2.84s (avg 28.43us)
// AI solution:  2.69s (avg 26.86us)
// -> AI solution is 1.06x faster
// Reason: Early length check avoids scanning both strings when ransom_note
//   is longer than magazine, but on non-trivial inputs the difference is
//   marginal since both do the same O(m+n) frequency count.

/// My solution module — character frequency array.
///
/// Count character frequencies in magazine, then decrement for each
/// character in ransom_note. If any count goes negative, return false.
///
/// Time:  O(m + n) where m = magazine.len(), n = ransom_note.len()
/// Space: O(1) — fixed 26-element array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if `ransom_note` can be constructed from `magazine`.
        ///
        /// # Arguments
        ///
        /// * `ransom_note` - The string to construct
        /// * `magazine` - The source of available letters
        ///
        /// # Returns
        ///
        /// `true` if constructible, `false` otherwise
        pub fn can_construct(ransom_note: String, magazine: String) -> bool {
            // Fixed-size array for 26 lowercase letters — O(1) space
            let mut counts = [0i32; 26];
            for b in magazine.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
            for b in ransom_note.bytes() {
                counts[(b - b'a') as usize] -= 1;
                if counts[(b - b'a') as usize] < 0 {
                    return false;
                }
            }
            true
        }
    }
}

/// AI solution module — early termination with length check + single pass.
///
/// If ransom_note is longer than magazine, it's impossible immediately.
/// Otherwise uses the same frequency-count approach but with an early
/// length bail-out to skip unnecessary work on trivially-false inputs.
///
/// Time:  O(m + n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if `ransom_note` can be constructed from `magazine`.
        ///
        /// # Arguments
        ///
        /// * `ransom_note` - The string to construct
        /// * `magazine` - The source of available letters
        ///
        /// # Returns
        ///
        /// `true` if constructible, `false` otherwise
        pub fn can_construct(ransom_note: String, magazine: String) -> bool {
            // Quick bail-out: if ransom_note has more chars, impossible
            if ransom_note.len() > magazine.len() {
                return false;
            }
            let mut counts = [0i32; 26];
            // Build frequency from magazine
            for b in magazine.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
            // Subtract and check
            for b in ransom_note.bytes() {
                let idx = (b - b'a') as usize;
                counts[idx] -= 1;
                if counts[idx] < 0 {
                    return false;
                }
            }
            true
        }
    }
}

fn main() {
    let cases = [
        ("a", "b", false),
        ("aa", "ab", false),
        ("aa", "aab", true),
    ];

    println!("--- My Solution ---");
    for (note, mag, expected) in &cases {
        let result =
            my_solution::Solution::can_construct(note.to_string(), mag.to_string());
        println!(
            "  can_construct(\"{note}\", \"{mag}\") = {result} (expected {expected})"
        );
    }

    println!("--- AI Solution ---");
    for (note, mag, expected) in &cases {
        let result =
            ai_solution::Solution::can_construct(note.to_string(), mag.to_string());
        println!(
            "  can_construct(\"{note}\", \"{mag}\") = {result} (expected {expected})"
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_can_construct {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert!(!Solution::can_construct(
                        "a".to_string(),
                        "b".to_string()
                    ));
                }

                #[test]
                fn test_example2() {
                    assert!(!Solution::can_construct(
                        "aa".to_string(),
                        "ab".to_string()
                    ));
                }

                #[test]
                fn test_example3() {
                    assert!(Solution::can_construct(
                        "aa".to_string(),
                        "aab".to_string()
                    ));
                }

                #[test]
                fn test_empty_ransom_note() {
                    // Empty ransom note can always be constructed
                    assert!(Solution::can_construct(
                        String::new(),
                        "abc".to_string()
                    ));
                }

                #[test]
                fn test_empty_both() {
                    assert!(Solution::can_construct(
                        String::new(),
                        String::new()
                    ));
                }

                #[test]
                fn test_exact_match() {
                    assert!(Solution::can_construct(
                        "abc".to_string(),
                        "abc".to_string()
                    ));
                }

                #[test]
                fn test_longer_than_magazine() {
                    assert!(!Solution::can_construct(
                        "aaab".to_string(),
                        "ab".to_string()
                    ));
                }

                #[test]
                fn test_all_same_chars() {
                    assert!(Solution::can_construct(
                        "aaa".to_string(),
                        "aaaa".to_string()
                    ));
                }
            }
        };
    }

    test_can_construct!(my_tests, crate::my_solution::Solution);
    test_can_construct!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let magazine: String = "abcdefghijklmnopqrstuvwxyz".repeat(100);
        let ransom_note: String = "zyxwvutsrqponmlkjihgfedcba".repeat(50);

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::can_construct(
                std::hint::black_box(ransom_note.clone()),
                std::hint::black_box(magazine.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::can_construct(
                std::hint::black_box(ransom_note.clone()),
                std::hint::black_box(magazine.clone()),
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
