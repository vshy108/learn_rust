// LeetCode 187: Repeated DNA Sequences [Medium]
// https://leetcode.com/problems/repeated-dna-sequences/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 26.671us
//   AI solution avg: 51.503us
//   Ratio: My 1.93x faster
//
// Given a string s representing a DNA sequence (chars A, C, G, T), return
// all the 10-letter-long sequences that occur more than once.
//
// Constraints: 1 <= s.length <= 10^5, s[i] is 'A', 'C', 'G', or 'T'
//
// Run:  cargo run --example lc_0187_repeated_dna_sequences
// Test: cargo test --example lc_0187_repeated_dna_sequences
// Bench: cargo test --example lc_0187_repeated_dna_sequences bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all 10-letter sequences that appear more than once.
        pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
            use std::collections::HashMap;
            if s.len() < 10 {
                return vec![];
            }
            let mut counts: HashMap<&str, usize> = HashMap::new();
            for i in 0..=s.len() - 10 {
                *counts.entry(&s[i..i + 10]).or_insert(0) += 1;
            }
            counts
                .into_iter()
                .filter(|&(_, c)| c > 1)
                .map(|(k, _)| k.to_string())
                .collect()
        }
    }
}

/// AI solution — sliding window with HashSet/HashMap.
///
/// Track seen substrings; add to result on second occurrence.
/// Uses a HashSet for "seen once" and another for "added to result".
///
/// Time: O(n) — each 10-char window is O(1) to hash with rolling.
/// Space: O(n) for the hash sets.
mod ai_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Sliding window with two sets to find repeated 10-letter sequences.
        pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
            if s.len() < 10 {
                return vec![];
            }
            let mut seen = HashSet::new();
            let mut repeated = HashSet::new();
            let bytes = s.as_bytes();

            for i in 0..=bytes.len() - 10 {
                let window = &s[i..i + 10];
                if !seen.insert(window) {
                    // Already seen once — it's a repeat
                    repeated.insert(window.to_string());
                }
            }
            repeated.into_iter().collect()
        }
    }
}

fn main() {
    let my = my_solution::Solution::find_repeated_dna_sequences(
        "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into(),
    );
    let ai = ai_solution::Solution::find_repeated_dna_sequences(
        "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into(),
    );
    println!("repeated: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    macro_rules! test_dna {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_example1() {
                    let result = <$solution>::find_repeated_dna_sequences(
                        "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into(),
                    );
                    assert_eq!(sorted(result), vec!["AAAAACCCCC", "CCCCCAAAAA"],);
                }

                #[test]
                fn test_all_same() {
                    let result = <$solution>::find_repeated_dna_sequences("AAAAAAAAAAAAA".into());
                    assert_eq!(result, vec!["AAAAAAAAAA"]);
                }

                #[test]
                fn test_too_short() {
                    let result = <$solution>::find_repeated_dna_sequences("ACGT".into());
                    assert!(result.is_empty());
                }

                #[test]
                fn test_no_repeats() {
                    let result = <$solution>::find_repeated_dna_sequences("ACGTACGTACG".into());
                    assert!(result.is_empty());
                }
            }
        };
    }

    test_dna!(my, crate::my_solution::Solution);
    test_dna!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a realistic DNA string
        let s: String = (0..1000)
            .map(|i| match i % 4 {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                _ => 'T',
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_repeated_dna_sequences(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_repeated_dna_sequences(
                std::hint::black_box(s.clone()),
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
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
