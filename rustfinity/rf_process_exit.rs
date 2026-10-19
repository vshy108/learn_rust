//! # Rustfinity: Process Exit
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/process-exit
//!
//! # Process and Exit
//!
//! In Rust, the `std::process` module provides functionality for working with the current process and spawning new processes. This challenge focuses on working with the current process - getting its ID, handling exit codes, and managing process information.
//!
//! ## Key Concepts
//!
//! ### Process ID
//!
//! Every running process has a unique identifier (PID) assigned by the operating system. You can get the current process ID using `std::process::id()`.
//!
//! ### Exit Codes
//!
//! When a program terminates, it returns an exit code to the operating system:
//!
//! - **0** typically indicates success
//! - **Non-zero values** indicate various types of failures
//!
//! The `std::process::ExitCode` type provides a type-safe way to represent exit codes. You can also use `std::process::exit()` to terminate a program immediately with a specific exit code.
//!
//! ### Abort
//!
//! In extreme cases, you can use `std::process::abort()` to terminate the process abnormally. This is typically used when the program is in an unrecoverable state.
//!
//! ## Your Task
//!
//! Implement the following functions to work with process information and exit codes:
//!
//! 1. **`get_process_id`** - Returns the current process ID as a `u32`
//!
//! 2. **`exit_code_success`** - Returns an `ExitCode` representing success (0)
//!
//! 3. **`exit_code_failure`** - Returns an `ExitCode` representing failure (1)
//!
//! 4. **`exit_code_from_u8`** - Converts a `u8` value to an `ExitCode`
//!
//! 5. **`is_success_code`** - Checks if a given `u8` exit code represents success (0)
//!
//! 6. **`is_failure_code`** - Checks if a given `u8` exit code represents failure (non-zero)
//!
//! 7. **`describe_exit_code`** - Returns a description string for common exit codes:
//!    - 0 -> "success"
//!    - 1 -> "general error"
//!    - 2 -> "misuse of command"
//!    - 126 -> "command not executable"
//!    - 127 -> "command not found"
//!    - 128 -> "invalid exit argument"
//!    - 130 -> "script terminated by ctrl-c"
//!    - Any other value -> "unknown exit code: N" (where N is the code)
//!
//! 8. **`validate_exit_code`** - Takes an `Option<u8>` and returns:
//!    - `Ok(code)` if Some and code is 0
//!    - `Err("process failed with code N")` if Some and code is non-zero
//!    - `Err("process did not return an exit code")` if None
//!
//! ## Examples
//!
//! ```rust
//! use process_exit::*;
//!
//! // Get current process ID
//! let pid = get_process_id();
//! assert!(pid > 0); // PID is always positive
//!
//! // Create exit codes
//! let success = exit_code_success();
//! let failure = exit_code_failure();
//! let custom = exit_code_from_u8(42);
//!
//! // Check exit codes
//! assert!(is_success_code(0));
//! assert!(!is_success_code(1));
//! assert!(is_failure_code(1));
//! assert!(!is_failure_code(0));
//!
//! // Describe exit codes
//! assert_eq!(describe_exit_code(0), "success");
//! assert_eq!(describe_exit_code(127), "command not found");
//! assert_eq!(describe_exit_code(42), "unknown exit code: 42");
//!
//! // Validate exit codes
//! assert_eq!(validate_exit_code(Some(0)), Ok(0));
//! assert_eq!(
//!     validate_exit_code(Some(1)),
//!     Err("process failed with code 1".to_string())
//! );
//! assert_eq!(
//!     validate_exit_code(None),
//!     Err("process did not return an exit code".to_string())
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `std::process::id()` to get the current process ID
//! - `ExitCode::SUCCESS` and `ExitCode::FAILURE` are predefined constants
//! - Use `ExitCode::from(u8)` to create an exit code from a number
//! - For `describe_exit_code`, use a `match` expression with the common Unix exit codes
//! - Remember that exit code 0 is the only "success" code; all others indicate failure
//!
//! </details>
//!
//! Run: `cargo run --example rf_process_exit`
//! Test: `cargo test --example rf_process_exit`
//! Filter: `cargo test --example rf_process_exit <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::process::ExitCode;

/// Returns the current process ID.
///
/// Every running process has a unique identifier (PID) assigned by the
/// operating system.
pub fn get_process_id() -> u32 {
    // TODO: Use std::process::id() to get the current process ID
    unimplemented!()
}

/// Returns an ExitCode representing success (0).
///
/// By convention, exit code 0 indicates that a program completed successfully.
pub fn exit_code_success() -> ExitCode {
    // TODO: Return ExitCode::SUCCESS
    unimplemented!()
}

