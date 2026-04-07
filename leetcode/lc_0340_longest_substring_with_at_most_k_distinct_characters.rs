// @lc app=leetcode id=340 lang=rust
// #[Medium] - Longest Substring with At Most K Distinct Characters
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.013us
// ai_solution  | avg: 0.011us
// Performance: AI 1.13x faster

/// # 340. Longest Substring with At Most K Distinct Characters
///
/// Given string `s` and integer `k`, return the length of the longest
/// substring containing at most `k` distinct characters.
///
/// Constraints:
/// - 1 <= s.length <= 5 * 10^4
/// - 0 <= k <= 50

/// ## My Solution — Sliding window with [u8; 128] count array
///
/// Expand right pointer, shrink left when distinct count exceeds k.
/// Use a fixed-size array for O(1) character lookups.
///
/// - Time: O(n)
/// - Space: O(1) (128-byte array)
mod my_solution {
    pub fn length_of_longest_substring_k_distinct(s: &str, k: usize) -> usize {
        if k == 0 {
            return 0;
        }
        let bytes = s.as_bytes();
        let mut count = [0u32; 128];
        let mut distinct = 0usize;
        let mut left = 0;
        let mut ans = 0;
        for right in 0..bytes.len() {
            let c = bytes[right] as usize;
            if count[c] == 0 {
                distinct += 1;
            }
            count[c] += 1;
            // Shrink window until distinct <= k
            while distinct > k {
                let lc = bytes[left] as usize;
                count[lc] -= 1;
                if count[lc] == 0 {
                    distinct -= 1;
                }
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans
    }
}

/// ## AI Solution — Non-shrinking sliding window
///
/// Same sliding window, but instead of tracking max explicitly, keep
/// the window as large as possible: only advance left by 1 when
/// distinct > k. Final answer = len - left.
///
/// - Time: O(n)
/// - Space: O(1)
mod ai_solution {
    pub fn length_of_longest_substring_k_distinct(s: &str, k: usize) -> usize {
        if k == 0 {
            return 0;
        }
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut count = [0u32; 128];
        let mut distinct = 0usize;
        let mut left = 0;
        for right in 0..n {
            let c = bytes[right] as usize;
            if count[c] == 0 {
                distinct += 1;
            }
            count[c] += 1;
            // Only shrink by 1 to maintain max window size
            if distinct > k {
                let lc = bytes[left] as usize;
                count[lc] -= 1;
                if count[lc] == 0 {
                    distinct -= 1;
                }
                left += 1;
            }
        }
        n - left
    }
}

fn run_tests(f: fn(&str, usize) -> usize) {
    assert_eq!(f("eceba", 2), 3);    // "ece"
    assert_eq!(f("aa", 1), 2);       // "aa"
    assert_eq!(f("a", 0), 0);        // k=0 => nothing
    assert_eq!(f("abcabcabc", 3), 9); // all 3 distinct
    assert_eq!(f("aabbcc", 2), 4);   // "aabb" or "bbcc"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::length_of_longest_substring_k_distinct);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::length_of_longest_substring_k_distinct);
    }
}

fn main() {
    let s = "eceba";
    let k = 2;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(
            my_solution::length_of_longest_substring_k_distinct(
                std::hint::black_box(s),
                std::hint::black_box(k),
            ),
        );
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(
            my_solution::length_of_longest_substring_k_distinct(
                std::hint::black_box(s),
                std::hint::black_box(k),
            ),
        );
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(
            ai_solution::length_of_longest_substring_k_distinct(
                std::hint::black_box(s),
                std::hint::black_box(k),
            ),
        );
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(
            ai_solution::length_of_longest_substring_k_distinct(
                std::hint::black_box(s),
                std::hint::black_box(k),
            ),
        );
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 340 - Longest Substring with At Most K Distinct Characters");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
