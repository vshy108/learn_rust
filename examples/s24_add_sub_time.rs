// Demonstrates adding and subtracting time with chrono.
//
// You can add/subtract TimeDelta from dates and datetimes.
//
// Run: cargo run --example s24_add_sub_time

use chrono::{NaiveDate, TimeDelta};

fn main() {
    println!("=== Adding and Subtracting Time ===");

    let today = NaiveDate::from_ymd_opt(2024, 6, 15).expect("invalid date");

    // --- 1. Add days ---
    let next_week = today + TimeDelta::days(7);
    println!("today: {today}");
    println!("next week: {next_week}");

    // --- 2. Subtract days ---
    let last_month = today - TimeDelta::days(30);
    println!("30 days ago: {last_month}");

    // --- 3. Difference between dates ---
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).expect("invalid");
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).expect("invalid");
    let diff = end - start;
    println!("days in 2024: {}", diff.num_days());

    // --- 4. Add weeks ---
    let two_weeks = today + TimeDelta::weeks(2);
    println!("2 weeks from now: {two_weeks}");

    // --- 5. Calculate age in days ---
    let birthdate = NaiveDate::from_ymd_opt(1990, 5, 20).expect("invalid");
    let age_days = (today - birthdate).num_days();
    println!("age in days since 1990-05-20: {age_days}");

    // --- 6. Iterate over date range ---
    let start = NaiveDate::from_ymd_opt(2024, 12, 25).expect("invalid");
    let mut d = start;
    println!("holiday week:");
    for _ in 0..7 {
        println!("  {}", d.format("%A, %B %d"));
        d += TimeDelta::days(1);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, TimeDelta};

    #[test]
    fn test_add_days() {
        let d = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = d + TimeDelta::days(31);
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 2, 1).unwrap());
    }

    #[test]
    fn test_difference() {
        let a = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let b = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!((b - a).num_days(), 30);
    }
}
