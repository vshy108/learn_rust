// Topic 131: Module as folder.
//
// This file (`shapes/mod.rs`) is the root of the `shapes` module.
// Declared in lib.rs with: `pub mod shapes;`
// The compiler looks for `shapes/mod.rs` (or `shapes.rs`).
//
// Topic 134: Submodules are declared here.

// Submodules — each is a file in the `shapes/` directory.
pub mod circle;
pub mod rectangle;

/// Describes a shape by name.
pub fn describe(name: &str) -> String {
    format!("Shape: {name}")
}
