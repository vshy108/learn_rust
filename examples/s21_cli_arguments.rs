// Demonstrates collecting command line arguments with iterators.
//
// `std::env::args()` returns an iterator over CLI arguments.
//
// Run: cargo run --example s21_cli_arguments -- hello world 42

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("=== CLI Arguments ===");
    println!("raw args: {args:?}");
    println!("program: {}", args[0]);

    // --- 1. Skip program name ---
    let user_args: Vec<&String> = args.iter().skip(1).collect();
    println!("user args: {user_args:?}");

    // --- 2. Enumerate ---
    for (i, arg) in args.iter().enumerate() {
        println!("  [{i}] {arg}");
    }

    // --- 3. Parse numeric arguments ---
    let numbers: Vec<i64> = args.iter().skip(1).filter_map(|a| a.parse().ok()).collect();
    println!("numbers found: {numbers:?}");

    if !numbers.is_empty() {
        let sum: i64 = numbers.iter().sum();
        println!("sum: {sum}");
    }

    // --- 4. Find a flag ---
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    println!("verbose mode: {verbose}");

    // --- 5. Argument count ---
    let arg_count = args.len() - 1; // minus program name
    println!("argument count: {arg_count}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_args_exist() {
        // std::env::args() always has at least the program name
        let args: Vec<String> = std::env::args().collect();
        assert!(!args.is_empty());
    }
}
