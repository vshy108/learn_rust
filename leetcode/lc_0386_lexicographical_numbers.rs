// LeetCode 386: Lexicographical Numbers [Medium]
// https://leetcode.com/problems/lexicographical-numbers/
//
// Benchmark: 100,000 iterations (2025-04-08)
//   My solution (iterative trie walk): avg 69.11us
//   AI solution (recursive DFS):       avg 105.09us
//   -> My solution is 1.52x faster
//
// Given an integer n, return all the numbers in the range [1, n] sorted in
// lexicographical order.
//
// You must write an algorithm that runs in O(n) time and uses O(1) extra space.
//
// Examples:
//   n = 13 -> [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
//   n = 2  -> [1, 2]
//
// Constraints:
//   - 1 <= n <= 5 * 10^4
//
// Run:  cargo run --example lc_0386_lexicographical_numbers
// Test: cargo test --example lc_0386_lexicographical_numbers
// Bench: cargo test --example lc_0386_lexicographical_numbers bench -- --nocapture

/// My solution module — iterative "next lexicographic number" traversal.
///
/// Walk the implicit 10-ary trie of numbers 1..n:
///   - If v*10 <= n, go deeper (v *= 10).
///   - Otherwise, go to the next sibling or backtrack:
///     while v%10 == 9 or v+1 > n, backtrack (v /= 10), then v += 1.
///
/// Translated from Rust solution in doocs/leetcode.
///
/// Time:  O(n)
/// Space: O(1) extra (output vec excluded)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all numbers in [1, n] in lexicographical order.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound of the range
        ///
        /// # Returns
        ///
        /// Numbers 1..n sorted lexicographically
        pub fn lexical_order(n: i32) -> Vec<i32> {
            let mut ans = Vec::with_capacity(n as usize);
            let mut v = 1;
            for _ in 0..n {
                ans.push(v);
                if v * 10 <= n {
                    // Go one level deeper in the trie
                    v *= 10;
                } else {
                    // Backtrack past trailing 9s or past n
                    while v % 10 == 9 || v + 1 > n {
                        v /= 10;
                    }
                    v += 1;
                }
            }
            ans
        }
    }
}

/// AI solution module — recursive DFS approach.
///
/// DFS from each root digit 1..9, appending digits 0..9 at each level.
/// This is conceptually cleaner (explicit trie traversal) but uses O(log n)
/// stack depth due to recursion.
///
/// Time:  O(n)
/// Space: O(log n) recursion stack
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all numbers in [1, n] in lexicographical order.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound of the range
        ///
        /// # Returns
        ///
        /// Numbers 1..n sorted lexicographically
        pub fn lexical_order(n: i32) -> Vec<i32> {
            let mut ans = Vec::with_capacity(n as usize);
            // Start DFS from each single-digit root: 1, 2, ..., 9
            for start in 1..10 {
                Self::dfs(start, n, &mut ans);
            }
            ans
        }

        /// DFS: add `cur` to result, then recurse on cur*10+0 .. cur*10+9.
        fn dfs(cur: i32, n: i32, ans: &mut Vec<i32>) {
            if cur > n {
                return;
            }
            ans.push(cur);
            // Try appending each digit 0-9
            for d in 0..10 {
                let next = cur * 10 + d;
                if next > n {
                    return; // No point trying larger digits
                }
                Self::dfs(next, n, ans);
            }
        }
    }
}

fn main() {
    let cases = [13, 2, 25];

    println!("--- My Solution ---");
    for &n in &cases {
        let result = my_solution::Solution::lexical_order(n);
        println!("  n={n} -> {result:?}");
    }

    println!("--- AI Solution ---");
    for &n in &cases {
        let result = ai_solution::Solution::lexical_order(n);
        println!("  n={n} -> {result:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_lexical_order {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::lexical_order(13),
                        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::lexical_order(2), vec![1, 2]);
                }

                #[test]
                fn test_single() {
                    assert_eq!(Solution::lexical_order(1), vec![1]);
                }

                #[test]
                fn test_ten() {
                    assert_eq!(
                        Solution::lexical_order(10),
                        vec![1, 10, 2, 3, 4, 5, 6, 7, 8, 9]
                    );
                }

                #[test]
                fn test_twenty_five() {
                    let result = Solution::lexical_order(25);
                    assert_eq!(result.len(), 25);
                    assert_eq!(result[0], 1);
                    assert_eq!(result[1], 10);
                    // Verify it's actually sorted lexicographically
                    let strs: Vec<String> =
                        result.iter().map(|x| x.to_string()).collect();
                    let mut sorted = strs.clone();
                    sorted.sort();
                    assert_eq!(strs, sorted);
                }
            }
        };
    }

    test_lexical_order!(my_tests, crate::my_solution::Solution);
    test_lexical_order!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let n = 50_000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::lexical_order(
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::lexical_order(
                std::hint::black_box(n),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
