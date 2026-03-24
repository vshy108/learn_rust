// Demonstrates opening a file with std::fs::File.
//
// File::open returns Result<File, io::Error>.
// You must handle the error case (file might not exist).
//
// Run: cargo run --example s17_open_file

use std::fs::File;
use std::io;

fn main() {
    // --- 1. Opening a file that exists ---
    match File::open("Cargo.toml") {
        Ok(file) => println!("Opened Cargo.toml: {file:?}"),
        Err(e) => eprintln!("Failed to open: {e}"),
    }

    // --- 2. Opening a file that doesn't exist ---
    match File::open("nonexistent.txt") {
        Ok(_) => println!("Opened (unexpected!)"),
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Kind: {:?}", e.kind());
        }
    }

    // --- 3. Matching on error kind ---
    match File::open("missing.txt") {
        Ok(_) => println!("Found!"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => eprintln!("File not found"),
            io::ErrorKind::PermissionDenied => eprintln!("Permission denied"),
            other => eprintln!("Other error: {other:?}"),
        },
    }

    // --- 4. Creating a file ---
    // File::create creates or truncates.
    // Using a temp path to be safe in examples.
    let temp_path = std::env::temp_dir().join("s17_test_file.txt");
    match File::create(&temp_path) {
        Ok(_) => {
            println!("Created: {}", temp_path.display());
            // Clean up
            let _ = std::fs::remove_file(&temp_path);
        }
        Err(e) => eprintln!("Failed to create: {e}"),
    }

    // --- 5. OpenOptions for more control ---
    use std::fs::OpenOptions;
    let result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(std::env::temp_dir().join("s17_open_options.txt"));
    match result {
        Ok(_) => {
            println!("Opened with options");
            let _ = std::fs::remove_file(std::env::temp_dir().join("s17_open_options.txt"));
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn test_open_existing() {
        let result = File::open("Cargo.toml");
        assert!(result.is_ok());
    }

    #[test]
    fn test_open_missing() {
        let result = File::open("definitely_does_not_exist_abc123.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_kind() {
        use std::io;
        let err = File::open("missing_xyz.txt").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
