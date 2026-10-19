//! # Rustfinity: Path Operations
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/path-operations
//!
//! Working with file paths is essential in any program that interacts with the filesystem. Rust provides the `Path` and `PathBuf` types in `std::path` for safe, cross-platform path manipulation. Unlike raw strings, these types understand path semantics and handle platform differences automatically.
//!
//! ## Path vs PathBuf
//!
//! `Path` is an unsized type (like `str`) that represents a borrowed path, while `PathBuf` is an owned, mutable path (like `String`). You'll typically use `&Path` for function parameters and `PathBuf` when you need to own or modify a path:
//!
//! ```rust
//! use std::path::{Path, PathBuf};
//!
//! let path: &Path = Path::new("/home/user/file.txt");
//! let mut owned: PathBuf = PathBuf::from("/home/user");
//! owned.push("documents");
//! owned.push("file.txt");
//! ```
//!
//! ## Building Paths Safely
//!
//! Use `.join()` to combine path components safely - it handles separators automatically:
//!
//! ```rust
//! let base = Path::new("/home/user");
//! let full = base.join("documents").join("report.pdf");
//! // Result: /home/user/documents/report.pdf
//! ```
//!
//! ## Extracting Path Components
//!
//! The `Path` type provides methods to extract various components:
//!
//! ```rust
//! let path = Path::new("/home/user/document.txt");
//!
//! path.file_name();      // Some("document.txt")
//! path.extension();      // Some("txt")
//! path.file_stem();      // Some("document")
//! path.parent();         // Some("/home/user")
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions for path manipulation:
//!
//! 1. `join_paths(base: &str, parts: &[&str]) -> PathBuf` - Join a base path with multiple components
//! 2. `get_extension(path: &str) -> Option<String>` - Extract the file extension (without the dot)
//! 3. `get_file_name(path: &str) -> Option<String>` - Extract the file name (including extension)
//! 4. `get_file_stem(path: &str) -> Option<String>` - Extract the file name without extension
//! 5. `get_parent(path: &str) -> Option<PathBuf>` - Get the parent directory
//! 6. `change_extension(path: &str, new_ext: &str) -> PathBuf` - Change the file extension
//! 7. `is_absolute(path: &str) -> bool` - Check if a path is absolute
//! 8. `normalize_path(path: &str) -> PathBuf` - Convert a path to use the platform's separator
//!
//! ## Examples
//!
//! ```rust
//! // Join paths safely
//! let result = join_paths(
//!     "/home/user",
//!     &["documents", "reports", "2024"]
//! );
//! let expected = PathBuf::from(
//!     "/home/user/documents/reports/2024"
//! );
//! assert_eq!(result, expected);
//!
//! // Extract extension
//! assert_eq!(
//!     get_extension("report.pdf"),
//!     Some("pdf".to_string())
//! );
//! assert_eq!(get_extension("Makefile"), None);
//!
//! // Get file name
//! assert_eq!(
//!     get_file_name("/home/user/doc.txt"),
//!     Some("doc.txt".to_string())
//! );
//!
//! // Get file stem (name without extension)
//! assert_eq!(
//!     get_file_stem("archive.tar.gz"),
//!     Some("archive.tar".to_string())
//! );
//!
//! // Get parent directory
//! assert_eq!(
//!     get_parent("/home/user/file.txt"),
//!     Some(PathBuf::from("/home/user"))
//! );
//! assert_eq!(get_parent("/"), None);
//!
//! // Change extension
//! assert_eq!(
//!     change_extension("report.doc", "pdf"),
//!     PathBuf::from("report.pdf")
//! );
//! assert_eq!(
//!     change_extension("Makefile", "bak"),
//!     PathBuf::from("Makefile.bak")
//! );
//!
//! // Check if absolute
//! assert!(is_absolute("/home/user"));
//! assert!(!is_absolute("relative/path"));
//!
//! // Normalize path (result depends on platform)
//! let normalized = normalize_path("home/user/file.txt");
//! // On Unix: "home/user/file.txt"
//! // On Windows: "home\\user\\file.txt"
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `Path::new()` to create a `Path` reference from a string
//! - Use `PathBuf::from()` to create an owned path
//! - The `.join()` method returns a new `PathBuf` with the path component appended
//! - `.extension()` returns an `Option<&OsStr>` - convert with `.to_str()` and `.to_string()`
//! - `.file_name()` and `.file_stem()` also return `Option<&OsStr>`
//! - `.parent()` returns `Option<&Path>` - convert to `PathBuf` with `.to_path_buf()`
//! - Use `.set_extension()` on a `PathBuf` to change
//!   the extension
//! - `.is_absolute()` checks if a path starts from a root
//! - `PathBuf::from()` with a string already normalizes separators for the current platform
//!
//! </details>
//!
//! Run: `cargo run --example rf_path_operations`
//! Test: `cargo test --example rf_path_operations`
//! Filter: `cargo test --example rf_path_operations <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::path::{Path, PathBuf};

