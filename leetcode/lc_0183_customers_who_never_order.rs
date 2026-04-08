// LeetCode 183: Customers Who Never Order [Easy]
// https://leetcode.com/problems/customers-who-never-order/
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
//   SELECT name AS Customers
//   FROM Customers
//   WHERE id NOT IN (SELECT customerId FROM Orders);
//
// Run:  cargo run --example lc_0183_customers_who_never_order
// Test: cargo test --example lc_0183_customers_who_never_order

use std::collections::HashSet;

/// A row in the Customers table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Customer {
    id: i32,
    name: String,
}

/// A row in the Orders table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Order {
    id: i32,
    customer_id: i32,
}

/// Find customers who have never placed an order.
///
/// # Arguments
/// * `customers` - Slice of `Customer` rows
/// * `orders` - Slice of `Order` rows
///
/// # Returns
/// Sorted names of customers with no matching order
///
/// # Approach
/// Collect all customer_ids from Orders into a HashSet (the subquery),
/// then filter Customers whose id is NOT IN that set.
fn customers_who_never_order(customers: &[Customer], orders: &[Order]) -> Vec<String> {
    // Build the subquery result: SELECT customerId FROM Orders
    let ordered: HashSet<i32> = orders.iter().map(|o| o.customer_id).collect();

    // WHERE id NOT IN (subquery)
    let mut result: Vec<String> = customers
        .iter()
        .filter(|c| !ordered.contains(&c.id))
        .map(|c| c.name.clone())
        .collect();

    result.sort();
    result
}

fn main() {
    println!("LeetCode 183: Customers Who Never Order");

    let customers = vec![
        Customer {
            id: 1,
            name: "Joe".into(),
        },
        Customer {
            id: 2,
            name: "Henry".into(),
        },
        Customer {
            id: 3,
            name: "Sam".into(),
        },
        Customer {
            id: 4,
            name: "Max".into(),
        },
    ];
    let orders = vec![
        Order {
            id: 1,
            customer_id: 3,
        },
        Order {
            id: 2,
            customer_id: 1,
        },
    ];

    let result = customers_who_never_order(&customers, &orders);
    println!("  Customers who never order: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cust(id: i32, name: &str) -> Customer {
        Customer {
            id,
            name: name.to_string(),
        }
    }

    fn ord(id: i32, customer_id: i32) -> Order {
        Order { id, customer_id }
    }

    #[test]
    fn example_main() {
        let customers = vec![
            cust(1, "Joe"),
            cust(2, "Henry"),
            cust(3, "Sam"),
            cust(4, "Max"),
        ];
        let orders = vec![ord(1, 3), ord(2, 1)];
        assert_eq!(
            customers_who_never_order(&customers, &orders),
            vec!["Henry", "Max"]
        );
    }

    #[test]
    fn all_customers_ordered() {
        let customers = vec![cust(1, "Alice"), cust(2, "Bob")];
        let orders = vec![ord(1, 1), ord(2, 2)];
        assert_eq!(
            customers_who_never_order(&customers, &orders),
            Vec::<String>::new()
        );
    }

    #[test]
    fn no_orders_at_all() {
        let customers = vec![cust(1, "Alice"), cust(2, "Bob")];
        let orders: Vec<Order> = vec![];
        assert_eq!(
            customers_who_never_order(&customers, &orders),
            vec!["Alice", "Bob"]
        );
    }

    #[test]
    fn single_customer_no_order() {
        let customers = vec![cust(1, "Solo")];
        let orders: Vec<Order> = vec![];
        assert_eq!(customers_who_never_order(&customers, &orders), vec!["Solo"]);
    }

    #[test]
    fn customer_with_multiple_orders_still_excluded() {
        let customers = vec![cust(1, "Alice"), cust(2, "Bob")];
        // Alice ordered 3 times, Bob never ordered
        let orders = vec![ord(1, 1), ord(2, 1), ord(3, 1)];
        assert_eq!(customers_who_never_order(&customers, &orders), vec!["Bob"]);
    }

    #[test]
    fn empty_customers() {
        let customers: Vec<Customer> = vec![];
        let orders = vec![ord(1, 1)];
        assert_eq!(
            customers_who_never_order(&customers, &orders),
            Vec::<String>::new()
        );
    }
}
