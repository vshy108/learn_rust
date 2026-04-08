// LeetCode 177: Nth Highest Salary [Medium]
// https://leetcode.com/problems/nth-highest-salary/
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
//   SELECT DISTINCT salary FROM Employee
//   ORDER BY salary DESC LIMIT 1 OFFSET N-1;
//
// Write a function to find the Nth highest **distinct** salary from the
// Employee table. If there are fewer than N distinct salaries, return None.
//
// Employee table: (id, salary)
//
// Example 1: [(1,100),(2,200),(3,300)], N=2 -> Some(200)
// Example 2: [(1,100)],                 N=2 -> None
//
// Run:  cargo run --example lc_0177_nth_highest_salary
// Test: cargo test --example lc_0177_nth_highest_salary

use std::collections::BTreeSet;

/// A row in the Employee table.
#[derive(Debug, Clone)]
struct Employee {
    #[allow(dead_code)]
    id: i32,
    salary: i32,
}

/// Find the Nth highest distinct salary.
///
/// Collects distinct salaries into a sorted set, then picks the Nth
/// element from the end (descending order). `nth(n - 1)` because
/// iterators are 0-indexed while N is 1-indexed.
///
/// # Arguments
/// * `employees` - Slice of Employee rows
/// * `n`         - 1-based rank (1 = highest, 2 = second highest, etc.)
///
/// # Returns
/// `Some(salary)` if the Nth distinct salary exists, `None` otherwise.
fn nth_highest_salary(employees: &[Employee], n: usize) -> Option<i32> {
    if n == 0 {
        // N is 1-based; 0 is invalid
        return None;
    }

    let distinct: BTreeSet<i32> = employees.iter().map(|e| e.salary).collect();

    // `.rev()` gives descending order; `.nth(n - 1)` is 0-indexed
    distinct.iter().rev().nth(n - 1).copied()
}

fn main() {
    println!("LeetCode 177: Nth Highest Salary");

    let employees = vec![
        Employee { id: 1, salary: 100 },
        Employee { id: 2, salary: 200 },
        Employee { id: 3, salary: 300 },
    ];

    for n in 1..=4 {
        let result = nth_highest_salary(&employees, n);
        println!("  N={n} => {result:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build Employee rows from a salary slice.
    fn employees(salaries: &[i32]) -> Vec<Employee> {
        salaries
            .iter()
            .enumerate()
            .map(|(i, &s)| Employee {
                id: i as i32 + 1,
                salary: s,
            })
            .collect()
    }

    #[test]
    fn example1_second_highest() {
        assert_eq!(
            nth_highest_salary(&employees(&[100, 200, 300]), 2),
            Some(200)
        );
    }

    #[test]
    fn example2_not_enough_distinct() {
        assert_eq!(nth_highest_salary(&employees(&[100]), 2), None);
    }

    #[test]
    fn first_highest() {
        assert_eq!(
            nth_highest_salary(&employees(&[100, 200, 300]), 1),
            Some(300)
        );
    }

    #[test]
    fn third_highest() {
        assert_eq!(
            nth_highest_salary(&employees(&[100, 200, 300]), 3),
            Some(100)
        );
    }

    #[test]
    fn n_exceeds_distinct_count() {
        assert_eq!(nth_highest_salary(&employees(&[100, 200]), 5), None);
    }

    #[test]
    fn n_is_zero() {
        assert_eq!(nth_highest_salary(&employees(&[100]), 0), None);
    }

    #[test]
    fn duplicates_ignored() {
        // Distinct set: {100, 200, 300} -- duplicates of 200 don't matter
        assert_eq!(
            nth_highest_salary(&employees(&[200, 300, 200, 100, 300]), 2),
            Some(200),
        );
    }

    #[test]
    fn empty_table() {
        assert_eq!(nth_highest_salary(&[], 1), None);
    }

    #[test]
    fn all_same_salary() {
        // Only 1 distinct value; N=1 succeeds, N=2 fails
        assert_eq!(nth_highest_salary(&employees(&[50, 50, 50]), 1), Some(50));
        assert_eq!(nth_highest_salary(&employees(&[50, 50, 50]), 2), None);
    }

    #[test]
    fn negative_salaries() {
        assert_eq!(
            nth_highest_salary(&employees(&[-5, -10, -1, -20]), 2),
            Some(-5),
        );
    }
}
