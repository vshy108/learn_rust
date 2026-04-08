// LeetCode 180: Consecutive Numbers [Medium]
// https://leetcode.com/problems/consecutive-numbers/
//
// This is a SQL problem. Below is the Rust equivalent using standard library
// collections to demonstrate the relational operations.
//
// In production Rust, you would use crates like:
//   - `polars` for DataFrame-style data manipulation
//   - `rusqlite` or `sqlx` for running actual SQL queries
//   - `sea-orm` or `diesel` for ORM-based database access
//
// SQL Solution:
//   SELECT DISTINCT l2.num AS ConsecutiveNums
//   FROM Logs l1
//   JOIN Logs l2 ON l1.id = l2.id - 1 AND l1.num = l2.num
//   JOIN Logs l3 ON l2.id = l3.id - 1 AND l2.num = l3.num;
//
// Run:  cargo run --example lc_0180_consecutive_numbers
// Test: cargo test --example lc_0180_consecutive_numbers

use std::collections::HashSet;

/// A row in the Logs table. `id` is auto-increment starting from 1.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Log {
    id: i32,
    num: i32,
}

/// Find all numbers that appear at least three times consecutively.
///
/// # Arguments
/// * `logs` - Slice of `Log` rows, assumed sorted by `id` with consecutive ids
///
/// # Returns
/// A `Vec<i32>` of distinct numbers appearing 3+ times in a row
///
/// # Approach
/// Slide a window of size 3 over the sorted rows. When all three nums
/// match, insert into a `HashSet` for automatic deduplication.
/// This mirrors the SQL triple self-join on consecutive ids.
fn consecutive_numbers(logs: &[Log]) -> Vec<i32> {
    let mut result: HashSet<i32> = HashSet::new();

    // The SQL joins on l1.id = l2.id - 1 AND l2.id = l3.id - 1,
    // which is equivalent to checking every triplet of consecutive rows.
    for window in logs.windows(3) {
        let (a, b, c) = (&window[0], &window[1], &window[2]);
        // Verify ids are truly consecutive (mirrors the JOIN condition)
        if b.id == a.id + 1 && c.id == b.id + 1 && a.num == b.num && b.num == c.num {
            result.insert(b.num);
        }
    }

    let mut out: Vec<i32> = result.into_iter().collect();
    out.sort_unstable();
    out
}

fn main() {
    println!("LeetCode 180: Consecutive Numbers");

    let logs = vec![
        Log { id: 1, num: 1 },
        Log { id: 2, num: 1 },
        Log { id: 3, num: 1 },
        Log { id: 4, num: 2 },
        Log { id: 5, num: 1 },
        Log { id: 6, num: 2 },
        Log { id: 7, num: 2 },
    ];

    let result = consecutive_numbers(&logs);
    println!("  Consecutive numbers: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a Logs table from (id, num) pairs.
    fn make_logs(pairs: &[(i32, i32)]) -> Vec<Log> {
        pairs.iter().map(|&(id, num)| Log { id, num }).collect()
    }

    #[test]
    fn example_main() {
        let logs = make_logs(&[(1, 1), (2, 1), (3, 1), (4, 2), (5, 1), (6, 2), (7, 2)]);
        assert_eq!(consecutive_numbers(&logs), vec![1]);
    }

    #[test]
    fn no_consecutive() {
        let logs = make_logs(&[(1, 1), (2, 2), (3, 1), (4, 2)]);
        assert_eq!(consecutive_numbers(&logs), Vec::<i32>::new());
    }

    #[test]
    fn all_same() {
        let logs = make_logs(&[(1, 5), (2, 5), (3, 5), (4, 5)]);
        assert_eq!(consecutive_numbers(&logs), vec![5]);
    }

    #[test]
    fn two_different_consecutive_nums() {
        let logs = make_logs(&[(1, 3), (2, 3), (3, 3), (4, 7), (5, 7), (6, 7)]);
        assert_eq!(consecutive_numbers(&logs), vec![3, 7]);
    }

    #[test]
    fn exactly_two_consecutive_not_enough() {
        let logs = make_logs(&[(1, 4), (2, 4), (3, 9)]);
        assert_eq!(consecutive_numbers(&logs), Vec::<i32>::new());
    }

    #[test]
    fn single_row() {
        let logs = make_logs(&[(1, 1)]);
        assert_eq!(consecutive_numbers(&logs), Vec::<i32>::new());
    }

    #[test]
    fn empty_table() {
        let logs: Vec<Log> = vec![];
        assert_eq!(consecutive_numbers(&logs), Vec::<i32>::new());
    }

    #[test]
    fn longer_streak() {
        let logs = make_logs(&[(1, 2), (2, 2), (3, 2), (4, 2), (5, 2)]);
        // The number 2 still appears only once in the result (DISTINCT)
        assert_eq!(consecutive_numbers(&logs), vec![2]);
    }
}
