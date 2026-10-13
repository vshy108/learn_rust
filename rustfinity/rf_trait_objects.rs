//! # Rustfinity: Trait Objects
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/trait-objects
//!
//! Now that we know a little bit about trait objects and how to work with them, let's use them in a different example, this challenge focuses on using `Box<dyn Trait>` as fields in structs to encapsulate objects with dynamic behavior.
//!
//! ## Your Task
//!
//! You need to define a trait called `Renderable` and create two structs, `Circle` and `Rectangle`, that implement this trait. Then, you will create another struct called `Canvas`, which can hold a collection of objects implementing the `Renderable` trait. The `Canvas` struct should have methods to add objects and render all objects.
//!
//! ### Requirements
//!
//! Here are the requirements for the program, make sure to read them carefully:
//!
//! #### Define the `Renderable` Trait
//!
//! - The trait should have a method `render(&self) -> String` to represent the object visually.
//!
//! #### Define the `Circle` and `Rectangle` Structs
//!
//! - `Circle` should have a `radius: f64` field.
//! - `Rectangle` should have `width: f64` and `height: f64` fields.
//!
//! #### Implement the `Renderable` Trait
//!
//! - For `Circle`, the `render` method should return a string like `"Circle with radius X"`.
//! - For `Rectangle`, the `render` method should return a string like `"Rectangle with width X and height Y"`.
//!
//! #### Define the `Canvas` Struct
//!
//! - The struct should have a `shapes` field that can store a `Vec` of the Renderable trait objects.
//! - Implement the following methods for `Canvas`:
//!   - `new() -> Canvas`: Initializes an empty canvas.
//!   - `add_shape()`: Adds a shape to the canvas.
//!   - `render_all()`: Returns a vector of strings, each representing a rendered shape.
//!
//! **⚠️ Make sure you make all the relevant items public with the `pub` keyword, this is required for the tests to work.**
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `Box<dyn Trait>` to store objects with dynamic behavior. e.g.
//!   ```rust
//!   pub struct Canvas {
//!     pub shapes: Vec<Box<dyn Renderable>>,
//!   }
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_trait_objects`
//! Test: `cargo test --example rf_trait_objects`
//! Filter: `cargo test --example rf_trait_objects <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Renderable for Circle {
    fn render(&self) -> String {
        todo!("render the circle")
    }
}

impl Renderable for Rectangle {
    fn render(&self) -> String {
        todo!("render the rectangle")
    }
}

pub struct Canvas {
    pub shapes: Vec<Box<dyn Renderable>>,
}

impl Canvas {
    pub fn new() -> Self {
        todo!("create an empty canvas")
    }

    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        todo!("add a shape")
    }

    pub fn render_all(&self) -> Vec<String> {
        todo!("render all shapes")
    }
}

// Example usage
pub fn main() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));
    let rendered_shapes = canvas.render_all();
    for shape in rendered_shapes {
        println!("{}", shape);
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_circle_render() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.render(), "Circle with radius 5");
    }

    #[test]
    fn test_rectangle_render() {
        let rectangle = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(rectangle.render(), "Rectangle with width 3 and height 4");
    }

    #[test]
    fn test_canvas_add_and_render() {
        let mut canvas = Canvas::new();
        canvas.add_shape(Box::new(Circle { radius: 5.0 }));
        canvas.add_shape(Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }));
        let rendered_shapes = canvas.render_all();
        assert_eq!(
            rendered_shapes,
            vec![
                "Circle with radius 5",
                "Rectangle with width 3 and height 4"
            ]
        );
    }

    #[test]
    fn test_empty_canvas_render() {
        let canvas = Canvas::new();
        let rendered_shapes = canvas.render_all();
        assert!(rendered_shapes.is_empty(), "Canvas should start empty");
    }

    #[test]
    fn test_add_multiple_shapes() {
        let mut canvas = Canvas::new();
        canvas.add_shape(Box::new(Circle { radius: 2.5 }));
        canvas.add_shape(Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }));
        canvas.add_shape(Box::new(Circle { radius: 7.0 }));

        let rendered_shapes = canvas.render_all();
        assert_eq!(
            rendered_shapes,
            vec![
                "Circle with radius 2.5",
                "Rectangle with width 4 and height 6",
                "Circle with radius 7"
            ]
        );
    }

    #[test]
    fn test_large_values() {
        let circle = Circle { radius: 1e9 };
        let rectangle = Rectangle {
            width: 1e9,
            height: 2e9,
        };

        assert_eq!(circle.render(), "Circle with radius 1000000000");
        assert_eq!(
            rectangle.render(),
            "Rectangle with width 1000000000 and height 2000000000"
        );
    }

    #[test]
    fn test_zero_values() {
        let circle = Circle { radius: 0.0 };
        let rectangle = Rectangle {
            width: 0.0,
            height: 0.0,
        };

        assert_eq!(circle.render(), "Circle with radius 0");
        assert_eq!(rectangle.render(), "Rectangle with width 0 and height 0");
    }

    #[test]
    fn test_negative_values() {
        let circle = Circle { radius: -3.0 };
        let rectangle = Rectangle {
            width: -4.0,
            height: -5.0,
        };

        assert_eq!(circle.render(), "Circle with radius -3");
        assert_eq!(rectangle.render(), "Rectangle with width -4 and height -5");
    }

    #[test]
    fn test_canvas_with_large_and_zero_shapes() {
        let mut canvas = Canvas::new();
        canvas.add_shape(Box::new(Circle { radius: 1e9 }));
        canvas.add_shape(Box::new(Rectangle {
            width: 0.0,
            height: 0.0,
        }));

        let rendered_shapes = canvas.render_all();
        assert_eq!(
            rendered_shapes,
            vec![
                "Circle with radius 1000000000",
                "Rectangle with width 0 and height 0"
            ]
        );
    }

    #[test]
    fn test_render_lots_of_items() {
        let mut canvas = Canvas::new();
        for i in 0..1000 {
            if i % 2 == 0 {
                canvas.add_shape(Box::new(Circle { radius: i as f64 }));
            } else {
                canvas.add_shape(Box::new(Rectangle {
                    width: i as f64,
                    height: i as f64,
                }));
            }
        }

        let rendered_shapes = canvas.render_all();
        for i in 0..1000 {
            if i % 2 == 0 {
                assert_eq!(rendered_shapes[i], format!("Circle with radius {}", i));
            } else {
                assert_eq!(
                    rendered_shapes[i],
                    format!("Rectangle with width {} and height {}", i, i)
                );
            }
        }
    }
}
