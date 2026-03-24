// Demonstrates the tests module and cfg(test) attribute.
//
// `#[cfg(test)]` means the module is only compiled when running tests.
// This prevents test code from being included in release builds.
//
// Run: cargo test --example s22_tests_module_cfg

/// A simple Calculator struct.
struct Calculator {
    value: f64,
}

impl Calculator {
    fn new(initial: f64) -> Self {
        Self { value: initial }
    }

    fn add(&mut self, n: f64) {
        self.value += n;
    }

    fn subtract(&mut self, n: f64) {
        self.value -= n;
    }

    fn result(&self) -> f64 {
        self.value
    }
}

fn main() {
    let mut calc = Calculator::new(0.0);
    calc.add(10.0);
    calc.subtract(3.0);
    println!("result: {}", calc.result());
}

// This entire module is only compiled during `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function only available in tests
    fn setup() -> Calculator {
        Calculator::new(100.0)
    }

    #[test]
    fn test_new() {
        let calc = Calculator::new(5.0);
        assert_eq!(calc.result(), 5.0);
    }

    #[test]
    fn test_add() {
        let mut calc = setup();
        calc.add(50.0);
        assert_eq!(calc.result(), 150.0);
    }

    #[test]
    fn test_subtract() {
        let mut calc = setup();
        calc.subtract(30.0);
        assert_eq!(calc.result(), 70.0);
    }

    #[test]
    fn test_chain_operations() {
        let mut calc = Calculator::new(0.0);
        calc.add(10.0);
        calc.add(20.0);
        calc.subtract(5.0);
        assert_eq!(calc.result(), 25.0);
    }
}
