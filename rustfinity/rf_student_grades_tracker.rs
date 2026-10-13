//! # Rustfinity: Student Grades Tracker
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/student-grades-tracker
//!
//! Now that we know about vectors and hashmaps, let’s tackle something more challenging. In this exercise, we’ll build a system to manage and analyze student grades. This challenge will help you practice working with structs, hashmaps, vectors, and methods while managing a collection of data.
//!
//! The focus here is on functionality. **Ignore error handling for this challenge**; assume the input is always valid.
//!
//! ## Your Task
//!
//! You need to define the `StudentGrades` struct, which contains a `HashMap` of student names (`String`) as keys and `Student` structs as values. Each `Student` struct should have the following fields:
//!
//! - `name`: The name of the student (`String`).
//! - `grades`: A `Vec<u8>` to store the student's grades.
//!
//! Implement the following methods for the `StudentGrades` struct:
//!
//! 1. **`add_student(name: &str)`:** Add a new student to the `HashMap`. If the student already exists, do nothing.
//! 2. **`add_grade(name: &str, grade: u8)`:** Add a grade to an existing student.
//! 3. **`get_grades(name: &str) -> &[u8]`:** Retrieve the grades of a student as an immutable reference.
//! 4. Since there’s no error handling, assume all inputs are valid, and keys will exist when accessed.
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - Use the `HashMap` methods like `entry`, `insert`, and `get` to manage student data.
//!
//! </details>
//!
//! Run: `cargo run --example rf_student_grades_tracker`
//! Test: `cargo test --example rf_student_grades_tracker`
//! Filter: `cargo test --example rf_student_grades_tracker <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        todo!("add or preserve student {name}")
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        todo!("add grade {grade} for student {name}")
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        todo!("get grades for student {name}")
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_add_student() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        assert!(tracker.students.contains_key("Alice"));
    }

    #[test]
    fn test_add_duplicate_student() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_student("Alice"); // Adding a duplicate student should not overwrite
        assert_eq!(tracker.students.len(), 1);
        assert!(tracker.students.contains_key("Alice"));
    }

    #[test]
    fn test_add_grade() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_grade("Alice", 85);
        assert_eq!(tracker.get_grades("Alice"), &[85][..]);
    }

    #[test]
    fn test_get_grades() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_grade("Alice", 85);
        tracker.add_grade("Alice", 90);
        assert_eq!(tracker.get_grades("Alice"), &[85, 90][..]);
    }

    #[test]
    fn test_multiple_students_grades() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_student("Bob");

        tracker.add_grade("Alice", 85);
        tracker.add_grade("Alice", 90);
        tracker.add_grade("Bob", 78);

        assert_eq!(tracker.get_grades("Alice"), &[85, 90][..]);
        assert_eq!(tracker.get_grades("Bob"), &[78][..]);
    }

    #[test]
    fn test_grades_edge_values() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_grade("Alice", 0);
        tracker.add_grade("Alice", 100);

        assert_eq!(tracker.get_grades("Alice"), &[0, 100][..]);
    }

    #[test]
    fn test_large_number_of_students() {
        let mut tracker = StudentGrades::new();
        for i in 0..10_000 {
            tracker.add_student(&format!("Student {}", i));
        }
        assert_eq!(tracker.students.len(), 10_000);
    }

    #[test]
    fn test_large_number_of_grades() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");

        for grade in 0..=255 {
            tracker.add_grade("Alice", grade as u8);
        }

        assert_eq!(tracker.get_grades("Alice").len(), 256);
        assert_eq!(tracker.get_grades("Alice")[..5], [0, 1, 2, 3, 4]);
        assert_eq!(
            tracker.get_grades("Alice")[251..],
            [251, 252, 253, 254, 255]
        );
    }

    #[test]
    fn test_grades_with_empty_student() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        assert_eq!(tracker.get_grades("Alice"), &[]);
    }

    #[test]
    fn test_add_student_with_special_characters() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Ålice_123!@#");
        assert!(tracker.students.contains_key("Ålice_123!@#"));
    }
}
