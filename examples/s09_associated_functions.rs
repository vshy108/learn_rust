// Demonstrates associated functions (functions in `impl` that DON'T take `self`).
//
// Associated functions are like Python's `@staticmethod` or `@classmethod`.
// They're called on the TYPE, not on an instance: `Type::function()`.
//
// The most common associated function is `new()` — Rust's convention
// for constructors (there's no special constructor syntax like `__init__`).
//
// Run: cargo run --example s09_associated_functions

#[derive(Debug, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Creates a Color from RGB values (associated function / constructor).
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Creates a pure red Color.
    fn red() -> Self {
        Color { r: 255, g: 0, b: 0 }
    }

    /// Creates a pure green Color.
    fn green() -> Self {
        Color { r: 0, g: 255, b: 0 }
    }

    /// Creates a pure blue Color.
    fn blue() -> Self {
        Color { r: 0, g: 0, b: 255 }
    }

    /// Creates a Color from a hex string like "#FF00FF".
    ///
    /// # Arguments
    ///
    /// * `hex` - A hex color string starting with '#'
    ///
    /// # Returns
    ///
    /// Ok(Color) or Err with a description
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is invalid
    fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(format!("expected 6 hex digits, got {}", hex.len()));
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("invalid red: {e}"))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| format!("invalid green: {e}"))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("invalid blue: {e}"))?;
        Ok(Color { r, g, b })
    }

    /// Converts to hex string (method — takes &self).
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

fn main() {
    // --- 1. new() — conventional constructor ---
    let c = Color::new(128, 64, 32);
    println!("new: {c:?}");

    // --- 2. Named constructors ---
    println!("red:   {:?}", Color::red());
    println!("green: {:?}", Color::green());
    println!("blue:  {:?}", Color::blue());

    // --- 3. Fallible constructor ---
    match Color::from_hex("#FF8000") {
        Ok(c) => println!("from_hex: {c:?} → {}", c.to_hex()),
        Err(e) => println!("Error: {e}"),
    }

    match Color::from_hex("#ZZZZZZ") {
        Ok(c) => println!("from_hex: {c:?}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- 4. Note the syntax difference ---
    // Associated functions: Type::function() — called on the type.
    // Methods: instance.method() — called on an instance.
    let white = Color::new(255, 255, 255); // associated function
    let hex = white.to_hex(); // method
    println!("White: {hex}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Color::new(1, 2, 3);
        assert_eq!(c.r, 1);
        assert_eq!(c.g, 2);
        assert_eq!(c.b, 3);
    }

    #[test]
    fn test_named_constructors() {
        assert_eq!(Color::red(), Color::new(255, 0, 0));
        assert_eq!(Color::green(), Color::new(0, 255, 0));
        assert_eq!(Color::blue(), Color::new(0, 0, 255));
    }

    #[test]
    fn test_from_hex_valid() {
        let c = Color::from_hex("#FF8000").unwrap();
        assert_eq!(c, Color::new(255, 128, 0));
    }

    #[test]
    fn test_from_hex_no_hash() {
        let c = Color::from_hex("00FF00").unwrap();
        assert_eq!(c, Color::green());
    }

    #[test]
    fn test_from_hex_invalid() {
        assert!(Color::from_hex("#GG0000").is_err());
        assert!(Color::from_hex("#FFF").is_err());
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(Color::new(255, 0, 128).to_hex(), "#FF0080");
    }
}
