// LeetCode 481: Magical String [Medium]
// https://leetcode.com/problems/magical-string/
//
// A magical string s consists of only '1' and '2' and obeys the following rules:
// The string is magical because concatenating the number of contiguous
// occurrences of characters '1' and '2' generates the string itself.
//
// The first few elements: s = "1221121221221121122..."
// Groups: 1, 22, 11, 2, 1, 22, 1, 22, 11, ...
// Counts: 1, 2,  2,  1, 1, 2,  1, 2,  2, ... -> which is s itself!
//
// Given n, return the number of '1's in the first n characters of the magical string.
//
// Examples:
//   n = 6  -> 3  (first 6 chars: "122112", three 1's)
//   n = 1  -> 1
//
// Constraints:
//   1 <= n <= 10^5
//
// Run:  cargo run --example lc_0481_magical_string
// Test: cargo test --example lc_0481_magical_string
// Bench: cargo test --example lc_0481_magical_string bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 133ns
//   AI solution: avg 133ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — self-generating sequence.
///
/// Time:  O(n)
/// Space: O(n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn magical_string(n: i32) -> i32 {
            let n = n as usize;
            if n == 0 {
                return 0;
            }
            if n <= 3 {
                return 1;
            }
            let mut s = Vec::with_capacity(n);
            s.extend_from_slice(&[1u8, 2, 2]);
            let mut i = 2usize;
            let mut next_digit = 1u8;
            while s.len() < n {
                let count = s[i] as usize;
                for _ in 0..count {
                    if s.len() < n {
                        s.push(next_digit);
                    }
                }
                next_digit = 3 - next_digit;
                i += 1;
            }
            s[..n].iter().filter(|&&x| x == 1).count() as i32
        }
    }
}

/// AI solution module — simulate building the magical string.
///
/// Maintain a pointer `i` that tells us how many of the current digit
/// to append. Start with "122" and keep growing.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn magical_string(n: i32) -> i32 {
            let n = n as usize;
            if n == 0 {
                return 0;
            }
            if n <= 3 {
                return 1;
            }

            let mut s = Vec::with_capacity(n);
            s.extend_from_slice(&[1u8, 2, 2]);
            // i is our "count pointer" into s
            let mut i = 2usize;
            // The next digit to append (alternates between 1 and 2)
            let mut next_digit = 1u8;

            while s.len() < n {
                let count = s[i] as usize;
                for _ in 0..count {
                    if s.len() < n {
                        s.push(next_digit);
                    }
                }
                // Alternate the digit
                next_digit = 3 - next_digit;
                i += 1;
            }

            s[..n].iter().filter(|&&x| x == 1).count() as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_6() {
            assert_eq!(Solution::magical_string(6), 3);
        }
        #[test]
        fn test_1() {
            assert_eq!(Solution::magical_string(1), 1);
        }
        #[test]
        fn test_10() {
            assert_eq!(Solution::magical_string(10), 5);
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
    F: Fn(i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(100)));
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
    bench_solution("My solution", my_solution::Solution::magical_string);
    bench_solution("AI solution", ai_solution::Solution::magical_string);
}

fn main() {
    println!("{}", ai_solution::Solution::magical_string(6));
}
