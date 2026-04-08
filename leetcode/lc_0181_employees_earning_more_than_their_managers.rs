// LeetCode 181: Employees Earning More Than Their Managers [Easy]
// https://leetcode.com/problems/employees-earning-more-than-their-managers/
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
//   SELECT e1.name AS Employee
//   FROM Employee e1
//   JOIN Employee e2 ON e1.managerId = e2.id
//   WHERE e1.salary > e2.salary;
//
// Run:  cargo run --example lc_0181_employees_earning_more_than_their_managers
// Test: cargo test --example lc_0181_employees_earning_more_than_their_managers

use std::collections::HashMap;

/// A row in the Employee table.
/// `manager_id` is `None` when the employee has no manager.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Employee {
    id: i32,
    name: String,
    salary: i32,
    manager_id: Option<i32>,
}

/// Find employees who earn more than their manager.
///
/// # Arguments
/// * `employees` - Slice of `Employee` rows
///
/// # Returns
/// Names of employees whose salary exceeds their manager's salary
///
/// # Approach
/// Build a HashMap of id -> salary (mirrors the self-join lookup).
/// For each employee with a manager, compare salaries.
fn employees_earning_more(employees: &[Employee]) -> Vec<String> {
    // Index salaries by id so we can look up any manager's salary in O(1).
    // This replaces the SQL self-join `JOIN Employee e2 ON e1.managerId = e2.id`.
    let salary_by_id: HashMap<i32, i32> = employees.iter().map(|e| (e.id, e.salary)).collect();

    let mut result: Vec<String> = employees
        .iter()
        .filter(|e| {
            // Only consider employees who have a manager (INNER JOIN)
            if let Some(mgr_id) = e.manager_id {
                if let Some(&mgr_salary) = salary_by_id.get(&mgr_id) {
                    return e.salary > mgr_salary;
                }
            }
            false
        })
        .map(|e| e.name.clone())
        .collect();

    result.sort();
    result
}

fn main() {
    println!("LeetCode 181: Employees Earning More Than Their Managers");

    let employees = vec![
        Employee {
            id: 1,
            name: "Joe".into(),
            salary: 70_000,
            manager_id: Some(3),
        },
        Employee {
            id: 2,
            name: "Henry".into(),
            salary: 80_000,
            manager_id: Some(4),
        },
        Employee {
            id: 3,
            name: "Sam".into(),
            salary: 60_000,
            manager_id: None,
        },
        Employee {
            id: 4,
            name: "Max".into(),
            salary: 90_000,
            manager_id: None,
        },
    ];

    let result = employees_earning_more(&employees);
    println!("  Employees earning more than manager: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build an Employee row.
    fn emp(id: i32, name: &str, salary: i32, mgr: Option<i32>) -> Employee {
        Employee {
            id,
            name: name.to_string(),
            salary,
            manager_id: mgr,
        }
    }

    #[test]
    fn example_main() {
        let employees = vec![
            emp(1, "Joe", 70_000, Some(3)),
            emp(2, "Henry", 80_000, Some(4)),
            emp(3, "Sam", 60_000, None),
            emp(4, "Max", 90_000, None),
        ];
        assert_eq!(employees_earning_more(&employees), vec!["Joe"]);
    }

    #[test]
    fn no_managers() {
        let employees = vec![emp(1, "Alice", 100_000, None), emp(2, "Bob", 90_000, None)];
        assert_eq!(employees_earning_more(&employees), Vec::<String>::new());
    }

    #[test]
    fn all_earn_less_than_manager() {
        let employees = vec![
            emp(1, "Alice", 50_000, Some(2)),
            emp(2, "Bob", 100_000, None),
        ];
        assert_eq!(employees_earning_more(&employees), Vec::<String>::new());
    }

    #[test]
    fn equal_salary_not_more() {
        let employees = vec![
            emp(1, "Alice", 100_000, Some(2)),
            emp(2, "Bob", 100_000, None),
        ];
        // "more than" is strictly greater, so equal doesn't count
        assert_eq!(employees_earning_more(&employees), Vec::<String>::new());
    }

    #[test]
    fn multiple_employees_earning_more() {
        let employees = vec![
            emp(1, "Alice", 90_000, Some(3)),
            emp(2, "Bob", 80_000, Some(3)),
            emp(3, "Charlie", 50_000, None),
        ];
        assert_eq!(employees_earning_more(&employees), vec!["Alice", "Bob"]);
    }

    #[test]
    fn chain_of_managers() {
        // Alice -> Bob -> Charlie. Each earns more than their direct manager.
        let employees = vec![
            emp(1, "Alice", 120_000, Some(2)),
            emp(2, "Bob", 100_000, Some(3)),
            emp(3, "Charlie", 80_000, None),
        ];
        assert_eq!(employees_earning_more(&employees), vec!["Alice", "Bob"]);
    }
}
