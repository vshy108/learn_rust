// Topic 145: Multiple binary crates.
//
// A package can have multiple binaries in src/bin/*.rs.
// This is one of them. Run with:
//   cargo run -p s14_project_structure --bin demo

use s14_project_structure::{math, run_demo, shapes};

fn main() {
    println!("=== Project Structure Demo ===");
    println!();
    println!("{}", run_demo());
    println!();

    // Using the library's modules directly:
    println!("math::add(5, 3) = {}", math::add(5, 3));
    println!("math::multiply(4, 7) = {}", math::multiply(4, 7));
    println!("{}", shapes::describe("circle"));
}
