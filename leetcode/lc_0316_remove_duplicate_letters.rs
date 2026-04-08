// LeetCode 316. Remove Duplicate Letters
// https://leetcode.com/problems/remove-duplicate-letters/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.094us
//   ai_solution  avg: 0.096us
//   Ratio: my_solution 1.02x faster

/// Given a string s, remove duplicate letters so every letter appears once.
/// Result must be the smallest in lexicographical order among all possible results.
///
/// Example 1:
///   Input: s = "bcabc"
///   Output: "abc"
///
/// Example 2:
///   Input: s = "cbacdcbc"
///   Output: "acdb"
///
/// Constraints:
///   - 1 <= s.length <= 10^4
///   - s consists of lowercase English letters

/// # my_solution: Monotonic stack with last-occurrence + bitmask
/// Time: O(n) — each character pushed/popped at most once
/// Space: O(1) — stack and bitmask bounded by 26 letters
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Remove duplicate letters for smallest lexicographic result.
        ///
        /// Use a monotonic stack: for each char c, if c < stack top and
        /// stack top appears again later, pop it. Track "in stack" via bitmask.
        ///
        /// # Arguments
        /// * `s` - Input string of lowercase letters
        ///
        /// # Returns
        /// Smallest lexicographic string with each letter appearing once
        pub fn remove_duplicate_letters(s: String) -> String {
            let bytes = s.as_bytes();

            // last[c] = last index where character c appears
            let mut last = [0usize; 26];
            for (i, &b) in bytes.iter().enumerate() {
                last[(b - b'a') as usize] = i;
            }

            let mut stk: Vec<u8> = Vec::new();
            // Bitmask: bit j set means letter j is currently in the stack
            let mut mask = 0u32;

            for (i, &b) in bytes.iter().enumerate() {
                let idx = (b - b'a') as u32;
                // Skip if already in stack
                if (mask >> idx) & 1 == 1 {
                    continue;
                }
                // Pop stack top while it's > current and appears later
                while let Some(&top) = stk.last() {
                    if top > b && last[(top - b'a') as usize] > i {
                        mask ^= 1 << (top - b'a');
                        stk.pop();
                    } else {
                        break;
                    }
                }
                stk.push(b);
                mask |= 1 << idx;
            }

            // Safety: all bytes are valid ASCII lowercase
            String::from_utf8(stk).expect("stack contains only ASCII lowercase")
        }
    }
}

/// # ai_solution: Monotonic stack with count array
/// Time: O(n) — single pass with amortized O(1) stack ops
/// Space: O(1) — bounded by 26 letters
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Remove duplicate letters using remaining-count approach.
        ///
        /// Maintain a count of remaining occurrences. For each char, decrement
        /// count. If already in result, skip. Otherwise, pop from stack while
        /// stack top > current and count[top] > 0 (will appear later).
        ///
        /// # Arguments
        /// * `s` - Input string of lowercase letters
        ///
        /// # Returns
        /// Smallest lexicographic string with each letter appearing once
        pub fn remove_duplicate_letters(s: String) -> String {
            let bytes = s.as_bytes();
            let mut count = [0u32; 128]; // indexed by ASCII value
            let mut in_stack = [false; 128];

            for &b in bytes {
                count[b as usize] += 1;
            }

            let mut stk: Vec<u8> = Vec::new();

            for &b in bytes {
                count[b as usize] -= 1;
                if in_stack[b as usize] {
                    continue;
                }
                while let Some(&top) = stk.last() {
                    if top > b && count[top as usize] > 0 {
                        in_stack[top as usize] = false;
                        stk.pop();
                    } else {
                        break;
                    }
                }
                stk.push(b);
                in_stack[b as usize] = true;
            }

            String::from_utf8(stk).expect("stack contains only ASCII lowercase")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(String) -> String) {
        // Example 1
        assert_eq!(solve("bcabc".to_string()), "abc");

        // Example 2
        assert_eq!(solve("cbacdcbc".to_string()), "acdb");

        // Single character
        assert_eq!(solve("a".to_string()), "a");

        // Already unique and sorted
        assert_eq!(solve("abc".to_string()), "abc");

        // Reverse sorted, unique
        assert_eq!(solve("cba".to_string()), "cba");

        // All same characters
        assert_eq!(solve("aaaa".to_string()), "a");

        // Two characters
        assert_eq!(solve("ba".to_string()), "ba");
        assert_eq!(solve("abba".to_string()), "ab");

        // Longer example
        assert_eq!(solve("abacb".to_string()), "abc");
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::remove_duplicate_letters);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::remove_duplicate_letters);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let s = "cbacdcbc".to_string();
    println!("Input: s = \"{}\"", s);
    println!(
        "Output (my): \"{}\"",
        my_solution::Solution::remove_duplicate_letters(s.clone())
    );
    println!(
        "Output (ai): \"{}\"",
        ai_solution::Solution::remove_duplicate_letters(s.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::remove_duplicate_letters(black_box(
            s.clone(),
        )));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::remove_duplicate_letters(black_box(
            s.clone(),
        )));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
