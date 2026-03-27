// LeetCode 3437: Permutations III [Medium] (Premium)
// https://leetcode.com/problems/permutations-iii/
//
// Given an integer n, an alternating permutation is a permutation of the
// first n positive integers such that no two adjacent elements are both
// odd or both even.
//
// Return all such alternating permutations sorted in lexicographical order.
//
// Examples:
//   n = 4 -> [[1,2,3,4],[1,4,3,2],[2,1,4,3],[2,3,4,1],
//             [3,2,1,4],[3,4,1,2],[4,1,2,3],[4,3,2,1]]
//   n = 2 -> [[1,2],[2,1]]
//   n = 3 -> [[1,2,3],[3,2,1]]
//
// Constraints:
//   - 1 <= n <= 10
//
// Run:  cargo run --example lc_3437_permutations_iii
// Test: cargo test --example lc_3437_permutations_iii
// Bench: cargo test --example lc_3437_permutations_iii bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all alternating permutations of [1..=n] in lexicographic order.
        ///
        /// # Arguments
        ///
        /// * `n` - The upper bound of the permutation range [1, n]
        ///
        /// # Returns
        ///
        /// All valid permutations where no two adjacent elements share parity
        pub fn permute(n: i32) -> Vec<Vec<i32>> {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all alternating permutations of [1..=n] in lexicographic order.
        ///
        /// # Arguments
        ///
        /// * `n` - The upper bound of the permutation range [1, n]
        ///
        /// # Returns
        ///
        /// All valid permutations where no two adjacent elements share parity
        pub fn permute(n: i32) -> Vec<Vec<i32>> {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, Vec<Vec<i32>>)> = vec![
        (2, vec![vec![1, 2], vec![2, 1]]),
        (3, vec![vec![1, 2, 3], vec![3, 2, 1]]),
    ];

    println!("--- My Solution ---");
    for (n, expected) in &test_cases {
        let result = my_solution::Solution::permute(*n);
        println!("  n={n} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &test_cases {
        let result = ai_solution::Solution::permute(*n);
        println!("  n={n} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_permute {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_n2() {
                    assert_eq!(Solution::permute(2), vec![vec![1, 2], vec![2, 1]],);
                }

                #[test]
                fn test_n3() {
                    assert_eq!(Solution::permute(3), vec![vec![1, 2, 3], vec![3, 2, 1]],);
                }

                #[test]
                fn test_n4() {
                    assert_eq!(
                        Solution::permute(4),
                        vec![
                            vec![1, 2, 3, 4],
                            vec![1, 4, 3, 2],
                            vec![2, 1, 4, 3],
                            vec![2, 3, 4, 1],
                            vec![3, 2, 1, 4],
                            vec![3, 4, 1, 2],
                            vec![4, 1, 2, 3],
                            vec![4, 3, 2, 1],
                        ],
                    );
                }

                #[test]
                fn test_n1() {
                    assert_eq!(Solution::permute(1), vec![vec![1]]);
                }

                #[test]
                fn test_n5_count() {
                    // n=5: 3 odd + 2 even. Alternating must start with odd.
                    // Verify we get some results (exact count: 12).
                    let result = Solution::permute(5);
                    assert!(!result.is_empty());
                    // All permutations should alternate parity.
                    for perm in &result {
                        for w in perm.windows(2) {
                            assert_ne!(w[0] % 2, w[1] % 2, "adjacent same parity in {perm:?}");
                        }
                    }
                }
            }
        };
    }

    test_permute!(my_tests, crate::my_solution::Solution);
    test_permute!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![2, 3, 4, 5];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::permute(std::hint::black_box(
                    n,
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::permute(std::hint::black_box(
                    n,
                )));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
