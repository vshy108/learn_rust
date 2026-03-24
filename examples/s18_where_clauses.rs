// Demonstrates where clauses for cleaner trait bounds.
//
// When trait bounds get complex, `where` clauses keep signatures readable.
//
//   fn foo<T>(x: T) where T: Display + Clone { ... }
//
// Run: cargo run --example s18_where_clauses

use std::fmt::{Debug, Display};

// WITHOUT where — cluttered signature:
fn _messy<T: Display + Clone + Debug, U: Display + Debug>(a: &T, b: &U) {
    println!("{a}, {b}");
}

// WITH where — much cleaner:
fn clean<T, U>(a: &T, b: &U)
where
    T: Display + Clone + Debug,
    U: Display + Debug,
{
    println!("a = {a} (debug: {a:?})");
    println!("b = {b} (debug: {b:?})");
}

/// Practical example: generic function with complex bounds.
fn largest_display<T>(list: &[T]) -> Option<&T>
where
    T: PartialOrd + Display,
{
    let mut max = list.first()?;
    for item in &list[1..] {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

/// Where clauses on impl blocks too:
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T>
where
    T: Display + Debug,
{
    fn show(&self) {
        println!("Display: {}, Debug: {:?}", self.value, self.value);
    }
}

fn main() {
    clean(&42, &"hello");
    println!();

    let nums = vec![3, 1, 4, 1, 5, 9];
    if let Some(max) = largest_display(&nums) {
        println!("largest: {max}");
    }

    let w = Wrapper { value: "wrapped" };
    w.show();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_display() {
        assert_eq!(largest_display(&[1, 5, 3]), Some(&5));
        assert_eq!(largest_display::<i32>(&[]), None);
    }
}
