//! # Rustfinity: Systemtime Usage
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/systemtime-usage
//!
//! The `std::time::SystemTime` type represents a point in time according to the system clock. Unlike `Instant` (which is monotonic and only useful for measuring elapsed time), `SystemTime` can be converted to and from calendar time, making it useful for timestamps, file modification times, and any operation that needs to interact with real-world time.
//!
//! `SystemTime` is anchored to a meaningful point: `UNIX_EPOCH` (January 1, 1970, 00:00:00 UTC). You can compute the duration since the epoch to get a Unix timestamp, or compare two `SystemTime` values to find the elapsed time between them.
//!
//! ## Key Concepts
//!
//! ```rust
//! use std::time::{SystemTime, UNIX_EPOCH, Duration};
//!
//! // Get the current time
//! let now = SystemTime::now();
//!
//! // Get duration since Unix epoch (Unix timestamp)
//! let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
//! let unix_timestamp = since_epoch.as_secs();
//!
//! // Create a SystemTime from a Unix timestamp
//! let timestamp = 1704067200; // Jan 1, 2024 00:00:00 UTC
//! let time = UNIX_EPOCH + Duration::from_secs(timestamp);
//!
//! // Compare times
//! let earlier = UNIX_EPOCH + Duration::from_secs(1000);
//! let later = UNIX_EPOCH + Duration::from_secs(2000);
//! let elapsed = later.duration_since(earlier).unwrap();
//! // 1000 seconds
//! ```
//!
//! ## Error Handling
//!
//! `duration_since` returns a `Result` because the second time might be later than the first (which would require negative duration). This can happen due to clock adjustments or when comparing times incorrectly:
//!
//! ```rust
//! use std::time::{SystemTime, UNIX_EPOCH, Duration};
//!
//! let earlier = UNIX_EPOCH + Duration::from_secs(1000);
//! let later = UNIX_EPOCH + Duration::from_secs(2000);
//!
//! // This works - later is after earlier
//! let ok = later.duration_since(earlier);
//! // Ok(Duration::from_secs(1000))
//!
//! // This fails - earlier is before later
//! let err = earlier.duration_since(later);
//! // Err(SystemTimeError)
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions to work with `SystemTime`:
//!
//! **1. `current_unix_timestamp() -> u64`**
//!
//! Return the current Unix timestamp (seconds since January 1, 1970 UTC).
//!
//! **2. `from_unix_timestamp(timestamp: u64) -> SystemTime`**
//!
//! Create a `SystemTime` from a Unix timestamp.
//!
//! **3. `to_unix_timestamp(time: SystemTime) -> Option<u64>`**
//!
//! Convert a `SystemTime` to a Unix timestamp. Return `None` if the time is before the Unix epoch.
//!
//! **4. `seconds_between(earlier: SystemTime, later: SystemTime) -> Option<u64>`**
//!
//! Calculate the number of seconds between two times. Return `None` if `earlier` is actually after `later`.
//!
//! **5. `is_in_past(time: SystemTime) -> bool`**
//!
//! Check if the given time is in the past (before the current time).
//!
//! **6. `is_in_future(time: SystemTime) -> bool`**
//!
//! Check if the given time is in the future (after the current time).
//!
//! **7. `add_seconds(time: SystemTime, seconds: u64) -> SystemTime`**
//!
//! Add a number of seconds to a `SystemTime`.
//!
//! **8. `time_until(deadline: SystemTime) -> Option<Duration>`**
//!
//! Return the duration until the deadline. Return `None` if the deadline has already passed.
//!
//! ## Examples
//!
//! ```rust
//! use std::time::{SystemTime, UNIX_EPOCH, Duration};
//!
//! // Unix timestamp operations
//! let timestamp = 1704067200; // Jan 1, 2024 00:00:00 UTC
//! let time = from_unix_timestamp(timestamp);
//! assert_eq!(to_unix_timestamp(time), Some(timestamp));
//!
//! // Seconds between times
//! let t1 = from_unix_timestamp(1000);
//! let t2 = from_unix_timestamp(2500);
//! assert_eq!(seconds_between(t1, t2), Some(1500));
//! assert_eq!(seconds_between(t2, t1), None); // t2 is after t1
//!
//! // Past and future checks
//! let past = UNIX_EPOCH + Duration::from_secs(1);
//! assert!(is_in_past(past));
//!
//! let far_future = UNIX_EPOCH
//!     + Duration::from_secs(u64::MAX / 2);
//! assert!(is_in_future(far_future));
//!
//! // Adding time
//! let t = from_unix_timestamp(1000);
//! let later = add_seconds(t, 500);
//! assert_eq!(to_unix_timestamp(later), Some(1500));
//!
//! // Time until deadline
//! let now = SystemTime::now();
//! let deadline = add_seconds(now, 60);
//! let remaining = time_until(deadline);
//! assert!(remaining.is_some());
//! assert!(remaining.unwrap().as_secs() <= 60);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `SystemTime::now()` to get the current time
//! - `UNIX_EPOCH` is available from `std::time`
//! - Use `Duration::from_secs()` to create durations from seconds
//! - `SystemTime` supports `+` and `-` with `Duration`
//! - `duration_since()` returns `Result<Duration, SystemTimeError>`
//! - Use `.ok()` to convert a `Result` to an `Option`, discarding the error
//! - For `is_in_past` and `is_in_future`, compare with `SystemTime::now()`
//! - Remember that `duration_since` fails if the argument is after `self`
//!
//! </details>
//!
//! Run: `cargo run --example rf_systemtime_usage`
//! Test: `cargo test --example rf_systemtime_usage`
//! Filter: `cargo test --example rf_systemtime_usage <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp (seconds since January 1, 1970 UTC).
pub fn current_unix_timestamp() -> u64 {
    // TODO: Get the current time and convert to seconds since UNIX_EPOCH
    todo!()
}

