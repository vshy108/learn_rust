// LeetCode 6: Zigzag Conversion [Medium]
// https://leetcode.com/problems/zigzag-conversion/
//
// The string "PAYPALISHIRING" is written in a zigzag pattern on a given
// number of rows like this:
//
//   P   A   H   N          (numRows = 3)
//   A P L S I I G
//   Y   I   R
//
// Then read line by line: "PAHNAPLSIIGYIR"
//
// Write the code that will take a string and make this conversion given
// a number of rows.
//
// Examples:
//   s = "PAYPALISHIRING", numRows = 3  -> "PAHNAPLSIIGYIR"
//   s = "PAYPALISHIRING", numRows = 4  -> "PINALSIGYAHRPI"
//     P     I    N
//     A   L S  I G
//     Y A   H R
//     P     I
//   s = "A", numRows = 1              -> "A"
//
// Constraints:
//   - 1 <= s.len() <= 1000
//   - s consists of English letters (lower/upper), ',' and '.'
//   - 1 <= numRows <= 1000
//
// Run:  cargo run --example lc_0006_zigzag_conversion
// Test: cargo test --example lc_0006_zigzag_conversion
// Bench: cargo test --example lc_0006_zigzag_conversion bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-26) ---
//   My solution (row-bucket simulation):       2.606s avg (26.057us/iter)
//   AI solution (direct index calculation):    890.698ms avg (8.906us/iter)
//   AI solution is 2.93x faster
//
// --- Rust concepts used ---
//
// Vec<String> vs vec![""; n]:
//   `vec![String::new(); n]` creates n owned, growable Strings.
//   `vec![""; n]` creates `&str` slices (immutable) — can't `.push()`.
//
// String::push(char) vs push_str(&str):
//   `.push(ch)` appends a single char. `.push_str(s)` appends a slice.
//   Ruby `<<` is equivalent to `.push()` / `.push_str()`.
//
// usize and unsigned subtraction:
//   `usize` can't hold negative values. `curr_row -= 1` at row 0
//   would panic (debug) or wrap (release). Guard with conditionals.
//
// Vec<String>::concat():
//   Joins all Strings with no separator. Like Ruby `rows.join`.
//
// String::with_capacity(n):
//   Pre-allocates space for n bytes, avoiding reallocations as
//   the string grows. Useful when the final size is known.
//
// as_bytes() for O(1) indexing:
//   `s.as_bytes()` gives `&[u8]` with O(1) random access.
//   `b[j] as char` converts a byte back to char (safe for ASCII).

/// My solution module — row-bucket simulation.
///
/// Time:  O(n) — single pass over the string, each char pushed once
/// Space: O(n) — the Vec<String> row buffers hold all n characters
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert string `s` into zigzag pattern with `num_rows` rows,
        /// then read line by line.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string
        /// * `num_rows` - Number of rows in the zigzag pattern
        ///
        /// # Returns
        ///
        /// The zigzag-converted string read row by row
        pub fn convert(s: String, num_rows: i32) -> String {
            if num_rows == 1 || num_rows >= (s.len() as i32) {
                return s;
            };
            // `vec![String::new(); n]` creates n independent empty Strings.
            // Can't use `vec![""; n]` because `""` is `&str` (immutable
            // borrowed slice) — you can't `.push()` onto a `&str`.
            // `String` is owned and growable, like Ruby's mutable strings.
            let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
            let mut curr_row = 0usize;
            let mut going_down = false;

            for ch in s.chars().into_iter() {
                // `.push(ch)` appends a single char to the String.
                // Also: `.push_str(s)` appends a `&str`.
                rows[curr_row].push(ch);

                if curr_row == 0 || curr_row == (num_rows as usize) - 1 {
                    going_down = !going_down
                }

                // Ruby: `curr_row += going_down ? 1 : -1`
                // Can't do `curr_row += -1` because curr_row is usize
                // (unsigned — no negative values). Use separate +=/−=.
                // Safe: the direction flip above guarantees going_down
                // is true at row 0, so we never subtract from 0.
                if going_down {
                    curr_row += 1;
                } else {
                    curr_row -= 1;
                }
            }

            // A: join vector without separator
            rows.concat()
        }
    }
}

