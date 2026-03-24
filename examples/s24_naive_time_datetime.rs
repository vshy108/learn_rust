// Demonstrates NaiveTime and NaiveDateTime.
//
// `NaiveTime` — time without timezone (HH:MM:SS)
// `NaiveDateTime` — date + time without timezone
//
// Run: cargo run --example s24_naive_time_datetime

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

fn main() {
    println!("=== NaiveTime and NaiveDateTime ===");

    // --- 1. NaiveTime ---
    let time = NaiveTime::from_hms_opt(14, 30, 0).expect("invalid time");
    println!("time: {time}");

    let precise = NaiveTime::from_hms_milli_opt(10, 15, 30, 500).expect("invalid");
    println!("precise: {precise}");

    // --- 2. NaiveDateTime ---
    let date = NaiveDate::from_ymd_opt(2024, 6, 15).expect("invalid");
    let dt = NaiveDateTime::new(date, time);
    println!("datetime: {dt}");

    // --- 3. From date and time ---
    let dt2 = date.and_hms_opt(9, 0, 0).expect("invalid");
    println!("morning: {dt2}");

    // --- 4. DateTime arithmetic ---
    let later = dt + TimeDelta::hours(3);
    println!("3 hours later: {later}");

    let earlier = dt - TimeDelta::minutes(45);
    println!("45 minutes earlier: {earlier}");

    // --- 5. Components ---
    println!("hour: {}", dt.format("%H"));
    println!("minute: {}", dt.format("%M"));
    println!("date part: {}", dt.date());

    // --- 6. Parse NaiveDateTime ---
    let parsed = NaiveDateTime::parse_from_str("2024-03-15 10:30:00", "%Y-%m-%d %H:%M:%S")
        .expect("parse failed");
    println!("parsed: {parsed}");
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn test_naive_time() {
        let t = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert_eq!(t.to_string(), "12:00:00");
    }

    #[test]
    fn test_naive_datetime() {
        let d = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let dt = NaiveDateTime::new(d, t);
        assert_eq!(dt.to_string(), "2024-01-01 00:00:00");
    }
}
