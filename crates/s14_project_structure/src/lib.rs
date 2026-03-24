//! # s14_project_structure
//!
//! This crate demonstrates Rust's module system with multiple files and folders.
//!
//! ## Module layout
//!
//! ```text
//! src/
//! ├── lib.rs         (crate root — this file)
//! ├── math.rs        (module as file — topic 130)
//! ├── shapes/        (module as folder — topic 131)
//! │   ├── mod.rs
//! │   ├── circle.rs  (submodule — topic 134)
//! │   └── rectangle.rs
//! └── utils.rs       (additional module)
//! ```
//!
//! Topics covered:
//! - 130. Module as file
//! - 131. Module as folder
//! - 134. Submodules
//! - 144. Library crate

// Topic 130: Module as file — `math.rs` is a module because we declare it here.
pub mod math;

// Topic 131: Module as folder — `shapes/mod.rs` is the root of this module.
pub mod shapes;

// Additional module as file.
pub mod utils;

/// Demonstrates the library's public API.
///
/// # Examples
///
/// ```
/// let result = s14_project_structure::run_demo();
/// assert!(result.contains("circle area"));
/// ```
pub fn run_demo() -> String {
    use math::add;
    use shapes::circle::Circle;
    use shapes::rectangle::Rectangle;

    let sum = add(10, 20);
    let c = Circle::new(5.0);
    let r = Rectangle::new(4.0, 6.0);

    format!(
        "sum: {sum}, circle area: {:.2}, rect area: {:.2}",
        c.area(),
        r.area()
    )
}
