// LeetCode 517: Super Washing Machines [Hard]
// https://leetcode.com/problems/super-washing-machines/
//
// You have n super washing machines on a line. Initially, each washing machine
// has some dresses or is empty. For each move, you could choose any m
// (1 <= m <= n) washing machines, and pass one dress of each to one of its
// adjacent washing machines at the same time.
//
// Return the minimum number of moves to make all washing machines have the
// same number of dresses. If it is not possible, return -1.
//
// Examples:
//   [1,0,5]  -> 3
//   [0,3,0]  -> 2
//   [0,2,0]  -> -1
//
// Constraints:
//   n == machines.length
//   1 <= n <= 10^4
//   0 <= machines[i] <= 10^5
//
// Run:  cargo run --example lc_0517_super_washing_machines
// Test: cargo test --example lc_0517_super_washing_machines
// Bench: cargo test --example lc_0517_super_washing_machines bench -- --nocapture
//
// First benchmark (100K iterations, 2026-04-08):
//   My solution: avg 118ns
//   AI solution: avg 117ns
//   Essentially equal
//   Reason: Both use identical O(n) greedy logic; fold vs explicit loop
//   compiles to nearly the same machine code.

/// Greedy with prefix sum — track the running balance of excess dresses.
///
/// Adapted from doocs/leetcode Python solution.
/// The answer is max(max|prefix_sum|, max(machines[i] - k)) where k is the
/// target per-machine count. The prefix sum represents net flow between
/// the left group and right group at each boundary.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum moves to equalize dresses across all machines.
        ///
        /// # Arguments
        /// * `machines` - number of dresses in each washing machine
        ///
        /// # Returns
        /// Minimum moves, or -1 if impossible
        pub fn find_min_moves(machines: Vec<i32>) -> i32 {
            let n = machines.len();
            let total: i32 = machines.iter().sum();
            // If total dresses can't be evenly divided, it's impossible
            if total % n as i32 != 0 {
                return -1;
            }
            let k = total / n as i32;
            let mut ans = 0;
            let mut s = 0i32; // running prefix sum of (machines[i] - k)
            for x in &machines {
                let diff = x - k;
                s += diff;
                // |s| = net flow needed across this boundary
                // diff = excess at this machine (if positive, must push out to both sides)
                ans = ans.max(s.abs()).max(diff);
            }
            ans
        }
    }
}

/// Iterator-based approach using fold to compute the answer in one pass.
///
/// Same greedy algorithm but expressed with iterator combinators for a
/// more functional style.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum moves to equalize dresses across all machines.
        ///
        /// # Arguments
        /// * `machines` - number of dresses in each washing machine
        ///
        /// # Returns
        /// Minimum moves, or -1 if impossible
        pub fn find_min_moves(machines: Vec<i32>) -> i32 {
            let n = machines.len() as i32;
            let total: i32 = machines.iter().sum();
            if total % n != 0 {
                return -1;
            }
            let target = total / n;
            // Fold over machines, accumulating (prefix_sum, answer)
            let (_, ans) = machines.iter().fold((0i32, 0i32), |(prefix, ans), &x| {
                let diff = x - target;
                let new_prefix = prefix + diff;
                // The answer is the max of:
                //   1. |prefix_sum| at each boundary (net flow between groups)
                //   2. machines[i] - target (excess that must be pushed out both sides)
                let new_ans = ans.max(new_prefix.abs()).max(diff);
                (new_prefix, new_ans)
            });
            ans
        }
    }
}

fn main() {
    let cases = vec![vec![1, 0, 5], vec![0, 3, 0], vec![0, 2, 0]];

    for machines in &cases {
        println!(
            "my  {:?}: {}",
            machines,
            my_solution::Solution::find_min_moves(machines.clone())
        );
        println!(
            "ai  {:?}: {}",
            machines,
            ai_solution::Solution::find_min_moves(machines.clone())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! washing_tests {
        ($mod:ident, $solution:path) => {
            mod $mod {
                use super::*;

                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::find_min_moves(vec![1, 0, 5]), 3);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::find_min_moves(vec![0, 3, 0]), 2);
                }

                #[test]
                fn test_impossible() {
                    assert_eq!(<$solution>::find_min_moves(vec![0, 2, 0]), -1);
                }

                #[test]
                fn test_already_equal() {
                    assert_eq!(<$solution>::find_min_moves(vec![3, 3, 3]), 0);
                }

                #[test]
                fn test_single_machine() {
                    assert_eq!(<$solution>::find_min_moves(vec![5]), 0);
                }

                #[test]
                fn test_two_machines() {
                    assert_eq!(<$solution>::find_min_moves(vec![0, 4]), 2);
                }

                #[test]
                fn test_large_imbalance() {
                    assert_eq!(<$solution>::find_min_moves(vec![0, 0, 11, 5]), 8);
                }
            }
        };
    }

    washing_tests!(my_tests, my_solution::Solution);
    washing_tests!(ai_tests, ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let machines: Vec<i32> = (0..1000).map(|i| (i * 7 + 3) % 100).collect();
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::find_min_moves(
                std::hint::black_box(machines.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::find_min_moves(
                std::hint::black_box(machines.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur / iterations;
        let ai_avg = ai_dur / iterations;
        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("  My solution (loop):     {my_avg:?} avg");
        println!("  AI solution (fold):     {ai_avg:?} avg");
        if ai_avg < my_avg {
            println!(
                "  AI is {:.2}x faster",
                my_avg.as_nanos() as f64 / ai_avg.as_nanos() as f64
            );
        } else {
            println!(
                "  My is {:.2}x faster",
                ai_avg.as_nanos() as f64 / my_avg.as_nanos() as f64
            );
        }
    }
}
