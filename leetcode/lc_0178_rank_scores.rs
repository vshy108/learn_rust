// LeetCode 178: Rank Scores [Medium]
// https://leetcode.com/problems/rank-scores/
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
//   SELECT score, DENSE_RANK() OVER (ORDER BY score DESC) AS 'rank'
//   FROM Scores;
//
// Write a query to display scores with their dense rank (no gaps).
// Scores should be ordered from highest to lowest.
//
// Scores table: (id, score)
//
// Example:
//   Input:  [(1,3.50),(2,3.65),(3,4.00),(4,3.85),(5,4.00),(6,3.65)]
//   Output: [(4.00,1),(4.00,1),(3.85,2),(3.65,3),(3.65,3),(3.50,4)]
//
// Run:  cargo run --example lc_0178_rank_scores
// Test: cargo test --example lc_0178_rank_scores

use std::collections::BTreeSet;
use std::collections::HashMap;

/// A row in the Scores table.
/// Uses `i64` (cents) internally to avoid floating-point comparison issues.
/// For display, we convert back to f64.
#[derive(Debug, Clone)]
struct Score {
    #[allow(dead_code)]
    id: i32,
    score: f64,
}

/// A result row: (score, dense_rank).
#[derive(Debug, Clone, PartialEq)]
struct RankedScore {
    score: f64,
    rank: i32,
}

/// Convert f64 score to an integer key (multiply by 100, round).
/// This avoids floating-point equality pitfalls when used as a
/// HashMap key. Works correctly for the problem's 2-decimal scores.
fn to_key(score: f64) -> i64 {
    (score * 100.0).round() as i64
}

/// Compute dense-ranked scores, ordered highest to lowest.
///
/// Dense ranking means tied scores share the same rank and no rank
/// values are skipped. E.g., ranks go 1, 1, 2, 3, 3, 4 -- not
/// 1, 1, 3, 4, 4, 6 (that would be standard/"competition" ranking).
///
/// # Algorithm
/// 1. Collect distinct score keys into a `BTreeSet` (sorted ascending).
/// 2. Reverse-iterate to assign rank 1, 2, 3, ... to descending scores.
/// 3. Sort original rows by score descending, then look up each rank.
///
/// # Arguments
/// * `scores` - Slice of Score rows
///
/// # Returns
/// `Vec<RankedScore>` ordered from highest to lowest score.
fn rank_scores(scores: &[Score]) -> Vec<RankedScore> {
    // Step 1: Collect distinct score keys in sorted order
    let distinct: BTreeSet<i64> = scores.iter().map(|s| to_key(s.score)).collect();

    // Step 2: Map each distinct score key to its dense rank.
    // BTreeSet is ascending, so `.rev()` gives descending order.
    let rank_map: HashMap<i64, i32> = distinct
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &key)| (key, i as i32 + 1))
        .collect();

    // Step 3: Sort rows descending by score, then attach ranks.
    // Clone and sort; original slice is immutable.
    let mut sorted: Vec<&Score> = scores.iter().collect();
    // `partial_cmp` is needed for f64 (no total ordering).
    // Reversed (b, a) gives descending order.
    sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    sorted
        .iter()
        .map(|s| RankedScore {
            score: s.score,
            rank: rank_map[&to_key(s.score)],
        })
        .collect()
}

fn main() {
    println!("LeetCode 178: Rank Scores");

    let scores = vec![
        Score { id: 1, score: 3.50 },
        Score { id: 2, score: 3.65 },
        Score { id: 3, score: 4.00 },
        Score { id: 4, score: 3.85 },
        Score { id: 5, score: 4.00 },
        Score { id: 6, score: 3.65 },
    ];

    let ranked = rank_scores(&scores);
    for r in &ranked {
        println!("  score={:.2} rank={}", r.score, r.rank);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build Score rows from (id, score) pairs.
    fn scores(pairs: &[(i32, f64)]) -> Vec<Score> {
        pairs
            .iter()
            .map(|&(id, score)| Score { id, score })
            .collect()
    }

    #[test]
    fn leetcode_example() {
        let input = scores(&[
            (1, 3.50),
            (2, 3.65),
            (3, 4.00),
            (4, 3.85),
            (5, 4.00),
            (6, 3.65),
        ]);
        let result = rank_scores(&input);
        let expected = vec![
            RankedScore {
                score: 4.00,
                rank: 1,
            },
            RankedScore {
                score: 4.00,
                rank: 1,
            },
            RankedScore {
                score: 3.85,
                rank: 2,
            },
            RankedScore {
                score: 3.65,
                rank: 3,
            },
            RankedScore {
                score: 3.65,
                rank: 3,
            },
            RankedScore {
                score: 3.50,
                rank: 4,
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn single_score() {
        let input = scores(&[(1, 5.00)]);
        let result = rank_scores(&input);
        assert_eq!(
            result,
            vec![RankedScore {
                score: 5.00,
                rank: 1
            }]
        );
    }

    #[test]
    fn all_tied() {
        let input = scores(&[(1, 7.00), (2, 7.00), (3, 7.00)]);
        let result = rank_scores(&input);
        // All share rank 1
        assert!(result.iter().all(|r| r.rank == 1));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn all_distinct() {
        let input = scores(&[(1, 1.00), (2, 2.00), (3, 3.00)]);
        let result = rank_scores(&input);
        assert_eq!(
            result,
            vec![
                RankedScore {
                    score: 3.00,
                    rank: 1
                },
                RankedScore {
                    score: 2.00,
                    rank: 2
                },
                RankedScore {
                    score: 1.00,
                    rank: 3
                },
            ],
        );
    }

    #[test]
    fn empty_table() {
        let result = rank_scores(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn dense_rank_no_gaps() {
        // Scores: 100, 100, 50 -> ranks 1, 1, 2 (not 1, 1, 3)
        let input = scores(&[(1, 100.0), (2, 100.0), (3, 50.0)]);
        let result = rank_scores(&input);
        assert_eq!(result[0].rank, 1);
        assert_eq!(result[1].rank, 1);
        assert_eq!(result[2].rank, 2); // Dense: no gap to 3
    }

    #[test]
    fn many_ties() {
        let input = scores(&[
            (1, 10.0),
            (2, 20.0),
            (3, 20.0),
            (4, 30.0),
            (5, 30.0),
            (6, 30.0),
        ]);
        let result = rank_scores(&input);
        // 30->1, 30->1, 30->1, 20->2, 20->2, 10->3
        assert_eq!(
            result.iter().map(|r| r.rank).collect::<Vec<_>>(),
            vec![1, 1, 1, 2, 2, 3],
        );
    }
}
