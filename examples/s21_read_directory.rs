// Demonstrates reading a directory with iterators.
//
// `std::fs::read_dir()` returns an iterator over directory entries.
//
// Run: cargo run --example s21_read_directory

use std::fs;

fn main() {
    let dir = "."; // current directory

    println!("=== Reading Directory: {dir} ===");

    // --- 1. Basic directory listing ---
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => println!("  {}", e.path().display()),
                    Err(e) => println!("  error: {e}"),
                }
            }
        }
        Err(e) => println!("Failed to read dir: {e}"),
    }

    // --- 2. Collect paths with filter ---
    let rs_files: Vec<String> = fs::read_dir("examples")
        .into_iter()
        .flatten() // unwrap the Result<ReadDir>
        .flatten() // unwrap each Result<DirEntry>
        .filter_map(|e| {
            let path = e.path();
            if path.extension().is_some_and(|ext| ext == "rs") {
                Some(path.display().to_string())
            } else {
                None
            }
        })
        .take(5) // just show first 5
        .collect();
    println!("\nFirst 5 .rs files in examples/:");
    for f in &rs_files {
        println!("  {f}");
    }

    // --- 3. Count entries ---
    let count = fs::read_dir("examples")
        .into_iter()
        .flatten()
        .flatten()
        .count();
    println!("\nTotal entries in examples/: {count}");
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_read_current_dir() {
        let entries: Vec<_> = fs::read_dir(".").expect("failed to read dir").collect();
        assert!(!entries.is_empty());
    }

    #[test]
    fn test_read_examples() {
        let count = fs::read_dir("examples")
            .into_iter()
            .flatten()
            .flatten()
            .count();
        assert!(count > 0);
    }
}
