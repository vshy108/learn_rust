// LeetCode 168: Excel Sheet Column Title [Easy]
// https://leetcode.com/problems/excel-sheet-column-title/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.221us
//   AI solution avg: 0.173us
//   Ratio: AI 1.28x faster
//
// Given an integer columnNumber, return its corresponding column title
// as it appears in an Excel sheet.
// 1 -> "A", 2 -> "B", ..., 26 -> "Z", 27 -> "AA", 28 -> "AB", ...
//
// Constraints: 1 <= columnNumber <= 2^31 - 1
//
// Run:  cargo run --example lc_0168_excel_sheet_column_title
// Test: cargo test --example lc_0168_excel_sheet_column_title
// Bench: cargo test --example lc_0168_excel_sheet_column_title bench -- --nocapture

/// My solution — repeatedly subtract 1 (to shift A..Z → 0..25),
/// take mod 26 to get the current character, divide by 26, and prepend.
///
/// Time: O(log₂₆ n).
/// Space: O(log₂₆ n) for the result characters.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert column number to Excel column title.
        pub fn convert_to_title(column_number: i32) -> String {
            let mut n = column_number;
            let mut result = Vec::new();
            while n > 0 {
                n -= 1; // shift to 0-indexed
                result.push((b'A' + (n % 26) as u8) as char);
                n /= 26;
            }
            result.reverse();
            result.into_iter().collect()
        }
    }
}

/// AI solution — repeatedly take mod-26 (shifted by 1 because 1-indexed).
///
/// Subtract 1 each iteration so that A=0..Z=25, then prepend the char.
///
/// Time: O(log26 n).
/// Space: O(log26 n) for the result string.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert column number to Excel column title.
        pub fn convert_to_title(column_number: i32) -> String {
            let mut n = column_number;
            let mut chars = Vec::new();
            while n > 0 {
                // Shift to 0-indexed: A=0, B=1, ..., Z=25
                n -= 1;
                chars.push((b'A' + (n % 26) as u8) as char);
                n /= 26;
            }
            chars.reverse();
            chars.into_iter().collect()
        }
    }
}

fn main() {
    let my = my_solution::Solution::convert_to_title(701);
    let ai = ai_solution::Solution::convert_to_title(701);
    println!("title: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_title {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_a() {
                    assert_eq!(<$solution>::convert_to_title(1), "A");
                }

                #[test]
                fn test_ab() {
                    assert_eq!(<$solution>::convert_to_title(28), "AB");
                }

                #[test]
                fn test_zy() {
                    assert_eq!(<$solution>::convert_to_title(701), "ZY");
                }

                #[test]
                fn test_z() {
                    assert_eq!(<$solution>::convert_to_title(26), "Z");
                }

                #[test]
                fn test_az() {
                    assert_eq!(<$solution>::convert_to_title(52), "AZ");
                }

                #[test]
                fn test_aaa() {
                    assert_eq!(<$solution>::convert_to_title(703), "AAA");
                }
            }
        };
    }

    test_title!(my, crate::my_solution::Solution);
    test_title!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::convert_to_title(
                std::hint::black_box(2_147_483_647),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::convert_to_title(
                std::hint::black_box(2_147_483_647),
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