/// Joins a base path with multiple path components.
///
/// # Arguments
///
/// * `base` - The base path as a string
/// * `parts` - A slice of path components to join
///
/// # Returns
///
/// A `PathBuf` containing the joined path
pub fn join_paths(base: &str, parts: &[&str]) -> PathBuf {
    // TODO: Create a PathBuf from base and join all parts
    unimplemented!()
}

/// Extracts the file extension from a path (without the leading dot).
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(String)` containing the extension, or `None` if no extension exists
pub fn get_extension(path: &str) -> Option<String> {
    // TODO: Use Path::new() and the .extension() method
    unimplemented!()
}

/// Extracts the file name (including extension) from a path.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(String)` containing the file name, or `None` if the path ends with `..`
pub fn get_file_name(path: &str) -> Option<String> {
    // TODO: Use Path::new() and the .file_name() method
    unimplemented!()
}

/// Extracts the file stem (file name without extension) from a path.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(String)` containing the file stem, or `None` if no file name exists
pub fn get_file_stem(path: &str) -> Option<String> {
    // TODO: Use Path::new() and the .file_stem() method
    unimplemented!()
}

/// Gets the parent directory of a path.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(PathBuf)` containing the parent directory, or `None` if the path has no parent
pub fn get_parent(path: &str) -> Option<PathBuf> {
    // TODO: Use Path::new() and the .parent() method
    unimplemented!()
}

/// Changes the extension of a path.
///
/// # Arguments
///
/// * `path` - The original path as a string
/// * `new_ext` - The new extension (without the leading dot)
///
/// # Returns
///
/// A `PathBuf` with the new extension
pub fn change_extension(path: &str, new_ext: &str) -> PathBuf {
    // TODO: Create a PathBuf and use .set_extension()
    unimplemented!()
}

/// Checks if a path is absolute.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `true` if the path is absolute, `false` otherwise
pub fn is_absolute(path: &str) -> bool {
    // TODO: Use Path::new() and the .is_absolute() method
    unimplemented!()
}

/// Normalizes a path to use the platform's native separators.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// A `PathBuf` with platform-appropriate separators
pub fn normalize_path(path: &str) -> PathBuf {
    // TODO: PathBuf::from() handles normalization automatically
    unimplemented!()
}

