// LeetCode 465: Optimal Account Balancing [Hard]
// https://leetcode.com/problems/optimal-account-balancing/
//
// Given an array of transactions where transactions[i] = [from_i, to_i, amount_i],
// return the minimum number of transactions required to settle all debts.
// A transaction [from, to, amount] means person `from` gave `amount` dollars
// to person `to`.
//
// Examples:
//   [[0,1,10],[2,0,5]]                    -> 2
//   [[0,1,10],[1,0,1],[1,2,5],[2,0,5]]    -> 1
//
// Constraints:
//   1 <= transactions.length <= 8
//   transactions[i].length == 3
//   0 <= from_i, to_i < 12, from_i != to_i
//   1 <= amount_i <= 100
//
// Run:  cargo run --example lc_0465_optimal_account_balancing
// Test: cargo test --example lc_0465_optimal_account_balancing
// Bench: cargo test --example lc_0465_optimal_account_balancing bench -- --nocapture
//
// First benchmark (200K iterations, 2026-04-08):
//   My solution: avg 1.984us
//   AI solution: avg 4.696us
//   Ratio: 0.42x (bitmask DP faster than backtracking)

/// Time: O(3^m), Space: O(2^m) where m = number of non-zero balances (<= 12)
/// Bitmask DP. Compute net balance for each person, filter zeros. For each
/// bitmask subset whose balances sum to zero, the minimum transactions to
/// clear that subset is popcount - 1. Enumerate all subsets of each mask
/// to find the optimal partition into zero-sum groups.
#[allow(dead_code)]
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the minimum number of transactions to settle all debts.
        ///
        /// # Arguments
        /// * `transactions` - Each element is [from, to, amount]
        ///
        /// # Returns
        /// Minimum number of transactions needed
        pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
            // Compute net balance for each person
            let mut balance: HashMap<i32, i64> = HashMap::new();
            for t in &transactions {
                *balance.entry(t[0]).or_default() -= t[2] as i64;
                *balance.entry(t[1]).or_default() += t[2] as i64;
            }

            // Keep only non-zero balances
            let debts: Vec<i64> = balance.into_values().filter(|&b| b != 0).collect();
            let m = debts.len();
            if m == 0 {
                return 0;
            }
            let full = 1usize << m;

            // sum[mask] = sum of balances in the subset indicated by mask
            let mut sum = vec![0i64; full];
            for mask in 1..full {
                // Use lowest set bit to build sum incrementally
                let lsb = mask & mask.wrapping_neg();
                let bit_idx = lsb.trailing_zeros() as usize;
                sum[mask] = sum[mask ^ lsb] + debts[bit_idx];
            }

            // dp[mask] = minimum transactions to settle the subset `mask`
            // For a zero-sum subset with k people, we need at most k-1 transactions.
            // We try to split mask into two non-empty zero-sum subsets to reduce cost.
            let mut dp = vec![i32::MAX; full];
            dp[0] = 0;

            for mask in 1..full {
                // Only consider masks whose balances sum to zero
                if sum[mask] != 0 {
                    continue;
                }

                // Baseline: all people in this zero-sum group need popcount-1 txns
                let popcount = mask.count_ones() as i32;
                dp[mask] = popcount - 1;

                // Try splitting into proper sub-subset `sub` and remainder `mask ^ sub`
                // Both must sum to zero. Enumerate all non-empty proper subsets.
                let mut sub = (mask - 1) & mask;
                while sub > 0 {
                    if sum[sub] == 0 && dp[sub] != i32::MAX && dp[mask ^ sub] != i32::MAX {
                        dp[mask] = dp[mask].min(dp[sub] + dp[mask ^ sub]);
                    }
                    sub = (sub - 1) & mask;
                }
            }

            dp[full - 1]
        }
    }
}

