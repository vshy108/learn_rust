//! # Rustfinity: File Metadata
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/file-metadata
//!
//! When working with files, you often need more than just their contents. File metadata provides crucial information about files: their size, when they were created or modified, what type they are (file, directory, symlink), and their permissions. Rust's `std::fs::Metadata` struct gives you access to all of this information through a clean, cross-platform API.
//!
//! The `metadata()` function (or `symlink_metadata()` for symlinks) returns a `Metadata` struct containing information about a file. This is useful for filtering files by size or date, checking permissions before attempting operations, or building file managers and backup tools.
//!
//! ## Getting File Metadata
//!
//! You can obtain metadata using `std::fs::metadata()` or from a path directly:
//!
//! ```rust
//! use std::fs;
//!
//! // From a path
//! let meta = fs::metadata("file.txt")?;
//!
//! // Check file size in bytes
//! let size = meta.len();
//!
//! // Check if it's a file, directory, or symlink
//! let is_file = meta.is_file();
//! let is_dir = meta.is_dir();
//! let file_type = meta.file_type();
//!
//! // Get modification and access times
//! let modified = meta.modified()?;
//! let accessed = meta.accessed()?;
//! ```
//!
//! ## File Types
//!
//! The `FileType` struct tells you what kind of filesystem entry you're dealing with:
//!
//! ```rust
//! let file_type = meta.file_type();
//! if file_type.is_file() {
//!     println!("Regular file");
//! } else if file_type.is_dir() {
//!     println!("Directory");
//! } else if file_type.is_symlink() {
//!     println!("Symbolic link");
//! }
//! ```
//!
//! ## Working with Times
//!
//! Modification and access times are returned as `SystemTime`, which you can compare or convert to durations:
//!
//! ```rust
//! use std::time::{SystemTime, Duration};
//!
//! let modified = meta.modified()?;
//! let age = SystemTime::now().duration_since(modified)?;
//! println!("File was modified {} seconds ago", age.as_secs());
//! ```
//!
//! ## Platform-Specific Permissions
//!
//! On Unix systems, you can access file permissions through the `PermissionsExt` trait:
//!
//! ```rust
//! #[cfg(unix)]
//! use std::os::unix::fs::PermissionsExt;
//!
//! let perms = meta.permissions();
//! #[cfg(unix)]
//! {
//!     let mode = perms.mode();
//!     // Check execute bits
//!     let is_executable = mode & 0o111 != 0;
//! }
//! ```
//!
//! ## Your Task
//!
//! Implement the following functions to work with file metadata:
//!
//! 1. `get_file_size(path: &Path) -> io::Result<u64>`
//!    - Returns the file size in bytes
//! 2. `get_file_type(
//!  path: &Path
//! ) -> io::Result<String>`
//!    - Returns "file", "directory", or "symlink"
//! 3. `is_readonly(path: &Path) -> io::Result<bool>`
//!    - Checks if the file is read-only
//! 4. `get_modified_time(
//!  path: &Path
//! ) -> io::Result<SystemTime>`
//!    - Returns when the file was last modified
//! 5. `was_modified_within(
//!  path: &Path,
//!  duration: Duration
//! ) -> io::Result<bool>`
//!    - Checks if file was modified within the given
//!      duration
//! 6. `is_executable(
//!  path: &Path
//! ) -> io::Result<bool>`
//!    - Checks if file has execute permissions (Unix)
//!      or has executable extension (Windows)
//! 7. `compare_modified_times(
//!  path1: &Path,
//!  path2: &Path
//! ) -> io::Result<Ordering>`
//!    - Compares which file was modified more recently
//!
//! ## Examples
//!
//! ```rust
//! use std::path::Path;
//! use std::time::Duration;
//! use std::cmp::Ordering;
//!
//! // Get file size
//! let size = get_file_size(Path::new("data.txt"))?;
//! assert!(size > 0);
//!
//! // Check file type
//! let file_type = get_file_type(Path::new("./src"))?;
//! assert_eq!(file_type, "directory");
//!
//! // Check if read-only
//! let readonly = is_readonly(Path::new("config.txt"))?;
//!
//! // Get modification time
//! let modified = get_modified_time(Path::new("file.txt"))?;
//!
//! // Check if recently modified (within last hour)
//! let recent = was_modified_within(
//!     Path::new("log.txt"),
//!     Duration::from_secs(3600)
//! )?;
//!
//! // Check if executable
//! let executable = is_executable(Path::new("script.sh"))?;
//!
//! // Compare modification times
//! let ordering = compare_modified_times(
//!     Path::new("file1.txt"),
//!     Path::new("file2.txt")
//! )?;
//! match ordering {
//!     Ordering::Less => println!("file1 is older"),
//!     Ordering::Greater => println!("file1 is newer"),
//!     Ordering::Equal => println!("Same modification time"),
//! }
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - Use `std::fs::metadata(path)` to get the
//!   `Metadata` struct
//! - `Metadata::len()` returns file size in bytes
//! - `Metadata::file_type()` returns a `FileType`
//!   with `is_file()`, `is_dir()`, `is_symlink()`
//!   methods
//! - `Metadata::permissions()` returns a
//!   `Permissions` struct with `readonly()` method
//! - `Metadata::modified()` returns
//!   `io::Result<SystemTime>` - the modification
//!   time
//! - For `was_modified_within`, use
//!   `SystemTime::now().duration_since(modified)`
//!   to get elapsed time
//! - For Unix executable check, use `#[cfg(unix)]`
//!   and
//!   `std::os::unix::fs::PermissionsExt::mode()`
//! - The execute bit can be checked with
//!   `mode & 0o111 != 0` (any of owner/group/
//!   other execute)
//! - For Windows, you might check file extension (
//!   `.exe`, `.bat`, `.cmd`) as an alternative
//! - For comparing times, `SystemTime` implements
//!   `PartialOrd` so you can compare directly
//!
//! </details>
//!
//! Run: `cargo run --example rf_file_metadata`
//! Test: `cargo test --example rf_file_metadata`
//! Filter: `cargo test --example rf_file_metadata <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Returns the size of a file in bytes.
///
/// Use `std::fs::metadata()` to get file information.
pub fn get_file_size(path: &Path) -> io::Result<u64> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns the type of a filesystem entry as a string.
///
/// Returns "file", "directory", or "symlink".
/// Use `std::fs::symlink_metadata()` to detect symlinks properly.
pub fn get_file_type(path: &Path) -> io::Result<String> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if a file is read-only.
///
/// Use the permissions() method on Metadata.
pub fn is_readonly(path: &Path) -> io::Result<bool> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns when a file was last modified.
///
/// Returns the modification time as a SystemTime.
pub fn get_modified_time(path: &Path) -> io::Result<SystemTime> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if a file was modified within a given duration from now.
///
/// Compare the modification time with the current time.
pub fn was_modified_within(path: &Path, duration: Duration) -> io::Result<bool> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if a file has execute permissions.
///
/// On Unix, check the execute bits using PermissionsExt.
/// On Windows, check for executable extensions (.exe, .bat, .cmd, .com).
pub fn is_executable(path: &Path) -> io::Result<bool> {
    // TODO: Implement this function
    unimplemented!()
}

