// LeetCode 288: Unique Word Abbreviation [Medium] (Premium)
// https://leetcode.com/problems/unique-word-abbreviation/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 0.622us
//   AI solution avg: 0.568us
//   Ratio: AI 1.09x faster
//
// The abbreviation of a word is: first letter + count of middle chars + last letter.
// Words with length < 3 abbreviate to themselves. E.g. dog -> d1g, it -> it.
//
// Implement ValidWordAbbr:
//   - ValidWordAbbr(dictionary) — init with a dictionary of words.
//   - is_unique(word) -> bool — true if either:
//     (a) No word in dictionary has the same abbreviation, OR
//     (b) All words in dictionary with that abbreviation are the same as word.
//
// Example:
//   dict = ["deer","door","cake","card"]
//   is_unique("dear") -> false  ("deer" has same abbr "d2r" but different word)
//   is_unique("cart") -> true   (no word has abbr "c2t")
//   is_unique("cane") -> false  ("cake" has same abbr "c2e" but different word)
//   is_unique("make") -> true   (no word has abbr "m2e")
//   is_unique("cake") -> true   ("cake" is already in dict, only one with "c2e")
//
// Constraints:
//   - 1 <= dictionary.length <= 3 * 10^4
//   - 1 <= dictionary[i].length <= 20
//   - 1 <= word.length <= 20
//   - At most 5000 calls to is_unique
//
// Run:  cargo run --example lc_0288_unique_word_abbreviation
// Test: cargo test --example lc_0288_unique_word_abbreviation
// Bench: cargo test --example lc_0288_unique_word_abbreviation bench -- --nocapture

use std::collections::{HashMap, HashSet};

/// Compute the abbreviation of a word.
fn abbr(s: &str) -> String {
    let n = s.len();
    if n < 3 {
        s.to_string()
    } else {
        // first char + (len - 2) + last char
        let bytes = s.as_bytes();
        format!("{}{}{}", bytes[0] as char, n - 2, bytes[n - 1] as char)
    }
}

/// My solution — HashMap<String, HashSet<String>> (doocs approach).
///
/// Store a set of words for each abbreviation. is_unique checks if the
/// abbreviation is absent or the set contains only the query word itself.
///
/// Time:  O(n) init, O(1) per query
/// Space: O(n)
mod my_solution {
    use super::*;

    pub struct ValidWordAbbr {
        d: HashMap<String, HashSet<String>>,
    }

    impl ValidWordAbbr {
        pub fn new(dictionary: &[&str]) -> Self {
            let mut d: HashMap<String, HashSet<String>> = HashMap::new();
            for &s in dictionary {
                d.entry(abbr(s)).or_default().insert(s.to_string());
            }
            ValidWordAbbr { d }
        }

        pub fn is_unique(&self, word: &str) -> bool {
            let a = abbr(word);
            match self.d.get(&a) {
                None => true,
                Some(set) => set.len() == 1 && set.contains(word),
            }
        }
    }
}

/// AI solution — HashMap<String, Option<String>> (single-word or conflict).
///
/// For each abbreviation, store Some(word) if exactly one unique word maps
/// to it, or None if multiple different words share it. This avoids the
/// overhead of maintaining a HashSet per abbreviation.
///
/// Time:  O(n) init, O(1) per query
/// Space: O(n)
mod ai_solution {
    use super::*;

    pub struct ValidWordAbbr {
        /// None means conflict (multiple different words share this abbreviation)
        d: HashMap<String, Option<String>>,
    }

    impl ValidWordAbbr {
        pub fn new(dictionary: &[&str]) -> Self {
            let mut d: HashMap<String, Option<String>> = HashMap::new();
            for &s in dictionary {
                let a = abbr(s);
                d.entry(a)
                    .and_modify(|existing| {
                        // If existing is Some(w) and w != s, mark as conflict
                        if let Some(w) = existing {
                            if w != s {
                                *existing = None;
                            }
                        }
                        // If already None (conflict), stays None
                    })
                    .or_insert_with(|| Some(s.to_string()));
            }
            ValidWordAbbr { d }
        }

