// LeetCode 79: Word Search [Medium]
// https://leetcode.com/problems/word-search/
//
// Given an m x n grid of characters board and a string word, return true
// if word exists in the grid.
//
// The word can be constructed from letters of sequentially adjacent cells,
// where adjacent cells are horizontally or vertically neighboring. The same
// letter cell may not be used more than once.
//
// Examples:
//   board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]],
//   word = "ABCCED" -> true
//
//   board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]],
//   word = "SEE" -> true
//
//   board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]],
//   word = "ABCB" -> false
//
// Constraints:
//   - m == board.length
//   - n = board[i].length
//   - 1 <= m, n <= 6
//   - 1 <= word.length <= 15
//   - board and word consists of only lowercase and uppercase English letters
//
// Run:  cargo run --example lc_0079_word_search
// Test: cargo test --example lc_0079_word_search
// Bench: cargo test --example lc_0079_word_search bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-03-28) ---
//   My solution:  2.870s total (avg 28.701us)
//   AI solution:  638.1ms total (avg 6.380us)
//   -> AI solution is 4.50x faster
//   Reason: flat u8 board avoids double indirection, u64 bitmask visited
//     replaces heap-allocated Vec<Vec<bool>> (single XOR for backtrack),
//     and [i8; 128] frequency array avoids HashMap hashing overhead.
//
// --- Rust concepts used ---
//   - HashMap entry API: or_insert(0) for default-or-mutate counting pattern
//   - .get().copied().unwrap_or(0): safe HashMap lookup returning owned value
//   - Variable shadowing: two word_bytes bindings (pre/post reversal)
//   - Nested fn vs closure: recursive DFS needs &mut visited, closures can't recurse
//   - isize for bounds checking: allows negative values without underflow panic
//   - as_bytes() / into_bytes(): O(1) byte-level access vs char iteration
//   - u64 bitmask: encodes 6x6 visited grid in a single integer (AI solution)
//   - Flat indexing: r*cols+c avoids Vec<Vec<>> double indirection (AI solution)
//   - iter().flatten().filter(): counting chars across 2D grid without nested loops

/// My solution module — HashMap frequency pruning + backtracking DFS.
///
/// Time:  O(m * n * 4^L) — for each cell, DFS branches 4 ways up to word length L
/// Space: O(m * n + L) — visited grid + recursion depth + HashMap
mod my_solution {
    pub struct Solution;

    use std::collections::HashMap;

    impl Solution {
        /// Determine if `word` exists in the `board` grid.
        ///
        /// # Arguments
        ///
        /// * `board` - A 2D grid of characters
        /// * `word` - The word to search for
        ///
        /// # Returns
        ///
        /// `true` if the word can be formed by adjacent cells, `false` otherwise
        pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
            let rows = board.len();
            let cols = board[0].len();

            let mut board_count: HashMap<char, i32> = HashMap::new();
            // Use &board to borrow, not consume — board is needed later for DFS
            for row in &board {
                for &ch in row {
                    *board_count.entry(ch).or_insert(0) += 1;
                }
            }

            for ch in word.chars() {
                // Use .get() instead of [] — avoids panic if char isn't on the board
                // .get() returns Option<&i32>, .copied() converts to Option<i32>
                // (dereferences the inner value), then .unwrap_or(0) provides a default
                let count = board_count.get(&ch).copied().unwrap_or(0);
                if count == 0 {
                    return false;
                }
                *board_count.entry(ch).or_insert(0) -= 1;
            }

            // Count occurrences of first and last word characters on the board
            // This word_bytes points to the ORIGINAL word (before potential reversal)
            let word_bytes = word.as_bytes();
            let first_count = board
                .iter()
                .flatten()
                .filter(|&&c| c == word_bytes[0] as char)
                .count();
            let last_count = board
                .iter()
                .flatten()
                .filter(|&&c| c == word_bytes[word_bytes.len() - 1] as char)
                .count();

            // Reverse word if last char is rarer on the board — prunes DFS earlier
            let word: String = if last_count < first_count {
                word.chars().rev().collect()
            } else {
                word
            };

            // 2D bool grid initialized to false — tracks cells already used in current path
            let mut visited = vec![vec![false; cols]; rows];

