// Demonstrates using traits as function parameter constraints.
//
// `fn foo(item: &impl TraitName)` means "any type that implements TraitName."
// This is called "impl Trait" syntax.
//
// Run: cargo run --example s18_trait_fn_constraints

trait Printable {
    /// Returns a print-ready string.
    fn to_print(&self) -> String;
}

#[derive(Debug)]
struct Document {
    title: String,
}

#[derive(Debug)]
struct Image {
    filename: String,
    width: u32,
}

impl Printable for Document {
    fn to_print(&self) -> String {
        format!("[DOC] {}", self.title)
    }
}

impl Printable for Image {
    fn to_print(&self) -> String {
        format!("[IMG] {} ({}px)", self.filename, self.width)
    }
}

/// Accepts ANY type that implements Printable.
/// This is "impl Trait" syntax — syntactic sugar for generics.
fn print_item(item: &impl Printable) {
    println!("Printing: {}", item.to_print());
}

/// Multiple trait-constrained parameters.
fn print_two(a: &impl Printable, b: &impl Printable) {
    println!("A: {}", a.to_print());
    println!("B: {}", b.to_print());
}

fn main() {
    let doc = Document {
        title: "Rust Guide".into(),
    };
    let img = Image {
        filename: "logo.png".into(),
        width: 256,
    };

    // Same function, different types:
    print_item(&doc);
    print_item(&img);

    println!();
    // Can mix different types:
    print_two(&doc, &img);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_printable() {
        let d = Document {
            title: "Test".into(),
        };
        assert!(d.to_print().contains("[DOC]"));
    }

    #[test]
    fn test_image_printable() {
        let i = Image {
            filename: "a.jpg".into(),
            width: 100,
        };
        assert!(i.to_print().contains("[IMG]"));
    }
}
