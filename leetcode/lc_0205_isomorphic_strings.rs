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
//
// --- Benchmark (100,000 iterations, 2026-04-04) ---
// My solution: avg 11.802us
// AI solution: avg 931ns
// Ratio: AI solution is 12.67x faster
// Reason: Fixed-size [u8; 256] arrays use direct byte-indexed lookup (O(1),
//         no hashing). HashMap has per-lookup hashing + probing overhead,
//         plus heap allocations that grow with insertions. On small ASCII
//         inputs, the hash overhead dominates.
/// My solution module.
///
/// Dual HashMap approach: map_st tracks s->t mappings, map_ts tracks t->s.
/// Both directions must be consistent for isomorphism.
///
/// Time:  O(n) - single pass through both strings
/// Space: O(k) - two HashMaps, k = number of unique chars (at most 256)
mod my_solution {
    pub struct Solution;

    use std::collections::HashMap;

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
            if s.len() != t.len() {
                return false;
            };

            let mut map_st: HashMap<u8, u8> = HashMap::new();
            let mut map_ts: HashMap<u8, u8> = HashMap::new();

            // Ruby: s.each_char.with_index do |char_s, i| char_t = t[i] ...
            // Rust: .zip() pairs two iterators element-wise — cleaner than
            //   indexing with t[i] (which doesn't work on String anyway;
            //   String has no Index<usize> trait — you'd need t.as_bytes()[i]).
            // `&cs` / `&ct` destructure the references from .iter() so we get
            //   u8 values directly instead of &u8 references.
            //
            // Step-by-step for s="egg", t="add":
            //   s.as_bytes()        -> [101, 103, 103]       (e, g, g)
            //   t.as_bytes()        -> [97, 100, 100]        (a, d, d)
            //   .iter()             -> Iterator<Item = &u8>
            //   .zip()              -> Iterator<Item = (&u8, &u8)>
            //     yields: (&101, &97), (&103, &100), (&103, &100)
            //   for (&cs, &ct)      -> destructures &u8 to u8:
            //     iter 1: cs=101(e), ct=97(a)
            //     iter 2: cs=103(g), ct=100(d)
            //     iter 3: cs=103(g), ct=100(d)
            for (&cs, &ct) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
                // Ruby: map_st.key?(cs) -> Rust: map_st.contains_key(&cs)
                // Ruby: map_st[cs]      -> Rust: map_st[&cs] (Index trait)
                //   BUT map_st[&cs] panics if key missing. Use .get() instead.
                //
                // Ruby: return false if map_st[cs] != ct
                // Rust: match on .get() — returns Option<&V>
                //   Some(&v) if v != ct -> conflict, not isomorphic
                //   None -> first time seeing cs, insert the mapping
                match map_st.get(&cs) {
                    Some(&v) if v != ct => return false,
                    None => {
                        map_st.insert(cs, ct);
                    }
                    _ => {} // existing mapping matches, continue
                }
                match map_ts.get(&ct) {
                    Some(&v) if v != cs => return false,
                    None => {
                        map_ts.insert(ct, cs);
                    }
                    _ => {}
                }
            }

            true
        }
    }
}

/// AI solution module.
///
/// Fixed-size array approach: use `[u16; 256]` arrays indexed by byte value
/// instead of HashMaps. O(1) direct lookup with zero hashing overhead.
/// Uses u16 (not u8) to store mapped values + 1: avoids overflow when
/// byte 0xFF maps to 0xFF+1=256 which doesn't fit in u8. With u16,
/// values 1..=256 all fit, and 0 safely means "unmapped".
///
/// Time:  O(n) - single pass through both strings
/// Space: O(1) - two fixed 256-element arrays (constant regardless of input)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if two strings are isomorphic.
        ///
        /// Uses two `[u16; 256]` arrays as direct-indexed lookup tables.
        /// Each byte value maps to its partner + 1 (0 = unmapped sentinel).
        /// u16 avoids overflow: max stored value is 255 + 1 = 256 (fits u16).
        /// Avoids HashMap hashing overhead entirely.
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
            if s.len() != t.len() {
                return false;
            }

            // Fixed-size arrays: index by byte value (0..255).
            // Value 0 = unmapped; stored values are value + 1.
            // u16 instead of u8: byte 0xFF + 1 = 256 overflows u8 but fits u16.
            let mut map_st = [0u16; 256];
            let mut map_ts = [0u16; 256];

            for (&cs, &ct) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
                let si = cs as usize; // index into map_st
                let ti = ct as usize; // index into map_ts
                let ct_val = ct as u16 + 1; // stored value for ct (1..=256)
                let cs_val = cs as u16 + 1; // stored value for cs (1..=256)

                // Check s->t direction
                if map_st[si] == 0 {
                    map_st[si] = ct_val;
                } else if map_st[si] != ct_val {
                    return false; // conflict: cs already maps to different char
                }

                // Check t->s direction
                if map_ts[ti] == 0 {
                    map_ts[ti] = cs_val;
                } else if map_ts[ti] != cs_val {
                    return false; // conflict: ct already maps to different char
                }
            }

            true
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
