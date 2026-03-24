// Demonstrates the TimeDelta struct from chrono.
//
// `TimeDelta` (formerly `Duration`) represents a span of time.
//
// Run: cargo run --example s24_time_delta

use chrono::TimeDelta;

fn main() {
    println!("=== TimeDelta ===");

    // --- 1. Create durations ---
    let one_day = TimeDelta::days(1);
    let two_hours = TimeDelta::hours(2);
    let thirty_mins = TimeDelta::minutes(30);
    let ten_secs = TimeDelta::seconds(10);

    println!("1 day: {one_day}");
    println!("2 hours: {two_hours}");
    println!("30 mins: {thirty_mins}");
    println!("10 secs: {ten_secs}");

    // --- 2. Arithmetic ---
    let total = one_day + two_hours + thirty_mins;
    println!("1d + 2h + 30m = {total}");

    // --- 3. Components ---
    let week = TimeDelta::weeks(1);
    println!("1 week in days: {}", week.num_days());
    println!("1 week in hours: {}", week.num_hours());
    println!("1 week in seconds: {}", week.num_seconds());

    // --- 4. Negative durations ---
    let neg = TimeDelta::days(-3);
    println!("negative: {neg}");

    // --- 5. Zero duration ---
    let zero = TimeDelta::zero();
    println!("zero: {zero}");
    println!("is zero? {}", zero.is_zero());
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    #[test]
    fn test_days() {
        let d = TimeDelta::days(1);
        assert_eq!(d.num_hours(), 24);
        assert_eq!(d.num_seconds(), 86400);
    }

    #[test]
    fn test_arithmetic() {
        let a = TimeDelta::hours(1);
        let b = TimeDelta::minutes(30);
        assert_eq!((a + b).num_minutes(), 90);
    }

    #[test]
    fn test_zero() {
        assert!(TimeDelta::zero().is_zero());
    }
}
