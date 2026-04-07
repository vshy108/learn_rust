// LeetCode 96: Unique Binary Search Trees [Medium]
// https://leetcode.com/problems/unique-binary-search-trees/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.908us
//   AI solution avg: 0.157us
//   Ratio: AI 18.52x faster
//
// Given an integer n, return the number of structurally unique BSTs that
// store values 1 to n. (Catalan number)
//
// Constraints: 1 <= n <= 19
//
// Run:  cargo run --example lc_0096_unique_bst
// Test: cargo test --example lc_0096_unique_bst
// Bench: cargo test --example lc_0096_unique_bst bench -- --nocapture

/// My solution — DP computing Catalan numbers.
///
/// dp[i] = sum of dp[j-1] * dp[i-j] for j in 1..=i.
///
/// Time: O(n^2).
/// Space: O(n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count structurally unique BSTs with n nodes.
        ///
        /// # Arguments
        /// * `n` - number of nodes
        ///
        /// # Returns
        /// Number of unique BSTs
        pub fn num_trees(n: i32) -> i32 {
            let n = n as usize;
            let mut dp = vec![0i32; n + 1];
            dp[0] = 1;
            dp[1] = 1;

            for i in 2..=n {
                for j in 1..=i {
                    dp[i] += dp[j - 1] * dp[i - j];
                }
            }
            dp[n]
        }
    }
}

/// AI solution — direct Catalan number formula.
///
/// C(n) = (2n)! / ((n+1)! * n!) = product of (n+1+k)/k for k=2..=n.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count structurally unique BSTs with n nodes.
        ///
        /// # Arguments
        /// * `n` - number of nodes
        ///
        /// # Returns
        /// Number of unique BSTs
        pub fn num_trees(n: i32) -> i32 {
            // C(n) = C(2n, n) / (n + 1)
            // Compute iteratively: C(0)=1, C(i) = C(i-1) * 2*(2*i-1) / (i+1)
            let mut catalan = 1u64;
            for i in 1..=(n as u64) {
                catalan = catalan * 2 * (2 * i - 1) / (i + 1);
            }
            catalan as i32
        }
    }
}

fn main() {
    for n in 1..=10 {
        let my = my_solution::Solution::num_trees(n);
        let ai = ai_solution::Solution::num_trees(n);
        println!("n={n}: my={my}, ai={ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_num_trees {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_n3() {
                    assert_eq!(<$solution>::num_trees(3), 5);
                }

                #[test]
                fn test_n1() {
                    assert_eq!(<$solution>::num_trees(1), 1);
                }

                #[test]
                fn test_n4() {
                    assert_eq!(<$solution>::num_trees(4), 14);
                }

                #[test]
                fn test_n5() {
                    assert_eq!(<$solution>::num_trees(5), 42);
                }

                #[test]
                fn test_n19() {
                    assert_eq!(<$solution>::num_trees(19), 1_767_263_190);
                }
            }
        };
    }

    test_num_trees!(my, crate::my_solution::Solution);
    test_num_trees!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::num_trees(
                std::hint::black_box(19),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::num_trees(
                std::hint::black_box(19),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