/// AI solution module — direct index calculation approach.
///
/// Instead of simulating the zigzag row by row, this computes the
/// exact indices that belong to each row using the cycle length
/// `cycle = 2 * num_rows - 2`. For each row, the characters appear
/// at regular intervals, with middle rows having an extra diagonal
/// character per cycle.
///
/// Avoids allocating a Vec<String> of row buffers — writes directly
/// into a single pre-allocated output String.
///
/// ## Walkthrough: "PAYPALISHIRING", num_rows = 4
///
/// The zigzag looks like:
///
/// ```text
/// Index:  0  1  2  3  4  5  6  7  8  9 10 11 12 13
/// Char:   P  A  Y  P  A  L  I  S  H  I  R  I  N  G
///
/// Row 0:  P           I           N        -> indices 0, 6, 12
/// Row 1:  A     L     S     I     G        -> indices 1, 5, 7, 11, 13
/// Row 2:  Y  A        H  R                 -> indices 2, 4, 8, 10
/// Row 3:  P           I                    -> indices 3, 9
/// ```
///
/// cycle = 2 * 4 - 2 = 6 (down 4, back up 2 = 6 steps before repeating)
///
/// Each row has two kinds of hits per cycle:
///
/// | Row | Main indices (every `cycle` apart) | Diagonal (`j + cycle - 2*row`) |
/// |-----|-------------------------------------|--------------------------------|
/// |  0  | 0, 6, 12                            | (none -- first row)            |
/// |  1  | 1, 7, 13                            | 1+4=5, 7+4=11                 |
/// |  2  | 2, 8                                | 2+2=4, 8+2=10                 |
/// |  3  | 3, 9                                | (none -- last row)             |
///
/// Row 1 example: j=1 -> push 'A', diag=5 -> push 'L';
///                j=7 -> push 'S', diag=11 -> push 'I';
///                j=13 -> push 'G', diag=17 >= 14 skip.
/// Result for row 1: A L S I G
///
/// Time:  O(n) — each character is visited exactly once
/// Space: O(n) — the output string (no intermediate row buffers)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert string `s` into zigzag pattern with `num_rows` rows,
        /// then read line by line.
        ///
        /// Computes output by iterating each row and jumping through
        /// the input at `cycle = 2 * num_rows - 2` intervals. Middle
        /// rows also pick up a diagonal character at offset
        /// `cycle - 2 * row` within each cycle.
        ///
        /// # Arguments
        ///
        /// * `s` - The input string
        /// * `num_rows` - Number of rows in the zigzag pattern
        ///
        /// # Returns
        ///
        /// The zigzag-converted string read row by row
        ///
        /// # Complexity
        ///
        /// Time:  O(n) — single pass collecting characters
        /// Space: O(n) — output string only
        pub fn convert(s: String, num_rows: i32) -> String {
            let n = s.len();
            let num_rows = num_rows as usize;
            if num_rows == 1 || num_rows >= n {
                return s;
            }

            // `as_bytes()` gives &[u8] with O(1) random access by index.
            // Safe for ASCII — each byte is one character.
            let b = s.as_bytes();
            // cycle = distance (in indices) before the zigzag pattern
            // repeats. E.g. num_rows=4 -> cycle=6: down 4 then up 2.
            let cycle = 2 * num_rows - 2;
            // A: Performance
            // Pre-allocate the exact capacity we need — avoids
            // reallocations as we push characters one by one.
            let mut result = String::with_capacity(n);

            // Process one row at a time (row 0, 1, ..., num_rows-1).
            // Within each row, jump through the input at `cycle` intervals.
            for row in 0..num_rows {
                let mut j = row; // first index belonging to this row
                while j < n {
                    // "Main" character: appears at every `cycle` interval
                    // starting from the row index.
                    // E.g. row 1, cycle 6 -> indices 1, 7, 13, ...
                    result.push(b[j] as char);

                    // Middle rows (not first or last) have an extra
                    // "diagonal" character between two main characters.
                    // Its offset from j is `cycle - 2 * row`.
                    // E.g. row 1, cycle 6 -> diag offset = 6-2 = 4,
                    //       so j=1 -> diag=5 ('L'), j=7 -> diag=11 ('I').
                    // First/last rows have no diagonal because their
                    // offset would be 0 or `cycle` (same as next main).
                    if row != 0 && row != num_rows - 1 {
                        let diag = j + cycle - 2 * row;
                        if diag < n {
                            result.push(b[diag] as char);
                        }
                    }

                    j += cycle;
                }
            }

            result
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32, &str)> = vec![
        ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
        ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
        ("A", 1, "A"),
    ];

    println!("--- My Solution ---");
    for (input, rows, expected) in &test_cases {
        let result = my_solution::Solution::convert(input.to_string(), *rows);
        println!("  \"{input}\", rows={rows} -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (input, rows, expected) in &test_cases {
        let result = ai_solution::Solution::convert(input.to_string(), *rows);
        println!("  \"{input}\", rows={rows} -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_zigzag {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_three_rows() {
                    assert_eq!(
                        Solution::convert("PAYPALISHIRING".to_string(), 3,),
                        "PAHNAPLSIIGYIR"
                    );
                }

                #[test]
                fn test_four_rows() {
                    assert_eq!(
                        Solution::convert("PAYPALISHIRING".to_string(), 4,),
                        "PINALSIGYAHRPI"
                    );
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::convert("A".to_string(), 1), "A");
                }

                #[test]
                fn test_one_row() {
                    // With 1 row, output == input (no zigzag)
                    assert_eq!(Solution::convert("ABCDEF".to_string(), 1), "ABCDEF");
                }

                #[test]
                fn test_two_rows() {
                    // A C E
                    // B D F
                    // -> "ACEBDF"
                    assert_eq!(Solution::convert("ABCDEF".to_string(), 2), "ACEBDF");
                }

                #[test]
                fn test_rows_equal_length() {
                    // Each char on its own row -> same as input
                    assert_eq!(Solution::convert("ABC".to_string(), 3), "ABC");
                }

                #[test]
                fn test_rows_exceed_length() {
                    // More rows than chars -> same as input
                    assert_eq!(Solution::convert("AB".to_string(), 5), "AB");
                }

                #[test]
                fn test_longer_string() {
                    // Verify the result has the same length and chars
                    let input = "ABCDEFGHIJKLMNO";
                    let result = Solution::convert(input.to_string(), 5);
                    assert_eq!(result.len(), input.len());
                    // Same characters, just reordered
                    let mut input_sorted: Vec<char> = input.chars().collect();
                    let mut result_sorted: Vec<char> = result.chars().collect();
                    input_sorted.sort_unstable();
                    result_sorted.sort_unstable();
                    assert_eq!(input_sorted, result_sorted);
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_zigzag!(my_tests, crate::my_solution::Solution);
    test_zigzag!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0006_zigzag_conversion bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Use a moderately sized string to exercise a realistic workload.
        let input: String = "ABCDEFGHIJ".repeat(100); // 1000 chars
        let num_rows = 7;

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::convert(
                std::hint::black_box(input.clone()),
                num_rows,
            ));
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::convert(
                std::hint::black_box(input.clone()),
                num_rows,
            ));
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
