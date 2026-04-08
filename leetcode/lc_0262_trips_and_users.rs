// LeetCode 262: Trips and Users [Hard]
// https://leetcode.com/problems/trips-and-users/
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
//   SELECT request_at AS Day,
//          ROUND(AVG(status != 'completed'), 2) AS 'Cancellation Rate'
//   FROM Trips t
//   JOIN Users u1 ON t.client_id = u1.users_id AND u1.banned = 'No'
//   JOIN Users u2 ON t.driver_id = u2.users_id AND u2.banned = 'No'
//   WHERE request_at BETWEEN '2013-10-01' AND '2013-10-03'
//   GROUP BY request_at;
//
// Run:  cargo run --example lc_0262_trips_and_users
// Test: cargo test --example lc_0262_trips_and_users

use std::collections::{BTreeMap, HashSet};

/// A row in the Trips table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Trip {
    id: i32,
    client_id: i32,
    driver_id: i32,
    city_id: i32,
    status: String,
    request_at: String,
}

/// A row in the Users table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct User {
    users_id: i32,
    banned: String,
    role: String,
}

/// Result row: (day, cancellation_rate).
#[derive(Debug, Clone, PartialEq)]
struct CancellationRow {
    day: String,
    cancellation_rate: f64,
}

/// Compute the cancellation rate per day for trips where both the
/// client and driver are unbanned, within the given date range.
///
/// # Arguments
/// * `trips` - Slice of `Trip` rows
/// * `users` - Slice of `User` rows
/// * `start_date` - Inclusive start of date range (e.g. "2013-10-01")
/// * `end_date` - Inclusive end of date range (e.g. "2013-10-03")
///
/// # Returns
/// Rows of (Day, Cancellation Rate) sorted by day, rate rounded to 2 decimals
///
/// # Approach
/// 1. Build a set of banned user ids (mirrors the JOIN filter).
/// 2. Filter trips to the date range and exclude trips involving
///    banned clients or drivers (the two JOIN conditions).
/// 3. GROUP BY request_at and compute AVG(status != 'completed').
/// 4. Round to 2 decimal places.
fn cancellation_rate(
    trips: &[Trip],
    users: &[User],
    start_date: &str,
    end_date: &str,
) -> Vec<CancellationRow> {
    // Build set of banned user ids.
    // The SQL JOINs on u1.banned = 'No' AND u2.banned = 'No'; we
    // invert this to a banned set for O(1) exclusion checks.
    let banned: HashSet<i32> = users
        .iter()
        .filter(|u| u.banned == "Yes")
        .map(|u| u.users_id)
        .collect();

    // BTreeMap keeps days sorted (like ORDER BY request_at).
    // Each entry: (total_trips, cancelled_count) for the GROUP BY.
    let mut by_day: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for t in trips {
        // WHERE request_at BETWEEN start_date AND end_date
        if t.request_at.as_str() < start_date || t.request_at.as_str() > end_date {
            continue;
        }

        // JOIN Users u1 ON t.client_id = u1.users_id AND u1.banned = 'No'
        // JOIN Users u2 ON t.driver_id = u2.users_id AND u2.banned = 'No'
        if banned.contains(&t.client_id) || banned.contains(&t.driver_id) {
            continue;
        }

        let entry = by_day.entry(&t.request_at).or_insert((0, 0));
        entry.0 += 1; // total trips for this day
        if t.status != "completed" {
            entry.1 += 1; // cancelled trips (status != 'completed')
        }
    }

    // Compute AVG(status != 'completed') per day, rounded to 2 decimals
    by_day
        .into_iter()
        .map(|(day, (total, cancelled))| {
            // ROUND(..., 2): multiply by 100, round, divide by 100
            let rate = if total == 0 {
                0.0
            } else {
                (f64::from(cancelled) / f64::from(total) * 100.0).round() / 100.0
            };
            CancellationRow {
                day: day.to_string(),
                cancellation_rate: rate,
            }
        })
        .collect()
}

