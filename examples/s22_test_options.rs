// Demonstrates various cargo test options.
//
// Common test flags:
// - `cargo test`                       — run all tests
// - `cargo test -- --show-output`      — show println! in passing tests
// - `cargo test -- --nocapture`        — don't capture output at all
// - `cargo test -- --ignored`          — run ignored tests only
// - `cargo test -- --include-ignored`  — run all including ignored
// - `cargo test -- --test-threads=1`   — run tests serially
// - `cargo test -- --exact test_name`  — match exact test name
// - `cargo test -p crate_name`         — test a specific crate
// - `cargo test --lib`                 — only library tests
// - `cargo test --doc`                 — only doc tests
// - `cargo test --examples`            — only example tests
//
// Run: cargo test --example s22_test_options

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

fn main() {
    println!("factorial(5) = {}", factorial(5));
    println!("fibonacci(10) = {}", fibonacci(10));
    println!();
    println!("Test options showcase. Run with various flags:");
    println!("  cargo test --example s22_test_options");
    println!("  cargo test --example s22_test_options -- --show-output");
    println!("  cargo test --example s22_test_options -- --ignored");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_with_output() {
        println!("This appears with --show-output");
        assert!(true);
    }

    #[test]
    #[ignore = "slow test"]
    fn test_large_factorial() {
        assert_eq!(factorial(20), 2432902008176640000);
    }
}
