#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates the parse_from_str function for parsing dates and times.
//
// Format specifiers: %Y=year, %m=month, %d=day, %H=hour, %M=min, %S=sec, etc.
//
// Run: cargo run --example s24_parse_from_str

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};

fn main() {
    println!("=== parse_from_str ===");

    // --- 1. Parse NaiveDate ---
    let date = NaiveDate::parse_from_str("2024-03-15", "%Y-%m-%d").expect("failed to parse date");
    println!("date: {date}");

    // --- 2. Parse NaiveDateTime ---
    let dt = NaiveDateTime::parse_from_str("2024-03-15 10:30:00", "%Y-%m-%d %H:%M:%S")
        .expect("failed to parse datetime");
    println!("datetime: {dt}");

    // --- 3. Different formats ---
    let american = NaiveDate::parse_from_str("03/15/2024", "%m/%d/%Y").expect("parse failed");
    println!("American format: {american}");

    let european = NaiveDate::parse_from_str("15.03.2024", "%d.%m.%Y").expect("parse failed");
    println!("European format: {european}");

    // --- 4. Parse with timezone ---
    let rfc3339 = DateTime::parse_from_rfc3339("2024-03-15T10:30:00+05:00").expect("parse failed");
    println!("RFC 3339: {rfc3339}");

    let rfc2822 =
        DateTime::parse_from_rfc2822("Fri, 15 Mar 2024 10:30:00 +0000").expect("parse failed");
    println!("RFC 2822: {rfc2822}");

    // --- 5. Custom format with timezone ---
    let custom = DateTime::parse_from_str("2024-03-15 10:30:00 +0900", "%Y-%m-%d %H:%M:%S %z")
        .expect("parse failed");
    println!("custom with tz: {custom}");

    // --- 6. Handle parse errors ---
    let bad = NaiveDate::parse_from_str("not-a-date", "%Y-%m-%d");
    println!("bad parse: {bad:?}");
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset, NaiveDate};

    #[test]
    fn test_parse_date() {
        let d = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    }

    #[test]
    fn test_parse_rfc3339() {
        let dt: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap();
        assert_eq!(dt.timestamp(), 1704067200);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(NaiveDate::parse_from_str("invalid", "%Y-%m-%d").is_err());
    }
}
