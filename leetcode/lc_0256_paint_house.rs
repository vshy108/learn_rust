// LeetCode 256: Paint House [Medium] (Premium)
// https://leetcode.com/problems/paint-house/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 5.526us
//   AI solution avg: 5.791us
//   Ratio: My 1.05x faster
//
// There is a row of n houses, each can be painted red, blue, or green.
// No two adjacent houses may have the same color.
// costs[i][j] = cost of painting house i with color j (0=red, 1=blue, 2=green).
// Return the minimum total cost.
//
// Example 1: costs = [[17,2,17],[16,16,5],[14,3,19]] -> 10
//   (blue=2 + green=5 + blue=3 = 10)
// Example 2: costs = [[7,6,2]] -> 2
//
// Constraints:
//   - costs.length == n, costs[i].length == 3
//   - 1 <= n <= 100
//   - 1 <= costs[i][j] <= 20
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0256.Paint%20House/README_EN.md
//
// Run:  cargo run --example lc_0256_paint_house
// Test: cargo test --example lc_0256_paint_house
// Bench: cargo test --example lc_0256_paint_house bench -- --nocapture

/// My solution — O(1) space DP with 3 variables (from doocs Python).
///
/// Track cost of painting the previous house with each color (a, b, c).
/// For the current house, each color's cost = min of the other two
/// previous costs + current cost. Final answer is min(a, b, c).
///
/// Time: O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum cost to paint all houses.
        pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
            let (mut a, mut b, mut c) = (0, 0, 0);
            for cost in &costs {
                // Save previous values before overwriting
                let (pa, pb, pc) = (a, b, c);
                a = pb.min(pc) + cost[0];
                b = pa.min(pc) + cost[1];
                c = pa.min(pb) + cost[2];
            }
            a.min(b).min(c)
        }
    }
}

/// AI solution — in-place DP on the costs array (mutating input).
///
/// Modify `costs[i][j]` in place to hold the cumulative minimum cost
/// of painting houses 0..=i with house i painted color j.
///
/// Time: O(n)
/// Space: O(1) extra (modifies input)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum cost to paint all houses.
        pub fn min_cost(mut costs: Vec<Vec<i32>>) -> i32 {
            for i in 1..costs.len() {
                // Each color picks the min of the other two from previous row
                costs[i][0] += costs[i - 1][1].min(costs[i - 1][2]);
                costs[i][1] += costs[i - 1][0].min(costs[i - 1][2]);
                costs[i][2] += costs[i - 1][0].min(costs[i - 1][1]);
            }
            let last = costs.last().unwrap();
            last[0].min(last[1]).min(last[2])
        }
    }
}

fn main() {
    let costs = vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]];
    println!("my: {}", my_solution::Solution::min_cost(costs.clone()));
    println!("ai: {}", ai_solution::Solution::min_cost(costs));
}

#[cfg(test)]
mod tests {
    macro_rules! test_paint_house {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Sol::min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19],]),
                        10
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::min_cost(vec![vec![7, 6, 2]]), 2);
                }

                #[test]
                fn test_two_houses() {
                    assert_eq!(
                        Sol::min_cost(vec![vec![1, 2, 3], vec![1, 2, 3]]),
                        3, // house0=red(1), house1=blue(2)
                    );
                }

                #[test]
                fn test_all_same_cost() {
                    assert_eq!(
                        Sol::min_cost(vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]]),
                        15,
                    );
                }

                #[test]
                fn test_force_alternation() {
                    // Cheapest for each house is color 0, but can't repeat adjacent
                    // Best: red(1) + blue(20) + red(1) = 22
                    assert_eq!(
                        Sol::min_cost(vec![vec![1, 20, 20], vec![1, 20, 20], vec![1, 20, 20]]),
                        22,
                    );
                }

                #[test]
                fn test_large_descending() {
                    // Costs favor later colors more and more
                    assert_eq!(
                        Sol::min_cost(vec![vec![10, 1, 10], vec![10, 10, 1], vec![1, 10, 10],]),
                        3,
                    );
                }

                #[test]
                fn test_single_house_pick_min() {
                    assert_eq!(Sol::min_cost(vec![vec![20, 15, 18]]), 15);
                }
            }
        };
    }

    test_paint_house!(my, crate::my_solution::Solution);
    test_paint_house!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let costs: Vec<Vec<i32>> = (0..100)
            .map(|i| vec![i % 20 + 1, (i * 3) % 20 + 1, (i * 7) % 20 + 1])
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_cost(costs.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_cost(costs.clone()));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
