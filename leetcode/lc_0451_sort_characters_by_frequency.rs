// LeetCode 451: Sort Characters By Frequency [Medium]
// https://leetcode.com/problems/sort-characters-by-frequency/
//
// Given a string s, sort it in decreasing order based on the frequency of
// characters. Return the sorted string.
//
// Examples:
//   "tree" -> "eert" (or "eetr")
//   "cccaaa" -> "aaaccc" (or "cccaaa")
//   "Aabb" -> "bbAa"
//
// Constraints:
//   1 <= s.length <= 5 * 10^5
//   s consists of uppercase/lowercase English letters and digits
//
// Run:  cargo run --example lc_0451_sort_characters_by_frequency
// Test: cargo test --example lc_0451_sort_characters_by_frequency
// Bench: cargo test --example lc_0451_sort_characters_by_frequency bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 228ns
//   AI solution: avg 231ns
//   Ratio: 0.99x (same algorithm)

/// My solution module — hash map + sort by frequency.
///
/// Time:  O(n + k log k) where k = distinct chars
/// Space: O(n + k)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn frequency_sort(s: String) -> String {
            use std::collections::HashMap;
            let mut freq: HashMap<char, usize> = HashMap::new();
            for c in s.chars() {
                *freq.entry(c).or_insert(0) += 1;
            }
            let mut chars: Vec<(usize, char)> = freq.into_iter().map(|(c, f)| (f, c)).collect();
            chars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
            let mut result = String::with_capacity(s.len());
            for (f, c) in chars {
                for _ in 0..f {
                    result.push(c);
                }
            }
            result
        }
    }
}

/// AI solution module — hash map + sort by frequency (doocs Rust approach).
///
/// Time:  O(n + k log k)
/// Space: O(n + k)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn frequency_sort(s: String) -> String {
            use std::collections::HashMap;
            let mut cnt = HashMap::new();
            for c in s.chars() {
                *cnt.entry(c).or_insert(0i32) += 1;
            }
            let mut cs: Vec<(char, i32)> = cnt.into_iter().collect();
            cs.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
            cs.into_iter()
                .flat_map(|(c, v)| std::iter::repeat_n(c, v as usize))
                .collect()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_tree() {
            let r = Solution::frequency_sort("tree".to_string());
            assert!(r == "eert" || r == "eetr");
        }
        #[test]
        fn test_cccaaa() {
            let r = Solution::frequency_sort("cccaaa".to_string());
            assert!(r == "cccaaa" || r == "aaaccc");
        }
        #[test]
        fn test_aabb() {
            let r = Solution::frequency_sort("Aabb".to_string());
            assert!(r == "bbAa" || r == "bbaA" || r == "aAbb" || r == "Aabb");
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> String,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box("tree".to_string())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::frequency_sort);
    bench_solution("AI solution", ai_solution::Solution::frequency_sort);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::frequency_sort("tree".to_string())
    );
}