/// Compares the modification times of two files.
///
/// Returns Ordering::Less if path1 was modified before path2,
/// Ordering::Greater if path1 was modified after path2,
/// Ordering::Equal if they have the same modification time.
pub fn compare_modified_times(path1: &Path, path2: &Path) -> io::Result<Ordering> {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    use std::fs::File;
    use std::io::Write;

    // Create a test file
    let test_dir = Path::new("metadata_test");
    let _ = fs::create_dir_all(test_dir);

    let file_path = test_dir.join("test_file.txt");
    let _ = File::create(&file_path).and_then(|mut f| f.write_all(b"Hello, World!"));

    println!("=== File Metadata Examples ===\n");

    // Test your implementations here
    match get_file_size(&file_path) {
        Ok(size) => println!("File size: {} bytes", size),
        Err(e) => println!("Error getting size: {}", e),
    }

    match get_file_type(&file_path) {
        Ok(ft) => println!("File type: {}", ft),
        Err(e) => println!("Error getting type: {}", e),
    }

    match is_readonly(&file_path) {
        Ok(ro) => println!("Is read-only: {}", ro),
        Err(e) => println!("Error checking readonly: {}", e),
    }

    match get_modified_time(&file_path) {
        Ok(time) => println!("Modified time: {:?}", time),
        Err(e) => println!("Error getting modified time: {}", e),
    }

    match was_modified_within(&file_path, Duration::from_secs(60)) {
        Ok(recent) => println!("Modified within last minute: {}", recent),
        Err(e) => println!("Error checking recency: {}", e),
    }

    match is_executable(&file_path) {
        Ok(exec) => println!("Is executable: {}", exec),
        Err(e) => println!("Error checking executable: {}", e),
    }

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
    println!("\nTest directory cleaned up.");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::cmp::Ordering;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;

    // Helper to create a temporary test directory
    fn setup_test_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("file_metadata_test_{}", name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    // Helper to cleanup test directory
    fn cleanup_test_dir(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    // ==================== get_file_size tests ====================

    #[test]
    fn test_get_file_size_basic() {
        let dir = setup_test_dir("size_basic");
        let file_path = dir.join("test.txt");
        let content = "Hello, World!";
        File::create(&file_path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();

        let size = get_file_size(&file_path).unwrap();
        assert_eq!(size, content.len() as u64);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_file_size_empty_file() {
        let dir = setup_test_dir("size_empty");
        let file_path = dir.join("empty.txt");
        File::create(&file_path).unwrap();

        let size = get_file_size(&file_path).unwrap();
        assert_eq!(size, 0);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_file_size_larger_file() {
        let dir = setup_test_dir("size_large");
        let file_path = dir.join("large.txt");
        let content = "x".repeat(10000);
        File::create(&file_path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();

        let size = get_file_size(&file_path).unwrap();
        assert_eq!(size, 10000);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_file_size_nonexistent() {
        let result = get_file_size(Path::new("/nonexistent/path/file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_size_binary_content() {
        let dir = setup_test_dir("size_binary");
        let file_path = dir.join("binary.bin");
        let content: Vec<u8> = (0..=255).collect();
        File::create(&file_path)
            .unwrap()
            .write_all(&content)
            .unwrap();

        let size = get_file_size(&file_path).unwrap();
        assert_eq!(size, 256);

        cleanup_test_dir(&dir);
    }

    // ==================== get_file_type tests ====================

    #[test]
    fn test_get_file_type_regular_file() {
        let dir = setup_test_dir("type_file");
        let file_path = dir.join("regular.txt");
        File::create(&file_path).unwrap();

        let file_type = get_file_type(&file_path).unwrap();
        assert_eq!(file_type, "file");

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_file_type_directory() {
        let dir = setup_test_dir("type_dir");
        let subdir = dir.join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file_type = get_file_type(&subdir).unwrap();
        assert_eq!(file_type, "directory");

        cleanup_test_dir(&dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_get_file_type_symlink() {
        use std::os::unix::fs::symlink;

        let dir = setup_test_dir("type_symlink");
        let file_path = dir.join("original.txt");
        let link_path = dir.join("link.txt");
        File::create(&file_path).unwrap();
        symlink(&file_path, &link_path).unwrap();

        let file_type = get_file_type(&link_path).unwrap();
        assert_eq!(file_type, "symlink");

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_file_type_nonexistent() {
        let result = get_file_type(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_type_nested_directory() {
        let dir = setup_test_dir("type_nested");
        let nested = dir.join("a").join("b").join("c");
        fs::create_dir_all(&nested).unwrap();

        let file_type = get_file_type(&nested).unwrap();
        assert_eq!(file_type, "directory");

        cleanup_test_dir(&dir);
    }

    // ==================== is_readonly tests ====================

    #[test]
    fn test_is_readonly_writable_file() {
        let dir = setup_test_dir("readonly_writable");
        let file_path = dir.join("writable.txt");
        File::create(&file_path).unwrap();

        let readonly = is_readonly(&file_path).unwrap();
        assert!(!readonly);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_is_readonly_readonly_file() {
        let dir = setup_test_dir("readonly_readonly");
        let file_path = dir.join("readonly.txt");
        File::create(&file_path).unwrap();

        // Set file to read-only
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&file_path, perms).unwrap();

        let readonly = is_readonly(&file_path).unwrap();
        assert!(readonly);

        // Restore write permissions for cleanup
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&file_path, perms).unwrap();

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_is_readonly_nonexistent() {
        let result = is_readonly(Path::new("/nonexistent/path/file.txt"));
        assert!(result.is_err());
    }

    // ==================== get_modified_time tests ====================

    #[test]
    fn test_get_modified_time_exists() {
        let dir = setup_test_dir("modified_exists");
        let file_path = dir.join("test.txt");
        File::create(&file_path).unwrap();

        let result = get_modified_time(&file_path);
        assert!(result.is_ok());

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_modified_time_recent() {
        let dir = setup_test_dir("modified_recent");
        let file_path = dir.join("recent.txt");
        File::create(&file_path).unwrap();

        let modified = get_modified_time(&file_path).unwrap();
        let now = std::time::SystemTime::now();
        let elapsed = now.duration_since(modified).unwrap();

        // File should have been modified within the last few seconds
        assert!(elapsed.as_secs() < 10);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_get_modified_time_nonexistent() {
        let result = get_modified_time(Path::new("/nonexistent/path/file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_modified_time_after_write() {
        let dir = setup_test_dir("modified_after_write");
        let file_path = dir.join("test.txt");
        File::create(&file_path).unwrap();

        let time1 = get_modified_time(&file_path).unwrap();

        // Small delay to ensure different timestamp
        thread::sleep(Duration::from_millis(100));

        // Modify the file
        let mut file = fs::OpenOptions::new().write(true).open(&file_path).unwrap();
        file.write_all(b"new content").unwrap();
        drop(file);

        let time2 = get_modified_time(&file_path).unwrap();

        // time2 should be >= time1 (could be equal on low-resolution filesystems)
        assert!(time2 >= time1);

        cleanup_test_dir(&dir);
    }

    // ==================== was_modified_within tests ====================

    #[test]
    fn test_was_modified_within_recent() {
        let dir = setup_test_dir("within_recent");
        let file_path = dir.join("recent.txt");
        File::create(&file_path).unwrap();

        // File was just created, should be within the last hour
        let result = was_modified_within(&file_path, Duration::from_secs(3600)).unwrap();
        assert!(result);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_was_modified_within_very_short_duration() {
        let dir = setup_test_dir("within_short");
        let file_path = dir.join("test.txt");
        File::create(&file_path).unwrap();

        // Wait a bit
        thread::sleep(Duration::from_millis(200));

        // Check with a very short duration (less than the wait)
        let result = was_modified_within(&file_path, Duration::from_millis(50)).unwrap();
        assert!(!result);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_was_modified_within_exact_timing() {
        let dir = setup_test_dir("within_exact");
        let file_path = dir.join("test.txt");
        File::create(&file_path).unwrap();

        // File was just created, should be within the last minute
        let result = was_modified_within(&file_path, Duration::from_secs(60)).unwrap();
        assert!(result);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_was_modified_within_nonexistent() {
        let result =
            was_modified_within(Path::new("/nonexistent/file.txt"), Duration::from_secs(60));
        assert!(result.is_err());
    }

    // ==================== is_executable tests ====================

    #[test]
    #[cfg(unix)]
    fn test_is_executable_regular_file() {
        let dir = setup_test_dir("exec_regular");
        let file_path = dir.join("regular.txt");
        File::create(&file_path).unwrap();

        // Regular files typically aren't executable by default
        let result = is_executable(&file_path).unwrap();
        assert!(!result);

        cleanup_test_dir(&dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_is_executable_with_execute_permission() {
        use std::os::unix::fs::PermissionsExt;

        let dir = setup_test_dir("exec_permission");
        let file_path = dir.join("script.sh");
        File::create(&file_path)
            .unwrap()
            .write_all(b"#!/bin/bash\necho hello")
            .unwrap();

        // Set execute permission
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&file_path, perms).unwrap();

        let result = is_executable(&file_path).unwrap();
        assert!(result);

        cleanup_test_dir(&dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_is_executable_owner_only() {
        use std::os::unix::fs::PermissionsExt;

        let dir = setup_test_dir("exec_owner");
        let file_path = dir.join("script.sh");
        File::create(&file_path).unwrap();

        // Set execute permission for owner only
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o700);
        fs::set_permissions(&file_path, perms).unwrap();

        let result = is_executable(&file_path).unwrap();
        assert!(result);

        cleanup_test_dir(&dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_is_executable_directory() {
        let dir = setup_test_dir("exec_dir");
        let subdir = dir.join("subdir");
        fs::create_dir(&subdir).unwrap();

        // Directories aren't "executable" in the program sense
        let result = is_executable(&subdir).unwrap();
        assert!(!result);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_is_executable_nonexistent() {
        let result = is_executable(Path::new("/nonexistent/path/script.sh"));
        assert!(result.is_err());
    }

    // ==================== compare_modified_times tests ====================

    #[test]
    fn test_compare_modified_times_same_time() {
        let dir = setup_test_dir("compare_same");
        let file_path = dir.join("test.txt");
        File::create(&file_path).unwrap();

        // Compare file with itself
        let result = compare_modified_times(&file_path, &file_path).unwrap();
        assert_eq!(result, Ordering::Equal);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_compare_modified_times_different_files() {
        let dir = setup_test_dir("compare_different");
        let file1 = dir.join("first.txt");
        File::create(&file1).unwrap();

        // Small delay to ensure different timestamp
        thread::sleep(Duration::from_millis(100));

        let file2 = dir.join("second.txt");
        File::create(&file2).unwrap();

        let result = compare_modified_times(&file1, &file2).unwrap();
        // file1 was created first, so it should be Less (older)
        assert_eq!(result, Ordering::Less);

        // Reverse comparison
        let result_reverse = compare_modified_times(&file2, &file1).unwrap();
        assert_eq!(result_reverse, Ordering::Greater);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_compare_modified_times_after_modification() {
        let dir = setup_test_dir("compare_modified");
        let file1 = dir.join("file1.txt");
        let file2 = dir.join("file2.txt");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        thread::sleep(Duration::from_millis(100));

        // Modify file1 to make it newer
        let mut file = fs::OpenOptions::new().write(true).open(&file1).unwrap();
        file.write_all(b"modified").unwrap();
        drop(file);

        let result = compare_modified_times(&file1, &file2).unwrap();
        // file1 was modified more recently
        assert_eq!(result, Ordering::Greater);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_compare_modified_times_first_nonexistent() {
        let dir = setup_test_dir("compare_noexist1");
        let file_path = dir.join("exists.txt");
        File::create(&file_path).unwrap();

        let result = compare_modified_times(Path::new("/nonexistent"), &file_path);
        assert!(result.is_err());

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_compare_modified_times_second_nonexistent() {
        let dir = setup_test_dir("compare_noexist2");
        let file_path = dir.join("exists.txt");
        File::create(&file_path).unwrap();

        let result = compare_modified_times(&file_path, Path::new("/nonexistent"));
        assert!(result.is_err());

        cleanup_test_dir(&dir);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_integration_file_lifecycle() {
        let dir = setup_test_dir("integration_lifecycle");
        let file_path = dir.join("lifecycle.txt");

        // Create file
        let content = "Initial content";
        File::create(&file_path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();

        // Check initial state
        assert_eq!(get_file_size(&file_path).unwrap(), content.len() as u64);
        assert_eq!(get_file_type(&file_path).unwrap(), "file");
        assert!(!is_readonly(&file_path).unwrap());
        assert!(was_modified_within(&file_path, Duration::from_secs(60)).unwrap());

        // Modify file
        thread::sleep(Duration::from_millis(100));
        let new_content = "Modified content with more data";
        fs::write(&file_path, new_content).unwrap();

        // Check after modification
        assert_eq!(get_file_size(&file_path).unwrap(), new_content.len() as u64);
        assert!(was_modified_within(&file_path, Duration::from_secs(60)).unwrap());

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_integration_directory_structure() {
        let dir = setup_test_dir("integration_dir_structure");

        // Create files and directories
        let subdir = dir.join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file1 = dir.join("file1.txt");
        let file2 = subdir.join("file2.txt");

        File::create(&file1)
            .unwrap()
            .write_all(b"content1")
            .unwrap();
        thread::sleep(Duration::from_millis(100));
        File::create(&file2)
            .unwrap()
            .write_all(b"content2")
            .unwrap();

        // Check types
        assert_eq!(get_file_type(&dir).unwrap(), "directory");
        assert_eq!(get_file_type(&subdir).unwrap(), "directory");
        assert_eq!(get_file_type(&file1).unwrap(), "file");
        assert_eq!(get_file_type(&file2).unwrap(), "file");

        // Compare times
        let ordering = compare_modified_times(&file1, &file2).unwrap();
        assert_eq!(ordering, Ordering::Less);

        cleanup_test_dir(&dir);
    }

    #[test]
    fn test_integration_multiple_file_comparison() {
        let dir = setup_test_dir("integration_multi");

        let files: Vec<_> = (0..5)
            .map(|i| {
                let path = dir.join(format!("file{}.txt", i));
                File::create(&path).unwrap();
                thread::sleep(Duration::from_millis(50));
                path
            })
            .collect();

        // Files should be in chronological order
        for i in 0..files.len() - 1 {
            let ordering = compare_modified_times(&files[i], &files[i + 1]).unwrap();
            assert_eq!(
                ordering,
                Ordering::Less,
                "File {} should be older than file {}",
                i,
                i + 1
            );
        }

        cleanup_test_dir(&dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_integration_permissions_workflow() {
        use std::os::unix::fs::PermissionsExt;

        let dir = setup_test_dir("integration_perms");
        let file_path = dir.join("script.sh");

        // Create non-executable file
        File::create(&file_path)
            .unwrap()
            .write_all(b"#!/bin/bash\necho hello")
            .unwrap();

        assert!(!is_executable(&file_path).unwrap());
        assert!(!is_readonly(&file_path).unwrap());

        // Make executable
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&file_path, perms).unwrap();

        assert!(is_executable(&file_path).unwrap());

        // Make read-only
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&file_path, perms).unwrap();

        assert!(is_readonly(&file_path).unwrap());

        // Restore for cleanup
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&file_path, perms).unwrap();

        cleanup_test_dir(&dir);
    }
}
