// Demonstrates implementing the Drop trait.
//
// `Drop` lets you run cleanup code when a value goes out of scope.
// It's deterministic — the compiler calls `drop()` automatically.
//
// Run: cargo run --example s18_drop_trait

struct TempFile {
    name: String,
}

impl TempFile {
    fn new(name: &str) -> Self {
        println!("  Created temp file: {name}");
        Self {
            name: name.to_string(),
        }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // This runs automatically when the value goes out of scope.
        // In real code, you'd delete the file here.
        println!("  Dropping temp file: {}", self.name);
    }
}

struct DatabaseConnection {
    url: String,
}

impl DatabaseConnection {
    fn new(url: &str) -> Self {
        println!("  Connected to: {url}");
        Self {
            url: url.to_string(),
        }
    }

    fn query(&self, sql: &str) -> String {
        format!("Result from {}: {sql}", self.url)
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("  Disconnected from: {}", self.url);
    }
}

fn main() {
    println!("=== Drop order is reverse of creation ===");
    {
        let _f1 = TempFile::new("a.tmp");
        let _f2 = TempFile::new("b.tmp");
        let _f3 = TempFile::new("c.tmp");
        println!("  --- end of scope ---");
        // Drop order: c, b, a (LIFO — last in, first out)
    }

    println!();
    println!("=== Drop with database ===");
    {
        let db = DatabaseConnection::new("postgres://localhost/mydb");
        println!("  {}", db.query("SELECT 1"));
        // db.drop() called automatically here
    }

    println!();
    println!("=== Early drop with std::mem::drop() ===");
    {
        let f = TempFile::new("early.tmp");
        println!("  Using file...");
        // Force early cleanup (consumes the value):
        drop(f);
        println!("  File already dropped!");
        // f is no longer accessible here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_runs() {
        // Just verify no panic during drop:
        let _f = TempFile::new("test.tmp");
    }

    #[test]
    fn test_early_drop() {
        let f = TempFile::new("early_test.tmp");
        drop(f);
        // No panic = success
    }
}
