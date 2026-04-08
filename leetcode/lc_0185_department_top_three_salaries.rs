// LeetCode 185: Department Top Three Salaries [Hard]
// https://leetcode.com/problems/department-top-three-salaries/
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
//   WITH T AS (
//       SELECT *,
//              DENSE_RANK() OVER (
//                  PARTITION BY departmentId ORDER BY salary DESC
//              ) AS rk
//       FROM Employee
//   )
//   SELECT d.name AS Department, t.name AS Employee, salary
//   FROM T t
//   JOIN Department d ON t.departmentId = d.id
//   WHERE rk <= 3;
//
// Run:  cargo run --example lc_0185_department_top_three_salaries
// Test: cargo test --example lc_0185_department_top_three_salaries

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
struct TopSalaryRow {
    department: String,
    employee: String,
    salary: i32,
}

/// Find employees earning one of the top 3 unique salaries in their dept.
///
/// # Arguments
/// * `employees` - Slice of `Employee` rows
/// * `departments` - Slice of `Department` rows
///
/// # Returns
/// Rows of (Department, Employee, Salary) for employees within the top 3
/// distinct salary tiers per department
///
/// # Approach
/// 1. Group employees by department_id.
/// 2. For each department, collect distinct salaries, sort descending,
///    and take the top 3 -- these form the DENSE_RANK() <= 3 cutoff.
/// 3. Keep employees whose salary is in that top-3 set (the WHERE rk <= 3).
/// 4. Join with Department for the department name.
fn department_top_three_salaries(
    employees: &[Employee],
    departments: &[Department],
) -> Vec<TopSalaryRow> {
    // PARTITION BY departmentId: group employees by dept
    let mut by_dept: HashMap<i32, Vec<&Employee>> = HashMap::new();
    for e in employees {
        by_dept.entry(e.department_id).or_default().push(e);
    }

    // Department id -> name lookup (the JOIN)
    let dept_name: HashMap<i32, &str> = departments
        .iter()
        .map(|d| (d.id, d.name.as_str()))
        .collect();

    let mut result: Vec<TopSalaryRow> = Vec::new();

    for (&dept_id, emps) in &by_dept {
        // Skip if department doesn't exist (INNER JOIN)
        let dname = match dept_name.get(&dept_id) {
            Some(n) => *n,
            None => continue,
        };

        // Compute DENSE_RANK: collect distinct salaries, sort desc, take top 3
        let mut distinct_salaries: Vec<i32> = emps.iter().map(|e| e.salary).collect();
        distinct_salaries.sort_unstable_by(|a, b| b.cmp(a));
        distinct_salaries.dedup();
        // The cutoff is the 3rd-highest distinct salary (or the lowest if < 3)
        let cutoff = distinct_salaries
            .get(2)
            .copied()
            .unwrap_or(*distinct_salaries.last().expect("dept has employees"));

        // WHERE rk <= 3: keep employees at or above the cutoff salary
        for e in emps {
            if e.salary >= cutoff {
                result.push(TopSalaryRow {
                    department: dname.to_string(),
                    employee: e.name.clone(),
                    salary: e.salary,
                });
            }
        }
    }

    // Sort for deterministic output: dept asc, salary desc, name asc
    result.sort_by(|a, b| {
        a.department
            .cmp(&b.department)
            .then(b.salary.cmp(&a.salary))
            .then(a.employee.cmp(&b.employee))
    });
    result
}

