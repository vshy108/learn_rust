// Section 1, Topic 2: Parameters in the main function
//
// In Python, the entry point is `if __name__ == "__main__":` and you use
// `sys.argv` or `argparse` for command-line arguments.
//
// In Rust, `fn main()` is the entry point. It can have two signatures:
//   1. `fn main()`            — no parameters, no return value
//   2. `fn main() -> Result<(), E>`  — returns a Result for error handling
//
// Rust does NOT allow `fn main(args: ...)`. Instead, use `std::env::args()`
// to access command-line arguments (similar to Python's `sys.argv`).

use std::env;

/// Collect command-line arguments into a Vec<String>.
///
/// # Returns
///
/// A vector of all arguments, where index 0 is the program path
/// (like Python's `sys.argv[0]`).
fn collect_args() -> Vec<String> {
    // `env::args()` returns an iterator over the CLI arguments.
    // The first element is the path to the binary (like sys.argv[0] in Python).
    env::args().collect()
}

/// Count the number of CLI arguments (excluding the program name).
///
/// # Arguments
///
/// * `args` - Slice of argument strings (including program name at index 0).
///
/// # Returns
///
/// The number of user-provided arguments.
fn count_user_args(args: &[String]) -> usize {
    // Skip the first element (program path), count the rest.
    // In Python: len(sys.argv) - 1
    args.len().saturating_sub(1)
}

fn main() {
    let args = collect_args();

    // The program name is always args[0].
    println!("Program: {}", args[0]);
    println!("Number of user arguments: {}", count_user_args(&args));

    // Print each user-supplied argument.
    // `enumerate()` is like Python's enumerate() — gives (index, value).
    // We skip(1) to ignore the program name.
    for (i, arg) in args.iter().skip(1).enumerate() {
        println!("  arg[{}]: {arg}", i + 1);
    }

    // TRY IT: cargo run --example s01_params_in_main -- hello world
    // Output:
    //   Program: target/debug/examples/s01_params_in_main
    //   Number of user arguments: 2
    //     arg[1]: hello
    //     arg[2]: world
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_user_args_with_empty_slice() {
        // No args at all (unusual, but defensive).
        let args: Vec<String> = vec![];
        assert_eq!(count_user_args(&args), 0);
    }

    #[test]
    fn test_count_user_args_with_only_program_name() {
        let args = vec!["my_program".to_string()];
        assert_eq!(count_user_args(&args), 0);
    }

    #[test]
    fn test_count_user_args_with_several_args() {
        let args = vec![
            "my_program".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        assert_eq!(count_user_args(&args), 2);
    }
}
