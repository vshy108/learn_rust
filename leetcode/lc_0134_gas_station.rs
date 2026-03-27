// LeetCode 134: Gas Station [Medium]
// https://leetcode.com/problems/gas-station/
//
// There are n gas stations along a circular route, where the amount of gas
// at the ith station is gas[i]. You have a car with an unlimited gas tank
// and it costs cost[i] of gas to travel from the ith station to the next
// (i + 1)th station. You begin the journey with an empty tank at one of
// the gas stations.
//
// Given two integer arrays gas and cost, return the starting gas station's
// index if you can travel around the circuit once in the clockwise
// direction, otherwise return -1. If there exists a solution, it is
// guaranteed to be unique.
//
// Examples:
//   gas = [1,2,3,4,5], cost = [3,4,5,1,2] -> 3
//   gas = [2,3,4],     cost = [3,4,3]      -> -1
//
// Constraints:
//   - n == gas.length == cost.length
//   - 1 <= n <= 10^5
//   - 0 <= gas[i], cost[i] <= 10^4
//
// Run:  cargo run --example lc_0134_gas_station
// Test: cargo test --example lc_0134_gas_station
// Bench: cargo test --example lc_0134_gas_station bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the starting gas station index for a complete circuit.
        ///
        /// # Arguments
        ///
        /// * `gas` - Amount of gas available at each station
        /// * `cost` - Gas cost to travel from station i to i+1
        ///
        /// # Returns
        ///
        /// Starting station index, or -1 if no solution exists
        pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
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
        /// Find the starting gas station index for a complete circuit.
        ///
        /// # Arguments
        ///
        /// * `gas` - Amount of gas available at each station
        /// * `cost` - Gas cost to travel from station i to i+1
        ///
        /// # Returns
        ///
        /// Starting station index, or -1 if no solution exists
        pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>, i32)> = vec![
        (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3),
        (vec![2, 3, 4], vec![3, 4, 3], -1),
    ];

    println!("--- My Solution ---");
    for (gas, cost, expected) in &test_cases {
        let result = my_solution::Solution::can_complete_circuit(gas.clone(), cost.clone());
        println!("  gas={gas:?}, cost={cost:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (gas, cost, expected) in &test_cases {
        let result = ai_solution::Solution::can_complete_circuit(gas.clone(), cost.clone());
        println!("  gas={gas:?}, cost={cost:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_gas_station {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic_circuit() {
                    assert_eq!(
                        Solution::can_complete_circuit(
                            vec![1, 2, 3, 4, 5],
                            vec![3, 4, 5, 1, 2]
                        ),
                        3
                    );
                }

                #[test]
                fn test_no_solution() {
                    assert_eq!(
                        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
                        -1
                    );
                }

                #[test]
                fn test_single_station() {
                    assert_eq!(
                        Solution::can_complete_circuit(vec![5], vec![4]),
                        0
                    );
                }

                #[test]
                fn test_single_station_no_solution() {
                    assert_eq!(
                        Solution::can_complete_circuit(vec![1], vec![2]),
                        -1
                    );
                }

                #[test]
                fn test_start_at_zero() {
                    assert_eq!(
                        Solution::can_complete_circuit(
                            vec![5, 1, 2, 3, 4],
                            vec![4, 4, 1, 5, 1]
                        ),
                        4
                    );
                }

                #[test]
                fn test_all_equal() {
                    assert_eq!(
                        Solution::can_complete_circuit(vec![3, 3, 3], vec![3, 3, 3]),
                        0
                    );
                }

                #[test]
                fn test_two_stations() {
                    assert_eq!(
                        Solution::can_complete_circuit(vec![1, 2], vec![2, 1]),
                        1
                    );
                }
            }
        };
    }

    test_gas_station!(my_tests, crate::my_solution::Solution);
    test_gas_station!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            (vec![2, 3, 4], vec![3, 4, 3]),
            (vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (gas, cost) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::can_complete_circuit(
                    std::hint::black_box(gas.clone()),
                    std::hint::black_box(cost.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (gas, cost) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::can_complete_circuit(
                    std::hint::black_box(gas.clone()),
                    std::hint::black_box(cost.clone()),
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
