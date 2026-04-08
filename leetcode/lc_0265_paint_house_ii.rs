// LeetCode 265: Paint House II [Hard] (Premium)
// https://leetcode.com/problems/paint-house-ii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 40.797us
//   AI solution avg: 7.385us
//   Ratio: AI 5.52x faster
//
// There are n houses, each can be painted with one of k colors.
// No two adjacent houses may have the same color.
// costs[i][j] = cost of painting house i with color j.
// Return the minimum total cost.
//
// Example 1: costs = [[1,5,3],[2,9,4]] -> 5
// Example 2: costs = [[1,3],[2,4]]     -> 5
//
// Constraints:
//   - costs.length == n, costs[i].length == k
//   - 1 <= n <= 100, 2 <= k <= 20
//   - 1 <= costs[i][j] <= 20
//
// Follow up: Could you solve it in O(nk) runtime?
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0265.Paint%20House%20II/README_EN.md
//
// Run:  cargo run --example lc_0265_paint_house_ii
// Test: cargo test --example lc_0265_paint_house_ii
// Bench: cargo test --example lc_0265_paint_house_ii bench -- --nocapture

/// My solution — O(nk^2) DP (from doocs Python).
///
/// Maintain a dp array f of size k. For each house, compute new costs
/// by adding min of all other colors from previous row. Direct
/// translation: for each color j, find min(f[h]) where h != j.
///
/// Time: O(nk^2)
/// Space: O(k)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum cost to paint all houses.
        pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
            let k = costs[0].len();
            let mut f = costs[0].clone();
            for i in 1..costs.len() {
                let mut g = costs[i].clone();
                for j in 0..k {
                    // Find min of f[h] for h != j
                    let mut t = i32::MAX;
                    for h in 0..k {
                        if h != j && f[h] < t {
                            t = f[h];
                        }
                    }
                    g[j] += t;
                }
                f = g;
            }
            *f.iter().min().unwrap()
        }
    }
}

/// AI solution — O(nk) DP tracking two smallest (follow-up).
///
/// For each row, only need the smallest and second-smallest from the
/// previous row. If current color matches the previous minimum color,
/// use second-smallest; otherwise use smallest. This reduces the inner
/// loop from O(k) to O(1) per color.
///
/// Time: O(nk)
/// Space: O(1) extra (modifies input-sized state)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum cost to paint all houses.
        pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
            let k = costs[0].len();
            // Track (min1_val, min1_idx, min2_val) from previous row
            let mut min1 = 0i32;
            let mut min1_idx: usize = k; // sentinel: no color
            let mut min2 = 0i32;

            for cost_row in &costs {
                let mut new_min1 = i32::MAX;
                let mut new_min1_idx = 0usize;
                let mut new_min2 = i32::MAX;

                for j in 0..k {
                    // Previous best for this color: use min2 if j was the min1 color
                    let prev = if j == min1_idx { min2 } else { min1 };
                    let total = cost_row[j] + prev;

                    if total < new_min1 {
                        new_min2 = new_min1;
                        new_min1 = total;
                        new_min1_idx = j;
                    } else if total < new_min2 {
                        new_min2 = total;
                    }
                }
                min1 = new_min1;
                min1_idx = new_min1_idx;
                min2 = new_min2;
            }
            min1
        }
    }
}

fn main() {
    let costs = vec![vec![1, 5, 3], vec![2, 9, 4]];
    println!("my: {}", my_solution::Solution::min_cost_ii(costs.clone()));
    println!("ai: {}", ai_solution::Solution::min_cost_ii(costs));
}

#[cfg(test)]
mod tests {
    macro_rules! test_paint_house_ii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(Sol::min_cost_ii(vec![vec![1, 5, 3], vec![2, 9, 4]]), 5);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::min_cost_ii(vec![vec![1, 3], vec![2, 4]]), 5);
                }

                #[test]
                fn test_single_house() {
                    assert_eq!(Sol::min_cost_ii(vec![vec![5, 1, 3, 9]]), 1);
                }

                #[test]
                fn test_two_colors() {
                    assert_eq!(
                        Sol::min_cost_ii(vec![vec![1, 2], vec![2, 1], vec![1, 2]]),
                        3
                    );
                }

                #[test]
                fn test_three_houses_four_colors() {
                    // house0=1(idx1), house1=6(idx2), house2=4(idx0) = 11
                    assert_eq!(
                        Sol::min_cost_ii(vec![
                            vec![8, 1, 12, 7],
                            vec![15, 3, 6, 9],
                            vec![4, 11, 2, 10],
                        ]),
                        11,
                    );
                }

                #[test]
                fn test_all_same_cost() {
                    assert_eq!(Sol::min_cost_ii(vec![vec![5, 5, 5], vec![5, 5, 5]]), 10);
                }
            }
        };
    }

    test_paint_house_ii!(my, crate::my_solution::Solution);
    test_paint_house_ii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let costs: Vec<Vec<i32>> = (0..100)
            .map(|i| (0..20).map(|j| (i * 3 + j * 7) % 20 + 1).collect())
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_cost_ii(costs.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_cost_ii(costs.clone()));
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
