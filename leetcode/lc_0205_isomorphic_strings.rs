// LeetCode 205: Isomorphic Strings [Easy]
// https://leetcode.com/problems/isomorphic-strings/
//
// Given two strings s and t, determine if they are isomorphic. Two strings
// s and t are isomorphic if the characters in s can be replaced to get t.
//
// All occurrences of a character must be replaced with another character
// while preserving the order. No two characters may map to the same
// character, but a character may map to itself.
//
// Examples:
//   s = "egg",   t = "add"   -> true
//   s = "foo",   t = "bar"   -> false
//   s = "paper", t = "title" -> true
//
// Constraints:
//   - 1 <= s.length <= 5 * 10^4
//   - t.length == s.length
//   - s and t consist of any valid ASCII character
//
// Run:  cargo run --example lc_0205_isomorphic_strings
// Test: cargo test --example lc_0205_isomorphic_strings
// Bench: cargo test --example lc_0205_isomorphic_strings bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if two strings are isomorphic.
        ///
        /// # Arguments
        ///
        /// * `s` - First string
        /// * `t` - Second string (same length as s)
        ///
        /// # Returns
        ///
        /// `true` if s and t are isomorphic, `false` otherwise
        pub fn is_isomorphic(s: String, t: String) -> bool {
            // return false if s.length != t.length
            // map_st = {}
            // map_ts = {}
            // s.each_char.with_index do |char_s, i|
            //   char_t = t[i]
            //   if map_st.key?(char_s)
            //     return false if map_st[char_s] != char_t
            //   else
            //     map_st[char_s] = char_t
            //   end
            //   if map_ts.key?(char_t)
            //     return false if map_ts[char_t] != char_s
            //   else
            //     map_ts[char_t] = char_s
            //   end
            // end
            // true
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if two strings are isomorphic.
        ///
        /// # Arguments
        ///
        /// * `s` - First string
        /// * `t` - Second string (same length as s)
        ///
        /// # Returns
        ///
        /// `true` if s and t are isomorphic, `false` otherwise
        pub fn is_isomorphic(s: String, t: String) -> bool {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, bool)> = vec![
        ("egg", "add", true),
        ("foo", "bar", false),
        ("paper", "title", true),
    ];

    println!("--- My Solution ---");
    for (s, t, expected) in &test_cases {
        let result = my_solution::Solution::is_isomorphic(s.to_string(), t.to_string());
        println!("  \"{s}\", \"{t}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, t, expected) in &test_cases {
        let result = ai_solution::Solution::is_isomorphic(s.to_string(), t.to_string());
        println!("  \"{s}\", \"{t}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_isomorphic {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_egg_add() {
                    assert!(Solution::is_isomorphic(
                        "egg".to_string(),
                        "add".to_string()
                    ));
                }

                #[test]
                fn test_foo_bar() {
                    assert!(!Solution::is_isomorphic(
                        "foo".to_string(),
                        "bar".to_string()
                    ));
                }

                #[test]
                fn test_paper_title() {
                    assert!(Solution::is_isomorphic(
                        "paper".to_string(),
                        "title".to_string()
                    ));
                }

                #[test]
                fn test_single_char() {
                    assert!(Solution::is_isomorphic("a".to_string(), "b".to_string()));
                }

                #[test]
                fn test_same_string() {
                    assert!(Solution::is_isomorphic(
                        "abc".to_string(),
                        "abc".to_string()
                    ));
                }

                #[test]
                fn test_badc_baba() {
                    // b->b, a->a, d->b -- but b is already mapped from b,
                    // and d != b, so two chars map to same target
                    assert!(!Solution::is_isomorphic(
                        "badc".to_string(),
                        "baba".to_string()
                    ));
                }

                #[test]
                fn test_all_same_chars() {
                    assert!(Solution::is_isomorphic(
                        "aaa".to_string(),
                        "bbb".to_string()
                    ));
                }
            }
        };
    }

    test_isomorphic!(my_tests, crate::my_solution::Solution);
    test_isomorphic!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> = vec![
            ("egg", "add"),
            ("foo", "bar"),
            ("paper", "title"),
            ("badc", "baba"),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::is_isomorphic(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(s, t) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::is_isomorphic(
                    std::hint::black_box(s.to_string()),
                    std::hint::black_box(t.to_string()),
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
