// LeetCode 241: Different Ways to Add Parentheses [Medium]
// https://leetcode.com/problems/different-ways-to-add-parentheses/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 36.388us
//   AI solution avg: 12.322us
//   Ratio: AI 2.95x faster
//
// Given a string expression of numbers and operators, return all possible
// results from computing all the different possible ways to group numbers
// and operators. You may return the answer in any order.
//
// The test cases are generated such that the output values fit in a
// 32-bit signed integer and the number of different results does not
// exceed 10^4.
//
// Constraints: 1 <= expression.length <= 20, expression consists of
// digits and the operators '+', '-', and '*'.
//
// Run:  cargo run --example lc_0241_different_ways_to_add_parentheses
// Test: cargo test --example lc_0241_different_ways_to_add_parentheses
// Bench: cargo test --example lc_0241_different_ways_to_add_parentheses bench -- --nocapture

/// My solution — recursive divide and conquer (no memoization).
///
/// For each operator in the expression, split into left and right
/// sub-expressions. Recursively compute all results for each half,
/// then combine every pair with the operator. Base case: if the
/// expression is a single number, return it.
///
/// Time: O(Catalan(n)) where n is the number of operators — exponential.
/// Space: O(Catalan(n)) for the result lists.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all possible computation results.
        pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
            Self::compute(expression.as_bytes())
        }

        fn compute(expr: &[u8]) -> Vec<i32> {
            let mut results = Vec::new();

            for i in 0..expr.len() {
                let ch = expr[i];
                if ch == b'+' || ch == b'-' || ch == b'*' {
                    // Split at this operator
                    let left = Self::compute(&expr[..i]);
                    let right = Self::compute(&expr[i + 1..]);

                    // Combine every pair
                    for &l in &left {
                        for &r in &right {
                            results.push(match ch {
                                b'+' => l + r,
                                b'-' => l - r,
                                b'*' => l * r,
                                _ => unreachable!(),
                            });
                        }
                    }
                }
            }

            // Base case: no operator found — it's a plain number
            if results.is_empty() {
                let num: i32 = std::str::from_utf8(expr)
                    .expect("valid utf8")
                    .parse()
                    .expect("valid integer");
                results.push(num);
            }

            results
        }
    }
}

/// AI solution — recursive divide and conquer with memoization.
///
/// Same approach but caches results for sub-expressions in a HashMap
/// to avoid redundant computation on overlapping sub-problems.
///
/// Time: O(Catalan(n)) overall results, but sub-expressions are computed
///       only once due to memoization.
/// Space: O(Catalan(n)) for results + O(n^2) memo entries.
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return all possible computation results.
        pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
            let mut memo: HashMap<String, Vec<i32>> = HashMap::new();
            Self::compute(&expression, &mut memo)
        }

        fn compute(expr: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
            if let Some(cached) = memo.get(expr) {
                return cached.clone();
            }

            let mut results = Vec::new();
            let bytes = expr.as_bytes();

            for i in 0..bytes.len() {
                let ch = bytes[i];
                if ch == b'+' || ch == b'-' || ch == b'*' {
                    let left = Self::compute(&expr[..i], memo);
                    let right = Self::compute(&expr[i + 1..], memo);

                    for &l in &left {
                        for &r in &right {
                            results.push(match ch {
                                b'+' => l + r,
                                b'-' => l - r,
                                b'*' => l * r,
                                _ => unreachable!(),
                            });
                        }
                    }
                }
            }

            if results.is_empty() {
                results.push(expr.parse::<i32>().expect("valid integer"));
            }

            memo.insert(expr.to_string(), results.clone());
            results
        }
    }
}

fn main() {
    let cases = [("2-1-1", vec![0, 2]), ("2*3-4*5", vec![-34, -14, -10, -10, 10])];
    for (expr, expected) in &cases {
        let mut my = my_solution::Solution::diff_ways_to_compute(expr.to_string());
        let mut ai = ai_solution::Solution::diff_ways_to_compute(expr.to_string());
        my.sort();
        ai.sort();
        println!("{expr}: my={my:?}, ai={ai:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    macro_rules! test_ways {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        sorted(<$solution>::diff_ways_to_compute("2-1-1".into())),
                        vec![0, 2],
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        sorted(<$solution>::diff_ways_to_compute("2*3-4*5".into())),
                        vec![-34, -14, -10, -10, 10],
                    );
                }

                #[test]
                fn test_single_number() {
                    assert_eq!(
                        <$solution>::diff_ways_to_compute("11".into()),
                        vec![11],
                    );
                }

                #[test]
                fn test_two_numbers() {
                    assert_eq!(
                        <$solution>::diff_ways_to_compute("3+4".into()),
                        vec![7],
                    );
                }
            }
        };
    }

    test_ways!(my, crate::my_solution::Solution);
    test_ways!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let expr = "2*3-4*5+1-2*3".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::diff_ways_to_compute(
                std::hint::black_box(expr.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::diff_ways_to_compute(
                std::hint::black_box(expr.clone()),
            ));
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
