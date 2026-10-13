//! # Rustfinity: Duration Operations
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/duration-operations
//!
//! The `std::time::Duration` type represents a span of time, typically used for system timeouts, sleep operations, and measuring elapsed time. Unlike calendar time (which can have leap seconds and timezone issues), a `Duration` is a simple, monotonic count of seconds and nanoseconds.
//!
//! `Duration` provides methods to create durations from various units (seconds, milliseconds, microseconds, nanoseconds), extract values in different units, and perform arithmetic operations. It's designed to be precise and overflow-safe, making it ideal for timing operations in systems programming.
//!
//! ## Creating Durations
//!
//! ```rust
//! use std::time::Duration;
//!
//! // From various units
//! let d1 = Duration::from_secs(5);           // 5 seconds
//! let d2 = Duration::from_millis(1500);      // 1.5 seconds
//! let d3 = Duration::from_micros(1_000_000); // 1 second
//! let d4 = Duration::from_nanos(1_000_000_000); // 1 second
//! let d5 = Duration::from_secs_f64(2.5);     // 2.5 seconds
//!
//! // Zero and maximum
//! let zero = Duration::ZERO;
//! let max = Duration::MAX;
//! ```
//!
//! ## Extracting Values
//!
//! ```rust
//! use std::time::Duration;
//!
//! let d = Duration::from_millis(2500);
//!
//! // Total values (truncates)
//! d.as_secs();    // 2
//! d.as_millis();  // 2500
//! d.as_micros();  // 2_500_000
//! d.as_nanos();   // 2_500_000_000
//!
//! // Floating point total
//! d.as_secs_f64(); // 2.5
//! d.as_secs_f32(); // 2.5
//!
//! // Sub-second component (nanoseconds portion)
//! d.subsec_nanos();  // 500_000_000
//! d.subsec_micros(); // 500_000
//! d.subsec_millis(); // 500
//! ```
//!
//! ## Arithmetic Operations
//!
//! ```rust
//! use std::time::Duration;
//!
//! let d1 = Duration::from_secs(5);
//! let d2 = Duration::from_secs(3);
//!
//! // Addition and subtraction
//! let sum = d1 + d2;           // 8 seconds
//! let diff = d1 - d2;          // 2 seconds
//!
//! // Checked operations (return Option)
//! let checked = d1.checked_add(d2);  // Some(8 seconds)
//! let overflow = Duration::MAX.checked_add(d1); // None
//!
//! // Saturating operations (clamp at bounds)
//! let saturated = Duration::MAX.saturating_add(d1);
//! // Duration::MAX
//!
//! // Multiplication and division
//! let doubled = d1 * 2;        // 10 seconds
//! let halved = d1 / 2;         // 2.5 seconds
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions to work with `Duration`:
//!
//! **1. `from_minutes(minutes: u64) -> Duration`**
//!
//! Create a Duration from a number of minutes.
//!
//! **2. `from_hours(hours: u64) -> Duration`**
//!
//! Create a Duration from a number of hours.
//!
//! **3. `to_minutes(duration: Duration) -> u64`**
//!
//! Convert a Duration to total minutes (truncated).
//!
//! **4. `to_hours(duration: Duration) -> u64`**
//!
//! Convert a Duration to total hours (truncated).
//!
//! **5. `format_duration(duration: Duration) -> String`**
//!
//! Format a duration as "Xh Ym Zs" (e.g., "2h 30m 45s").
//! Omit zero components at the start (e.g., "30m 45s" not
//! "0h 30m 45s"), but always show seconds.
//!
//! **6. `add_durations(durations: &[Duration]) -> Duration`**
//!
//! Sum all durations in a slice. Return `Duration::ZERO` for
//! an empty slice.
//!
//! **7. `average_duration(durations: &[Duration]) -> Option<Duration>`**
//!
//! Calculate the average duration. Return `None` for an
//! empty slice.
//!
//! **8. `is_longer_than(d1: Duration, d2: Duration) -> bool`**
//!
//! Check if the first duration is strictly longer than the
//! second.
//!
//! ## Examples
//!
//! ```rust
//! use std::time::Duration;
//!
//! // Creating from minutes and hours
//! let thirty_min = from_minutes(30);
//! assert_eq!(thirty_min, Duration::from_secs(1800));
//!
//! let two_hours = from_hours(2);
//! assert_eq!(two_hours, Duration::from_secs(7200));
//!
//! // Converting to minutes and hours
//! let d = Duration::from_secs(7500); // 2h 5m
//! assert_eq!(to_minutes(d), 125);
//! assert_eq!(to_hours(d), 2);
//!
//! // Formatting
//! let d = Duration::from_secs(9045); // 2h 30m 45s
//! assert_eq!(format_duration(d), "2h 30m 45s");
//!
//! let d = Duration::from_secs(45);
//! assert_eq!(format_duration(d), "45s");
//!
//! let d = Duration::from_secs(1845); // 30m 45s
//! assert_eq!(format_duration(d), "30m 45s");
//!
//! // Adding durations
//! let durations = vec![
//!     Duration::from_secs(60),
//!     Duration::from_secs(120),
//!     Duration::from_secs(180),
//! ];
//! assert_eq!(
//!     add_durations(&durations),
//!     Duration::from_secs(360)
//! );
//!
//! // Average duration
//! let durations = vec![
//!     Duration::from_secs(10),
//!     Duration::from_secs(20),
//!     Duration::from_secs(30),
//! ];
//! assert_eq!(
//!     average_duration(&durations),
//!     Some(Duration::from_secs(20))
//! );
//! assert_eq!(average_duration(&[]), None);
//!
//! // Comparison
//! assert!(is_longer_than(
//!     Duration::from_secs(10),
//!     Duration::from_secs(5)
//! ));
//! assert!(!is_longer_than(
//!     Duration::from_secs(5),
//!     Duration::from_secs(5)
//! ));
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - One minute is 60 seconds, one hour is 3600 seconds
//! - Use `as_secs()` to get the total seconds, then divide to get minutes/hours
//! - For `format_duration`, calculate hours, minutes, and seconds separately using division and modulo
//! - Use `Duration::ZERO` for the identity element when summing
//! - For average, sum all durations then divide by the count
//! - `Duration` implements `Ord`, so you can use comparison operators directly
//! - Remember that `Duration` division by a number uses `duration / n` where n is `u32`
//!
//! </details>
//!
//! Run: `cargo run --example rf_duration_operations`
//! Test: `cargo test --example rf_duration_operations`
//! Filter: `cargo test --example rf_duration_operations <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::time::Duration;

