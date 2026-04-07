// LeetCode 1672: Richest Customer Wealth [Easy]
// https://leetcode.com/problems/richest-customer-wealth/
//
// You are given an m x n integer grid `accounts` where accounts[i][j] is the
// amount of money the i-th customer has in the j-th bank. Return the wealth
// that the richest customer has.
//
// A customer's wealth is the amount of money they have in all their bank
// accounts. The richest customer is the customer that has the maximum wealth.
//
// Examples:
//   accounts = [[1,2,3],[3,2,1]]         -> 6
//   accounts = [[1,5],[7,3],[3,5]]       -> 10
//   accounts = [[2,8,7],[7,1,3],[1,9,5]] -> 17
//
// Constraints:
//   - m == accounts.length
//   - n == accounts[i].length
//   - 1 <= m, n <= 50
//   - 1 <= accounts[i][j] <= 100
//
// Run:  cargo run --example lc_1672_richest_customer_wealth
// Test: cargo test --example lc_1672_richest_customer_wealth
// Bench: cargo test --example lc_1672_richest_customer_wealth bench -- --nocapture
//
// --- Rust concepts used ---
// - `i32::max(self, other)`: method-style max comparison, equivalent to
//   `std::cmp::max(a, b)` but reads more naturally in a chain
// - `.iter().map(|r| r.iter().sum::<i32>()).max()`: nested iterator chain
//   that sums each row then finds the global maximum
// - `.unwrap_or(0)` on Option<i32>: handles empty iterator case safely
//   (Previously covered: .iter().sum() -> lc_0724, .max() -> lc_0121)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  124.41ms (avg 1.244us)
// AI solution:  129.58ms (avg 1.295us)
// -> My solution is 1.04x faster
// Reason: nearly identical performance; imperative loop avoids the tiny
//   overhead of nested iterator adaptors.

/// My solution module — imperative loop.
///
/// Time:  O(m * n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the wealth of the richest customer.
        ///
        /// # Arguments
        ///
        /// * `accounts` - m x n grid of bank account balances
        ///
        /// # Returns
        ///
        /// Maximum total wealth across all customers
        pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
            let mut max_wealth = 0;
            for customer in &accounts {
                let wealth: i32 = customer.iter().sum();
                max_wealth = max_wealth.max(wealth);
            }
            max_wealth
        }
    }
}

/// AI solution module — iterator chain.
///
/// `.iter().map(sum).max()` in a single expression.
///
/// Time:  O(m * n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the wealth of the richest customer.
        ///
        /// # Arguments
        ///
        /// * `accounts` - m x n grid of bank account balances
        ///
        /// # Returns
        ///
        /// Maximum total wealth across all customers
        pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
            accounts
                .iter()
                .map(|customer| customer.iter().sum::<i32>())
                .max()
                .unwrap_or(0)
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
        (vec![vec![1, 2, 3], vec![3, 2, 1]], 6),
        (vec![vec![1, 5], vec![7, 3], vec![3, 5]], 10),
        (vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]], 17),
    ];

    println!("--- My Solution ---");
    for (accounts, expected) in &test_cases {
        let result = my_solution::Solution::maximum_wealth(accounts.clone());
        println!("  {accounts:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (accounts, expected) in &test_cases {
        let result = ai_solution::Solution::maximum_wealth(accounts.clone());
        println!("  {accounts:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_maximum_wealth {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
                        6
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5],]),
                        10
                    );
                }

                #[test]
                fn test_example_3() {
                    assert_eq!(
                        Solution::maximum_wealth(
                            vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5],]
                        ),
                        17
                    );
                }

                #[test]
                fn test_single_customer() {
                    assert_eq!(Solution::maximum_wealth(vec![vec![5, 10, 15]]), 30);
                }

                #[test]
                fn test_single_bank() {
                    assert_eq!(Solution::maximum_wealth(vec![vec![3], vec![7], vec![1]]), 7);
                }

                #[test]
                fn test_equal_wealth() {
                    assert_eq!(Solution::maximum_wealth(vec![vec![1, 2], vec![2, 1]]), 3);
                }

                #[test]
                fn test_one_cell() {
                    assert_eq!(Solution::maximum_wealth(vec![vec![100]]), 100);
                }
            }
        };
    }

    test_maximum_wealth!(my_tests, crate::my_solution::Solution);
    test_maximum_wealth!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<Vec<i32>>> = vec![
            vec![vec![1, 2, 3], vec![3, 2, 1]],
            vec![vec![1, 5], vec![7, 3], vec![3, 5]],
            vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for accounts in &inputs {
                std::hint::black_box(crate::my_solution::Solution::maximum_wealth(
                    std::hint::black_box(accounts.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for accounts in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::maximum_wealth(
                    std::hint::black_box(accounts.clone()),
                ));
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