            // Nested fn instead of closure — Rust closures can't recurse
            // while mutably borrowing `visited`. All state passed as params.
            // Shadows the first word_bytes — now points to the REVERSED word
            // (if reversed above). The first word_bytes is no longer valid
            // because `word` was moved/reassigned by the reversal.
            let word_bytes = word.as_bytes();
            #[allow(clippy::too_many_arguments)]
            fn dfs(
                board: &[Vec<char>],
                word: &[u8],
                visited: &mut [Vec<bool>],
                r: isize,
                c: isize,
                idx: usize,
                rows: isize,
                cols: isize,
            ) -> bool {
                if idx == word.len() {
                    return true;
                }
                // isize allows r-1/c-1 to go negative without panic
                if r < 0 || c < 0 || r >= rows || c >= cols {
                    return false;
                }
                let (ru, cu) = (r as usize, c as usize);
                if visited[ru][cu] {
                    return false;
                }
                // Compare board char (as u8) with word byte
                if board[ru][cu] as u8 != word[idx] {
                    return false;
                }

                visited[ru][cu] = true;
                let found = dfs(board, word, visited, r + 1, c, idx + 1, rows, cols)
                    || dfs(board, word, visited, r - 1, c, idx + 1, rows, cols)
                    || dfs(board, word, visited, r, c + 1, idx + 1, rows, cols)
                    || dfs(board, word, visited, r, c - 1, idx + 1, rows, cols);
                visited[ru][cu] = false; // backtrack
                found
            }

            let (rows_i, cols_i) = (rows as isize, cols as isize);
            for r in 0..rows_i {
                for c in 0..cols_i {
                    if dfs(&board, word_bytes, &mut visited, r, c, 0, rows_i, cols_i) {
                        return true;
                    }
                }
            }

            false
        }
    }
}

