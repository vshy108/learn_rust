// Demonstrates the NaiveDate struct from chrono.
//
// `NaiveDate` represents a date without timezone information.
// "Naive" = no timezone awareness.
//
// Run: cargo run --example s24_naive_date

use chrono::NaiveDate;

fn main() {
    println!("=== NaiveDate ===");

    // --- 1. Create a date ---
    let date = NaiveDate::from_ymd_opt(2024, 3, 15).expect("invalid date");
    println!("date: {date}");

    // --- 2. Today (requires Local) ---
    let today = chrono::Local::now().date_naive();
    println!("today: {today}");

    // --- 3. Components ---
    println!("year: {}", date.format("%Y"));
    println!("month: {}", date.format("%m"));
    println!("day: {}", date.format("%d"));
    println!("weekday: {}", date.format("%A"));

    // --- 4. Date arithmetic ---
    let tomorrow = today.succ_opt().expect("no tomorrow");
    let yesterday = today.pred_opt().expect("no yesterday");
    println!("yesterday: {yesterday}, today: {today}, tomorrow: {tomorrow}");

    // --- 5. Parse from string ---
    let parsed = NaiveDate::parse_from_str("2024-12-25", "%Y-%m-%d").expect("parse failed");
    println!("parsed: {parsed}");

    // --- 6. Comparisons ---
    let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).expect("invalid");
    println!("is today before new year? {}", today < new_year);
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[test]
    fn test_create_date() {
        let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert_eq!(d.to_string(), "2024-03-15");
    }

    #[test]
    fn test_invalid_date() {
        assert!(NaiveDate::from_ymd_opt(2024, 13, 1).is_none());
        assert!(NaiveDate::from_ymd_opt(2024, 2, 30).is_none());
    }

    #[test]
    fn test_comparisons() {
        let a = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let b = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        assert!(a < b);
    }
}