fn main() {
    println!("LeetCode 262: Trips and Users");

    let trips = vec![
        Trip {
            id: 1,
            client_id: 1,
            driver_id: 10,
            city_id: 1,
            status: "completed".into(),
            request_at: "2013-10-01".into(),
        },
        Trip {
            id: 2,
            client_id: 2,
            driver_id: 11,
            city_id: 1,
            status: "cancelled_by_driver".into(),
            request_at: "2013-10-01".into(),
        },
        Trip {
            id: 3,
            client_id: 3,
            driver_id: 12,
            city_id: 6,
            status: "completed".into(),
            request_at: "2013-10-01".into(),
        },
        Trip {
            id: 4,
            client_id: 4,
            driver_id: 13,
            city_id: 6,
            status: "cancelled_by_client".into(),
            request_at: "2013-10-01".into(),
        },
        Trip {
            id: 5,
            client_id: 1,
            driver_id: 10,
            city_id: 1,
            status: "completed".into(),
            request_at: "2013-10-02".into(),
        },
        Trip {
            id: 6,
            client_id: 2,
            driver_id: 11,
            city_id: 6,
            status: "completed".into(),
            request_at: "2013-10-02".into(),
        },
        Trip {
            id: 7,
            client_id: 3,
            driver_id: 12,
            city_id: 6,
            status: "completed".into(),
            request_at: "2013-10-02".into(),
        },
        Trip {
            id: 8,
            client_id: 2,
            driver_id: 12,
            city_id: 12,
            status: "completed".into(),
            request_at: "2013-10-03".into(),
        },
        Trip {
            id: 9,
            client_id: 3,
            driver_id: 10,
            city_id: 12,
            status: "completed".into(),
            request_at: "2013-10-03".into(),
        },
        Trip {
            id: 10,
            client_id: 4,
            driver_id: 13,
            city_id: 12,
            status: "cancelled_by_driver".into(),
            request_at: "2013-10-03".into(),
        },
    ];

    let users = vec![
        User {
            users_id: 1,
            banned: "No".into(),
            role: "client".into(),
        },
        User {
            users_id: 2,
            banned: "Yes".into(),
            role: "client".into(),
        },
        User {
            users_id: 3,
            banned: "No".into(),
            role: "client".into(),
        },
        User {
            users_id: 4,
            banned: "No".into(),
            role: "client".into(),
        },
        User {
            users_id: 10,
            banned: "No".into(),
            role: "driver".into(),
        },
        User {
            users_id: 11,
            banned: "No".into(),
            role: "driver".into(),
        },
        User {
            users_id: 12,
            banned: "No".into(),
            role: "driver".into(),
        },
        User {
            users_id: 13,
            banned: "No".into(),
            role: "driver".into(),
        },
    ];

    let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-03");
    for row in &result {
        println!("  {} : {:.2}", row.day, row.cancellation_rate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trip(
        id: i32,
        client_id: i32,
        driver_id: i32,
        city_id: i32,
        status: &str,
        request_at: &str,
    ) -> Trip {
        Trip {
            id,
            client_id,
            driver_id,
            city_id,
            status: status.to_string(),
            request_at: request_at.to_string(),
        }
    }

    fn user(users_id: i32, banned: &str, role: &str) -> User {
        User {
            users_id,
            banned: banned.to_string(),
            role: role.to_string(),
        }
    }

    fn row(day: &str, rate: f64) -> CancellationRow {
        CancellationRow {
            day: day.to_string(),
            cancellation_rate: rate,
        }
    }

    fn default_trips() -> Vec<Trip> {
        vec![
            trip(1, 1, 10, 1, "completed", "2013-10-01"),
            trip(2, 2, 11, 1, "cancelled_by_driver", "2013-10-01"),
            trip(3, 3, 12, 6, "completed", "2013-10-01"),
            trip(4, 4, 13, 6, "cancelled_by_client", "2013-10-01"),
            trip(5, 1, 10, 1, "completed", "2013-10-02"),
            trip(6, 2, 11, 6, "completed", "2013-10-02"),
            trip(7, 3, 12, 6, "completed", "2013-10-02"),
            trip(8, 2, 12, 12, "completed", "2013-10-03"),
            trip(9, 3, 10, 12, "completed", "2013-10-03"),
            trip(10, 4, 13, 12, "cancelled_by_driver", "2013-10-03"),
        ]
    }

    fn default_users() -> Vec<User> {
        vec![
            user(1, "No", "client"),
            user(2, "Yes", "client"),
            user(3, "No", "client"),
            user(4, "No", "client"),
            user(10, "No", "driver"),
            user(11, "No", "driver"),
            user(12, "No", "driver"),
            user(13, "No", "driver"),
        ]
    }

    #[test]
    fn example_main() {
        let result = cancellation_rate(
            &default_trips(),
            &default_users(),
            "2013-10-01",
            "2013-10-03",
        );
        assert_eq!(
            result,
            vec![
                row("2013-10-01", 0.33),
                row("2013-10-02", 0.00),
                row("2013-10-03", 0.50),
            ]
        );
    }

    #[test]
    fn all_completed_no_cancellations() {
        let trips = vec![
            trip(1, 1, 10, 1, "completed", "2013-10-01"),
            trip(2, 3, 12, 1, "completed", "2013-10-01"),
        ];
        let users = vec![
            user(1, "No", "client"),
            user(3, "No", "client"),
            user(10, "No", "driver"),
            user(12, "No", "driver"),
        ];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-01");
        assert_eq!(result, vec![row("2013-10-01", 0.00)]);
    }

    #[test]
    fn all_cancelled() {
        let trips = vec![
            trip(1, 1, 10, 1, "cancelled_by_client", "2013-10-01"),
            trip(2, 3, 12, 1, "cancelled_by_driver", "2013-10-01"),
        ];
        let users = vec![
            user(1, "No", "client"),
            user(3, "No", "client"),
            user(10, "No", "driver"),
            user(12, "No", "driver"),
        ];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-01");
        assert_eq!(result, vec![row("2013-10-01", 1.00)]);
    }

    #[test]
    fn banned_driver_excludes_trip() {
        let trips = vec![trip(1, 1, 10, 1, "cancelled_by_driver", "2013-10-01")];
        // Driver 10 is banned — trip should be excluded entirely
        let users = vec![user(1, "No", "client"), user(10, "Yes", "driver")];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-01");
        // No qualifying trips => no rows in output
        assert!(result.is_empty());
    }

    #[test]
    fn date_range_filters() {
        // Trips outside the range should be excluded
        let trips = vec![
            trip(1, 1, 10, 1, "completed", "2013-09-30"),
            trip(2, 1, 10, 1, "cancelled_by_client", "2013-10-01"),
            trip(3, 1, 10, 1, "completed", "2013-10-04"),
        ];
        let users = vec![user(1, "No", "client"), user(10, "No", "driver")];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-03");
        assert_eq!(result, vec![row("2013-10-01", 1.00)]);
    }

    #[test]
    fn empty_trips() {
        let trips: Vec<Trip> = vec![];
        let users = vec![user(1, "No", "client")];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-03");
        assert!(result.is_empty());
    }

    #[test]
    fn all_trips_from_banned_users() {
        let trips = vec![
            trip(1, 2, 10, 1, "completed", "2013-10-01"),
            trip(2, 2, 10, 1, "cancelled_by_client", "2013-10-02"),
        ];
        let users = vec![user(2, "Yes", "client"), user(10, "No", "driver")];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-03");
        assert!(result.is_empty());
    }

    #[test]
    fn single_day_mixed() {
        let trips = vec![
            trip(1, 1, 10, 1, "completed", "2013-10-01"),
            trip(2, 3, 12, 1, "cancelled_by_client", "2013-10-01"),
            trip(3, 4, 13, 1, "completed", "2013-10-01"),
        ];
        let users = vec![
            user(1, "No", "client"),
            user(3, "No", "client"),
            user(4, "No", "client"),
            user(10, "No", "driver"),
            user(12, "No", "driver"),
            user(13, "No", "driver"),
        ];
        let result = cancellation_rate(&trips, &users, "2013-10-01", "2013-10-01");
        assert_eq!(result, vec![row("2013-10-01", 0.33)]);
    }
}
