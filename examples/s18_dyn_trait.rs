// Demonstrates &dyn Trait — dynamic dispatch via trait objects.
//
// `&dyn Trait` (or `Box<dyn Trait>`) is a "fat pointer":
// - One pointer to the data
// - One pointer to the vtable (virtual method table)
//
// This enables runtime polymorphism.
//
// Run: cargo run --example s18_dyn_trait

use std::fmt;

trait Drawable {
    fn draw(&self) -> String;
}

struct Text {
    content: String,
}
struct Line {
    length: u32,
}
struct Box_ {
    width: u32,
    height: u32,
}

impl Drawable for Text {
    fn draw(&self) -> String {
        format!("Text: \"{}\"", self.content)
    }
}

impl Drawable for Line {
    fn draw(&self) -> String {
        format!("Line: {}", "-".repeat(self.length as usize))
    }
}

impl Drawable for Box_ {
    fn draw(&self) -> String {
        format!("Box: {}x{}", self.width, self.height)
    }
}

// For nice display:
impl fmt::Display for dyn Drawable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.draw())
    }
}

/// Renders all drawable items.
fn render(items: &[&dyn Drawable]) {
    for item in items {
        println!("  {}", item.draw());
    }
}

fn main() {
    let t = Text {
        content: "Hello".into(),
    };
    let l = Line { length: 10 };
    let b = Box_ {
        width: 5,
        height: 3,
    };

    // Heterogeneous collection via trait objects:
    let items: Vec<&dyn Drawable> = vec![&t, &l, &b];
    println!("Rendering:");
    render(&items);

    // With Box for owned storage:
    let owned: Vec<Box<dyn Drawable>> = vec![
        Box::new(Text {
            content: "World".into(),
        }),
        Box::new(Line { length: 5 }),
    ];
    for item in &owned {
        println!("  {}", item.draw());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_object() {
        let items: Vec<Box<dyn Drawable>> = vec![
            Box::new(Text {
                content: "test".into(),
            }),
            Box::new(Line { length: 3 }),
        ];
        assert!(items[0].draw().contains("test"));
        assert!(items[1].draw().contains("---"));
    }
}
