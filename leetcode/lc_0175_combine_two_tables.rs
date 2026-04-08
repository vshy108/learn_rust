// LeetCode 175: Combine Two Tables [Easy]
// https://leetcode.com/problems/combine-two-tables/
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
//   SELECT firstName, lastName, city, state
//   FROM Person LEFT JOIN Address USING (personId);
//
// Report the first name, last name, city, and state of each person in the
// Person table. If the address of a personId is not present in the Address
// table, report null (None) instead.
//
// Person table:   (personId, lastName, firstName)
// Address table:  (addressId, personId, city, state)
//
// Example:
//   Person:  [(1,"Wang","Allen"), (2,"Alice","Bob")]
//   Address: [(1,2,"New York City","New York"), (2,3,"Leetcode","California")]
//   Output:  [("Allen","Wang",None,None), ("Bob","Alice","New York City","New York")]
//
// Run:  cargo run --example lc_0175_combine_two_tables
// Test: cargo test --example lc_0175_combine_two_tables

use std::collections::HashMap;

/// A row in the Person table.
#[derive(Debug, Clone)]
struct Person {
    person_id: i32,
    last_name: String,
    first_name: String,
}

/// A row in the Address table.
#[derive(Debug, Clone)]
struct Address {
    #[allow(dead_code)]
    address_id: i32,
    person_id: i32,
    city: String,
    state: String,
}

/// A single result row after the LEFT JOIN.
#[derive(Debug, Clone, PartialEq)]
struct ResultRow {
    first_name: String,
    last_name: String,
    city: Option<String>,
    state: Option<String>,
}

/// Perform a LEFT JOIN of Person with Address on `person_id`.
///
/// Every person appears exactly once in the output. If no matching
/// address exists, `city` and `state` are `None`.
///
/// # Arguments
/// * `persons`   - Slice of Person rows
/// * `addresses` - Slice of Address rows
///
/// # Returns
/// A `Vec<ResultRow>` preserving the order of `persons`.
fn combine_two_tables(persons: &[Person], addresses: &[Address]) -> Vec<ResultRow> {
    // Build a lookup: person_id -> (city, state).
    // HashMap gives O(1) lookup per person, making the join O(n + m).
    let addr_map: HashMap<i32, (&str, &str)> = addresses
        .iter()
        .map(|a| (a.person_id, (a.city.as_str(), a.state.as_str())))
        .collect();

    persons
        .iter()
        .map(|p| {
            // `.get()` returns Option<&V> -- None when no address exists
            let (city, state) = match addr_map.get(&p.person_id) {
                Some(&(c, s)) => (Some(c.to_owned()), Some(s.to_owned())),
                None => (None, None),
            };
            ResultRow {
                first_name: p.first_name.clone(),
                last_name: p.last_name.clone(),
                city,
                state,
            }
        })
        .collect()
}

fn main() {
    println!("LeetCode 175: Combine Two Tables");

    let persons = vec![
        Person {
            person_id: 1,
            last_name: "Wang".into(),
            first_name: "Allen".into(),
        },
        Person {
            person_id: 2,
            last_name: "Alice".into(),
            first_name: "Bob".into(),
        },
    ];
    let addresses = vec![
        Address {
            address_id: 1,
            person_id: 2,
            city: "New York City".into(),
            state: "New York".into(),
        },
        Address {
            address_id: 2,
            person_id: 3,
            city: "Leetcode".into(),
            state: "California".into(),
        },
    ];

    let results = combine_two_tables(&persons, &addresses);
    for row in &results {
        println!(
            "  {} {} | city: {:?} | state: {:?}",
            row.first_name, row.last_name, row.city, row.state,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a Person quickly.
    fn person(id: i32, last: &str, first: &str) -> Person {
        Person {
            person_id: id,
            last_name: last.into(),
            first_name: first.into(),
        }
    }

    /// Helper to build an Address quickly.
    fn address(aid: i32, pid: i32, city: &str, state: &str) -> Address {
        Address {
            address_id: aid,
            person_id: pid,
            city: city.into(),
            state: state.into(),
        }
    }

    #[test]
    fn leetcode_example() {
        let persons = vec![person(1, "Wang", "Allen"), person(2, "Alice", "Bob")];
        let addresses = vec![
            address(1, 2, "New York City", "New York"),
            address(2, 3, "Leetcode", "California"),
        ];
        let result = combine_two_tables(&persons, &addresses);
        assert_eq!(
            result,
            vec![
                ResultRow {
                    first_name: "Allen".into(),
                    last_name: "Wang".into(),
                    city: None,
                    state: None,
                },
                ResultRow {
                    first_name: "Bob".into(),
                    last_name: "Alice".into(),
                    city: Some("New York City".into()),
                    state: Some("New York".into()),
                },
            ]
        );
    }

    #[test]
    fn all_have_addresses() {
        let persons = vec![person(1, "Doe", "Jane")];
        let addresses = vec![address(1, 1, "Boston", "MA")];
        let result = combine_two_tables(&persons, &addresses);
        assert_eq!(result[0].city, Some("Boston".into()));
        assert_eq!(result[0].state, Some("MA".into()));
    }

    #[test]
    fn no_addresses_at_all() {
        let persons = vec![person(1, "Doe", "John"), person(2, "Doe", "Jane")];
        let addresses: Vec<Address> = vec![];
        let result = combine_two_tables(&persons, &addresses);
        assert!(result.iter().all(|r| r.city.is_none() && r.state.is_none()));
    }

    #[test]
    fn empty_person_table() {
        let persons: Vec<Person> = vec![];
        let addresses = vec![address(1, 1, "LA", "CA")];
        let result = combine_two_tables(&persons, &addresses);
        assert!(result.is_empty());
    }

    #[test]
    fn duplicate_person_id_in_address_last_wins() {
        // HashMap overwrites earlier entries -- mirrors SQL behavior when
        // address is unique per person_id. If multiple rows per person
        // existed, a real LEFT JOIN would produce multiple output rows.
        let persons = vec![person(1, "X", "Y")];
        let addresses = vec![address(1, 1, "First", "AA"), address(2, 1, "Second", "BB")];
        let result = combine_two_tables(&persons, &addresses);
        // The last inserted address for person_id=1 wins in HashMap
        assert_eq!(result[0].city, Some("Second".into()));
    }
}
