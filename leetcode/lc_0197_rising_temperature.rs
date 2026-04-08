// LeetCode 197: Rising Temperature [Easy]
// https://leetcode.com/problems/rising-temperature/
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
//   SELECT w1.id
//   FROM Weather w1
//   JOIN Weather w2
//     ON DATEDIFF(w1.recordDate, w2.recordDate) = 1
//    AND w1.temperature > w2.temperature;
//
// Run:  cargo run --example lc_0197_rising_temperature
// Test: cargo test --example lc_0197_rising_temperature

use std::collections::HashMap;

/// A row in the Weather table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Weather {
    id: i32,
    record_date: String,
    temperature: i32,
}

/// Parse a "YYYY-MM-DD" date string into a serial day number for easy
/// subtraction. Uses a simplified calculation (not a full calendar
/// library) that correctly handles month lengths and leap years.
///
/// We avoid external crates like `chrono` by computing a serial day
/// number. This mirrors what SQL's DATEDIFF does internally.
fn date_to_days(date: &str) -> i64 {
    let parts: Vec<i64> = date
        .split('-')
        .map(|s| s.parse().expect("valid date part"))
        .collect();
    let (mut y, mut m, d) = (parts[0], parts[1], parts[2]);

    // Shift Jan/Feb to months 13/14 of the previous year so the
    // leap day falls at the end of the "year" for easier arithmetic.
    if m <= 2 {
        y -= 1;
        m += 12;
    }

    // Serial day number (Julian Day Number variant)
    365 * y + y / 4 - y / 100 + y / 400 + (153 * (m - 3) + 2) / 5 + d
}

/// Find ids of weather records where the temperature is higher than
/// the previous day's temperature.
///
/// # Arguments
/// * `weather` - Slice of `Weather` rows
///
/// # Returns
/// A sorted `Vec<i32>` of ids with rising temperature vs. prior day
///
/// # Approach
/// Build a HashMap keyed by serial day number (like a self-join index).
/// For each row, look up the record from the previous calendar day
/// (day_number - 1). If found and today's temp > yesterday's temp,
/// include today's id. This mirrors the SQL self-JOIN with DATEDIFF = 1.
fn rising_temperature(weather: &[Weather]) -> Vec<i32> {
    // Index weather by serial day number for O(1) lookup.
    // This is analogous to joining on DATEDIFF(w1.date, w2.date) = 1.
    let by_day: HashMap<i64, &Weather> = weather
        .iter()
        .map(|w| (date_to_days(&w.record_date), w))
        .collect();

    let mut result: Vec<i32> = weather
        .iter()
        .filter(|w| {
            let today = date_to_days(&w.record_date);
            // Look up yesterday's record (DATEDIFF = 1)
            by_day
                .get(&(today - 1))
                .is_some_and(|yesterday| w.temperature > yesterday.temperature)
        })
        .map(|w| w.id)
        .collect();

    result.sort_unstable();
    result
}

fn main() {
    println!("LeetCode 197: Rising Temperature");

    let weather = vec![
        Weather {
            id: 1,
            record_date: "2015-01-01".into(),
            temperature: 10,
        },
        Weather {
            id: 2,
            record_date: "2015-01-02".into(),
            temperature: 25,
        },
        Weather {
            id: 3,
            record_date: "2015-01-03".into(),
            temperature: 20,
        },
        Weather {
            id: 4,
            record_date: "2015-01-04".into(),
            temperature: 30,
        },
    ];

    let result = rising_temperature(&weather);
    println!("  Rising temperature ids: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(id: i32, date: &str, temp: i32) -> Weather {
        Weather {
            id,
            record_date: date.to_string(),
            temperature: temp,
        }
    }

    #[test]
    fn example_main() {
        let weather = vec![
            w(1, "2015-01-01", 10),
            w(2, "2015-01-02", 25),
            w(3, "2015-01-03", 20),
            w(4, "2015-01-04", 30),
        ];
        assert_eq!(rising_temperature(&weather), vec![2, 4]);
    }

    #[test]
    fn no_rise() {
        let weather = vec![
            w(1, "2021-03-01", 30),
            w(2, "2021-03-02", 20),
            w(3, "2021-03-03", 10),
        ];
        assert_eq!(rising_temperature(&weather), Vec::<i32>::new());
    }

    #[test]
    fn all_rising() {
        let weather = vec![
            w(1, "2020-06-01", 10),
            w(2, "2020-06-02", 20),
            w(3, "2020-06-03", 30),
        ];
        assert_eq!(rising_temperature(&weather), vec![2, 3]);
    }

    #[test]
    fn non_consecutive_dates_skipped() {
        // Gap between Jan 1 and Jan 3 — no "yesterday" for Jan 3
        let weather = vec![w(1, "2022-01-01", 5), w(2, "2022-01-03", 50)];
        assert_eq!(rising_temperature(&weather), Vec::<i32>::new());
    }

    #[test]
    fn same_temperature_not_rising() {
        let weather = vec![w(1, "2023-07-10", 25), w(2, "2023-07-11", 25)];
        // Equal temperature does not count — must be strictly higher
        assert_eq!(rising_temperature(&weather), Vec::<i32>::new());
    }

    #[test]
    fn single_row() {
        let weather = vec![w(1, "2024-01-01", 15)];
        assert_eq!(rising_temperature(&weather), Vec::<i32>::new());
    }

    #[test]
    fn empty_table() {
        let weather: Vec<Weather> = vec![];
        assert_eq!(rising_temperature(&weather), Vec::<i32>::new());
    }

    #[test]
    fn cross_month_boundary() {
        // Jan 31 -> Feb 1 is a consecutive day pair
        let weather = vec![w(1, "2023-01-31", 10), w(2, "2023-02-01", 20)];
        assert_eq!(rising_temperature(&weather), vec![2]);
    }

    #[test]
    fn leap_year_feb_boundary() {
        // 2024 is a leap year: Feb 28 -> Feb 29 -> Mar 1
        let weather = vec![
            w(1, "2024-02-28", 5),
            w(2, "2024-02-29", 10),
            w(3, "2024-03-01", 15),
        ];
        assert_eq!(rising_temperature(&weather), vec![2, 3]);
    }

    #[test]
    fn unordered_input() {
        // Rows not sorted by date — should still work via HashMap lookup
        let weather = vec![
            w(4, "2015-01-04", 30),
            w(1, "2015-01-01", 10),
            w(3, "2015-01-03", 20),
            w(2, "2015-01-02", 25),
        ];
        assert_eq!(rising_temperature(&weather), vec![2, 4]);
    }
}
