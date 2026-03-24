// Topic 145: Second binary crate.
//
// Run with: cargo run -p s14_project_structure --bin shapes_demo

use s14_project_structure::shapes::{circle::Circle, rectangle::Rectangle};

fn main() {
    println!("=== Shapes Demo ===");
    println!();

    let c = Circle::new(10.0);
    println!(
        "Circle(r=10): area={:.2}, circumference={:.2}",
        c.area(),
        c.circumference()
    );

    let r = Rectangle::new(5.0, 8.0);
    println!(
        "Rectangle(5x8): area={:.2}, perimeter={:.2}, square={}",
        r.area(),
        r.perimeter(),
        r.is_square()
    );

    let sq = Rectangle::new(4.0, 4.0);
    println!("Rectangle(4x4): square={}", sq.is_square());
}
