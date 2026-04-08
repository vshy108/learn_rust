// LeetCode 182: Duplicate Emails [Easy]
// https://leetcode.com/problems/duplicate-emails/
//
// This is a SQL problem. Below is the Rust equivalent using standard library
// collections to demonstrate the relational operations.
//
// In production Rust, you would use crates like:
//   - `polars` for DataFrame-style data manipulation
//   - `rusqlite` or `sqlx` for running actual SQL queries
//   - `sea-orm` or `diesel` for ORM-based database access
//
// SQL Solution:
//   SELECT email
//   FROM Person
//   GROUP BY email
//   HAVING COUNT(*) > 1;
//
// Run:  cargo run --example lc_0182_duplicate_emails
// Test: cargo test --example lc_0182_duplicate_emails

use std::collections::HashMap;

/// A row in the Person table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Person {
    id: i32,
    email: String,
}

/// Report all emails that appear more than once.
///
/// # Arguments
/// * `persons` - Slice of `Person` rows
///
/// # Returns
/// A sorted `Vec<String>` of duplicate email addresses
///
/// # Approach
/// Count occurrences of each email (GROUP BY + COUNT), then keep only
/// those with count > 1 (HAVING COUNT(*) > 1).
fn duplicate_emails(persons: &[Person]) -> Vec<String> {
    // GROUP BY email, counting occurrences
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for p in persons {
        *counts.entry(p.email.as_str()).or_insert(0) += 1;
    }

    // HAVING COUNT(*) > 1
    let mut result: Vec<String> = counts
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(email, _)| email.to_string())
        .collect();

    result.sort();
    result
}

fn main() {
    println!("LeetCode 182: Duplicate Emails");

    let persons = vec![
        Person {
            id: 1,
            email: "a@b.com".into(),
        },
        Person {
            id: 2,
            email: "c@d.com".into(),
        },
        Person {
            id: 3,
            email: "a@b.com".into(),
        },
    ];

    let result = duplicate_emails(&persons);
    println!("  Duplicate emails: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a Person row.
    fn person(id: i32, email: &str) -> Person {
        Person {
            id,
            email: email.to_string(),
        }
    }

    #[test]
    fn example_main() {
        let persons = vec![
            person(1, "a@b.com"),
            person(2, "c@d.com"),
            person(3, "a@b.com"),
        ];
        assert_eq!(duplicate_emails(&persons), vec!["a@b.com"]);
    }

    #[test]
    fn no_duplicates() {
        let persons = vec![
            person(1, "a@b.com"),
            person(2, "c@d.com"),
            person(3, "e@f.com"),
        ];
        assert_eq!(duplicate_emails(&persons), Vec::<String>::new());
    }

    #[test]
    fn all_duplicates() {
        let persons = vec![
            person(1, "x@y.com"),
            person(2, "x@y.com"),
            person(3, "x@y.com"),
        ];
        assert_eq!(duplicate_emails(&persons), vec!["x@y.com"]);
    }

    #[test]
    fn multiple_duplicated_emails() {
        let persons = vec![
            person(1, "a@b.com"),
            person(2, "c@d.com"),
            person(3, "a@b.com"),
            person(4, "c@d.com"),
            person(5, "e@f.com"),
        ];
        assert_eq!(duplicate_emails(&persons), vec!["a@b.com", "c@d.com"]);
    }

    #[test]
    fn single_person() {
        let persons = vec![person(1, "only@one.com")];
        assert_eq!(duplicate_emails(&persons), Vec::<String>::new());
    }

    #[test]
    fn empty_table() {
        let persons: Vec<Person> = vec![];
        assert_eq!(duplicate_emails(&persons), Vec::<String>::new());
    }

    #[test]
    fn triple_occurrence_still_one_entry() {
        let persons = vec![
            person(1, "dup@dup.com"),
            person(2, "dup@dup.com"),
            person(3, "dup@dup.com"),
            person(4, "unique@one.com"),
        ];
        // Even 3 copies yields just one entry in the result
        assert_eq!(duplicate_emails(&persons), vec!["dup@dup.com"]);
    }
}
