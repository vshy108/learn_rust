// Demonstrates &dyn Trait — dynamic dispatch via trait objects.
//
// `dyn Trait` means "dynamic dispatch" — the concrete type is resolved at
// runtime via a vtable (virtual function table), NOT at compile time.
//
// `&dyn Trait` (or `Box<dyn Trait>`) is a "fat pointer":
// - One pointer to the data
// - One pointer to the vtable (virtual method table)
//
// This enables runtime polymorphism — a collection can hold DIFFERENT types
// that all implement the same trait.
//
// WHY a pointer is required (Box, &, Arc):
// `dyn Trait` is "unsized" — the compiler doesn't know its size at compile time
// (could be a Text, Line, or Box_ — all different sizes). So it MUST live
// behind a pointer:
//   let e: Box<dyn Error>;     // owned, heap-allocated
//   let e: &dyn Error;         // borrowed reference
//   let e: Arc<dyn Error>;     // shared ownership
//   let e: dyn Error;          // COMPILE ERROR — size unknown
//
// dyn (dynamic dispatch) vs generics (static dispatch):
// ┌──────────────────┬────────────────────────────────────────┐
// │ Static (generics) │ Dynamic (dyn Trait)                   │
// ├──────────────────┼────────────────────────────────────────┤
// │ Resolved at       │ Compile time          │ Runtime       │
// │ Performance       │ Zero-cost, inlinable  │ Vtable lookup │
// │ Binary size       │ Larger (monomorphized)│ Smaller       │
// │ Heterogeneous?    │ No                    │ Yes           │
// └──────────────────┴────────────────────────────────────────┘
//
// Where dyn appears in error handling:
//   anyhow::Error ≈ Box<dyn std::error::Error + Send + Sync + 'static>
//   Box<dyn Error> as a quick-and-dirty error type (prefer anyhow instead)
//
// Gotcha: once behind `dyn`, you lose the concrete type — you can only call
// methods defined on the trait. To get the original type back, use downcast:
//   if let Some(io_err) = err.downcast_ref::<io::Error>() { ... }
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

/// Renders all drawable items using dynamic dispatch.
/// `&[&dyn Drawable]` — each element can be a DIFFERENT concrete type.
/// Compare with static dispatch: `fn render<T: Drawable>(items: &[T])` which
/// requires all elements to be the SAME concrete type.
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