/// AI solution module — bitmask DFS with flat board.
///
/// Flattens the 2D board into a `Vec<u8>` and tracks visited cells
/// with a `u64` bitmask instead of a `Vec<Vec<bool>>`. Since the board
/// is at most 6x6 = 36 cells, each cell maps to one bit. This avoids
/// heap-allocated visited grids and makes backtracking a single XOR.
///
/// Time:  O(m * n * 4^L) — DFS branches 4 ways for each of L word chars
/// Space: O(L) — recursion depth equals word length; no heap for visited
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if `word` exists in the `board` grid via bitmask DFS.
        ///
        /// # Arguments
        ///
        /// * `board` - A 2D grid of characters
        /// * `word` - The word to search for
        ///
        /// # Returns
        ///
        /// `true` if the word can be formed by adjacent cells, `false` otherwise
        pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
            let rows = board.len();
            let cols = board[0].len();

            // Flatten board to 1D u8 slice — avoids double indirection
            let flat: Vec<u8> = board.iter().flatten().map(|&c| c as u8).collect();
            let word = word.into_bytes();

            // Quick prune: every char in word must exist on board
            let mut freq = [0i8; 128];
            for &b in &flat {
                freq[b as usize] += 1;
            }
            for &b in &word {
                freq[b as usize] -= 1;
                if freq[b as usize] < 0 {
                    return false;
                }
            }

            // Reverse word if last char is rarer — same pruning as my_solution
            let first_cnt = flat.iter().filter(|&&b| b == word[0]).count();
            // .last() returns Option<&u8> — use .expect() to unwrap since
            // constraint guarantees word.length >= 1 (never None)
            let last_cnt = flat
                .iter()
                // .last() returns Option<&u8> (reference), so * dereferences
                // to compare u8 == u8. .expect() unwraps the Option since
                // constraint guarantees word.length >= 1 (never None)
                .filter(|&&b| b == *word.last().expect("non-empty word"))
                .count();
            let word: Vec<u8> = if last_cnt < first_cnt {
                word.into_iter().rev().collect()
            } else {
                word
            };

            /// Recursive DFS using a `u64` bitmask for visited state.
            ///
            /// # Arguments
            ///
            /// * `flat` - Flattened board as `&[u8]`
            /// * `word` - Target word bytes
            /// * `visited` - Bitmask where bit `r*cols+c` = 1 means visited
            /// * `pos` - Current flat index (r * cols + c)
            /// * `idx` - Current index into `word`
            /// * `rows` - Number of rows
            /// * `cols` - Number of columns
            fn dfs(
                flat: &[u8],
                word: &[u8],
                visited: u64,
                pos: usize,
                idx: usize,
                rows: usize,
                cols: usize,
            ) -> bool {
                if idx == word.len() {
                    return true;
                }
                if flat[pos] != word[idx] {
                    return false;
                }
                let next_idx = idx + 1;
                if next_idx == word.len() {
                    return true;
                }
                // Mark current cell visited via bitmask
                let new_visited = visited | (1u64 << pos);
                let r = pos / cols;
                let c = pos % cols;
                // Check all 4 neighbours
                if r > 0 {
                    let np = pos - cols;
                    if new_visited & (1u64 << np) == 0
                        && dfs(flat, word, new_visited, np, next_idx, rows, cols)
                    {
                        return true;
                    }
                }
                if r + 1 < rows {
                    let np = pos + cols;
                    if new_visited & (1u64 << np) == 0
                        && dfs(flat, word, new_visited, np, next_idx, rows, cols)
                    {
                        return true;
                    }
                }
                if c > 0 {
                    let np = pos - 1;
                    if new_visited & (1u64 << np) == 0
                        && dfs(flat, word, new_visited, np, next_idx, rows, cols)
                    {
                        return true;
                    }
                }
                if c + 1 < cols {
                    let np = pos + 1;
                    if new_visited & (1u64 << np) == 0
                        && dfs(flat, word, new_visited, np, next_idx, rows, cols)
                    {
                        return true;
                    }
                }
                false
            }

            let total = rows * cols;
            for pos in 0..total {
                if flat[pos] == word[0] && dfs(&flat, &word, 0, pos, 0, rows, cols) {
                    return true;
                }
            }
            false
        }
    }
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let test_cases: Vec<(&str, bool)> = vec![("ABCCED", true), ("SEE", true), ("ABCB", false)];

    println!("--- My Solution ---");
    for (word, expected) in &test_cases {
        let result = my_solution::Solution::exist(board.clone(), word.to_string());
        println!("  word=\"{word}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (word, expected) in &test_cases {
        let result = ai_solution::Solution::exist(board.clone(), word.to_string());
        println!("  word=\"{word}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_exist {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn default_board() -> Vec<Vec<char>> {
                    vec![
                        vec!['A', 'B', 'C', 'E'],
                        vec!['S', 'F', 'C', 'S'],
                        vec!['A', 'D', 'E', 'E'],
                    ]
                }

                #[test]
                fn test_abcced() {
                    assert!(Solution::exist(default_board(), "ABCCED".to_string()));
                }

                #[test]
                fn test_see() {
                    assert!(Solution::exist(default_board(), "SEE".to_string()));
                }

                #[test]
                fn test_abcb_no_reuse() {
                    assert!(!Solution::exist(default_board(), "ABCB".to_string()));
                }

                #[test]
                fn test_single_char_found() {
                    assert!(Solution::exist(vec![vec!['A']], "A".to_string()));
                }

                #[test]
                fn test_single_char_not_found() {
                    assert!(!Solution::exist(vec![vec!['A']], "B".to_string()));
                }

                #[test]
                fn test_word_longer_than_board() {
                    assert!(!Solution::exist(vec![vec!['A']], "AB".to_string()));
                }

                #[test]
                fn test_full_board_path() {
                    // Snake through entire 2x2 board
                    assert!(Solution::exist(
                        vec![vec!['A', 'B'], vec!['D', 'C']],
                        "ABCD".to_string()
                    ));
                }
            }
        };
    }

    test_exist!(my_tests, crate::my_solution::Solution);
    test_exist!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let words: Vec<&str> = vec!["ABCCED", "SEE", "ABCB", "ADEE"];

        let start = Instant::now();
        for _ in 0..iterations {
            for &w in &words {
                std::hint::black_box(crate::my_solution::Solution::exist(
                    std::hint::black_box(board.clone()),
                    std::hint::black_box(w.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &w in &words {
                std::hint::black_box(crate::ai_solution::Solution::exist(
                    std::hint::black_box(board.clone()),
                    std::hint::black_box(w.to_string()),
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