/// Returns an ExitCode representing failure (1).
///
/// By convention, exit code 1 indicates a general error condition.
pub fn exit_code_failure() -> ExitCode {
    // TODO: Return ExitCode::FAILURE
    unimplemented!()
}

/// Converts a u8 value to an ExitCode.
///
/// This allows creating an exit code with any value from 0 to 255.
pub fn exit_code_from_u8(code: u8) -> ExitCode {
    // TODO: Use ExitCode::from() to convert the u8 to an ExitCode
    unimplemented!()
}

/// Checks if a given exit code represents success.
///
/// In Unix conventions, exit code 0 indicates success.
pub fn is_success_code(code: u8) -> bool {
    // TODO: Return true if code is 0, false otherwise
    unimplemented!()
}

/// Checks if a given exit code represents failure.
///
/// In Unix conventions, any non-zero exit code indicates failure.
pub fn is_failure_code(code: u8) -> bool {
    // TODO: Return true if code is non-zero, false otherwise
    unimplemented!()
}

/// Returns a description string for common exit codes.
///
/// Common Unix exit codes have specific meanings:
/// - 0: success
/// - 1: general error
/// - 2: misuse of command
/// - 126: command not executable
/// - 127: command not found
/// - 128: invalid exit argument
/// - 130: script terminated by ctrl-c
/// - Any other value: "unknown exit code: N"
pub fn describe_exit_code(code: u8) -> String {
    // TODO: Use a match expression to return the appropriate description
    unimplemented!()
}

/// Validates an optional exit code.
///
/// Returns Ok with the code if it's Some(0), indicating success.
/// Returns Err with an appropriate message otherwise.
pub fn validate_exit_code(code: Option<u8>) -> Result<u8, String> {
    // TODO: Match on the Option
    unimplemented!()
}

