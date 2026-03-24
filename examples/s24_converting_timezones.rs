// Demonstrates converting between timezones with chrono.
//
// Use `.with_timezone(&tz)` to convert a DateTime from one timezone to another.
//
// Run: cargo run --example s24_converting_timezones

use chrono::{FixedOffset, Utc};

fn main() {
    println!("=== Converting Timezones ===");

    let utc_now = Utc::now();
    println!("UTC: {utc_now}");

    // --- 1. UTC to fixed offset ---
    // EST = UTC-5
    let est = FixedOffset::west_opt(5 * 3600).expect("invalid offset");
    let est_time = utc_now.with_timezone(&est);
    println!("EST (UTC-5): {est_time}");

    // JST = UTC+9
    let jst = FixedOffset::east_opt(9 * 3600).expect("invalid offset");
    let jst_time = utc_now.with_timezone(&jst);
    println!("JST (UTC+9): {jst_time}");

    // --- 2. IST = UTC+5:30 ---
    let ist = FixedOffset::east_opt(5 * 3600 + 30 * 60).expect("invalid offset");
    let ist_time = utc_now.with_timezone(&ist);
    println!("IST (UTC+5:30): {ist_time}");

    // --- 3. Convert between non-UTC timezones ---
    let est_to_jst = est_time.with_timezone(&jst);
    println!("EST to JST: {est_to_jst}");

    // --- 4. Convert back to UTC ---
    let back_to_utc = jst_time.with_timezone(&Utc);
    println!("back to UTC: {back_to_utc}");

    // --- 5. All have the same instant ---
    assert_eq!(utc_now.timestamp(), est_time.timestamp());
    assert_eq!(utc_now.timestamp(), jst_time.timestamp());
    println!("All timestamps equal: {}", utc_now.timestamp());
}

#[cfg(test)]
mod tests {
    use chrono::{FixedOffset, Utc};

    #[test]
    fn test_timezone_conversion() {
        let utc = Utc::now();
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let converted = utc.with_timezone(&jst);
        // Same instant in time
        assert_eq!(utc.timestamp(), converted.timestamp());
    }

    #[test]
    fn test_roundtrip() {
        let utc = Utc::now();
        let est = FixedOffset::west_opt(5 * 3600).unwrap();
        let back = utc.with_timezone(&est).with_timezone(&Utc);
        assert_eq!(utc.timestamp(), back.timestamp());
    }
}
