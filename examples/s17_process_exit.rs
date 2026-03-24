// Demonstrates std::process for exiting the program.
//
// `std::process::exit(code)` terminates immediately.
// Exit code 0 = success, non-zero = failure.
//
// Unlike panic!, exit does NOT unwind the stack or run destructors.
//
// Run: cargo run --example s17_process_exit

use std::process;

fn main() {
    println!("=== process::exit ===");
    println!();

    // --- 1. Basic exit ---
    // process::exit(0); // terminates immediately with code 0

    // --- 2. Exit with error code ---
    // process::exit(1); // non-zero indicates failure

    // --- 3. Practical: validate then exit ---
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 100 {
        // Obviously won't happen in this demo, but shows the pattern
        eprintln!("Error: too many arguments");
        process::exit(1);
    }

    // --- 4. process::abort —  even more abrupt ---
    // process::abort(); // like exit but generates a core dump signal

    // --- 5. Exit vs Panic comparison ---
    println!("exit() vs panic!:");
    println!("  exit(code):   no unwind, no destructors, immediate termination");
    println!("  panic!(msg):  unwinds stack, runs Drop, prints backtrace");
    println!("  abort():      no unwind, no destructors, generates signal");
    println!();

    // --- 6. Exit code via main's return ---
    // Preferred way: return Result from main, or use ExitCode.
    println!("Better approach: use ExitCode or Result from main().");
    println!("  fn main() -> Result<(), Box<dyn Error>> {{ ... }}");
    println!("  fn main() -> ExitCode {{ ExitCode::SUCCESS }}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_compiles() {
        // process::exit can't be meaningfully tested (it kills the process).
        assert!(true);
    }
}
