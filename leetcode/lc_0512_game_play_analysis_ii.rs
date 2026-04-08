// LeetCode 512: Game Play Analysis II [Easy] (Premium)
// https://leetcode.com/problems/game-play-analysis-ii/
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
//   SELECT player_id, device_id
//   FROM Activity
//   WHERE (player_id, event_date) IN (
//       SELECT player_id, MIN(event_date) FROM Activity GROUP BY player_id
//   );
//
// Problem: Report the device that each player first logged in with.
//
// Activity table:
//   (player_id, device_id, event_date, games_played)
//   Primary key: (player_id, event_date)
//
// Run:  cargo run --example lc_0512_game_play_analysis_ii
// Test: cargo test --example lc_0512_game_play_analysis_ii

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Activity {
    player_id: i32,
    device_id: i32,
    event_date: String,
    _games_played: i32,
}

/// Report the device each player first logged in with.
///
/// Equivalent to the SQL subquery approach: find min event_date per player,
/// then return the device_id for that row.
fn first_device(activities: &[Activity]) -> Vec<(i32, i32)> {
    // GROUP BY player_id, find the row with MIN(event_date)
    let mut first: HashMap<i32, (String, i32)> = HashMap::new();
    for a in activities {
        first
            .entry(a.player_id)
            .and_modify(|(date, dev)| {
                if a.event_date < *date {
                    *date = a.event_date.clone();
                    *dev = a.device_id;
                }
            })
            .or_insert((a.event_date.clone(), a.device_id));
    }

    let mut result: Vec<(i32, i32)> = first
        .into_iter()
        .map(|(pid, (_, dev))| (pid, dev))
        .collect();
    result.sort_by_key(|(pid, _)| *pid);
    result
}

fn main() {
    let activities = vec![
        Activity {
            player_id: 1,
            device_id: 2,
            event_date: "2016-03-01".to_string(),
            _games_played: 5,
        },
        Activity {
            player_id: 1,
            device_id: 2,
            event_date: "2016-05-02".to_string(),
            _games_played: 6,
        },
        Activity {
            player_id: 2,
            device_id: 3,
            event_date: "2017-06-25".to_string(),
            _games_played: 1,
        },
        Activity {
            player_id: 3,
            device_id: 1,
            event_date: "2016-03-02".to_string(),
            _games_played: 0,
        },
        Activity {
            player_id: 3,
            device_id: 4,
            event_date: "2018-07-03".to_string(),
            _games_played: 5,
        },
    ];

    let result = first_device(&activities);
    for (pid, dev) in &result {
        println!("player_id: {pid}, device_id: {dev}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_activity(pid: i32, did: i32, date: &str, games: i32) -> Activity {
        Activity {
            player_id: pid,
            device_id: did,
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
        let result = first_device(&activities);
        assert_eq!(result, vec![(1, 2), (2, 3), (3, 1)]);
    }

    #[test]
    fn test_single_player() {
        let activities = vec![make_activity(1, 5, "2020-01-01", 10)];
        assert_eq!(first_device(&activities), vec![(1, 5)]);
    }
}
