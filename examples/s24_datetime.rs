// Demonstrates the DateTime struct with timezones.
//
// `DateTime<Tz>` is a date+time that knows its timezone.
// Common timezone types: `Utc`, `Local`, `FixedOffset`.
//
// Run: cargo run --example s24_datetime

use chrono::{Local, Utc};

fn main() {
    println!("=== DateTime ===");

    // --- 1. UTC now ---
    let utc_now = Utc::now();
    println!("UTC now: {utc_now}");

    // --- 2. Local now ---
    let local_now = Local::now();
    println!("Local now: {local_now}");

    // --- 3. Components ---
    println!("year: {}", local_now.format("%Y"));
    println!("month: {}", local_now.format("%B"));
    println!("day: {}", local_now.format("%d"));
    println!("weekday: {}", local_now.format("%A"));
    println!("time: {}", local_now.format("%H:%M:%S"));

    // --- 4. Timestamp ---
    println!("unix timestamp: {}", utc_now.timestamp());
    println!("millis: {}", utc_now.timestamp_millis());

    // --- 5. Format ---
    println!("RFC 2822: {}", utc_now.to_rfc2822());
    println!("RFC 3339: {}", utc_now.to_rfc3339());
    println!("custom: {}", utc_now.format("%Y/%m/%d %H:%M"));

    // --- 6. Compare ---
    let earlier = Utc::now();
    let _delay = (0..10000).sum::<u64>(); // tiny pause
    let later = Utc::now();
    println!("earlier < later: {}", earlier <= later);
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    #[test]
    fn test_utc_now() {
        let now = Utc::now();
        assert!(now.timestamp() > 0);
    }

    #[test]
    fn test_rfc3339() {
        let now = Utc::now();
        let s = now.to_rfc3339();
        assert!(s.contains('T'));
    }
}