fn main() {
    println!("LeetCode 185: Department Top Three Salaries");

    let employees = vec![
        Employee {
            id: 1,
            name: "Joe".into(),
            salary: 85_000,
            department_id: 1,
        },
        Employee {
            id: 2,
            name: "Henry".into(),
            salary: 80_000,
            department_id: 2,
        },
        Employee {
            id: 3,
            name: "Sam".into(),
            salary: 60_000,
            department_id: 2,
        },
        Employee {
            id: 4,
            name: "Max".into(),
            salary: 90_000,
            department_id: 1,
        },
        Employee {
            id: 5,
            name: "Janet".into(),
            salary: 69_000,
            department_id: 1,
        },
        Employee {
            id: 6,
            name: "Randy".into(),
            salary: 85_000,
            department_id: 1,
        },
        Employee {
            id: 7,
            name: "Will".into(),
            salary: 70_000,
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

    let result = department_top_three_salaries(&employees, &departments);
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

    fn row(department: &str, employee: &str, salary: i32) -> TopSalaryRow {
        TopSalaryRow {
            department: department.to_string(),
            employee: employee.to_string(),
            salary,
        }
    }

    #[test]
    fn example_main() {
        let employees = vec![
            emp(1, "Joe", 85_000, 1),
            emp(2, "Henry", 80_000, 2),
            emp(3, "Sam", 60_000, 2),
            emp(4, "Max", 90_000, 1),
            emp(5, "Janet", 69_000, 1),
            emp(6, "Randy", 85_000, 1),
            emp(7, "Will", 70_000, 1),
        ];
        let departments = vec![dept(1, "IT"), dept(2, "Sales")];
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            vec![
                // IT top 3 distinct: 90k, 85k, 70k
                row("IT", "Max", 90_000),
                row("IT", "Joe", 85_000),
                row("IT", "Randy", 85_000),
                row("IT", "Will", 70_000),
                // Sales top 3 distinct (only 2 exist): 80k, 60k
                row("Sales", "Henry", 80_000),
                row("Sales", "Sam", 60_000),
            ]
        );
    }

    #[test]
    fn fewer_than_three_distinct_salaries() {
        let employees = vec![emp(1, "A", 100_000, 1), emp(2, "B", 100_000, 1)];
        let departments = vec![dept(1, "Eng")];
        // Only 1 distinct salary tier -- both employees included
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            vec![row("Eng", "A", 100_000), row("Eng", "B", 100_000),]
        );
    }

    #[test]
    fn exactly_three_tiers() {
        let employees = vec![
            emp(1, "A", 300, 1),
            emp(2, "B", 200, 1),
            emp(3, "C", 100, 1),
        ];
        let departments = vec![dept(1, "D")];
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            vec![row("D", "A", 300), row("D", "B", 200), row("D", "C", 100),]
        );
    }

    #[test]
    fn four_tiers_excludes_lowest() {
        let employees = vec![
            emp(1, "A", 400, 1),
            emp(2, "B", 300, 1),
            emp(3, "C", 200, 1),
            emp(4, "D", 100, 1),
        ];
        let departments = vec![dept(1, "X")];
        // Top 3 distinct: 400, 300, 200 -- employee D (100) is excluded
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            vec![row("X", "A", 400), row("X", "B", 300), row("X", "C", 200),]
        );
    }

    #[test]
    fn dense_rank_ties() {
        // 5 employees, 3 distinct salaries -- all should be included
        let employees = vec![
            emp(1, "A", 100, 1),
            emp(2, "B", 200, 1),
            emp(3, "C", 200, 1),
            emp(4, "D", 300, 1),
            emp(5, "E", 300, 1),
        ];
        let departments = vec![dept(1, "Z")];
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            vec![
                row("Z", "D", 300),
                row("Z", "E", 300),
                row("Z", "B", 200),
                row("Z", "C", 200),
                row("Z", "A", 100),
            ]
        );
    }

    #[test]
    fn multiple_departments() {
        let employees = vec![
            emp(1, "A", 500, 1),
            emp(2, "B", 400, 1),
            emp(3, "C", 300, 1),
            emp(4, "D", 200, 1),
            emp(5, "X", 900, 2),
            emp(6, "Y", 800, 2),
        ];
        let departments = vec![dept(1, "Eng"), dept(2, "Sales")];
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            vec![
                row("Eng", "A", 500),
                row("Eng", "B", 400),
                row("Eng", "C", 300),
                row("Sales", "X", 900),
                row("Sales", "Y", 800),
            ]
        );
    }

    #[test]
    fn no_matching_department() {
        let employees = vec![emp(1, "Ghost", 999, 99)];
        let departments = vec![dept(1, "Eng")];
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            Vec::<TopSalaryRow>::new()
        );
    }

    #[test]
    fn empty_employees() {
        let employees: Vec<Employee> = vec![];
        let departments = vec![dept(1, "Eng")];
        assert_eq!(
            department_top_three_salaries(&employees, &departments),
            Vec::<TopSalaryRow>::new()
        );
    }
}
