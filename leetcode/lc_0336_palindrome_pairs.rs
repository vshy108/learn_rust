// @lc app=leetcode id=336 lang=rust
// #[Hard] - Palindrome Pairs
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.744us
// ai_solution  | avg: 0.509us
// Performance: AI 1.46x faster

/// # 336. Palindrome Pairs
///
/// Given unique strings `words`, find all pairs (i,j) where i != j and
/// words[i] + words[j] is a palindrome.
///
/// Constraints:
/// - 1 <= words.length <= 5000
/// - 0 <= words[i].length <= 300

/// ## My Solution — HashMap + split palindrome check
///
/// For each word, check all splits: if prefix is palindrome, see if
/// reversed suffix exists; if suffix is palindrome, see if reversed
/// prefix exists. O(n * L^2) where L is max word length.
///
/// - Time: O(n * L^2)
/// - Space: O(n * L)
mod my_solution {
    use std::collections::HashMap;

    pub fn palindrome_pairs(words: &[&str]) -> Vec<[usize; 2]> {
        let rev_map: HashMap<Vec<u8>, usize> = words
            .iter()
            .enumerate()
            .map(|(i, &w)| (w.bytes().rev().collect::<Vec<u8>>(), i))
            .collect();

        let mut ans = Vec::new();
        for (i, &w) in words.iter().enumerate() {
            let wb = w.as_bytes();
            let m = wb.len();
            for j in 0..=m {
                let prefix = &wb[..j];
                let suffix = &wb[j..];
                // If suffix is palindrome, look up reversed prefix
                if is_palindrome(suffix) {
                    if let Some(&k) = rev_map.get(prefix) {
                        if k != i {
                            ans.push([i, k]);
                        }
                    }
                }
                // If prefix is palindrome (j > 0 to avoid double-counting
                // full-word case), look up reversed suffix
                if j > 0 && is_palindrome(prefix) {
                    if let Some(&k) = rev_map.get(suffix) {
                        if k != i {
                            ans.push([k, i]);
                        }
                    }
                }
            }
        }
        ans
    }

    fn is_palindrome(s: &[u8]) -> bool {
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

/// ## AI Solution — HashMap of originals + split original word
///
/// Store original words in a map. For each word, check every split
/// of the *original* word: if suffix is palindrome + reversed prefix
/// exists => [i, k]; if prefix is palindrome + reversed suffix exists
/// => [k, i]. Uses `j > 0` guard to avoid double-counting the full-
/// reverse pair.
///
/// - Time: O(n * L^2)
/// - Space: O(n * L)
mod ai_solution {
    use std::collections::HashMap;

    pub fn palindrome_pairs(words: &[&str]) -> Vec<[usize; 2]> {
        // Map reversed words to their index
        let rev_map: HashMap<&[u8], usize> = words
            .iter()
            .enumerate()
            .map(|(i, w)| (w.as_bytes(), i))
            .collect();

        let mut ans = Vec::new();
        for (i, &w) in words.iter().enumerate() {
            let wb = w.as_bytes();
            let m = wb.len();
            let rev: Vec<u8> = wb.iter().rev().copied().collect();
            for j in 0..=m {
                let prefix = &rev[m - j..]; // reversed prefix of w[0..j]
                let suffix = &rev[..m - j]; // reversed suffix of w[j..m]
                // If w[j..] is palindrome, look up reversed w[0..j]
                if is_palindrome(&wb[j..]) {
                    if let Some(&k) = rev_map.get(prefix) {
                        if k != i {
                            ans.push([i, k]);
                        }
                    }
                }
                // If w[0..j] is palindrome, look up reversed w[j..m]
                if j > 0 && is_palindrome(&wb[..j]) {
                    if let Some(&k) = rev_map.get(suffix) {
                        if k != i {
                            ans.push([k, i]);
                        }
                    }
                }
            }
        }
        ans
    }

    fn is_palindrome(s: &[u8]) -> bool {
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
fn sorted(mut v: Vec<[usize; 2]>) -> Vec<[usize; 2]> {
    v.sort_unstable();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[&str]) -> Vec<[usize; 2]>) {
        // Example 1
        let result = sorted(f(&["abcd", "dcba", "lls", "s", "sssll"]));
        assert_eq!(result, sorted(vec![[0, 1], [1, 0], [3, 2], [2, 4]]));
        // Example 2
        let result = sorted(f(&["bat", "tab", "cat"]));
        assert_eq!(result, sorted(vec![[0, 1], [1, 0]]));
        // Example 3: empty string
        let result = sorted(f(&["a", ""]));
        assert_eq!(result, sorted(vec![[0, 1], [1, 0]]));
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::palindrome_pairs);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::palindrome_pairs);
    }
}

fn main() {
    let words: Vec<&str> = vec!["abcd", "dcba", "lls", "s", "sssll"];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::palindrome_pairs(std::hint::black_box(&words)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::palindrome_pairs(std::hint::black_box(&words)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::palindrome_pairs(std::hint::black_box(&words)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::palindrome_pairs(std::hint::black_box(&words)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 336 - Palindrome Pairs");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
