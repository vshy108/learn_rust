// LeetCode 184: Department Highest Salary [Medium]
// https://leetcode.com/problems/department-highest-salary/
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
//   SELECT d.name AS Department, e.name AS Employee, salary
//   FROM Employee e
//   JOIN Department d ON e.departmentId = d.id
//   WHERE (d.id, salary) IN (
//       SELECT departmentId, MAX(salary)
//       FROM Employee
//       GROUP BY departmentId
//   );
//
// Run:  cargo run --example lc_0184_department_highest_salary
// Test: cargo test --example lc_0184_department_highest_salary

use std::collections::HashMap;

/// A row in the Employee table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Employee {
    id: i32,
    name: String,
    salary: i32,
    department_id: i32,
}

/// A row in the Department table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Department {
    id: i32,
    name: String,
}

/// Result row: (department_name, employee_name, salary).
#[derive(Debug, Clone, PartialEq, Eq)]
struct HighestSalaryRow {
    department: String,
    employee: String,
    salary: i32,
}

/// Find employees with the highest salary in each department.
///
/// # Arguments
/// * `employees` - Slice of `Employee` rows
/// * `departments` - Slice of `Department` rows
///
/// # Returns
/// Rows of (Department, Employee, Salary) for top earners in each dept
///
/// # Approach
/// 1. Compute MAX(salary) per department (the subquery).
/// 2. Join Employee with Department and filter where salary == max for
///    that department.
fn department_highest_salary(
    employees: &[Employee],
    departments: &[Department],
) -> Vec<HighestSalaryRow> {
    // Subquery: SELECT departmentId, MAX(salary) FROM Employee GROUP BY 1
    let mut max_salary_by_dept: HashMap<i32, i32> = HashMap::new();
    for e in employees {
        max_salary_by_dept
            .entry(e.department_id)
            .and_modify(|cur| *cur = (*cur).max(e.salary))
            .or_insert(e.salary);
    }

    // Build department id -> name lookup for the JOIN
    let dept_name: HashMap<i32, &str> = departments
        .iter()
        .map(|d| (d.id, d.name.as_str()))
        .collect();

    // Main query: JOIN + WHERE (dept_id, salary) IN (subquery)
    let mut result: Vec<HighestSalaryRow> = employees
        .iter()
        .filter(|e| {
            max_salary_by_dept
                .get(&e.department_id)
                .is_some_and(|&max_sal| e.salary == max_sal)
        })
        .filter_map(|e| {
            dept_name
                .get(&e.department_id)
                .map(|&dname| HighestSalaryRow {
                    department: dname.to_string(),
                    employee: e.name.clone(),
                    salary: e.salary,
                })
        })
        .collect();

    // Sort by department then employee for deterministic output
    result.sort_by(|a, b| {
        a.department
            .cmp(&b.department)
            .then(a.employee.cmp(&b.employee))
    });
    result
}

fn main() {
    println!("LeetCode 184: Department Highest Salary");

    let employees = vec![
        Employee {
            id: 1,
            name: "Joe".into(),
            salary: 70_000,
            department_id: 1,
        },
        Employee {
            id: 2,
            name: "Jim".into(),
            salary: 90_000,
            department_id: 1,
        },
        Employee {
            id: 3,
            name: "Henry".into(),
            salary: 80_000,
            department_id: 2,
        },
        Employee {
            id: 4,
            name: "Sam".into(),
            salary: 60_000,
            department_id: 2,
        },
        Employee {
            id: 5,
            name: "Max".into(),
            salary: 90_000,
            department_id: 1,
        },
    ];
    let departments = vec![
        Department {
            id: 1,
            name: "IT".into(),
        },
        Department {
            id: 2,
            name: "Sales".into(),
        },
    ];

    let result = department_highest_salary(&employees, &departments);
    for row in &result {
        println!("  {} - {} : {}", row.department, row.employee, row.salary);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn emp(id: i32, name: &str, salary: i32, dept: i32) -> Employee {
        Employee {
            id,
            name: name.to_string(),
            salary,
            department_id: dept,
        }
    }

    fn dept(id: i32, name: &str) -> Department {
        Department {
            id,
            name: name.to_string(),
        }
    }

    fn row(department: &str, employee: &str, salary: i32) -> HighestSalaryRow {
        HighestSalaryRow {
            department: department.to_string(),
            employee: employee.to_string(),
            salary,
        }
    }

    #[test]
    fn example_main() {
        let employees = vec![
            emp(1, "Joe", 70_000, 1),
            emp(2, "Jim", 90_000, 1),
            emp(3, "Henry", 80_000, 2),
            emp(4, "Sam", 60_000, 2),
            emp(5, "Max", 90_000, 1),
        ];
        let departments = vec![dept(1, "IT"), dept(2, "Sales")];
        assert_eq!(
            department_highest_salary(&employees, &departments),
            vec![
                row("IT", "Jim", 90_000),
                row("IT", "Max", 90_000),
                row("Sales", "Henry", 80_000),
            ]
        );
    }

    #[test]
    fn single_employee_per_dept() {
        let employees = vec![emp(1, "Alice", 50_000, 1), emp(2, "Bob", 60_000, 2)];
        let departments = vec![dept(1, "Eng"), dept(2, "HR")];
        assert_eq!(
            department_highest_salary(&employees, &departments),
            vec![row("Eng", "Alice", 50_000), row("HR", "Bob", 60_000),]
        );
    }

    #[test]
    fn all_same_salary_in_dept() {
        let employees = vec![
            emp(1, "A", 100_000, 1),
            emp(2, "B", 100_000, 1),
            emp(3, "C", 100_000, 1),
        ];
        let departments = vec![dept(1, "Eng")];
        assert_eq!(
            department_highest_salary(&employees, &departments),
            vec![
                row("Eng", "A", 100_000),
                row("Eng", "B", 100_000),
                row("Eng", "C", 100_000),
            ]
        );
    }

    #[test]
    fn employee_with_no_matching_department() {
        // Employee references dept 99 which doesn't exist
        let employees = vec![emp(1, "Ghost", 200_000, 99)];
        let departments = vec![dept(1, "Eng")];
        assert_eq!(
            department_highest_salary(&employees, &departments),
            Vec::<HighestSalaryRow>::new()
        );
    }

    #[test]
    fn empty_employees() {
        let employees: Vec<Employee> = vec![];
        let departments = vec![dept(1, "Eng")];
        assert_eq!(
            department_highest_salary(&employees, &departments),
            Vec::<HighestSalaryRow>::new()
        );
    }
}
