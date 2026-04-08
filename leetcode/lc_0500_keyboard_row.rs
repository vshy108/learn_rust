// LeetCode 500: Keyboard Row [Easy]
// https://leetcode.com/problems/keyboard-row/
//
// Given an array of strings words, return the words that can be typed using
// letters of the alphabet on only one row of the American QWERTY keyboard.
//
// Row 1: qwertyuiop
// Row 2: asdfghjkl
// Row 3: zxcvbnm
//
// Examples:
//   ["Hello","Alaska","Dad","Peace"] -> ["Alaska","Dad"]
//   ["omk"]                          -> []
//   ["adsdf","sfd"]                  -> ["adsdf","sfd"]
//
// Constraints:
//   1 <= words.len() <= 20
//   1 <= words[i].len() <= 100
//   words[i] consists of English letters (both lowercase and uppercase)
//
// Run:  cargo run --example lc_0500_keyboard_row
// Test: cargo test --example lc_0500_keyboard_row
// Bench: cargo test --example lc_0500_keyboard_row bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 356ns
//   AI solution: avg 352ns
//   Ratio: 1.01x (same algorithm)

/// My solution module — row lookup table.
///
/// Time:  O(n * m) where m = avg word length
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_words(words: Vec<String>) -> Vec<String> {
            let row_map: [u8; 26] = {
                let mut map = [0u8; 26];
                for &c in b"qwertyuiop" {
                    map[(c - b'a') as usize] = 0;
                }
                for &c in b"asdfghjkl" {
                    map[(c - b'a') as usize] = 1;
                }
                for &c in b"zxcvbnm" {
                    map[(c - b'a') as usize] = 2;
                }
                map
            };
            words
                .into_iter()
                .filter(|word| {
                    let bytes: Vec<u8> = word
                        .bytes()
                        .map(|b| b.to_ascii_lowercase() - b'a')
                        .collect();
                    let target = row_map[bytes[0] as usize];
                    bytes.iter().all(|&b| row_map[b as usize] == target)
                })
                .collect()
        }
    }
}

/// AI solution module — map each letter to its row, then check if all
/// letters in a word share the same row.
///
/// Time:  O(n * m)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_words(words: Vec<String>) -> Vec<String> {
            // Map each letter to row number (0, 1, or 2)
            let row_map: [u8; 26] = {
                let mut map = [0u8; 26];
                for &c in b"qwertyuiop" {
                    map[(c - b'a') as usize] = 0;
                }
                for &c in b"asdfghjkl" {
                    map[(c - b'a') as usize] = 1;
                }
                for &c in b"zxcvbnm" {
                    map[(c - b'a') as usize] = 2;
                }
                map
            };

            words
                .into_iter()
                .filter(|word| {
                    let bytes: Vec<u8> = word
                        .bytes()
                        .map(|b| b.to_ascii_lowercase() - b'a')
                        .collect();
                    let target_row = row_map[bytes[0] as usize];
                    bytes.iter().all(|&b| row_map[b as usize] == target_row)
                })
                .collect()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            let words: Vec<String> = vec!["Hello", "Alaska", "Dad", "Peace"]
                .into_iter()
                .map(String::from)
                .collect();
            assert_eq!(
                Solution::find_words(words),
                vec!["Alaska".to_string(), "Dad".to_string()]
            );
        }
        #[test]
        fn test_empty_result() {
            let words: Vec<String> = vec!["omk"].into_iter().map(String::from).collect();
            let result: Vec<String> = Vec::new();
            assert_eq!(Solution::find_words(words), result);
        }
        #[test]
        fn test_multiple() {
            let words: Vec<String> = vec!["adsdf", "sfd"].into_iter().map(String::from).collect();
            assert_eq!(
                Solution::find_words(words),
                vec!["adsdf".to_string(), "sfd".to_string()]
            );
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
    F: Fn(Vec<String>) -> Vec<String>,
{
    let iterations = 500_000;
    let words: Vec<String> = vec!["Hello", "Alaska", "Dad", "Peace"]
        .into_iter()
        .map(String::from)
        .collect();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(words.clone())));
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
    bench_solution("My solution", my_solution::Solution::find_words);
    bench_solution("AI solution", ai_solution::Solution::find_words);
}

fn main() {
    let words: Vec<String> = vec!["Hello", "Alaska", "Dad", "Peace"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{:?}", ai_solution::Solution::find_words(words));
}
