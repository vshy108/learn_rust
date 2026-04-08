// LeetCode 484: Find Permutation [Medium]
// https://leetcode.com/problems/find-permutation/
//
// A permutation of n integers can be represented as a string of length n-1
// where s[i] == 'I' if perm[i] < perm[i+1] and s[i] == 'D' if perm[i] > perm[i+1].
// Given string s, reconstruct the lexicographically smallest permutation.
//
// Examples:
//   s = "I" -> [1,2]
//   s = "DI" -> [2,1,3]
//
// Constraints:
//   1 <= s.length <= 10^5
//   s[i] is either 'I' or 'D'
//
// Run:  cargo run --example lc_0484_find_permutation
// Test: cargo test --example lc_0484_find_permutation
// Bench: cargo test --example lc_0484_find_permutation bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 162ns
//   AI solution: avg 280ns
//   Ratio: 0.58x (greedy reverse faster than stack)

/// Time: O(n), Space: O(n)
/// Greedy: start with [1,2,...,n+1], then reverse each contiguous
/// run of 'D' characters (plus one trailing position).
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn find_permutation(s: String) -> Vec<i32> {
            let n = s.len();
            let chars: Vec<u8> = s.into_bytes();
            // Start with identity permutation [1, 2, ..., n+1]
            let mut ans: Vec<i32> = (1..=(n + 1) as i32).collect();
            let mut i = 0;
            while i < n {
                // Find end of contiguous 'D' run
                let mut j = i;
                while j < n && chars[j] == b'D' {
                    j += 1;
                }
                // Reverse the segment [i..j] (inclusive)
                ans[i..=j].reverse();
                // Skip past the D-run; if no D, move forward by 1
                i = if j > i { j } else { i + 1 };
            }
            ans
        }
    }
}

/// Time: O(n), Space: O(n)
/// Stack-based approach: push numbers 1..n+1 onto a stack.
/// Whenever we see 'I' or reach the end, pop everything out
/// to the result (this naturally reverses D-runs).
#[allow(dead_code)]
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn find_permutation(s: String) -> Vec<i32> {
            let n = s.len();
            let chars: Vec<u8> = s.into_bytes();
            let mut result = Vec::with_capacity(n + 1);
            let mut stack: Vec<i32> = Vec::new();
            for i in 1..=(n + 1) as i32 {
                stack.push(i);
                // Flush stack at 'I' or at the end (index n)
                if i as usize == n + 1 || chars[i as usize - 1] == b'I' {
                    while let Some(val) = stack.pop() {
                        result.push(val);
                    }
                }
            }
            result
        }
    }
}

fn main() {
    println!("LeetCode 484: Find Permutation");
    let cases = vec![
        ("I".to_string(), vec![1, 2]),
        ("DI".to_string(), vec![2, 1, 3]),
    ];
    for (s, expected) in &cases {
        let r1 = my_solution::Solution::find_permutation(s.clone());
        let r2 = ai_solution::Solution::find_permutation(s.clone());
        println!("  s=\"{s}\" => my={r1:?}, ai={r2:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $s:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(
                        my_solution::Solution::find_permutation($s.to_string()),
                        $expected
                    );
                }
                #[test]
                fn ai() {
                    assert_eq!(
                        ai_solution::Solution::find_permutation($s.to_string()),
                        $expected
                    );
                }
            }
        };
    }

    test_both!(example1, "I", vec![1, 2]);
    test_both!(example2, "DI", vec![2, 1, 3]);
    test_both!(all_increasing, "III", vec![1, 2, 3, 4]);
    test_both!(all_decreasing, "DDD", vec![4, 3, 2, 1]);
    test_both!(mixed, "DDIID", vec![3, 2, 1, 4, 6, 5]);
    test_both!(single_d, "D", vec![2, 1]);
    test_both!(long_di, "DDII", vec![3, 2, 1, 4, 5]);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> Vec<i32>,
{
    let iterations = 500_000;
    let s = "DDIIDDI".repeat(10);
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(s.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::find_permutation);
    bench_solution("AI solution", ai_solution::Solution::find_permutation);
}
