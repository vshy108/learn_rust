// Demonstrates using the Rust standard library.
//
// The standard library (`std`) is always available without adding it
// to Cargo.toml. The prelude auto-imports common types.
// For everything else, use `std::module::Type`.
//
// Run: cargo run --example s14_standard_library

use std::collections::HashMap;
use std::io;

fn main() {
    println!("=== Standard Library Highlights ===");
    println!();

    // --- 1. std::collections ---
    let mut map = HashMap::new();
    map.insert("Rust", 2015);
    map.insert("Python", 1991);
    println!("Languages: {map:?}");

    // --- 2. std::env ---
    let current_dir = std::env::current_dir().unwrap_or_default();
    println!("Current dir: {}", current_dir.display());

    // --- 3. std::fs (file system) ---
    // std::fs::read_to_string("file.txt") — read file
    // std::fs::write("file.txt", "content") — write file

    // --- 4. std::io ---
    // io::stdin() for reading input
    let _ = io::stdout(); // just demonstrating it's accessible
    println!("io::stdout() is accessible");

    // --- 5. std::path ---
    use std::path::Path;
    let p = Path::new("/usr/local/bin");
    println!("path parent: {:?}", p.parent());
    println!("path filename: {:?}", p.file_name());

    // --- 6. std::time ---
    use std::time::Instant;
    let start = Instant::now();
    let _sum: u64 = (0..1_000_000).sum();
    let elapsed = start.elapsed();
    println!("sum took: {elapsed:?}");

    // --- 7. std::fmt ---
    let formatted = format!("{:>10}", "right");
    println!("formatted: \"{formatted}\"");

    // --- 8. std::thread ---
    let handle = std::thread::spawn(|| 42);
    println!("thread result: {}", handle.join().unwrap());

    // --- 9. Key std modules ---
    println!();
    println!("Key std modules:");
    println!("  std::collections - HashMap, HashSet, BTreeMap, VecDeque");
    println!("  std::fs - file system operations");
    println!("  std::io - input/output");
    println!("  std::net - networking (TcpStream, UdpSocket)");
    println!("  std::path - filesystem paths");
    println!("  std::sync - Arc, Mutex, channels");
    println!("  std::thread - threading");
    println!("  std::time - Duration, Instant");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        assert_eq!(m.get("a"), Some(&1));
    }

    #[test]
    fn test_path() {
        use std::path::Path;
        let p = Path::new("/foo/bar.txt");
        assert_eq!(p.file_name().unwrap(), "bar.txt");
    }
}
