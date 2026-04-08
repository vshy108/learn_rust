// LeetCode 345: Reverse Vowels of a String [Easy]
// https://leetcode.com/problems/reverse-vowels-of-a-string/
//
// Given a string s, reverse only all the vowels in the string and
// return it.
//
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear
// in both lower and upper cases, more than once.
//
// Examples:
//   s = "IcesCreAm" -> "AcesCreIm"
//   s = "leetcode"  -> "leotcede"
//
// Constraints:
//   - 1 <= s.length <= 3 * 10^5
//   - s consists of printable ASCII characters
//
// Run:  cargo run --example lc_0345_reverse_vowels_of_a_string
// Test: cargo test --example lc_0345_reverse_vowels_of_a_string
// Bench: cargo test --example lc_0345_reverse_vowels_of_a_string bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-06) ---
//   My solution: 210.1ms (avg 2.1us)
//   AI solution: 122.0ms (avg 1.22us)
//   -> AI solution is 1.72x faster
//   Reason: const lookup table gives O(1) vowel check via array index,
//     vs my_solution's b"aeiouAEIOU".contains() which does a linear
//     scan over 10 bytes each time. Also uses unsafe
//     String::from_utf8_unchecked to skip the UTF-8 validation pass.
/// My solution module.
///
/// Time:  O(n) — single pass with two pointers
/// Space: O(n) — into_bytes() owns the byte vec
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse only the vowels in string `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string (printable ASCII)
        ///
        /// # Returns
        ///
        /// A new string with vowels reversed in position
        pub fn reverse_vowels(s: String) -> String {
            fn is_vowel(b: u8) -> bool {
                b"aeiouAEIOU".contains(&b)
            }
            // Ruby: s.chars returns an array of chars you can index/swap.
            // Rust: String is UTF-8 and can't be indexed by position.
            // .into_bytes() converts String into Vec<u8> — owned, mutable,
            // and indexable. Safe here because the problem guarantees ASCII.
            let mut bytes = s.into_bytes();
            let mut left = 0;
            // Ruby: chars.length - 1
            // Rust: .len() returns usize. Subtract is fine here because
            // constraints guarantee s.length >= 1, so len() - 1 won't underflow.
            let mut right = bytes.len() - 1;

            while left < right {
                // Ruby: left += 1 while !vowels.include?(chars[left])
                // Rust: no postfix while — use a standard while loop.
                while left < right && !is_vowel(bytes[left]) {
                    left += 1;
                }
                while left < right && !is_vowel(bytes[right]) {
                    right -= 1;
                }
                if left < right {
                    // Ruby: swap is a method on Array — chars[left], chars[right] = chars[right], chars[left]
                    // Rust: Vec/slice has .swap(i, j) — swaps elements in place.
                    bytes.swap(left, right);
                }
                left += 1;
                // right is usize — if it's 0, subtracting 1 would underflow/panic.
                // The outer `while left < right` guard ensures right > left >= 0
                // at this point, but using saturating_sub is defensive.
                right = right.saturating_sub(1);
            }

            // Ruby: chars.join
            // Rust: String::from_utf8(bytes).unwrap() converts Vec<u8> back to
            // String. .unwrap() is safe because we only swapped ASCII bytes —
            // the result is still valid UTF-8.
            String::from_utf8(bytes).unwrap()
        }
    }
}

/// AI solution module.
///
/// Same two-pointer approach but uses a const lookup table for O(1)
/// vowel checks instead of linear scan over 10 bytes.
///
/// Time:  O(n) — single pass with two pointers
/// Space: O(n) — into_bytes() owns the byte vec (+ 128-byte const table)
mod ai_solution {
    pub struct Solution;

    /// Compile-time lookup table: VOWEL_LUT[byte] == true if byte is a vowel.
    /// 128 entries cover all ASCII. Built at compile time with a const block
    /// — zero runtime cost.
    const VOWEL_LUT: [bool; 128] = {
        let mut t = [false; 128];
        t[b'a' as usize] = true;
        t[b'e' as usize] = true;
        t[b'i' as usize] = true;
        t[b'o' as usize] = true;
        t[b'u' as usize] = true;
        t[b'A' as usize] = true;
        t[b'E' as usize] = true;
        t[b'I' as usize] = true;
        t[b'O' as usize] = true;
        t[b'U' as usize] = true;
        t
    };

    impl Solution {
        /// Reverse only the vowels in string `s`.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string (printable ASCII)
        ///
        /// # Returns
        ///
        /// A new string with vowels reversed in position
        pub fn reverse_vowels(s: String) -> String {
            let mut bytes = s.into_bytes();
            let mut left = 0;
            let mut right = bytes.len() - 1;

            while left < right {
                while left < right && !VOWEL_LUT[bytes[left] as usize] {
                    left += 1;
                }
                while left < right && !VOWEL_LUT[bytes[right] as usize] {
                    right -= 1;
                }
                if left < right {
                    bytes.swap(left, right);
                }
                left += 1;
                right = right.saturating_sub(1);
            }

            // Safety: only ASCII bytes were swapped — still valid UTF-8.
            unsafe { String::from_utf8_unchecked(bytes) }
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str)> = vec![("IceCreAm", "AceCreIm"), ("leetcode", "leotcede")];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::reverse_vowels(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::reverse_vowels(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_reverse_vowels {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_mixed_case_vowels() {
                    assert_eq!(Solution::reverse_vowels("IceCreAm".to_string()), "AceCreIm");
                }

                #[test]
                fn test_lowercase() {
                    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");
                }

                #[test]
                fn test_no_vowels() {
                    assert_eq!(Solution::reverse_vowels("bcdfg".to_string()), "bcdfg");
                }

                #[test]
                fn test_all_vowels() {
                    assert_eq!(Solution::reverse_vowels("aeiou".to_string()), "uoiea");
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::reverse_vowels("a".to_string()), "a");
                }

                #[test]
                fn test_uppercase_vowels() {
                    assert_eq!(Solution::reverse_vowels("AEIOU".to_string()), "UOIEA");
                }

                #[test]
                fn test_single_vowel() {
                    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");
                }
            }
        };
    }

    test_reverse_vowels!(my_tests, crate::my_solution::Solution);
    test_reverse_vowels!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec![
            "IceCreAm",
            "leetcode",
            "aeiou",
            "bcdfg",
            "The quick brown fox jumps over the lazy dog",
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::reverse_vowels(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::reverse_vowels(
                    std::hint::black_box(s.to_string()),
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
