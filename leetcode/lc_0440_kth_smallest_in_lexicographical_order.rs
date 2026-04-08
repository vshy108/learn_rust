// LeetCode 440: K-th Smallest in Lexicographical Order [Hard]
// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/
//
// Given two integers n and k, return the k-th lexicographically smallest
// integer in the range [1, n].
//
// Examples:
//   n = 13, k = 2 -> 10  (lex order: [1,10,11,12,13,2,3,...,9])
//   n = 1, k = 1 -> 1
//
// Constraints:
//   1 <= k <= n <= 10^9
//
// Run:  cargo run --example lc_0440_kth_smallest_in_lexicographical_order
// Test: cargo test --example lc_0440_kth_smallest_in_lexicographical_order
// Bench: cargo test --example lc_0440_kth_smallest_in_lexicographical_order bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 246ns
//   AI solution: avg 263ns
//   Ratio: 0.94x (same algorithm)

/// My solution module — trie-based counting + greedy.
///
/// Treat [1..n] as a 10-ary prefix tree. Starting at prefix=1, count how
/// many numbers share this prefix subtree. If k >= count, skip the subtree
/// (move to sibling). Otherwise, descend into the subtree (prefix *= 10).
///
/// Time:  O(log^2 n) -- log steps, each counting takes O(log n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the k-th smallest number in lexicographical order.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound of the range [1, n]
        /// * `k` - Which number to find (1-indexed)
        ///
        /// # Returns
        ///
        /// The k-th lexicographically smallest integer
        pub fn find_kth_number(n: i32, k: i32) -> i32 {
            let n = n as i64;
            let mut k = k as i64 - 1; // 0-indexed steps
            let mut curr = 1i64;
            while k > 0 {
                // Count nodes in trie between curr and curr+1
                let mut steps = 0i64;
                let (mut first, mut last) = (curr, curr);
                while first <= n {
                    steps += last.min(n) - first + 1;
                    first *= 10;
                    last = last * 10 + 9;
                }
                if steps <= k {
                    k -= steps;
                    curr += 1;
                } else {
                    k -= 1;
                    curr *= 10;
                }
            }
            curr as i32
        }
    }
}

/// AI solution module — trie-based counting + greedy (i64 arithmetic).
///
/// Same approach, using i64 to avoid overflow when curr and next are
/// multiplied by 10 repeatedly up to 10^10.
///
/// Time:  O(log^2 n) -- log steps, each counting takes O(log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the k-th smallest number in lexicographical order.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound of the range [1, n]
        /// * `k` - Which number to find (1-indexed)
        ///
        /// # Returns
        ///
        /// The k-th lexicographically smallest integer
        pub fn find_kth_number(n: i32, k: i32) -> i32 {
            /// Count how many numbers in [1..n] have prefix `curr`.
            ///
            /// Walk level by level: at each level, the valid range is
            /// [curr, min(n, next-1)]. Multiply both by 10 to go deeper.
            fn count(mut curr: i64, n: i64) -> i64 {
                let mut next = curr + 1;
                let mut total = 0i64;
                while curr <= n {
                    total += (n - curr + 1).min(next - curr);
                    curr *= 10;
                    next *= 10;
                }
                total
            }

            let n_i64 = n as i64;
            let mut curr: i64 = 1;
            let mut k = k - 1; // curr=1 is already the 1st number

            while k > 0 {
                let cnt = count(curr, n_i64) as i32;
                if k >= cnt {
                    // Skip entire subtree rooted at `curr`
                    k -= cnt;
                    curr += 1;
                } else {
                    // Descend into this subtree
                    k -= 1;
                    curr *= 10;
                }
            }

            curr as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_n13_k2() {
            assert_eq!(Solution::find_kth_number(13, 2), 10);
        }

        #[test]
        fn test_n1_k1() {
            assert_eq!(Solution::find_kth_number(1, 1), 1);
        }

        #[test]
        fn test_n10_k3() {
            // Lex order: 1, 10, 2, 3, 4, 5, 6, 7, 8, 9
            assert_eq!(Solution::find_kth_number(10, 3), 2);
        }

        #[test]
        fn test_large_n() {
            // n=1_000_000, k=1 -> 1
            assert_eq!(Solution::find_kth_number(1_000_000, 1), 1);
        }

        #[test]
        fn test_n_equals_k() {
            // n=2, k=2 -> lex order [1,2], so 2nd is 2
            assert_eq!(Solution::find_kth_number(2, 2), 2);
        }

        #[test]
        fn test_n100_k10() {
            // Lex: 1,10,100,11,12,...,19,2,20,...
            // Wait, n=100 so: 1, 10, 100, 11, 12, 13, 14, 15, 16, 17, ...
            // k=10 -> 17
            assert_eq!(Solution::find_kth_number(100, 10), 17);
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
    F: Fn(i32, i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(1_000_000),
            std::hint::black_box(500_000),
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
    bench_solution("My solution", my_solution::Solution::find_kth_number);
    bench_solution("AI solution", ai_solution::Solution::find_kth_number);
}

fn main() {
    let result = ai_solution::Solution::find_kth_number(13, 2);
    println!("K-th smallest: {}", result);
}
