// Demonstrates the format method for formatting dates and times.
//
// Common format specifiers:
// %Y=year(4), %m=month(01-12), %d=day(01-31), %H=hour(00-23),
// %M=minute(00-59), %S=second(00-59), %A=weekday, %B=month name
//
// Run: cargo run --example s24_format

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

fn main() {
    println!("=== The format Method ===");

    let date = NaiveDate::from_ymd_opt(2024, 12, 25).expect("invalid");
    let time = NaiveTime::from_hms_opt(14, 30, 0).expect("invalid");
    let dt = NaiveDateTime::new(date, time);

    // --- 1. Common formats ---
    println!("ISO 8601: {}", dt.format("%Y-%m-%d"));
    println!("American: {}", dt.format("%m/%d/%Y"));
    println!("European: {}", dt.format("%d.%m.%Y"));
    println!("Long: {}", dt.format("%B %d, %Y"));
    println!("With day: {}", dt.format("%A, %B %d, %Y"));

    // --- 2. Time formats ---
    println!("24h: {}", dt.format("%H:%M:%S"));
    println!("12h: {}", dt.format("%I:%M %p"));

    // --- 3. Full datetime ---
    println!("full: {}", dt.format("%Y-%m-%d %H:%M:%S"));
    println!("readable: {}", dt.format("%A, %B %d, %Y at %I:%M %p"));

    // --- 4. UTC with format ---
    let now = Utc::now();
    println!("UTC now: {}", now.format("%Y-%m-%d %H:%M:%S UTC"));

    // --- 5. to_string vs format ---
    println!("to_string: {dt}");
    println!("format:    {}", dt.format("%+"));

    // --- 6. Use in logging ---
    println!("[{}] Application started", now.format("%Y-%m-%d %H:%M:%S"));
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn test_format_date() {
        let d = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
        assert_eq!(d.format("%Y-%m-%d").to_string(), "2024-01-05");
    }

    #[test]
    fn test_format_datetime() {
        let d = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        let t = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
        let dt = NaiveDateTime::new(d, t);
        assert_eq!(dt.format("%Y-%m-%d %H:%M").to_string(), "2024-12-25 10:00");
    }
}
