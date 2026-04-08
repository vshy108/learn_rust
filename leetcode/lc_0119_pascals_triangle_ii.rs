// LeetCode 119: Pascal's Triangle II [Easy]
// https://leetcode.com/problems/pascals-triangle-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.449us
//   AI solution avg: 0.174us
//   Ratio: AI 2.58x faster
//
// Given an integer rowIndex, return the rowIndex-th (0-indexed) row of
// Pascal's triangle. Can you optimize to use only O(rowIndex) extra space?
//
// Constraints: 0 <= rowIndex <= 33
//
// Run:  cargo run --example lc_0119_pascals_triangle_ii
// Test: cargo test --example lc_0119_pascals_triangle_ii
// Bench: cargo test --example lc_0119_pascals_triangle_ii bench -- --nocapture

/// My solution — build rows iteratively until target row.
///
/// Time: O(rowIndex^2).
/// Space: O(rowIndex) — only keep current row.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the rowIndex-th row of Pascal's triangle.
        pub fn get_row(row_index: i32) -> Vec<i32> {
            let n = row_index as usize;
            let mut row = vec![1; n + 1];
            for i in 2..=n {
                // Update in reverse to avoid overwriting values we still need
                for j in (1..i).rev() {
                    row[j] += row[j - 1];
                }
            }
            row
        }
    }
}

/// AI solution — use combinatorial formula: C(n, k) = C(n, k-1) * (n-k+1) / k.
///
/// Time: O(rowIndex).
/// Space: O(rowIndex) for result only.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return row using combinatorial formula, O(rowIndex) time.
        pub fn get_row(row_index: i32) -> Vec<i32> {
            let n = row_index as usize;
            let mut row = Vec::with_capacity(n + 1);
            row.push(1);
            for k in 1..=n {
                // C(n,k) = C(n,k-1) * (n-k+1) / k
                // Use i64 to avoid overflow for large row indices
                let prev = *row.last().expect("non-empty") as i64;
                let val = prev * (n - k + 1) as i64 / k as i64;
                row.push(val as i32);
            }
            row
        }
    }
}

fn main() {
    let my = my_solution::Solution::get_row(3);
    let ai = ai_solution::Solution::get_row(3);
    println!("row 3: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_row {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_row_3() {
                    assert_eq!(<$solution>::get_row(3), vec![1, 3, 3, 1]);
                }

                #[test]
                fn test_row_0() {
                    assert_eq!(<$solution>::get_row(0), vec![1]);
                }

                #[test]
                fn test_row_1() {
                    assert_eq!(<$solution>::get_row(1), vec![1, 1]);
                }

                #[test]
                fn test_row_4() {
                    assert_eq!(<$solution>::get_row(4), vec![1, 4, 6, 4, 1]);
                }

                #[test]
                fn test_row_5() {
                    assert_eq!(<$solution>::get_row(5), vec![1, 5, 10, 10, 5, 1]);
                }

                #[test]
                fn test_row_30() {
                    let row = <$solution>::get_row(30);
                    assert_eq!(row.len(), 31);
                    assert_eq!(row[0], 1);
                    assert_eq!(row[30], 1);
                    assert_eq!(row[15], 155117520);
                }
            }
        };
    }

    test_row!(my, crate::my_solution::Solution);
    test_row!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::get_row(std::hint::black_box(
                30,
            )));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::get_row(std::hint::black_box(
                30,
            )));
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
