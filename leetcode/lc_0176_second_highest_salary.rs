// LeetCode 176: Second Highest Salary [Medium]
// https://leetcode.com/problems/second-highest-salary/
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
//   SELECT (SELECT DISTINCT salary FROM Employee
//           ORDER BY salary DESC LIMIT 1 OFFSET 1) AS SecondHighestSalary;
//
// Report the second highest **distinct** salary from the Employee table.
// If there is no second highest salary (fewer than 2 distinct values),
// report None (null).
//
// Employee table: (id, salary)
//
// Example 1: [(1,100),(2,200),(3,300)] -> Some(200)
// Example 2: [(1,100)]                -> None
//
// Run:  cargo run --example lc_0176_second_highest_salary
// Test: cargo test --example lc_0176_second_highest_salary

use std::collections::BTreeSet;

/// A row in the Employee table.
#[derive(Debug, Clone)]
struct Employee {
    #[allow(dead_code)]
    id: i32,
    salary: i32,
}

/// Find the second highest distinct salary.
///
/// Uses a `BTreeSet` to deduplicate and sort salaries in one pass.
/// `BTreeSet` keeps elements in ascending order, so we iterate from
/// the back to get descending order.
///
/// # Arguments
/// * `employees` - Slice of Employee rows
///
/// # Returns
/// `Some(salary)` if a second distinct salary exists, `None` otherwise.
fn second_highest_salary(employees: &[Employee]) -> Option<i32> {
    // BTreeSet maintains sorted order and deduplicates automatically.
    // This is equivalent to SELECT DISTINCT salary ORDER BY salary.
    let distinct: BTreeSet<i32> = employees.iter().map(|e| e.salary).collect();

    // `.rev()` iterates from largest to smallest.
    // `.nth(1)` skips the first (highest) and returns the second.
    // `.copied()` converts &i32 to i32.
    distinct.iter().rev().nth(1).copied()
}

fn main() {
    println!("LeetCode 176: Second Highest Salary");

    let cases: Vec<(Vec<Employee>, Option<i32>)> = vec![
        (
            vec![
                Employee { id: 1, salary: 100 },
                Employee { id: 2, salary: 200 },
                Employee { id: 3, salary: 300 },
            ],
            Some(200),
        ),
        (vec![Employee { id: 1, salary: 100 }], None),
    ];

    for (employees, expected) in &cases {
        let result = second_highest_salary(employees);
        println!(
            "  salaries={:?} => {result:?} (expected {expected:?})",
            employees.iter().map(|e| e.salary).collect::<Vec<_>>(),
        );
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
    fn example1_three_distinct() {
        assert_eq!(
            second_highest_salary(&employees(&[100, 200, 300])),
            Some(200)
        );
    }

    #[test]
    fn example2_single_employee() {
        assert_eq!(second_highest_salary(&employees(&[100])), None);
    }

    #[test]
    fn all_same_salary() {
        // Only one distinct value exists -- no second highest
        assert_eq!(second_highest_salary(&employees(&[100, 100, 100])), None);
    }

    #[test]
    fn two_distinct() {
        assert_eq!(second_highest_salary(&employees(&[50, 100])), Some(50));
    }

    #[test]
    fn duplicates_among_top() {
        // 300 appears twice, but distinct set is {100, 200, 300}
        assert_eq!(
            second_highest_salary(&employees(&[300, 200, 100, 300])),
            Some(200),
        );
    }

    #[test]
    fn empty_table() {
        assert_eq!(second_highest_salary(&[]), None);
    }

    #[test]
    fn negative_salaries() {
        assert_eq!(
            second_highest_salary(&employees(&[-10, -20, -5])),
            Some(-10),
        );
    }
}
