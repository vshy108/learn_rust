// LeetCode 171: Excel Sheet Column Number [Easy]
// https://leetcode.com/problems/excel-sheet-column-number/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.032us
//   AI solution avg: 0.032us
//   Ratio: ~tied
//
// Given a string columnTitle representing the column title in an Excel
// sheet, return its corresponding column number.
// "A" -> 1, "B" -> 2, ..., "Z" -> 26, "AA" -> 27, "AB" -> 28, ...
//
// Constraints: 1 <= columnTitle.length <= 7, columnTitle is uppercase English
//
// Run:  cargo run --example lc_0171_excel_sheet_column_number
// Test: cargo test --example lc_0171_excel_sheet_column_number
// Bench: cargo test --example lc_0171_excel_sheet_column_number bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert Excel column title to column number.
        pub fn title_to_number(column_title: String) -> i32 {
            let mut result = 0i32;
            for b in column_title.bytes() {
                result = result * 26 + (b - b'A' + 1) as i32;
            }
            result
        }
    }
}

/// AI solution — treat as base-26 number (A=1, B=2, ..., Z=26).
///
/// Time: O(n) where n is the length of the title.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert Excel column title to column number using base-26.
        pub fn title_to_number(column_title: String) -> i32 {
            column_title.bytes().fold(0i32, |acc, b| {
                // Each char contributes (b - b'A' + 1) at position weight 26
                acc * 26 + (b - b'A' + 1) as i32
            })
        }
    }
}

fn main() {
    let my = my_solution::Solution::title_to_number("ZY".into());
    let ai = ai_solution::Solution::title_to_number("ZY".into());
    println!("column number: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_col {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_a() {
                    assert_eq!(<$solution>::title_to_number("A".into()), 1);
                }

                #[test]
                fn test_ab() {
                    assert_eq!(<$solution>::title_to_number("AB".into()), 28);
                }

                #[test]
                fn test_zy() {
                    assert_eq!(<$solution>::title_to_number("ZY".into()), 701);
                }

                #[test]
                fn test_z() {
                    assert_eq!(<$solution>::title_to_number("Z".into()), 26);
                }

                #[test]
                fn test_aaa() {
                    assert_eq!(<$solution>::title_to_number("AAA".into()), 703);
                }
            }
        };
    }

    test_col!(my, crate::my_solution::Solution);
    test_col!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let title = "FXSHRXW".to_string(); // = 2_147_483_647

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::title_to_number(
                std::hint::black_box(title.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::title_to_number(
                std::hint::black_box(title.clone()),
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
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