// Example usage
pub fn main() {
    // Join paths
    let full_path = join_paths("/home/user", &["documents", "reports"]);
    println!("Joined path: {:?}", full_path);

    // Get extension
    let ext = get_extension("document.pdf");
    println!("Extension: {:?}", ext);

    // Get file name
    let name = get_file_name("/home/user/document.pdf");
    println!("File name: {:?}", name);

    // Get parent
    let parent = get_parent("/home/user/document.pdf");
    println!("Parent: {:?}", parent);

    // Change extension
    let new_path = change_extension("report.doc", "pdf");
    println!("New extension: {:?}", new_path);

    // Check if absolute
    println!("Is absolute '/home': {}", is_absolute("/home"));
    println!("Is absolute 'relative': {}", is_absolute("relative"));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::path::PathBuf;

    // ============== join_paths tests ==============

    #[test]
    fn test_join_paths_single_part() {
        let result = join_paths("/home/user", &["documents"]);
        assert_eq!(result, PathBuf::from("/home/user/documents"));
    }

    #[test]
    fn test_join_paths_multiple_parts() {
        let result = join_paths("/home/user", &["documents", "reports", "2024"]);
        assert_eq!(result, PathBuf::from("/home/user/documents/reports/2024"));
    }

    #[test]
    fn test_join_paths_empty_parts() {
        let result = join_paths("/home/user", &[]);
        assert_eq!(result, PathBuf::from("/home/user"));
    }

    #[test]
    fn test_join_paths_relative_base() {
        let result = join_paths("relative/path", &["file.txt"]);
        assert_eq!(result, PathBuf::from("relative/path/file.txt"));
    }

    #[test]
    fn test_join_paths_with_file() {
        let result = join_paths("/var/log", &["app", "error.log"]);
        assert_eq!(result, PathBuf::from("/var/log/app/error.log"));
    }

    // ============== get_extension tests ==============

    #[test]
    fn test_get_extension_simple() {
        assert_eq!(get_extension("file.txt"), Some("txt".to_string()));
    }

    #[test]
    fn test_get_extension_pdf() {
        assert_eq!(get_extension("report.pdf"), Some("pdf".to_string()));
    }

    #[test]
    fn test_get_extension_double() {
        assert_eq!(get_extension("archive.tar.gz"), Some("gz".to_string()));
    }

    #[test]
    fn test_get_extension_none() {
        assert_eq!(get_extension("Makefile"), None);
    }

    #[test]
    fn test_get_extension_hidden_file() {
        assert_eq!(get_extension(".gitignore"), None);
    }

    #[test]
    fn test_get_extension_hidden_with_ext() {
        assert_eq!(get_extension(".bashrc.bak"), Some("bak".to_string()));
    }

    #[test]
    fn test_get_extension_full_path() {
        assert_eq!(get_extension("/home/user/doc.txt"), Some("txt".to_string()));
    }

    #[test]
    fn test_get_extension_empty_extension() {
        assert_eq!(get_extension("file."), Some("".to_string()));
    }

    // ============== get_file_name tests ==============

    #[test]
    fn test_get_file_name_simple() {
        assert_eq!(
            get_file_name("document.txt"),
            Some("document.txt".to_string())
        );
    }

    #[test]
    fn test_get_file_name_with_path() {
        assert_eq!(
            get_file_name("/home/user/document.txt"),
            Some("document.txt".to_string())
        );
    }

    #[test]
    fn test_get_file_name_no_extension() {
        assert_eq!(get_file_name("/bin/bash"), Some("bash".to_string()));
    }

    #[test]
    fn test_get_file_name_hidden() {
        assert_eq!(
            get_file_name("/home/user/.bashrc"),
            Some(".bashrc".to_string())
        );
    }

    #[test]
    fn test_get_file_name_trailing_slash() {
        // Rust's Path::file_name() ignores trailing slashes
        assert_eq!(get_file_name("/home/user/"), Some("user".to_string()));
    }

    #[test]
    fn test_get_file_name_root() {
        assert_eq!(get_file_name("/"), None);
    }

    #[test]
    fn test_get_file_name_double_dot() {
        assert_eq!(get_file_name("/home/user/.."), None);
    }

    // ============== get_file_stem tests ==============

    #[test]
    fn test_get_file_stem_simple() {
        assert_eq!(get_file_stem("document.txt"), Some("document".to_string()));
    }

    #[test]
    fn test_get_file_stem_double_extension() {
        assert_eq!(
            get_file_stem("archive.tar.gz"),
            Some("archive.tar".to_string())
        );
    }

    #[test]
    fn test_get_file_stem_no_extension() {
        assert_eq!(get_file_stem("Makefile"), Some("Makefile".to_string()));
    }

    #[test]
    fn test_get_file_stem_hidden() {
        assert_eq!(get_file_stem(".gitignore"), Some(".gitignore".to_string()));
    }

    #[test]
    fn test_get_file_stem_hidden_with_ext() {
        assert_eq!(get_file_stem(".bashrc.bak"), Some(".bashrc".to_string()));
    }

    #[test]
    fn test_get_file_stem_with_path() {
        assert_eq!(
            get_file_stem("/home/user/report.pdf"),
            Some("report".to_string())
        );
    }

    // ============== get_parent tests ==============

    #[test]
    fn test_get_parent_simple() {
        assert_eq!(
            get_parent("/home/user/file.txt"),
            Some(PathBuf::from("/home/user"))
        );
    }

    #[test]
    fn test_get_parent_nested() {
        assert_eq!(
            get_parent("/var/log/app/error.log"),
            Some(PathBuf::from("/var/log/app"))
        );
    }

    #[test]
    fn test_get_parent_root() {
        assert_eq!(get_parent("/"), None);
    }

    #[test]
    fn test_get_parent_single_component() {
        // A single component relative path has no meaningful parent
        assert_eq!(get_parent("file.txt"), None);
    }

    #[test]
    fn test_get_parent_relative() {
        assert_eq!(
            get_parent("path/to/file.txt"),
            Some(PathBuf::from("path/to"))
        );
    }

    #[test]
    fn test_get_parent_two_levels() {
        assert_eq!(get_parent("/home/user"), Some(PathBuf::from("/home")));
    }

    // ============== change_extension tests ==============

    #[test]
    fn test_change_extension_simple() {
        assert_eq!(
            change_extension("report.doc", "pdf"),
            PathBuf::from("report.pdf")
        );
    }

    #[test]
    fn test_change_extension_add_new() {
        assert_eq!(
            change_extension("Makefile", "bak"),
            PathBuf::from("Makefile.bak")
        );
    }

    #[test]
    fn test_change_extension_double() {
        assert_eq!(
            change_extension("archive.tar.gz", "bz2"),
            PathBuf::from("archive.tar.bz2")
        );
    }

    #[test]
    fn test_change_extension_with_path() {
        assert_eq!(
            change_extension("/home/user/doc.txt", "md"),
            PathBuf::from("/home/user/doc.md")
        );
    }

    #[test]
    fn test_change_extension_remove() {
        assert_eq!(change_extension("file.txt", ""), PathBuf::from("file"));
    }

    #[test]
    fn test_change_extension_hidden() {
        assert_eq!(
            change_extension(".gitignore", "bak"),
            PathBuf::from(".gitignore.bak")
        );
    }

    // ============== is_absolute tests ==============

    #[test]
    fn test_is_absolute_unix_root() {
        assert!(is_absolute("/"));
    }

    #[test]
    fn test_is_absolute_unix_path() {
        assert!(is_absolute("/home/user/documents"));
    }

    #[test]
    fn test_is_absolute_relative() {
        assert!(!is_absolute("relative/path"));
    }

    #[test]
    fn test_is_absolute_current_dir() {
        assert!(!is_absolute("./file.txt"));
    }

    #[test]
    fn test_is_absolute_parent_dir() {
        assert!(!is_absolute("../file.txt"));
    }

    #[test]
    fn test_is_absolute_just_filename() {
        assert!(!is_absolute("file.txt"));
    }

    // ============== normalize_path tests ==============

    #[test]
    fn test_normalize_path_simple() {
        let result = normalize_path("home/user/file.txt");
        assert_eq!(result, PathBuf::from("home/user/file.txt"));
    }

    #[test]
    fn test_normalize_path_absolute() {
        let result = normalize_path("/var/log/app.log");
        assert_eq!(result, PathBuf::from("/var/log/app.log"));
    }

    #[test]
    fn test_normalize_path_single_component() {
        let result = normalize_path("file.txt");
        assert_eq!(result, PathBuf::from("file.txt"));
    }

    #[test]
    fn test_normalize_path_with_dots() {
        let result = normalize_path("./relative/./path");
        assert_eq!(result, PathBuf::from("./relative/./path"));
    }

    // ============== Integration tests ==============

    #[test]
    fn test_build_and_extract_path() {
        // Build a path
        let full = join_paths("/home/user", &["documents", "report.pdf"]);

        // Extract components
        assert_eq!(
            get_file_name(full.to_str().unwrap()),
            Some("report.pdf".to_string())
        );
        assert_eq!(
            get_extension(full.to_str().unwrap()),
            Some("pdf".to_string())
        );
        assert_eq!(
            get_file_stem(full.to_str().unwrap()),
            Some("report".to_string())
        );
        assert_eq!(
            get_parent(full.to_str().unwrap()),
            Some(PathBuf::from("/home/user/documents"))
        );
    }

    #[test]
    fn test_change_extension_workflow() {
        let original = "/home/user/document.doc";

        // Change to different format
        let as_pdf = change_extension(original, "pdf");
        assert_eq!(as_pdf, PathBuf::from("/home/user/document.pdf"));

        // Create backup
        let backup = change_extension(original, "doc.bak");
        assert_eq!(backup, PathBuf::from("/home/user/document.doc.bak"));
    }

    #[test]
    fn test_path_analysis() {
        let paths = vec![
            ("/home/user/file.txt", true, Some("txt")),
            ("relative/path/doc.pdf", false, Some("pdf")),
            ("/bin/bash", true, None),
            ("Makefile", false, None),
        ];

        for (path, expected_absolute, expected_ext) in paths {
            assert_eq!(
                is_absolute(path),
                expected_absolute,
                "Failed for path: {}",
                path
            );
            assert_eq!(
                get_extension(path).as_deref(),
                expected_ext,
                "Failed extension for path: {}",
                path
            );
        }
    }

    #[test]
    fn test_nested_directories() {
        let base = "/var";
        let full = join_paths(base, &["log", "nginx", "access.log"]);

        assert_eq!(full, PathBuf::from("/var/log/nginx/access.log"));
        assert!(is_absolute(full.to_str().unwrap()));

        let parent = get_parent(full.to_str().unwrap()).unwrap();
        assert_eq!(parent, PathBuf::from("/var/log/nginx"));

        let grandparent = get_parent(parent.to_str().unwrap()).unwrap();
        assert_eq!(grandparent, PathBuf::from("/var/log"));
    }

    #[test]
    fn test_hidden_files() {
        let hidden = ".config";
        let hidden_with_ext = ".bashrc.backup";

        assert_eq!(get_extension(hidden), None);
        assert_eq!(get_file_stem(hidden), Some(".config".to_string()));

        assert_eq!(get_extension(hidden_with_ext), Some("backup".to_string()));
        assert_eq!(get_file_stem(hidden_with_ext), Some(".bashrc".to_string()));
    }
}
