// LeetCode 266: Palindrome Permutation [Easy] (Premium)
// https://leetcode.com/problems/palindrome-permutation/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 1.327us
//   AI solution avg: 0.237us
//   Ratio: AI 5.60x faster
//
// Given a string s, return true if a permutation of the string could form
// a palindrome and false otherwise.
//
// Example 1: s = "code"    -> false
// Example 2: s = "aab"     -> true
// Example 3: s = "carerac" -> true
//
// Constraints:
//   - 1 <= s.length <= 5000
//   - s consists of only lowercase English letters
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0266.Palindrome%20Permutation/README_EN.md
//
// Run:  cargo run --example lc_0266_palindrome_permutation
// Test: cargo test --example lc_0266_palindrome_permutation
// Bench: cargo test --example lc_0266_palindrome_permutation bench -- --nocapture

/// My solution — character count array (from doocs Java).
///
/// Count character frequencies. A palindrome permutation allows at most
/// one character with odd frequency.
///
/// Time: O(n)
/// Space: O(1) — fixed 26-element array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if some permutation of `s` is a palindrome.
        pub fn can_permute_palindrome(s: String) -> bool {
            let mut cnt = [0u16; 26];
            for b in s.bytes() {
                cnt[(b - b'a') as usize] += 1;
            }
            // At most one character may have an odd count
            cnt.iter().filter(|&&c| c & 1 == 1).count() < 2
        }
    }
}

/// AI solution — XOR bitmask toggling.
///
/// Use a u32 bitmask where bit i is 1 if character i has odd count.
/// Toggle the bit for each character. At the end, a palindrome is
/// possible if at most one bit is set (mask == 0 or has exactly one
/// bit set, i.e. mask & (mask-1) == 0).
///
/// Time: O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if some permutation of `s` is a palindrome.
        pub fn can_permute_palindrome(s: String) -> bool {
            let mut mask = 0u32;
            for b in s.bytes() {
                mask ^= 1 << (b - b'a');
            }
            // At most one bit set: mask is 0 or a power of 2
            mask & mask.wrapping_sub(1) == 0
        }
    }
}

fn main() {
    println!("my 'aab': {}", my_solution::Solution::can_permute_palindrome("aab".into()));
    println!("ai 'code': {}", ai_solution::Solution::can_permute_palindrome("code".into()));
}

#[cfg(test)]
mod tests {
    macro_rules! test_palindrome_perm {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert!(!Sol::can_permute_palindrome("code".into()));
                }

                #[test]
                fn test_example_2() {
                    assert!(Sol::can_permute_palindrome("aab".into()));
                }

                #[test]
                fn test_example_3() {
                    assert!(Sol::can_permute_palindrome("carerac".into()));
                }

                #[test]
                fn test_single_char() {
                    assert!(Sol::can_permute_palindrome("a".into()));
                }

                #[test]
                fn test_two_same() {
                    assert!(Sol::can_permute_palindrome("aa".into()));
                }

                #[test]
                fn test_two_diff() {
                    assert!(!Sol::can_permute_palindrome("ab".into()));
                }

                #[test]
                fn test_all_same() {
                    assert!(Sol::can_permute_palindrome("aaaaaa".into()));
                }

                #[test]
                fn test_abcba_scrambled() {
                    assert!(Sol::can_permute_palindrome("bacba".into()));
                }

                #[test]
                fn test_all_unique() {
                    assert!(!Sol::can_permute_palindrome("abcdef".into()));
                }
            }
        };
    }

    test_palindrome_perm!(my, crate::my_solution::Solution);
    test_palindrome_perm!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s: String = "abcdefghijklmnopqrstuvwxyz".repeat(100);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::can_permute_palindrome(
                s.clone(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::can_permute_palindrome(
                s.clone(),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
