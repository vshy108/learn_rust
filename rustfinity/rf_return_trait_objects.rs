//! # Rustfinity: Return Trait Objects
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/return-trait-objects
//!
//! Sometimes, the Rust compiler cannot determine the size of a trait object at compile time. For example, if a function returns a trait object based on a condition, in such cases, `impl Trait` is not going to work. This is because the size of the trait object is not known at compile time. In such cases, you can use `Box<dyn Trait>` to return a trait object.
//!
//! `Box<T>` lets you allocate memory on the heap and store the value of `T` in that memory. `Box<dyn Trait>` is a trait object that allows you to store a value of any type that implements `Trait` in the heap.
//!
//! This is called **dynamic dispatch** because the method to call is determined at runtime, not at compile time. This is in contrast to **static dispatch**, where the method to call is determined at compile time.
//!
//! ## Your Task
//!
//! You need to define a function that returns a `Box<dyn Speakable>` based on a condition.
//!
//! Here is what you need to do:
//!
//! - Define the `Speakable` trait with a method `speak` that returns a `String`.
//! - Define a struct `Dog` with two fields: `name` and `breed`, both of type `String`.
//! - Implement the `Speakable` trait for `Dog` to return a string `Woof`.
//! - Define a struct `Robot` with two fields: `model` and `purpose`, both of type `String`.
//! - Implement the `Speakable` trait for `Robot` to return a string `Beep boop`.
//! - Finish the function `get_speaker` that takes a `&str` parameter and returns either a `Dog` or a `Robot` based on the parameter.
//! - The parameter can either be `dog` or `robot`.
//!
//! Run: `cargo run --example rf_return_trait_objects`
//! Test: `cargo test --example rf_return_trait_objects`
//! Filter: `cargo test --example rf_return_trait_objects <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        todo!("make the dog speak")
    }
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        todo!("make the robot speak")
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => todo!("construct a dog speaker"),
        "robot" => todo!("construct a robot speaker"),
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    fn should_impl_speakable<T: Speakable>() {}

    #[test]
    fn test_dog_speaker() {
        let speaker = get_speaker("dog");
        assert_eq!(speaker.speak(), "Woof");
    }

    #[test]
    fn test_robot_speaker() {
        let speaker = get_speaker("robot");
        assert_eq!(speaker.speak(), "Beep boop");
    }

    #[test]
    #[should_panic]
    fn test_invalid_speaker_type() {
        get_speaker("alien");
    }

    #[test]
    fn test_dog_impl_speakable() {
        should_impl_speakable::<Dog>();
    }

    #[test]
    fn test_robot_impl_speakable() {
        should_impl_speakable::<Robot>();
    }

    #[test]
    fn test_correct_implementation() {
        let dog = Dog {
            name: "Rex".to_string(),
            breed: "Labrador".to_string(),
        };

        assert_eq!(dog.speak(), "Woof");

        let robot = Robot {
            model: "R2-D2".to_string(),
            purpose: "assistance".to_string(),
        };

        assert_eq!(robot.speak(), "Beep boop");
    }
}