/// Time: O(m!) worst case, Space: O(m) where m = number of non-zero balances
/// Backtracking/DFS. After computing net balances and filtering zeros,
/// greedily try to settle the first non-zero balance with each subsequent
/// non-zero balance. When they cancel or partially cancel, recurse on the
/// remaining. Pruning: skip duplicates and early-terminate on exact cancels.
#[allow(dead_code)]
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return the minimum number of transactions to settle all debts.
        ///
        /// # Arguments
        /// * `transactions` - Each element is [from, to, amount]
        ///
        /// # Returns
        /// Minimum number of transactions needed
        pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
            // Compute net balance for each person
            let mut balance: HashMap<i32, i64> = HashMap::new();
            for t in &transactions {
                *balance.entry(t[0]).or_default() -= t[2] as i64;
                *balance.entry(t[1]).or_default() += t[2] as i64;
            }

            let mut debts: Vec<i64> = balance.into_values().filter(|&b| b != 0).collect();
            Self::dfs(&mut debts, 0)
        }

        /// Recursive DFS starting at index `start`.
        ///
        /// For each non-zero balance at `start`, try pairing it with every
        /// subsequent balance of opposite sign. After the "transaction",
        /// recurse on the rest. Return the minimum number of total transactions.
        fn dfs(debts: &mut Vec<i64>, start: usize) -> i32 {
            // Skip already-settled (zero) balances
            let mut idx = start;
            while idx < debts.len() && debts[idx] == 0 {
                idx += 1;
            }
            if idx == debts.len() {
                return 0; // All settled
            }

            let mut result = i32::MAX;
            let current = debts[idx];

            for j in (idx + 1)..debts.len() {
                // Only pair with opposite sign to make progress
                if (debts[j] > 0) != (current > 0) {
                    // Settle: person j absorbs person idx's debt
                    debts[j] += current;
                    let candidate = 1 + Self::dfs(debts, idx + 1);
                    result = result.min(candidate);
                    // Undo the settlement
                    debts[j] -= current;

                    // Pruning: if they exactly cancelled, this is optimal
                    // for this pairing -- no need to try other partners
                    if debts[j] + current == 0 {
                        break;
                    }
                }
            }

            result
        }
    }
}

fn main() {
    println!("LeetCode 465: Optimal Account Balancing");
    let cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
        (vec![vec![0, 1, 10], vec![2, 0, 5]], 2),
        (
            vec![vec![0, 1, 10], vec![1, 0, 1], vec![1, 2, 5], vec![2, 0, 5]],
            1,
        ),
    ];
    for (txns, expected) in &cases {
        let r1 = my_solution::Solution::min_transfers(txns.clone());
        let r2 = ai_solution::Solution::min_transfers(txns.clone());
        println!("  {txns:?} => my={r1}, ai={r2} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $txns:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(my_solution::Solution::min_transfers($txns), $expected);
                }
                #[test]
                fn ai() {
                    assert_eq!(ai_solution::Solution::min_transfers($txns), $expected);
                }
            }
        };
    }

    test_both!(example1, vec![vec![0, 1, 10], vec![2, 0, 5]], 2);
    test_both!(
        example2,
        vec![vec![0, 1, 10], vec![1, 0, 1], vec![1, 2, 5], vec![2, 0, 5]],
        1
    );
    test_both!(single_txn, vec![vec![0, 1, 5]], 1);
    test_both!(
        circular,
        vec![vec![0, 1, 10], vec![1, 2, 10], vec![2, 0, 10]],
        0
    );
    test_both!(three_way, vec![vec![0, 1, 5], vec![0, 2, 5]], 2);
    test_both!(
        four_people_chain,
        vec![vec![0, 1, 10], vec![1, 2, 10], vec![2, 3, 10]],
        1
    );
    test_both!(two_pairs, vec![vec![0, 1, 10], vec![2, 3, 10]], 2);
    test_both!(
        complex,
        vec![vec![0, 1, 2], vec![1, 2, 1], vec![1, 3, 1]],
        2
    );
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<Vec<i32>>) -> i32,
{
    let iterations = 200_000;
    // Worst-case-ish input: 8 transactions among 12 people
    let txns = vec![
        vec![0, 1, 10],
        vec![1, 2, 5],
        vec![2, 3, 20],
        vec![3, 4, 15],
        vec![4, 5, 8],
        vec![5, 6, 12],
        vec![6, 7, 3],
        vec![7, 0, 7],
    ];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(txns.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::min_transfers);
    bench_solution("AI solution", ai_solution::Solution::min_transfers);
}