// Example usage
pub fn main() {
    // Get the current process ID
    let pid = get_process_id();
    println!("Current process ID: {}", pid);

    // Create different exit codes
    let success = exit_code_success();
    let failure = exit_code_failure();
    let custom = exit_code_from_u8(42);

    println!("Success code: {:?}", success);
    println!("Failure code: {:?}", failure);
    println!("Custom code (42): {:?}", custom);

    // Check if codes represent success or failure
    println!("\nChecking exit codes:");
    println!("Is 0 success? {}", is_success_code(0));
    println!("Is 1 success? {}", is_success_code(1));
    println!("Is 0 failure? {}", is_failure_code(0));
    println!("Is 1 failure? {}", is_failure_code(1));

    // Describe common exit codes
    println!("\nExit code descriptions:");
    for code in [0, 1, 2, 126, 127, 128, 130, 42] {
        println!("  {}: {}", code, describe_exit_code(code));
    }

    // Validate exit codes
    println!("\nValidating exit codes:");
    println!("  Some(0): {:?}", validate_exit_code(Some(0)));
    println!("  Some(1): {:?}", validate_exit_code(Some(1)));
    println!("  None: {:?}", validate_exit_code(None));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::process::ExitCode;

    // ==================== get_process_id tests ====================

    #[test]
    fn test_get_process_id_positive() {
        let pid = get_process_id();
        assert!(pid > 0, "Process ID should be positive, got {}", pid);
    }

    #[test]
    fn test_get_process_id_consistent() {
        let pid1 = get_process_id();
        let pid2 = get_process_id();
        assert_eq!(
            pid1, pid2,
            "Process ID should be consistent within same process"
        );
    }

    #[test]
    fn test_get_process_id_reasonable_value() {
        let pid = get_process_id();
        // PIDs are typically within a reasonable range (less than a million on most systems)
        assert!(
            pid < 10_000_000,
            "Process ID should be within reasonable bounds"
        );
    }

    // ==================== exit_code_success tests ====================

    #[test]
    fn test_exit_code_success_returns_success() {
        let code = exit_code_success();
        // ExitCode doesn't implement PartialEq, so we compare via Debug format
        assert_eq!(format!("{:?}", code), format!("{:?}", ExitCode::SUCCESS));
    }

    #[test]
    fn test_exit_code_success_is_standard() {
        let code = exit_code_success();
        // ExitCode doesn't expose its value directly, but we can verify it's SUCCESS
        let success = ExitCode::SUCCESS;
        // Both should have the same representation
        assert_eq!(format!("{:?}", code), format!("{:?}", success));
    }

    // ==================== exit_code_failure tests ====================

    #[test]
    fn test_exit_code_failure_returns_failure() {
        let code = exit_code_failure();
        // ExitCode doesn't implement PartialEq, so we compare via Debug format
        assert_eq!(format!("{:?}", code), format!("{:?}", ExitCode::FAILURE));
    }

    #[test]
    fn test_exit_code_failure_is_standard() {
        let code = exit_code_failure();
        let failure = ExitCode::FAILURE;
        assert_eq!(format!("{:?}", code), format!("{:?}", failure));
    }

    #[test]
    fn test_exit_code_success_and_failure_different() {
        let success = exit_code_success();
        let failure = exit_code_failure();
        // These should produce different debug representations
        assert_ne!(format!("{:?}", success), format!("{:?}", failure));
    }

    // ==================== exit_code_from_u8 tests ====================

    #[test]
    fn test_exit_code_from_u8_zero() {
        let code = exit_code_from_u8(0);
        assert_eq!(format!("{:?}", code), format!("{:?}", ExitCode::SUCCESS));
    }

    #[test]
    fn test_exit_code_from_u8_one() {
        let code = exit_code_from_u8(1);
        assert_eq!(format!("{:?}", code), format!("{:?}", ExitCode::FAILURE));
    }

    #[test]
    fn test_exit_code_from_u8_custom_value() {
        let code = exit_code_from_u8(42);
        let expected = ExitCode::from(42u8);
        assert_eq!(format!("{:?}", code), format!("{:?}", expected));
    }

    #[test]
    fn test_exit_code_from_u8_max_value() {
        let code = exit_code_from_u8(255);
        let expected = ExitCode::from(255u8);
        assert_eq!(format!("{:?}", code), format!("{:?}", expected));
    }

    #[test]
    fn test_exit_code_from_u8_common_codes() {
        // Test common Unix exit codes
        let codes = [2, 126, 127, 128, 130];
        for &c in &codes {
            let code = exit_code_from_u8(c);
            let expected = ExitCode::from(c);
            assert_eq!(format!("{:?}", code), format!("{:?}", expected));
        }
    }

    // ==================== is_success_code tests ====================

    #[test]
    fn test_is_success_code_zero() {
        assert!(is_success_code(0));
    }

    #[test]
    fn test_is_success_code_one() {
        assert!(!is_success_code(1));
    }

    #[test]
    fn test_is_success_code_various_failure_codes() {
        for code in [1, 2, 126, 127, 128, 130, 255] {
            assert!(
                !is_success_code(code),
                "Code {} should not be success",
                code
            );
        }
    }

    #[test]
    fn test_is_success_code_boundary() {
        assert!(is_success_code(0));
        assert!(!is_success_code(1));
        assert!(!is_success_code(255));
    }

    // ==================== is_failure_code tests ====================

    #[test]
    fn test_is_failure_code_zero() {
        assert!(!is_failure_code(0));
    }

    #[test]
    fn test_is_failure_code_one() {
        assert!(is_failure_code(1));
    }

    #[test]
    fn test_is_failure_code_various_codes() {
        for code in [1, 2, 126, 127, 128, 130, 255] {
            assert!(is_failure_code(code), "Code {} should be failure", code);
        }
    }

    #[test]
    fn test_is_failure_code_boundary() {
        assert!(!is_failure_code(0));
        assert!(is_failure_code(1));
        assert!(is_failure_code(255));
    }

    #[test]
    fn test_success_and_failure_mutually_exclusive() {
        for code in 0..=255u8 {
            assert_ne!(
                is_success_code(code),
                is_failure_code(code),
                "Code {} should be either success OR failure, not both or neither",
                code
            );
        }
    }

    // ==================== describe_exit_code tests ====================

    #[test]
    fn test_describe_exit_code_success() {
        assert_eq!(describe_exit_code(0), "success");
    }

    #[test]
    fn test_describe_exit_code_general_error() {
        assert_eq!(describe_exit_code(1), "general error");
    }

    #[test]
    fn test_describe_exit_code_misuse() {
        assert_eq!(describe_exit_code(2), "misuse of command");
    }

    #[test]
    fn test_describe_exit_code_not_executable() {
        assert_eq!(describe_exit_code(126), "command not executable");
    }

    #[test]
    fn test_describe_exit_code_not_found() {
        assert_eq!(describe_exit_code(127), "command not found");
    }

    #[test]
    fn test_describe_exit_code_invalid_argument() {
        assert_eq!(describe_exit_code(128), "invalid exit argument");
    }

    #[test]
    fn test_describe_exit_code_ctrl_c() {
        assert_eq!(describe_exit_code(130), "script terminated by ctrl-c");
    }

    #[test]
    fn test_describe_exit_code_unknown_small() {
        assert_eq!(describe_exit_code(3), "unknown exit code: 3");
    }

    #[test]
    fn test_describe_exit_code_unknown_large() {
        assert_eq!(describe_exit_code(255), "unknown exit code: 255");
    }

    #[test]
    fn test_describe_exit_code_unknown_medium() {
        assert_eq!(describe_exit_code(42), "unknown exit code: 42");
    }

    #[test]
    fn test_describe_exit_code_unknown_125() {
        assert_eq!(describe_exit_code(125), "unknown exit code: 125");
    }

    #[test]
    fn test_describe_exit_code_unknown_129() {
        assert_eq!(describe_exit_code(129), "unknown exit code: 129");
    }

    // ==================== validate_exit_code tests ====================

    #[test]
    fn test_validate_exit_code_success() {
        assert_eq!(validate_exit_code(Some(0)), Ok(0));
    }

    #[test]
    fn test_validate_exit_code_failure_one() {
        assert_eq!(
            validate_exit_code(Some(1)),
            Err("process failed with code 1".to_string())
        );
    }

    #[test]
    fn test_validate_exit_code_failure_127() {
        assert_eq!(
            validate_exit_code(Some(127)),
            Err("process failed with code 127".to_string())
        );
    }

    #[test]
    fn test_validate_exit_code_failure_255() {
        assert_eq!(
            validate_exit_code(Some(255)),
            Err("process failed with code 255".to_string())
        );
    }

    #[test]
    fn test_validate_exit_code_none() {
        assert_eq!(
            validate_exit_code(None),
            Err("process did not return an exit code".to_string())
        );
    }

    #[test]
    fn test_validate_exit_code_returns_code_on_success() {
        if let Ok(code) = validate_exit_code(Some(0)) {
            assert_eq!(code, 0);
        } else {
            panic!("Expected Ok(0)");
        }
    }

    #[test]
    fn test_validate_exit_code_various_failures() {
        for code in [1, 2, 42, 126, 127, 128, 130, 255] {
            let result = validate_exit_code(Some(code));
            assert!(result.is_err(), "Code {} should be an error", code);
            assert!(
                result.unwrap_err().contains(&code.to_string()),
                "Error message should contain the code"
            );
        }
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_exit_code_workflow() {
        // Simulate a command execution workflow
        let code: Option<u8> = Some(0);

        // Validate the exit code
        let result = validate_exit_code(code);
        assert!(result.is_ok());

        // Get description
        let description = describe_exit_code(result.unwrap());
        assert_eq!(description, "success");
    }

    #[test]
    fn test_integration_failed_command_workflow() {
        // Simulate a failed command
        let code: Option<u8> = Some(127);

        // Validate (should fail)
        let result = validate_exit_code(code);
        assert!(result.is_err());

        // Get description of the original code
        let description = describe_exit_code(127);
        assert_eq!(description, "command not found");
    }

    #[test]
    fn test_integration_success_failure_check_consistency() {
        for code in 0..=255u8 {
            let is_success = is_success_code(code);
            let is_failure = is_failure_code(code);
            let validation = validate_exit_code(Some(code));

            // Success check should match validation
            assert_eq!(
                is_success,
                validation.is_ok(),
                "Consistency failed for code {}",
                code
            );

            // Failure check should be opposite of validation success
            assert_eq!(
                is_failure,
                validation.is_err(),
                "Consistency failed for code {}",
                code
            );
        }
    }

    #[test]
    fn test_integration_describe_all_known_codes() {
        let known_codes = [
            (0, "success"),
            (1, "general error"),
            (2, "misuse of command"),
            (126, "command not executable"),
            (127, "command not found"),
            (128, "invalid exit argument"),
            (130, "script terminated by ctrl-c"),
        ];

        for (code, expected_desc) in known_codes {
            assert_eq!(describe_exit_code(code), expected_desc);
        }
    }

    #[test]
    fn test_integration_exit_codes_match_standard() {
        // Verify that our exit_code_from_u8 produces consistent results
        assert_eq!(
            format!("{:?}", exit_code_from_u8(0)),
            format!("{:?}", exit_code_success())
        );
        assert_eq!(
            format!("{:?}", exit_code_from_u8(1)),
            format!("{:?}", exit_code_failure())
        );
    }

    #[test]
    fn test_integration_process_info() {
        let pid = get_process_id();

        // Process ID should be valid
        assert!(pid > 0);

        // Should be consistent
        assert_eq!(pid, get_process_id());
    }

    #[test]
    fn test_integration_error_handling_pattern() {
        // Common pattern: run a command and handle the result
        fn handle_command_result(exit_code: Option<u8>) -> String {
            match validate_exit_code(exit_code) {
                Ok(_) => "Command succeeded".to_string(),
                Err(e) => {
                    if let Some(code) = exit_code {
                        format!("{}: {}", e, describe_exit_code(code))
                    } else {
                        e
                    }
                }
            }
        }

        assert_eq!(handle_command_result(Some(0)), "Command succeeded");
        assert_eq!(
            handle_command_result(Some(127)),
            "process failed with code 127: command not found"
        );
        assert_eq!(
            handle_command_result(None),
            "process did not return an exit code"
        );
    }
}
