//! # Rustfinity: The Drop Trait
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/the-drop-trait
//!
//! The `Drop` trait in Rust is used to run custom cleanup logic when a value goes out of scope. This is particularly useful for managing resources like files, network connections, or any other system resources that need to be explicitly released.
//!
//! When a value that implements `Drop` goes out of scope, the Rust runtime automatically calls the `drop` method, allowing you to define custom behavior for resource cleanup.
//!
//! In this challenge, you will implement a struct that represents a temporary file. The file should be automatically deleted when the struct is dropped, simulating the behavior of temporary files in real-world applications.
//!
//! ## Your Task
//!
//! 1. Create `TempFile` struct with a method `new` that takes a file name as input, it must accept both `&str` and `String` types.
//! 2. Implement the `Drop` trait for `TempFile` to delete the file automatically when the struct goes out of scope.
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Use `std::fs::File` to create a temporary file.
//! - Use `std::env::temp_dir()` to get the path for temporary files.
//! - The `std::fs::remove_file` method can be used to delete files.
//! - The `PathBuf` struct is helpful for managing file paths.
//! - Use the `AsRef<str>` trait to allow flexible input types for the file name.
//! - Implement the `Drop` trait for custom cleanup logic.
//!
//! </details>
//!
//! Run: `cargo run --example rf_the_drop_trait`
//! Test: `cargo test --example rf_the_drop_trait`
//! Filter: `cargo test --example rf_the_drop_trait <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::path::PathBuf;

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn new<N: AsRef<str>>(name: N) -> std::io::Result<Self> {
        todo!("create a temporary file named {}", name.as_ref())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use crate::TempFile;
    use std::path::Path;

    #[test]
    fn test_tempfile_creation_with_unique_name() {
        let temp_file =
            TempFile::new("unique_test_file.tmp").expect("Failed to create temporary file");
        assert!(temp_file.path.exists());
        // Ensure the file does not exist after the TempFile is dropped
        drop(temp_file);
        assert!(!Path::new("unique_test_file.tmp").exists());
    }

    #[test]
    fn test_tempfile_creation_with_same_name_overwrites() {
        let file_name = "overwrite_test_file.tmp";

        // Create the first TempFile
        let temp_file_1 = TempFile::new(file_name).expect("Failed to create first temporary file");
        assert!(temp_file_1.path.exists());

        // Create a second TempFile with the same name
        let temp_file_2 = TempFile::new(file_name).expect("Failed to create second temporary file");
        assert!(temp_file_2.path.exists());

        // The second creation overwrites the first one
        assert_eq!(temp_file_1.path, temp_file_2.path);

        // Ensure the file does not exist after both TempFiles are dropped
        drop(temp_file_1);
        drop(temp_file_2);
        assert!(!Path::new(file_name).exists());
    }

    #[test]
    fn test_tempfile_cleanup_on_scope_exit() {
        let path;
        {
            let temp_file =
                TempFile::new("scope_exit_test_file.tmp").expect("Failed to create temporary file");
            path = temp_file.path.clone();
            assert!(path.exists());
        }
        // TempFile should be deleted after leaving the scope
        assert!(!path.exists());
    }

    #[test]
    fn test_tempfile_creation_with_empty_name() {
        let result = TempFile::new("");
        assert!(
            result.is_err(),
            "Expected error when creating a file with an empty name"
        );
    }

    #[test]
    fn test_tempfile_does_not_leave_orphan_files() {
        let file_name = "orphan_test_file.tmp";
        {
            let temp_file = TempFile::new(file_name).expect("Failed to create temporary file");
            assert!(temp_file.path.exists());
        }
        // Ensure the file does not exist after the TempFile goes out of scope
        assert!(!Path::new(file_name).exists());
    }
}
