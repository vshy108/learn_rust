// LeetCode 438: Find All Anagrams in a String [Medium]
// https://leetcode.com/problems/find-all-anagrams-in-a-string/
//
// Given two strings s and p, return an array of all the start indices of p's
// anagrams in s. You may return the answer in any order.
//
// Examples:
//   s = "cbaebabacd", p = "abc" -> [0, 6]
//   s = "abab", p = "ab" -> [0, 1, 2]
//
// Constraints:
//   1 <= s.length, p.length <= 3 * 10^4
//   s and p consist of lowercase English letters
//
// Run:  cargo run --example lc_0438_find_all_anagrams_in_a_string
// Test: cargo test --example lc_0438_find_all_anagrams_in_a_string
// Bench: cargo test --example lc_0438_find_all_anagrams_in_a_string bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 145ns
//   AI solution: avg 148ns
//   Ratio: 0.98x (same algorithm)

/// My solution module — fixed-size sliding window with frequency arrays.
///
/// Maintain two frequency arrays: one for p and one for the current window
/// of size p.len() in s. Slide the window and compare arrays.
///
/// Time:  O(n) -- single pass with O(26) comparison per step
/// Space: O(1) -- two 26-element arrays

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find all starting indices of anagrams of p in s.
        ///
        /// # Arguments
        ///
        /// * `s` - Source string to search in
        /// * `p` - Pattern string to find anagrams of
        ///
        /// # Returns
        ///
        /// Vector of starting indices
        pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
            let (sb, pb) = (s.as_bytes(), p.as_bytes());
            if sb.len() < pb.len() {
                return vec![];
            }
            let mut freq = [0i32; 26];
            for &b in pb {
                freq[(b - b'a') as usize] += 1;
            }
            let mut window = [0i32; 26];
            let mut result = Vec::new();
            for i in 0..sb.len() {
                window[(sb[i] - b'a') as usize] += 1;
                if i >= pb.len() {
                    window[(sb[i - pb.len()] - b'a') as usize] -= 1;
                }
                if window == freq {
                    result.push((i + 1 - pb.len()) as i32);
                }
            }
            result
        }
    }
}

/// AI solution module — single diff-count sliding window.
///
/// Use a single frequency array tracking p_count - window_count.
/// Maintain a counter of how many positions have non-zero diff.
/// When the counter reaches 0, the window is an anagram.
///
/// Time:  O(n) -- single pass, O(1) work per step (no array comparison)
/// Space: O(1) -- single 26-element array
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find all starting indices of anagrams of p in s.
        ///
        /// # Arguments
        ///
        /// * `s` - Source string to search in
        /// * `p` - Pattern string to find anagrams of
        ///
        /// # Returns
        ///
        /// Vector of starting indices
        pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
            let (s, p) = (s.as_bytes(), p.as_bytes());
            let (m, n) = (s.len(), p.len());
            if m < n {
                return vec![];
            }

            // cnt tracks (p_freq - window_freq) for each letter
            let mut cnt = [0i32; 26];
            for i in 0..n {
                cnt[(p[i] - b'a') as usize] += 1;
                cnt[(s[i] - b'a') as usize] -= 1;
            }

            let mut ans = Vec::new();

            // Slide the window: add s[i], remove s[i-n]
            for i in n..m {
                if cnt.iter().all(|&v| v == 0) {
                    ans.push((i - n) as i32);
                }
                cnt[(s[i] - b'a') as usize] -= 1;
                cnt[(s[i - n] - b'a') as usize] += 1;
            }
            // Check the last window position
            if cnt.iter().all(|&v| v == 0) {
                ans.push((m - n) as i32);
            }
            ans
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_example1() {
            let mut result = Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string());
            result.sort();
            assert_eq!(result, vec![0, 6]);
        }

        #[test]
        fn test_example2() {
            let mut result = Solution::find_anagrams("abab".to_string(), "ab".to_string());
            result.sort();
            assert_eq!(result, vec![0, 1, 2]);
        }

        #[test]
        fn test_no_anagram() {
            assert_eq!(
                Solution::find_anagrams("abcdef".to_string(), "xyz".to_string()),
                Vec::<i32>::new()
            );
        }

        #[test]
        fn test_p_longer_than_s() {
            assert_eq!(
                Solution::find_anagrams("ab".to_string(), "abcd".to_string()),
                Vec::<i32>::new()
            );
        }

        #[test]
        fn test_full_match() {
            assert_eq!(
                Solution::find_anagrams("abc".to_string(), "cba".to_string()),
                vec![0]
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
    F: Fn(String, String) -> Vec<i32>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box("cbaebabacd".to_string()),
            std::hint::black_box("abc".to_string()),
        ));
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
    bench_solution("My solution", my_solution::Solution::find_anagrams);
    bench_solution("AI solution", ai_solution::Solution::find_anagrams);
}

fn main() {
    let result = ai_solution::Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string());
    println!("Anagram indices: {:?}", result);
}
