// LeetCode 196: Delete Duplicate Emails [Easy]
// https://leetcode.com/problems/delete-duplicate-emails/
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
//   DELETE FROM Person
//   WHERE id NOT IN (
//       SELECT MIN(id) FROM Person GROUP BY email
//   );
//
// Alternative (self-join):
//   DELETE p2
//   FROM Person p1
//   JOIN Person p2 ON p1.email = p2.email
//   WHERE p1.id < p2.id;
//
// Run:  cargo run --example lc_0196_delete_duplicate_emails
// Test: cargo test --example lc_0196_delete_duplicate_emails

use std::collections::HashMap;

/// A row in the Person table.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
struct Person {
    id: i32,
    email: String,
}

/// Delete duplicate emails in place, keeping only the row with the
/// smallest `id` per email.
///
/// # Arguments
/// * `persons` - Mutable reference to the `Person` table (Vec)
///
/// # Approach
/// This mirrors the SQL subquery approach:
///   1. GROUP BY email and find MIN(id) per group.
///   2. DELETE rows whose id is NOT IN that set.
///
/// In Rust we use `retain` on Vec to mutate in place, which is
/// analogous to SQL's DELETE ... WHERE id NOT IN (...).
fn delete_duplicate_emails(persons: &mut Vec<Person>) {
    // Subquery: SELECT MIN(id) FROM Person GROUP BY email
    // We use owned String keys so the HashMap doesn't borrow from
    // `persons`, which would conflict with the mutable `retain` below.
    let mut min_id_by_email: HashMap<String, i32> = HashMap::new();
    for p in persons.iter() {
        min_id_by_email
            .entry(p.email.clone())
            .and_modify(|cur| *cur = (*cur).min(p.id))
            .or_insert(p.id);
    }

    // DELETE WHERE id NOT IN (subquery result)
    // `retain` keeps elements where the closure returns true,
    // removing the rest — equivalent to deleting non-matching rows.
    persons.retain(|p| {
        min_id_by_email
            .get(&p.email)
            .is_some_and(|&min_id| p.id == min_id)
    });
}

fn main() {
    println!("LeetCode 196: Delete Duplicate Emails");

    let mut persons = vec![
        Person {
            id: 1,
            email: "john@example.com".into(),
        },
        Person {
            id: 2,
            email: "bob@example.com".into(),
        },
        Person {
            id: 3,
            email: "john@example.com".into(),
        },
    ];

    println!("  Before: {persons:?}");
    delete_duplicate_emails(&mut persons);
    println!("  After:  {persons:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn person(id: i32, email: &str) -> Person {
        Person {
            id,
            email: email.to_string(),
        }
    }

    #[test]
    fn example_main() {
        let mut persons = vec![
            person(1, "john@example.com"),
            person(2, "bob@example.com"),
            person(3, "john@example.com"),
        ];
        delete_duplicate_emails(&mut persons);
        assert_eq!(
            persons,
            vec![person(1, "john@example.com"), person(2, "bob@example.com"),]
        );
    }

    #[test]
    fn no_duplicates() {
        let mut persons = vec![
            person(1, "a@test.com"),
            person(2, "b@test.com"),
            person(3, "c@test.com"),
        ];
        delete_duplicate_emails(&mut persons);
        assert_eq!(
            persons,
            vec![
                person(1, "a@test.com"),
                person(2, "b@test.com"),
                person(3, "c@test.com"),
            ]
        );
    }

    #[test]
    fn all_same_email() {
        let mut persons = vec![
            person(3, "dup@test.com"),
            person(1, "dup@test.com"),
            person(5, "dup@test.com"),
        ];
        delete_duplicate_emails(&mut persons);
        // Only the row with the smallest id (1) survives
        assert_eq!(persons, vec![person(1, "dup@test.com")]);
    }

    #[test]
    fn single_row() {
        let mut persons = vec![person(1, "solo@test.com")];
        delete_duplicate_emails(&mut persons);
        assert_eq!(persons, vec![person(1, "solo@test.com")]);
    }

    #[test]
    fn empty_table() {
        let mut persons: Vec<Person> = vec![];
        delete_duplicate_emails(&mut persons);
        assert!(persons.is_empty());
    }

    #[test]
    fn multiple_duplicate_groups() {
        let mut persons = vec![
            person(1, "a@test.com"),
            person(2, "b@test.com"),
            person(3, "a@test.com"),
            person(4, "b@test.com"),
            person(5, "c@test.com"),
        ];
        delete_duplicate_emails(&mut persons);
        assert_eq!(
            persons,
            vec![
                person(1, "a@test.com"),
                person(2, "b@test.com"),
                person(5, "c@test.com"),
            ]
        );
    }

    #[test]
    fn keeps_smallest_id_not_first_occurrence() {
        // Rows are not sorted by id — MIN(id) should still win
        let mut persons = vec![
            person(5, "x@test.com"),
            person(2, "x@test.com"),
            person(9, "x@test.com"),
        ];
        delete_duplicate_emails(&mut persons);
        assert_eq!(persons, vec![person(2, "x@test.com")]);
    }
}
