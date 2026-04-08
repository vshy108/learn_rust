// LeetCode 118: Pascal's Triangle [Easy]
// https://leetcode.com/problems/pascals-triangle/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.199us
//   AI solution avg: 1.830us
//   Ratio: AI 1.20x faster
//
// Given an integer numRows, return the first numRows rows of Pascal's triangle.
//
// Constraints: 1 <= numRows <= 30
//
// Run:  cargo run --example lc_0118_pascals_triangle
// Test: cargo test --example lc_0118_pascals_triangle
// Bench: cargo test --example lc_0118_pascals_triangle bench -- --nocapture

/// My solution — iterative row-by-row construction.
///
/// Time: O(numRows^2).
/// Space: O(numRows^2) for result.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate Pascal's triangle with numRows rows.
        pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
            let n = num_rows as usize;
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(n);
            for i in 0..n {
                let mut row = vec![1; i + 1];
                for j in 1..i {
                    // Each interior element = sum of two elements above
                    row[j] = result[i - 1][j - 1] + result[i - 1][j];
                }
                result.push(row);
            }
            result
        }
    }
}

/// AI solution — same approach (this problem has one optimal method).
/// Uses iterators for slightly different style.
///
/// Time: O(numRows^2).
/// Space: O(numRows^2).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate Pascal's triangle using window-based computation.
        pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
            let n = num_rows as usize;
            let mut result = Vec::with_capacity(n);
            for i in 0..n {
                if i == 0 {
                    result.push(vec![1]);
                } else {
                    let prev = &result[i - 1];
                    let mut row = Vec::with_capacity(i + 1);
                    row.push(1);
                    // Use windows(2) on previous row
                    for w in prev.windows(2) {
                        row.push(w[0] + w[1]);
                    }
                    row.push(1);
                    result.push(row);
                }
            }
            result
        }
    }
}

fn main() {
    let my = my_solution::Solution::generate(5);
    let ai = ai_solution::Solution::generate(5);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_pascal {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_five_rows() {
                    assert_eq!(
                        <$solution>::generate(5),
                        vec![
                            vec![1],
                            vec![1, 1],
                            vec![1, 2, 1],
                            vec![1, 3, 3, 1],
                            vec![1, 4, 6, 4, 1],
                        ],
                    );
                }

                #[test]
                fn test_one_row() {
                    assert_eq!(<$solution>::generate(1), vec![vec![1]]);
                }

                #[test]
                fn test_two_rows() {
                    assert_eq!(<$solution>::generate(2), vec![vec![1], vec![1, 1]],);
                }

                #[test]
                fn test_six_rows() {
                    let result = <$solution>::generate(6);
                    assert_eq!(result[5], vec![1, 5, 10, 10, 5, 1]);
                }
            }
        };
    }

    test_pascal!(my, crate::my_solution::Solution);
    test_pascal!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::generate(
                std::hint::black_box(30),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::generate(
                std::hint::black_box(30),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
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
