//! # Rustfinity: Supertraits
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/supertraits
//!
//! In Rust, supertraits allow you to define a trait hierarchy where a derived trait requires another trait to be implemented first. This is useful when modeling interfaces that build upon or depend on other capabilities.
//!
//! In this challenge, you will define two traits: `Person` and `Student`. The `Person` trait provides the ability to retrieve a name, while the `Student` trait extends `Person` by adding additional fields specific to students, such as an ID and a field of study.
//!
//! ## Your Task
//!
//! 1. Define a trait `Person`:
//!    - It should require a method `fn name(&self) -> String` that returns the name of the person.
//!
//! 2. Define a trait `Student` that has `Person` as a supertrait:
//!    - It should require methods:
//!      - `fn id(&self) -> u32` to return the student ID.
//!      - `fn field_of_study(&self) -> String` to return the student's field of study.
//!
//! 3. Implement both traits for a struct `Undergraduate`:
//!    - The `Undergraduate` struct should have fields `id`, `name`, and `field_of_study`.
//!    - Use the `Student` trait to provide the student's ID, name, and field of study.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use the syntax `trait Student: Person {}` to define `Student` as a supertrait of `Person`.
//! - Remember to implement both `Person` and `Student` for the `Undergraduate` struct.
//! - Use `self.name.clone()` and `self.field_of_study.clone()` to return `String` values without ownership issues.
//! - You can call methods from the supertrait explicitly using `<YourType as YourTrait>::method_name()`.
//!
//! </details>
//!
//! Run: `cargo run --example rf_supertraits`
//! Test: `cargo test --example rf_supertraits`
//! Filter: `cargo test --example rf_supertraits <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub trait Person {
    fn name(&self) -> String;
}

pub trait Student: Person {
    fn id(&self) -> u32;
    fn field_of_study(&self) -> String;
}

pub struct Undergraduate {
    pub id: u32,
    pub name: String,
    pub field_of_study: String,
}

impl Person for Undergraduate {
    fn name(&self) -> String {
        todo!("return the student name")
    }
}

impl Student for Undergraduate {
    fn id(&self) -> u32 {
        todo!("return the student id")
    }
    fn field_of_study(&self) -> String {
        todo!("return the field of study")
    }
}

// Example usage
pub fn main() {
    let student = Undergraduate {
        id: 101,
        name: "John Doe".to_string(),
        field_of_study: "Computer Science".to_string(),
    };

    assert_eq!(student.name(), "John Doe");
    assert_eq!(student.id(), 101);
    assert_eq!(student.field_of_study(), "Computer Science");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    fn implements_person<T: Person>() {}
    fn implements_student<T: Student>() {}

    #[test]
    fn test_person_trait() {
        let student = Undergraduate {
            id: 101,
            name: "John Doe".to_string(),
            field_of_study: "Computer Science".to_string(),
        };

        assert_eq!(student.name(), "John Doe");
    }

    #[test]
    fn test_student_trait() {
        let student = Undergraduate {
            id: 101,
            name: "John Doe".to_string(),
            field_of_study: "Computer Science".to_string(),
        };

        assert_eq!(student.id(), 101);
        assert_eq!(student.field_of_study(), "Computer Science");
    }

    #[test]
    fn test_name_and_field_of_study() {
        let student = Undergraduate {
            id: 202,
            name: "Jane Smith".to_string(),
            field_of_study: "Mathematics".to_string(),
        };

        assert_eq!(student.name(), "Jane Smith");
        assert_eq!(student.id(), 202);
        assert_eq!(student.field_of_study(), "Mathematics");
    }

    #[test]
    fn test_multiple_undergraduates() {
        let student1 = Undergraduate {
            id: 1,
            name: "Alice".to_string(),
            field_of_study: "Physics".to_string(),
        };
        let student2 = Undergraduate {
            id: 2,
            name: "Bob".to_string(),
            field_of_study: "Chemistry".to_string(),
        };

        assert_eq!(student1.name(), "Alice");
        assert_eq!(student1.id(), 1);
        assert_eq!(student1.field_of_study(), "Physics");
        assert_eq!(student2.name(), "Bob");
        assert_eq!(student2.id(), 2);
        assert_eq!(student2.field_of_study(), "Chemistry");
    }

    #[test]
    fn test_empty_name_and_field_of_study() {
        let student = Undergraduate {
            id: 303,
            name: "".to_string(),
            field_of_study: "".to_string(),
        };

        assert_eq!(student.name(), "");
        assert_eq!(student.id(), 303);
        assert_eq!(student.field_of_study(), "");
    }

    #[test]
    fn test_special_characters_in_name_and_field_of_study() {
        let student = Undergraduate {
            id: 404,
            name: "Dr. Strange".to_string(),
            field_of_study: "Mystic Arts".to_string(),
        };

        assert_eq!(student.name(), "Dr. Strange");
        assert_eq!(student.id(), 404);
        assert_eq!(student.field_of_study(), "Mystic Arts");
    }

    #[test]
    fn test_numeric_field_of_study() {
        let student = Undergraduate {
            id: 505,
            name: "Number Theory Enthusiast".to_string(),
            field_of_study: "12345".to_string(),
        };

        assert_eq!(student.name(), "Number Theory Enthusiast");
        assert_eq!(student.id(), 505);
        assert_eq!(student.field_of_study(), "12345");
    }

    #[test]
    fn test_long_name_and_field_of_study() {
        let student = Undergraduate {
            id: 606,
            name: "A Very Long Name That Should Still Work Fine".to_string(),
            field_of_study: "A Field of Study That Is Also Surprisingly Long".to_string(),
        };

        assert_eq!(
            student.name(),
            "A Very Long Name That Should Still Work Fine"
        );
        assert_eq!(student.id(), 606);
        assert_eq!(
            student.field_of_study(),
            "A Field of Study That Is Also Surprisingly Long"
        );
    }

    #[test]
    fn test_edge_case_id_zero() {
        let student = Undergraduate {
            id: 0,
            name: "Zero ID".to_string(),
            field_of_study: "Edge Case Studies".to_string(),
        };

        assert_eq!(student.name(), "Zero ID");
        assert_eq!(student.id(), 0);
        assert_eq!(student.field_of_study(), "Edge Case Studies");
    }

    #[test]
    fn test_edge_case_id_large_value() {
        let student = Undergraduate {
            id: u32::MAX,
            name: "Max Value Student".to_string(),
            field_of_study: "Large Numbers".to_string(),
        };

        assert_eq!(student.name(), "Max Value Student");
        assert_eq!(student.id(), u32::MAX);
        assert_eq!(student.field_of_study(), "Large Numbers");
    }

    #[test]
    fn test_implements_person_trait() {
        implements_person::<Undergraduate>();
    }

    #[test]
    fn test_implements_student_trait() {
        implements_student::<Undergraduate>();
    }
}
