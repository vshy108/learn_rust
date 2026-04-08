// LeetCode 194: Transpose File [Medium]
// https://leetcode.com/problems/transpose-file/
//
// This is a Shell problem. Below is the Rust equivalent using standard library
// I/O and text processing to demonstrate the same operations.
//
// In production Rust, you would use:
//   - `std::io::BufRead` for line-by-line file reading
//   - `polars` for DataFrame-style transpose operations
//   - `ndarray` for matrix transposition
//
// Bash Solution:
//   awk '{
//     for (i=1; i<=NF; i++) {
//       if (NR == 1) { res[i] = $i }
//       else { res[i] = res[i] " " $i }
//     }
//   } END {
//     for (i=1; i<=NF; i++) { print res[i] }
//   }' file.txt
//
// Run:  cargo run --example lc_0194_transpose_file
// Test: cargo test --example lc_0194_transpose_file

/// Transpose the content of a text file.
///
/// # Arguments
/// * `text` - Multi-line text where each row has the same number of
///   space-separated columns
///
/// # Returns
/// A `String` where rows and columns are swapped:
///   row i, col j of input  ->  row j, col i of output
///
/// # Approach
/// 1. Parse each line into a `Vec<&str>` of fields (the "matrix").
/// 2. Allocate a transposed matrix of size `cols x rows`.
/// 3. Fill it with `transposed[col][row] = matrix[row][col]`.
/// 4. Join each transposed row with spaces, then join rows with newlines.
///
/// This mirrors the awk approach of accumulating columns across rows.
fn transpose(text: &str) -> String {
    // Parse into a 2D grid -- each inner Vec is one row's fields
    let matrix: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    if matrix.is_empty() || matrix[0].is_empty() {
        return String::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Build transposed output: iterate columns first, then rows
    // This is the Rust equivalent of the awk `res[i] = res[i] " " $i`
    (0..cols)
        .map(|col| {
            (0..rows)
                .map(|row| matrix[row][col])
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("LeetCode 194: Transpose File");

    let text = "name age\nalice 21\nryan 30";
    let result = transpose(text);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_main() {
        let text = "name age\nalice 21\nryan 30";
        assert_eq!(transpose(text), "name alice ryan\nage 21 30");
    }

    #[test]
    fn single_column() {
        let text = "a\nb\nc";
        assert_eq!(transpose(text), "a b c");
    }

    #[test]
    fn single_row() {
        let text = "a b c";
        assert_eq!(transpose(text), "a\nb\nc");
    }

    #[test]
    fn single_cell() {
        let text = "hello";
        assert_eq!(transpose(text), "hello");
    }

    #[test]
    fn square_matrix() {
        let text = "1 2 3\n4 5 6\n7 8 9";
        assert_eq!(transpose(text), "1 4 7\n2 5 8\n3 6 9");
    }

    #[test]
    fn wide_matrix() {
        let text = "a b c d\ne f g h";
        assert_eq!(transpose(text), "a e\nb f\nc g\nd h");
    }

    #[test]
    fn tall_matrix() {
        let text = "x y\na b\nc d\ne f";
        assert_eq!(transpose(text), "x a c e\ny b d f");
    }

    #[test]
    fn transpose_is_involutory() {
        // Transposing twice should yield the original
        let text = "name age\nalice 21\nryan 30";
        assert_eq!(transpose(&transpose(text)), text);
    }

    #[test]
    fn empty_input() {
        assert_eq!(transpose(""), String::new());
    }
}
