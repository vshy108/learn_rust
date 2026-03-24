// Demonstrates eprintln! for writing to standard error.
//
// - `println!` → writes to stdout (standard output)
// - `eprintln!` → writes to stderr (standard error)
//
// Why use stderr?
// - Error messages should go to stderr so stdout can be piped.
// - `command > output.txt` captures stdout but stderr still shows.
//
// Run: cargo run --example s17_eprintln
// Try: cargo run --example s17_eprintln > /dev/null
//      (You'll still see the error messages!)

fn main() {
    // --- 1. println! goes to stdout ---
    println!("This goes to stdout");

    // --- 2. eprintln! goes to stderr ---
    eprintln!("This goes to stderr (errors/diagnostics)");

    // --- 3. Practical: separate output from errors ---
    let values = vec!["42", "hello", "7", "world", "99"];
    for val in &values {
        match val.parse::<i32>() {
            Ok(n) => println!("Parsed: {n}"), // result to stdout
            Err(e) => eprintln!("Error parsing \"{val}\": {e}"), // error to stderr
        }
    }

    // --- 4. eprint! without newline ---
    eprint!("Loading");
    eprint!(".");
    eprint!(".");
    eprintln!(" done!"); // the newline version

    // --- 5. When to use which ---
    println!();
    println!("stdout (println!): program output, results, data");
    eprintln!("stderr (eprintln!): errors, warnings, progress");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eprintln_compiles() {
        // eprintln! doesn't return anything testable;
        // just verify it compiles.
        eprintln!("test error message");
        assert!(true);
    }
}
