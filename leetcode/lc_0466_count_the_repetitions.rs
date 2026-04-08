// LeetCode 466: Count The Repetitions [Hard]
// https://leetcode.com/problems/count-the-repetitions/
//
// Define str = [s, n] as n copies of s concatenated.
// Given s1, n1, s2, n2, find the max integer m such that
// [str2, m] can be obtained from str1 (by deleting chars).
//
// Examples:
//   s1="acb", n1=4, s2="ab", n2=2 -> 2
//   s1="acb", n1=1, s2="acb", n2=1 -> 1
//
// Constraints:
//   1 <= s1.length, s2.length <= 100
//   s1, s2 consist of lowercase English letters
//   1 <= n1, n2 <= 10^6
//
// Run:  cargo run --example lc_0466_count_the_repetitions
// Test: cargo test --example lc_0466_count_the_repetitions
// Bench: cargo test --example lc_0466_count_the_repetitions bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 72ns  (brute force simulation)
//   AI solution: avg 113ns (preprocessing + iteration)
//   Ratio: 0.64x (My 1.6x faster on small input)

/// My solution module — brute force simulation.
///
/// Walk through s1 repeated n1 times, counting how many complete s2
/// subsequences we can find, then divide by n2.
///
/// Time:  O(n1 * len(s1))
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            let mut s2_count = 0; // number of full s2 matched
            let mut j = 0; // position in s2

            // Simulate going through s1 repeated n1 times
            for _ in 0..n1 {
                for &ch in b1 {
                    if ch == b2[j] {
                        j += 1;
                        if j == b2.len() {
                            s2_count += 1;
                            j = 0;
                        }
                    }
                }
            }
            s2_count / n2
        }
    }
}

/// AI solution module — preprocessing + iteration (doocs approach).
///
/// For each possible starting position i in s2, precompute how many
/// complete s2 we get from one copy of s1, and the ending position.
/// Then iterate n1 times using the precomputed table.
///
/// Time:  O(len(s2) * len(s1) + n1)
/// Space: O(len(s2))
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            let n = b2.len();

            // d[i] = (count, next_j): starting at position i in s2,
            // after scanning one full s1, we complete `count` full s2s
            // and end at position next_j in s2.
            let mut d: Vec<(i32, usize)> = Vec::with_capacity(n);
            for i in 0..n {
                let mut cnt = 0;
                let mut j = i;
                for &ch in b1 {
                    if ch == b2[j] {
                        j += 1;
                        if j == n {
                            cnt += 1;
                            j = 0;
                        }
                    }
                }
                d.push((cnt, j));
            }

            let mut ans = 0;
            let mut j = 0;
            for _ in 0..n1 {
                let (cnt, next_j) = d[j];
                ans += cnt;
                j = next_j;
            }
            ans / n2
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_acb_ab() {
            assert_eq!(
                Solution::get_max_repetitions("acb".into(), 4, "ab".into(), 2),
                2
            );
        }
        #[test]
        fn test_same_string() {
            assert_eq!(
                Solution::get_max_repetitions("acb".into(), 1, "acb".into(), 1),
                1
            );
        }
        #[test]
        fn test_no_match() {
            assert_eq!(
                Solution::get_max_repetitions("abc".into(), 4, "d".into(), 1),
                0
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
    F: Fn(String, i32, String, i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box("acb".into()),
            std::hint::black_box(4),
            std::hint::black_box("ab".into()),
            std::hint::black_box(2),
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
    bench_solution("My solution", my_solution::Solution::get_max_repetitions);
    bench_solution("AI solution", ai_solution::Solution::get_max_repetitions);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::get_max_repetitions("acb".into(), 4, "ab".into(), 2)
    );
}