/// Creates a Duration from a number of minutes.
pub fn from_minutes(minutes: u64) -> Duration {
    // TODO: Implement this function
    unimplemented!()
}

/// Creates a Duration from a number of hours.
pub fn from_hours(hours: u64) -> Duration {
    // TODO: Implement this function
    unimplemented!()
}

/// Converts a Duration to total minutes (truncated).
pub fn to_minutes(duration: Duration) -> u64 {
    // TODO: Implement this function
    unimplemented!()
}

/// Converts a Duration to total hours (truncated).
pub fn to_hours(duration: Duration) -> u64 {
    // TODO: Implement this function
    unimplemented!()
}

/// Formats a duration as "Xh Ym Zs".
/// Omits zero components at the start (e.g., "30m 45s" not "0h 30m 45s"),
/// but always shows seconds.
pub fn format_duration(duration: Duration) -> String {
    // TODO: Implement this function
    unimplemented!()
}

/// Sums all durations in a slice.
/// Returns Duration::ZERO for an empty slice.
pub fn add_durations(durations: &[Duration]) -> Duration {
    // TODO: Implement this function
    unimplemented!()
}

/// Calculates the average duration.
/// Returns None for an empty slice.
pub fn average_duration(durations: &[Duration]) -> Option<Duration> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if the first duration is strictly longer than the second.
pub fn is_longer_than(d1: Duration, d2: Duration) -> bool {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    // Creating durations from minutes and hours
    let thirty_min = from_minutes(30);
    println!("30 minutes = {:?}", thirty_min);

    let two_hours = from_hours(2);
    println!("2 hours = {:?}", two_hours);

    // Converting to minutes and hours
    let d = Duration::from_secs(7500);
    println!(
        "{:?} = {} minutes = {} hours",
        d,
        to_minutes(d),
        to_hours(d)
    );

    // Formatting durations
    let d1 = Duration::from_secs(9045);
    let d2 = Duration::from_secs(45);
    let d3 = Duration::from_secs(1845);
    println!(
        "Formatted: {}, {}, {}",
        format_duration(d1),
        format_duration(d2),
        format_duration(d3)
    );

    // Adding durations
    let durations = vec![
        Duration::from_secs(60),
        Duration::from_secs(120),
        Duration::from_secs(180),
    ];
    println!("Sum: {:?}", add_durations(&durations));

    // Average duration
    println!("Average: {:?}", average_duration(&durations));

    // Comparison
    let short = Duration::from_secs(5);
    let long = Duration::from_secs(10);
    println!("{:?} > {:?}? {}", long, short, is_longer_than(long, short));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::time::Duration;

    // =============================================================================
    // from_minutes tests
    // =============================================================================

    #[test]
    fn test_from_minutes_zero() {
        assert_eq!(from_minutes(0), Duration::ZERO);
    }

    #[test]
    fn test_from_minutes_one() {
        assert_eq!(from_minutes(1), Duration::from_secs(60));
    }

    #[test]
    fn test_from_minutes_thirty() {
        assert_eq!(from_minutes(30), Duration::from_secs(1800));
    }

    #[test]
    fn test_from_minutes_sixty() {
        assert_eq!(from_minutes(60), Duration::from_secs(3600));
    }

    #[test]
    fn test_from_minutes_large() {
        assert_eq!(from_minutes(1440), Duration::from_secs(86400)); // 24 hours
    }

    // =============================================================================
    // from_hours tests
    // =============================================================================

    #[test]
    fn test_from_hours_zero() {
        assert_eq!(from_hours(0), Duration::ZERO);
    }

    #[test]
    fn test_from_hours_one() {
        assert_eq!(from_hours(1), Duration::from_secs(3600));
    }

    #[test]
    fn test_from_hours_two() {
        assert_eq!(from_hours(2), Duration::from_secs(7200));
    }

    #[test]
    fn test_from_hours_twenty_four() {
        assert_eq!(from_hours(24), Duration::from_secs(86400));
    }

    #[test]
    fn test_from_hours_large() {
        assert_eq!(from_hours(168), Duration::from_secs(604800)); // 1 week
    }

    // =============================================================================
    // to_minutes tests
    // =============================================================================

    #[test]
    fn test_to_minutes_zero() {
        assert_eq!(to_minutes(Duration::ZERO), 0);
    }

    #[test]
    fn test_to_minutes_one_minute() {
        assert_eq!(to_minutes(Duration::from_secs(60)), 1);
    }

    #[test]
    fn test_to_minutes_partial_minute() {
        assert_eq!(to_minutes(Duration::from_secs(90)), 1); // truncates
    }

    #[test]
    fn test_to_minutes_exact() {
        assert_eq!(to_minutes(Duration::from_secs(1800)), 30);
    }

    #[test]
    fn test_to_minutes_large() {
        assert_eq!(to_minutes(Duration::from_secs(7500)), 125);
    }

    #[test]
    fn test_to_minutes_with_millis() {
        assert_eq!(to_minutes(Duration::from_millis(90500)), 1); // sub-second ignored
    }

    // =============================================================================
    // to_hours tests
    // =============================================================================

    #[test]
    fn test_to_hours_zero() {
        assert_eq!(to_hours(Duration::ZERO), 0);
    }

    #[test]
    fn test_to_hours_one_hour() {
        assert_eq!(to_hours(Duration::from_secs(3600)), 1);
    }

    #[test]
    fn test_to_hours_partial_hour() {
        assert_eq!(to_hours(Duration::from_secs(5400)), 1); // 1.5 hours, truncates
    }

    #[test]
    fn test_to_hours_exact() {
        assert_eq!(to_hours(Duration::from_secs(7200)), 2);
    }

    #[test]
    fn test_to_hours_large() {
        assert_eq!(to_hours(Duration::from_secs(86400)), 24);
    }

    #[test]
    fn test_to_hours_less_than_hour() {
        assert_eq!(to_hours(Duration::from_secs(1800)), 0);
    }

    // =============================================================================
    // format_duration tests
    // =============================================================================

    #[test]
    fn test_format_duration_zero() {
        assert_eq!(format_duration(Duration::ZERO), "0s");
    }

    #[test]
    fn test_format_duration_seconds_only() {
        assert_eq!(format_duration(Duration::from_secs(45)), "45s");
    }

    #[test]
    fn test_format_duration_one_second() {
        assert_eq!(format_duration(Duration::from_secs(1)), "1s");
    }

    #[test]
    fn test_format_duration_minutes_and_seconds() {
        assert_eq!(format_duration(Duration::from_secs(1845)), "30m 45s");
    }

    #[test]
    fn test_format_duration_minutes_zero_seconds() {
        assert_eq!(format_duration(Duration::from_secs(1800)), "30m 0s");
    }

    #[test]
    fn test_format_duration_one_minute() {
        assert_eq!(format_duration(Duration::from_secs(60)), "1m 0s");
    }

    #[test]
    fn test_format_duration_hours_minutes_seconds() {
        assert_eq!(format_duration(Duration::from_secs(9045)), "2h 30m 45s");
    }

    #[test]
    fn test_format_duration_hours_zero_minutes_zero_seconds() {
        assert_eq!(format_duration(Duration::from_secs(7200)), "2h 0m 0s");
    }

    #[test]
    fn test_format_duration_hours_only_zero_minutes() {
        assert_eq!(format_duration(Duration::from_secs(7245)), "2h 0m 45s");
    }

    #[test]
    fn test_format_duration_large() {
        assert_eq!(format_duration(Duration::from_secs(90061)), "25h 1m 1s");
    }

    #[test]
    fn test_format_duration_fifty_nine_seconds() {
        assert_eq!(format_duration(Duration::from_secs(59)), "59s");
    }

    #[test]
    fn test_format_duration_fifty_nine_minutes() {
        assert_eq!(format_duration(Duration::from_secs(3599)), "59m 59s");
    }

    // =============================================================================
    // add_durations tests
    // =============================================================================

    #[test]
    fn test_add_durations_empty() {
        assert_eq!(add_durations(&[]), Duration::ZERO);
    }

    #[test]
    fn test_add_durations_single() {
        let durations = [Duration::from_secs(60)];
        assert_eq!(add_durations(&durations), Duration::from_secs(60));
    }

    #[test]
    fn test_add_durations_multiple() {
        let durations = vec![
            Duration::from_secs(60),
            Duration::from_secs(120),
            Duration::from_secs(180),
        ];
        assert_eq!(add_durations(&durations), Duration::from_secs(360));
    }

    #[test]
    fn test_add_durations_with_zero() {
        let durations = vec![
            Duration::from_secs(100),
            Duration::ZERO,
            Duration::from_secs(50),
        ];
        assert_eq!(add_durations(&durations), Duration::from_secs(150));
    }

    #[test]
    fn test_add_durations_all_same() {
        let durations = vec![Duration::from_secs(10); 5];
        assert_eq!(add_durations(&durations), Duration::from_secs(50));
    }

    #[test]
    fn test_add_durations_with_millis() {
        let durations = vec![
            Duration::from_millis(500),
            Duration::from_millis(750),
            Duration::from_millis(250),
        ];
        assert_eq!(add_durations(&durations), Duration::from_millis(1500));
    }

    // =============================================================================
    // average_duration tests
    // =============================================================================

    #[test]
    fn test_average_duration_empty() {
        assert_eq!(average_duration(&[]), None);
    }

    #[test]
    fn test_average_duration_single() {
        let durations = [Duration::from_secs(60)];
        assert_eq!(average_duration(&durations), Some(Duration::from_secs(60)));
    }

    #[test]
    fn test_average_duration_multiple() {
        let durations = vec![
            Duration::from_secs(10),
            Duration::from_secs(20),
            Duration::from_secs(30),
        ];
        assert_eq!(average_duration(&durations), Some(Duration::from_secs(20)));
    }

    #[test]
    fn test_average_duration_even() {
        let durations = vec![Duration::from_secs(100), Duration::from_secs(200)];
        assert_eq!(average_duration(&durations), Some(Duration::from_secs(150)));
    }

    #[test]
    fn test_average_duration_preserves_precision() {
        let durations = vec![Duration::from_secs(10), Duration::from_secs(11)];
        // Average is 10.5 seconds - Duration division preserves subsecond precision
        assert_eq!(
            average_duration(&durations),
            Some(Duration::from_millis(10500))
        );
    }

    #[test]
    fn test_average_duration_all_zeros() {
        let durations = vec![Duration::ZERO; 3];
        assert_eq!(average_duration(&durations), Some(Duration::ZERO));
    }

    #[test]
    fn test_average_duration_with_millis() {
        let durations = vec![
            Duration::from_millis(1000),
            Duration::from_millis(2000),
            Duration::from_millis(3000),
        ];
        assert_eq!(
            average_duration(&durations),
            Some(Duration::from_millis(2000))
        );
    }

    // =============================================================================
    // is_longer_than tests
    // =============================================================================

    #[test]
    fn test_is_longer_than_true() {
        assert!(is_longer_than(
            Duration::from_secs(10),
            Duration::from_secs(5)
        ));
    }

    #[test]
    fn test_is_longer_than_false_equal() {
        assert!(!is_longer_than(
            Duration::from_secs(5),
            Duration::from_secs(5)
        ));
    }

    #[test]
    fn test_is_longer_than_false_shorter() {
        assert!(!is_longer_than(
            Duration::from_secs(3),
            Duration::from_secs(5)
        ));
    }

    #[test]
    fn test_is_longer_than_zero() {
        assert!(is_longer_than(Duration::from_secs(1), Duration::ZERO));
        assert!(!is_longer_than(Duration::ZERO, Duration::ZERO));
        assert!(!is_longer_than(Duration::ZERO, Duration::from_secs(1)));
    }

    #[test]
    fn test_is_longer_than_millis() {
        assert!(is_longer_than(
            Duration::from_millis(1001),
            Duration::from_secs(1)
        ));
        assert!(!is_longer_than(
            Duration::from_millis(999),
            Duration::from_secs(1)
        ));
    }

    #[test]
    fn test_is_longer_than_nanos() {
        let d1 = Duration::from_nanos(1_000_000_001);
        let d2 = Duration::from_secs(1);
        assert!(is_longer_than(d1, d2));
    }

    // =============================================================================
    // Integration tests
    // =============================================================================

    #[test]
    fn test_round_trip_minutes() {
        let original = 45u64;
        let duration = from_minutes(original);
        assert_eq!(to_minutes(duration), original);
    }

    #[test]
    fn test_round_trip_hours() {
        let original = 12u64;
        let duration = from_hours(original);
        assert_eq!(to_hours(duration), original);
    }

    #[test]
    fn test_format_and_parse_consistency() {
        // 1 hour, 30 minutes, 15 seconds = 5415 seconds
        let duration = Duration::from_secs(5415);
        let formatted = format_duration(duration);
        assert_eq!(formatted, "1h 30m 15s");
    }

    #[test]
    fn test_add_and_average_relationship() {
        let durations = vec![
            Duration::from_secs(100),
            Duration::from_secs(200),
            Duration::from_secs(300),
        ];
        let sum = add_durations(&durations);
        let avg = average_duration(&durations).unwrap();
        assert_eq!(sum, Duration::from_secs(600));
        assert_eq!(avg, Duration::from_secs(200));
        assert_eq!(sum, avg * 3);
    }

    #[test]
    fn test_comparison_with_converted_values() {
        let one_hour = from_hours(1);
        let sixty_minutes = from_minutes(60);
        assert!(!is_longer_than(one_hour, sixty_minutes));
        assert!(!is_longer_than(sixty_minutes, one_hour));
    }

    #[test]
    fn test_mixed_units_addition() {
        let durations = vec![
            from_hours(1),           // 3600s
            from_minutes(30),        // 1800s
            Duration::from_secs(45), // 45s
        ];
        let total = add_durations(&durations);
        assert_eq!(total, Duration::from_secs(5445));
        assert_eq!(format_duration(total), "1h 30m 45s");
    }

    #[test]
    fn test_edge_case_just_under_minute() {
        let d = Duration::from_secs(59);
        assert_eq!(to_minutes(d), 0);
        assert_eq!(format_duration(d), "59s");
    }

    #[test]
    fn test_edge_case_just_under_hour() {
        let d = Duration::from_secs(3599);
        assert_eq!(to_hours(d), 0);
        assert_eq!(to_minutes(d), 59);
        assert_eq!(format_duration(d), "59m 59s");
    }

    #[test]
    fn test_subsecond_precision_ignored_in_format() {
        let d = Duration::from_millis(5500); // 5.5 seconds
        assert_eq!(format_duration(d), "5s");
    }

    #[test]
    fn test_many_small_durations() {
        let durations: Vec<Duration> = (0..100).map(|_| Duration::from_millis(10)).collect();
        assert_eq!(add_durations(&durations), Duration::from_secs(1));
    }
}
