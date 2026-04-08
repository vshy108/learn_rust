// LeetCode 511: Game Play Analysis I [Easy]
// https://leetcode.com/problems/game-play-analysis-i/
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
//   SELECT player_id, MIN(event_date) AS first_login
//   FROM Activity
//   GROUP BY player_id;
//
// Problem: Find the first login date for each player.
//
// Activity table:
//   (player_id, device_id, event_date, games_played)
//   Primary key: (player_id, event_date)
//
// Run:  cargo run --example lc_0511_game_play_analysis_i
// Test: cargo test --example lc_0511_game_play_analysis_i

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Activity {
    player_id: i32,
    _device_id: i32,
    event_date: String,
    _games_played: i32,
}

/// Find the first login date for each player.
///
/// Equivalent to: SELECT player_id, MIN(event_date) AS first_login
///                FROM Activity GROUP BY player_id;
fn first_login(activities: &[Activity]) -> Vec<(i32, String)> {
    let mut first: HashMap<i32, &str> = HashMap::new();
    for a in activities {
        first
            .entry(a.player_id)
            .and_modify(|d| {
                if a.event_date.as_str() < *d {
                    *d = a.event_date.as_str();
                }
            })
            .or_insert(a.event_date.as_str());
    }

    let mut result: Vec<(i32, String)> = first
        .into_iter()
        .map(|(pid, date)| (pid, date.to_string()))
        .collect();
    result.sort_by_key(|(pid, _)| *pid);
    result
}

fn main() {
    let activities = vec![
        Activity {
            player_id: 1,
            _device_id: 2,
            event_date: "2016-03-01".to_string(),
            _games_played: 5,
        },
        Activity {
            player_id: 1,
            _device_id: 2,
            event_date: "2016-05-02".to_string(),
            _games_played: 6,
        },
        Activity {
            player_id: 2,
            _device_id: 3,
            event_date: "2017-06-25".to_string(),
            _games_played: 1,
        },
        Activity {
            player_id: 3,
            _device_id: 1,
            event_date: "2016-03-02".to_string(),
            _games_played: 0,
        },
        Activity {
            player_id: 3,
            _device_id: 4,
            event_date: "2018-07-03".to_string(),
            _games_played: 5,
        },
    ];

    let result = first_login(&activities);
    for (pid, date) in &result {
        println!("player_id: {pid}, first_login: {date}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_activity(pid: i32, did: i32, date: &str, games: i32) -> Activity {
        Activity {
            player_id: pid,
            _device_id: did,
            event_date: date.to_string(),
            _games_played: games,
        }
    }

    #[test]
    fn test_example() {
        let activities = vec![
            make_activity(1, 2, "2016-03-01", 5),
            make_activity(1, 2, "2016-05-02", 6),
            make_activity(2, 3, "2017-06-25", 1),
            make_activity(3, 1, "2016-03-02", 0),
            make_activity(3, 4, "2018-07-03", 5),
        ];
        let result = first_login(&activities);
        assert_eq!(
            result,
            vec![
                (1, "2016-03-01".to_string()),
                (2, "2017-06-25".to_string()),
                (3, "2016-03-02".to_string()),
            ]
        );
    }

    #[test]
    fn test_single_player() {
        let activities = vec![make_activity(1, 1, "2020-01-01", 10)];
        assert_eq!(
            first_login(&activities),
            vec![(1, "2020-01-01".to_string())]
        );
    }
}
