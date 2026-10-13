#![allow(unused)]

use rf_procedural_macros::Describe;

trait Describe {
    fn describe(&self) -> String;
}

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Describe)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Describe)]
struct Numbers {
    integer: i32,
    unsigned: u64,
    float: f64,
}

#[derive(Describe)]
struct Mixed {
    flag: bool,
    optional: Option<i32>,
    items: Vec<String>,
}

#[test]
fn describes_point() {
    let point = Point { x: 1, y: 2 };
    assert_eq!(point.describe(), "Point { x: 1, y: 2 }");
}

#[test]
fn describes_person() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
}

#[test]
fn describes_different_numeric_types() {
    let numbers = Numbers {
        integer: -42,
        unsigned: 100,
        float: 3.14,
    };
    assert_eq!(
        numbers.describe(),
        "Numbers { integer: -42, unsigned: 100, float: 3.14 }"
    );
}

#[test]
fn describes_mixed_fields() {
    let mixed = Mixed {
        flag: true,
        optional: Some(123),
        items: vec!["hello".to_string(), "world".to_string()],
    };
    assert_eq!(
        mixed.describe(),
        "Mixed { flag: true, optional: Some(123), items: [\"hello\", \"world\"] }"
    );
}

#[test]
fn describes_none_and_empty_collections() {
    let mixed = Mixed {
        flag: false,
        optional: None,
        items: Vec::new(),
    };
    assert_eq!(
        mixed.describe(),
        "Mixed { flag: false, optional: None, items: [] }"
    );
}