        pub fn is_unique(&self, word: &str) -> bool {
            let a = abbr(word);
            match self.d.get(&a) {
                None => true, // no word in dict has this abbreviation
                Some(Some(w)) => w == word, // only one unique word, must match
                Some(None) => false, // conflict: multiple different words
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let dict = vec!["deer", "door", "cake", "card"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        // "dear" -> d2r, same abbr as "deer" and "door" -> false
        assert!(!my.is_unique("dear"), "my: dear");
        assert!(!ai.is_unique("dear"), "ai: dear");

        // "cart" -> c2t, no match -> true
        assert!(my.is_unique("cart"), "my: cart");
        assert!(ai.is_unique("cart"), "ai: cart");

        // "cane" -> c2e, same abbr as "cake" but different word -> false
        assert!(!my.is_unique("cane"), "my: cane");
        assert!(!ai.is_unique("cane"), "ai: cane");

        // "make" -> m2e, no match -> true
        assert!(my.is_unique("make"), "my: make");
        assert!(ai.is_unique("make"), "ai: make");

        // "cake" -> c2e, in dict and only "cake" has this abbr among dict -> true
        // Wait: "card" -> c2d, "cake" -> c2e. Only "cake" maps to c2e.
        assert!(my.is_unique("cake"), "my: cake");
        assert!(ai.is_unique("cake"), "ai: cake");
    }

    #[test]
    fn test_short_words() {
        let dict = vec!["it", "go"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        // "it" is in dict, abbr is "it" -> only "it" maps to it -> true
        assert!(my.is_unique("it"), "my: it");
        assert!(ai.is_unique("it"), "ai: it");

        // "at" -> abbr "at", not in dict abbr map -> true
        assert!(my.is_unique("at"), "my: at");
        assert!(ai.is_unique("at"), "ai: at");
    }

    #[test]
    fn test_single_char() {
        let dict = vec!["a"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        assert!(my.is_unique("a"), "my: a");
        assert!(ai.is_unique("a"), "ai: a");
        assert!(my.is_unique("b"), "my: b");
        assert!(ai.is_unique("b"), "ai: b");
    }

    #[test]
    fn test_duplicate_in_dict() {
        // Same word appearing multiple times in dictionary
        let dict = vec!["hello", "hello"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        assert!(my.is_unique("hello"), "my: hello");
        assert!(ai.is_unique("hello"), "ai: hello");
    }

    #[test]
    fn test_empty_after_abbreviation_conflict() {
        // "abc" and "adc" both abbreviate to "a1c"
        let dict = vec!["abc", "adc"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        assert!(!my.is_unique("abc"), "my: abc");
        assert!(!ai.is_unique("abc"), "ai: abc");
        assert!(!my.is_unique("adc"), "my: adc");
        assert!(!ai.is_unique("adc"), "ai: adc");
        assert!(!my.is_unique("aec"), "my: aec");
        assert!(!ai.is_unique("aec"), "ai: aec");
    }

    #[test]
    fn test_no_conflict() {
        let dict = vec!["abc", "def"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        assert!(my.is_unique("abc"), "my: abc");
        assert!(ai.is_unique("abc"), "ai: abc");
        assert!(my.is_unique("def"), "my: def");
        assert!(ai.is_unique("def"), "ai: def");
        // "xyz" -> x1z, not in dict -> true
        assert!(my.is_unique("xyz"), "my: xyz");
        assert!(ai.is_unique("xyz"), "ai: xyz");
    }

    #[test]
    fn test_two_char_word() {
        let dict = vec!["ab", "cd"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        assert!(my.is_unique("ab"), "my: ab");
        assert!(ai.is_unique("ab"), "ai: ab");
        assert!(my.is_unique("ef"), "my: ef");
        assert!(ai.is_unique("ef"), "ai: ef");
    }

    #[test]
    fn test_long_word() {
        let dict = vec!["internationalization"];
        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        // "internationalization" -> i18n, unique in dict
        assert!(my.is_unique("internationalization"), "my");
        assert!(ai.is_unique("internationalization"), "ai");
    }
}

fn main() {
    println!("LeetCode 288: Unique Word Abbreviation");
    let dict = vec!["deer", "door", "cake", "card"];
    let vwa = my_solution::ValidWordAbbr::new(&dict);
    for word in &["dear", "cart", "cane", "make", "cake"] {
        println!("is_unique(\"{word}\") = {}", vwa.is_unique(word));
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let dict: Vec<&str> = vec![
            "deer", "door", "cake", "card", "hello", "world", "apple",
            "apply", "grape", "graph", "liver", "lover", "river", "rider",
        ];
        let queries = vec![
            "dear", "cart", "cane", "make", "cake", "hello", "grope",
        ];
        let iterations = 100_000;

        let my = my_solution::ValidWordAbbr::new(&dict);
        let ai = ai_solution::ValidWordAbbr::new(&dict);

        // Warm up
        for _ in 0..1_000 {
            for q in &queries {
                std::hint::black_box(my.is_unique(q));
                std::hint::black_box(ai.is_unique(q));
            }
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for q in &queries {
                std::hint::black_box(my.is_unique(q));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for q in &queries {
                std::hint::black_box(ai.is_unique(q));
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
