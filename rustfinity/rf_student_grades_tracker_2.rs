//! # Rustfinity: Student Grades Tracker 2
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/student-grades-tracker-2
//!
//! If you’ve completed the first part of the Student Grades Tracker challenge, congratulations! Now it’s time to enhance the functionality by adding methods directly to the `Student` struct.
//!
//! This will help you practice designing methods for individual struct instances while integrating them into a larger system.
//!
//! For this challenge, we’ll build upon the `StudentGrades` system. In addition to the methods you’ve already implemented, you’ll now focus on adding two new methods to the `Student` struct itself.
//!
//! ## Your Task
//!
//! You need to extend the `Student` struct by implementing two methods:
//!
//! 1. **`add_grade`:** Add a grade to the student’s `grades` vector.
//! 2. **`average_grade(&self) -> f64`:** Calculate and return the average grade of the student. Return `0.0` if the student has no grades.
//!
//! The `StudentGrades` struct remains the same as in the previous challenge. You’ll modify its existing methods to use the newly added methods of the `Student` struct.
//!
//! Run: `cargo run --example rf_student_grades_tracker_2`
//! Test: `cargo test --example rf_student_grades_tracker_2`
//! Filter: `cargo test --example rf_student_grades_tracker_2 <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

impl Student {
    pub fn add_grade(&mut self, grade: u8) {
        // Implement here
        unimplemented!()
    }

    pub fn average_grade(&self) -> f64 {
        // Implement here
        unimplemented!()
    }
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

    pub fn add_student(&mut self, name: &str) {
        self.students.entry(name.to_string()).or_insert(Student {
            name: name.to_string(),
            grades: vec![],
        });
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_grade(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        &self.students.get(name).unwrap().grades
    }
}

pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    let alice = tracker.students.get_mut("Alice").unwrap();

    alice.add_grade(95);
    println!("{:?}", alice.grades);
    println!("{:?}", alice.average_grade());
    println!("{:?}", tracker.get_grades("Bob"));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_student_add_grade() {
        let mut student = Student {
            name: "Alice".to_string(),
            grades: vec![],
        };
        student.add_grade(85);
        student.add_grade(90);
        assert_eq!(student.grades, vec![85, 90]);
    }

    #[test]
    fn test_student_average_grade() {
        let student = Student {
            name: "Alice".to_string(),
            grades: vec![85, 90, 78],
        };
        assert!((student.average_grade() - 84.333).abs() < 0.01);
    }

    #[test]
    fn test_student_average_grade_no_grades() {
        let student = Student {
            name: "Alice".to_string(),
            grades: vec![],
        };
        assert_eq!(student.average_grade(), 0.0);
    }

    #[test]
    fn test_add_student() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_student("Bob");
        assert!(tracker.students.contains_key("Alice"));
        assert!(tracker.students.contains_key("Bob"));
        assert_eq!(tracker.students["Alice"].grades, vec![]);
    }

    #[test]
    fn test_add_grade_to_student() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_grade("Alice", 85);
        tracker.add_grade("Alice", 90);
        assert_eq!(tracker.students["Alice"].grades, vec![85, 90]);
    }

    #[test]
    fn test_multiple_students_grades_and_averages() {
        let mut tracker = StudentGrades::new();
        tracker.add_student("Alice");
        tracker.add_student("Bob");

        tracker.add_grade("Alice", 85);
        tracker.add_grade("Alice", 90);
        tracker.add_grade("Bob", 78);
        tracker.add_grade("Bob", 88);

        let alice = &tracker.students["Alice"];
        let bob = &tracker.students["Bob"];

        assert_eq!(alice.grades, vec![85, 90]);
        assert_eq!(bob.grades, vec![78, 88]);

        assert!((alice.average_grade() - 87.5).abs() < 0.01);
        assert!((bob.average_grade() - 83.0).abs() < 0.01);
    }

    #[test]
    fn test_large_dataset() {
        let mut tracker = StudentGrades::new();
        for i in 1..=100 {
            tracker.add_student(&format!("Student {}", i));
        }
        for i in 1..=100 {
            tracker.add_grade(&format!("Student {}", i), i as u8);
        }

        for i in 1..=100 {
            let student = &tracker.students[&format!("Student {}", i)];
            assert_eq!(student.grades, vec![i as u8]);
            assert_eq!(student.average_grade(), i as f64);
        }
    }
}