/// Creates a `SystemTime` from a Unix timestamp.
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    // TODO: Add a Duration to UNIX_EPOCH
    todo!()
}

/// Converts a `SystemTime` to a Unix timestamp.
/// Returns `None` if the time is before the Unix epoch.
pub fn to_unix_timestamp(time: SystemTime) -> Option<u64> {
    // TODO: Use duration_since(UNIX_EPOCH) and handle the Result
    todo!()
}

/// Calculates the number of seconds between two times.
/// Returns `None` if `earlier` is actually after `later`.
pub fn seconds_between(earlier: SystemTime, later: SystemTime) -> Option<u64> {
    // TODO: Use duration_since and handle the Result
    todo!()
}

/// Checks if the given time is in the past (before the current time).
pub fn is_in_past(time: SystemTime) -> bool {
    // TODO: Compare with SystemTime::now()
    todo!()
}

/// Checks if the given time is in the future (after the current time).
pub fn is_in_future(time: SystemTime) -> bool {
    // TODO: Compare with SystemTime::now()
    todo!()
}

/// Adds a number of seconds to a `SystemTime`.
pub fn add_seconds(time: SystemTime, seconds: u64) -> SystemTime {
    // TODO: Add a Duration to the SystemTime
    todo!()
}

/// Returns the duration until the deadline.
/// Returns `None` if the deadline has already passed.
pub fn time_until(deadline: SystemTime) -> Option<Duration> {
    // TODO: Calculate duration from now until the deadline
    todo!()
}

fn main() {
    // Example: Get current Unix timestamp
    let now_ts = current_unix_timestamp();
    println!("Current Unix timestamp: {}", now_ts);

    // Example: Create SystemTime from timestamp
    let jan_2024 = from_unix_timestamp(1704067200);
    println!("Jan 1, 2024: {:?}", jan_2024);

    // Example: Convert back to timestamp
    if let Some(ts) = to_unix_timestamp(jan_2024) {
        println!("Converted back: {}", ts);
    }

    // Example: Calculate seconds between times
    let t1 = from_unix_timestamp(1000);
    let t2 = from_unix_timestamp(2500);
    if let Some(secs) = seconds_between(t1, t2) {
        println!("Seconds between: {}", secs);
    }

    // Example: Check past/future
    let past = from_unix_timestamp(1);
    println!("Is in past: {}", is_in_past(past));

    // Example: Add seconds
    let later = add_seconds(t1, 500);
    println!("After adding 500s: {:?}", later);

    // Example: Time until deadline
    let deadline = add_seconds(SystemTime::now(), 60);
    if let Some(remaining) = time_until(deadline) {
        println!("Time until deadline: {:?}", remaining);
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    // ============================================================================
    // Tests for current_unix_timestamp
    // ============================================================================

    #[test]
    fn current_unix_timestamp_is_reasonable() {
        let timestamp = current_unix_timestamp();
        // Should be after Jan 1, 2020
        assert!(
            timestamp > 1577836800,
            "Timestamp should be after year 2020"
        );
        // Should be before year 2100
        assert!(
            timestamp < 4102444800,
            "Timestamp should be before year 2100"
        );
    }

    #[test]
    fn current_unix_timestamp_is_consistent() {
        let t1 = current_unix_timestamp();
        let t2 = current_unix_timestamp();
        // Second call should be >= first call
        assert!(t2 >= t1, "Timestamps should be monotonically increasing");
    }

    // ============================================================================
    // Tests for from_unix_timestamp
    // ============================================================================

    #[test]
    fn from_unix_timestamp_zero() {
        let time = from_unix_timestamp(0);
        assert_eq!(time, UNIX_EPOCH);
    }

    #[test]
    fn from_unix_timestamp_one() {
        let time = from_unix_timestamp(1);
        assert_eq!(time, UNIX_EPOCH + Duration::from_secs(1));
    }

    #[test]
    fn from_unix_timestamp_typical() {
        let timestamp = 1704067200; // Jan 1, 2024 00:00:00 UTC
        let time = from_unix_timestamp(timestamp);
        assert_eq!(time, UNIX_EPOCH + Duration::from_secs(timestamp));
    }

    #[test]
    fn from_unix_timestamp_large() {
        let timestamp = 4102444800; // Year 2100
        let time = from_unix_timestamp(timestamp);
        assert_eq!(time, UNIX_EPOCH + Duration::from_secs(timestamp));
    }

    // ============================================================================
    // Tests for to_unix_timestamp
    // ============================================================================

    #[test]
    fn to_unix_timestamp_epoch() {
        assert_eq!(to_unix_timestamp(UNIX_EPOCH), Some(0));
    }

    #[test]
    fn to_unix_timestamp_one_second() {
        let time = UNIX_EPOCH + Duration::from_secs(1);
        assert_eq!(to_unix_timestamp(time), Some(1));
    }

    #[test]
    fn to_unix_timestamp_typical() {
        let timestamp = 1704067200u64;
        let time = UNIX_EPOCH + Duration::from_secs(timestamp);
        assert_eq!(to_unix_timestamp(time), Some(timestamp));
    }

    #[test]
    fn to_unix_timestamp_round_trip() {
        let original = 1234567890u64;
        let time = from_unix_timestamp(original);
        assert_eq!(to_unix_timestamp(time), Some(original));
    }

    #[test]
    fn to_unix_timestamp_truncates_subseconds() {
        let time = UNIX_EPOCH + Duration::from_millis(1500); // 1.5 seconds
        assert_eq!(to_unix_timestamp(time), Some(1));
    }

    // ============================================================================
    // Tests for seconds_between
    // ============================================================================

    #[test]
    fn seconds_between_same_time() {
        let t = from_unix_timestamp(1000);
        assert_eq!(seconds_between(t, t), Some(0));
    }

    #[test]
    fn seconds_between_one_second() {
        let t1 = from_unix_timestamp(1000);
        let t2 = from_unix_timestamp(1001);
        assert_eq!(seconds_between(t1, t2), Some(1));
    }

    #[test]
    fn seconds_between_many_seconds() {
        let t1 = from_unix_timestamp(1000);
        let t2 = from_unix_timestamp(2500);
        assert_eq!(seconds_between(t1, t2), Some(1500));
    }

    #[test]
    fn seconds_between_wrong_order() {
        let t1 = from_unix_timestamp(1000);
        let t2 = from_unix_timestamp(2500);
        // t2 is after t1, so this should return None
        assert_eq!(seconds_between(t2, t1), None);
    }

    #[test]
    fn seconds_between_from_epoch() {
        let t1 = UNIX_EPOCH;
        let t2 = from_unix_timestamp(3600);
        assert_eq!(seconds_between(t1, t2), Some(3600));
    }

    #[test]
    fn seconds_between_large_gap() {
        let t1 = from_unix_timestamp(0);
        let t2 = from_unix_timestamp(1000000000);
        assert_eq!(seconds_between(t1, t2), Some(1000000000));
    }

    #[test]
    fn seconds_between_truncates_subseconds() {
        let t1 = UNIX_EPOCH + Duration::from_millis(500);
        let t2 = UNIX_EPOCH + Duration::from_millis(2000);
        // Difference is 1.5 seconds, should truncate to 1
        assert_eq!(seconds_between(t1, t2), Some(1));
    }

    // ============================================================================
    // Tests for is_in_past
    // ============================================================================

    #[test]
    fn is_in_past_epoch() {
        assert!(is_in_past(UNIX_EPOCH));
    }

    #[test]
    fn is_in_past_old_date() {
        let old = from_unix_timestamp(1000);
        assert!(is_in_past(old));
    }

    #[test]
    fn is_in_past_year_2020() {
        let y2020 = from_unix_timestamp(1577836800);
        assert!(is_in_past(y2020));
    }

    #[test]
    fn is_in_past_far_future() {
        // Year 3000 - definitely in the future
        let far_future = from_unix_timestamp(32503680000);
        assert!(!is_in_past(far_future));
    }

    // ============================================================================
    // Tests for is_in_future
    // ============================================================================

    #[test]
    fn is_in_future_epoch() {
        assert!(!is_in_future(UNIX_EPOCH));
    }

    #[test]
    fn is_in_future_old_date() {
        let old = from_unix_timestamp(1000);
        assert!(!is_in_future(old));
    }

    #[test]
    fn is_in_future_far_future() {
        // Year 3000 - definitely in the future
        let far_future = from_unix_timestamp(32503680000);
        assert!(is_in_future(far_future));
    }

    #[test]
    fn is_in_future_year_2200() {
        let y2200 = from_unix_timestamp(7258118400);
        assert!(is_in_future(y2200));
    }

    // ============================================================================
    // Tests for add_seconds
    // ============================================================================

    #[test]
    fn add_seconds_zero() {
        let t = from_unix_timestamp(1000);
        let result = add_seconds(t, 0);
        assert_eq!(to_unix_timestamp(result), Some(1000));
    }

    #[test]
    fn add_seconds_one() {
        let t = from_unix_timestamp(1000);
        let result = add_seconds(t, 1);
        assert_eq!(to_unix_timestamp(result), Some(1001));
    }

    #[test]
    fn add_seconds_many() {
        let t = from_unix_timestamp(1000);
        let result = add_seconds(t, 500);
        assert_eq!(to_unix_timestamp(result), Some(1500));
    }

    #[test]
    fn add_seconds_to_epoch() {
        let result = add_seconds(UNIX_EPOCH, 3600);
        assert_eq!(to_unix_timestamp(result), Some(3600));
    }

    #[test]
    fn add_seconds_large_value() {
        let t = from_unix_timestamp(1000);
        let result = add_seconds(t, 1000000);
        assert_eq!(to_unix_timestamp(result), Some(1001000));
    }

    #[test]
    fn add_seconds_preserves_subseconds() {
        let t = UNIX_EPOCH + Duration::from_millis(500);
        let result = add_seconds(t, 1);
        let expected = UNIX_EPOCH + Duration::from_millis(1500);
        assert_eq!(result, expected);
    }

    // ============================================================================
    // Tests for time_until
    // ============================================================================

    #[test]
    fn time_until_past_deadline() {
        let past = from_unix_timestamp(1000);
        assert_eq!(time_until(past), None);
    }

    #[test]
    fn time_until_epoch() {
        assert_eq!(time_until(UNIX_EPOCH), None);
    }

    #[test]
    fn time_until_future_deadline() {
        // Set a deadline 1 hour in the future
        let now = SystemTime::now();
        let deadline = add_seconds(now, 3600);
        let remaining = time_until(deadline);
        assert!(remaining.is_some());
        // Should be close to 3600 seconds (allowing for some execution time)
        let secs = remaining.unwrap().as_secs();
        assert!(secs <= 3600, "Remaining time should be <= 3600 seconds");
        assert!(
            secs >= 3598,
            "Remaining time should be close to 3600 seconds"
        );
    }

    #[test]
    fn time_until_far_future() {
        // Year 3000
        let far_future = from_unix_timestamp(32503680000);
        let remaining = time_until(far_future);
        assert!(remaining.is_some());
        // Should be many years worth of seconds
        assert!(remaining.unwrap().as_secs() > 1000000000);
    }

    // ============================================================================
    // Integration tests
    // ============================================================================

    #[test]
    fn integration_round_trip_timestamp() {
        let original = 1704067200u64;
        let time = from_unix_timestamp(original);
        let back = to_unix_timestamp(time);
        assert_eq!(back, Some(original));
    }

    #[test]
    fn integration_add_and_compare() {
        let t1 = from_unix_timestamp(1000);
        let t2 = add_seconds(t1, 500);
        assert_eq!(seconds_between(t1, t2), Some(500));
    }

    #[test]
    fn integration_past_future_consistency() {
        let now = SystemTime::now();
        let past = from_unix_timestamp(1000);
        let future = add_seconds(now, 3600);

        assert!(is_in_past(past));
        assert!(!is_in_future(past));
        assert!(is_in_future(future));
        assert!(!is_in_past(future));
    }

    #[test]
    fn integration_time_until_and_add() {
        let now = SystemTime::now();
        let deadline = add_seconds(now, 120);
        let remaining = time_until(deadline);
        assert!(remaining.is_some());
        let secs = remaining.unwrap().as_secs();
        assert!(secs <= 120);
        assert!(secs >= 118);
    }

    #[test]
    fn integration_seconds_between_and_to_timestamp() {
        let t1 = from_unix_timestamp(5000);
        let t2 = from_unix_timestamp(8000);
        let diff = seconds_between(t1, t2);
        assert_eq!(diff, Some(3000));

        let ts1 = to_unix_timestamp(t1).unwrap();
        let ts2 = to_unix_timestamp(t2).unwrap();
        assert_eq!(ts2 - ts1, 3000);
    }

    #[test]
    fn integration_chain_add_seconds() {
        let t = from_unix_timestamp(1000);
        let t = add_seconds(t, 100);
        let t = add_seconds(t, 200);
        let t = add_seconds(t, 300);
        assert_eq!(to_unix_timestamp(t), Some(1600));
    }

    #[test]
    fn integration_current_timestamp_to_systemtime() {
        let ts = current_unix_timestamp();
        let time = from_unix_timestamp(ts);
        let back = to_unix_timestamp(time);
        assert_eq!(back, Some(ts));
    }

    #[test]
    fn integration_seconds_between_epoch_and_now() {
        let now = SystemTime::now();
        let secs = seconds_between(UNIX_EPOCH, now);
        assert!(secs.is_some());
        // Should be after year 2020
        assert!(secs.unwrap() > 1577836800);
    }
}
