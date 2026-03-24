// Demonstrates the `crate` prefix for absolute paths within a crate.
//
// `crate::` refers to the root of the current crate.
// It's like `/` in a file system — an absolute path from the top.
//
// Path types:
// - `crate::module::item` — absolute path from crate root
// - `self::item` — relative to current module
// - `super::item` — relative to parent module
// - `module::item` — relative path
//
// Run: cargo run --example s14_crate_prefix

mod config {
    pub const MAX_RETRIES: u32 = 3;

    pub fn default_timeout() -> u64 {
        30
    }
}

mod network {
    // Using crate:: to refer to items at the crate root.
    // In a single-file example, the "crate root" is this file.
    pub fn connect() -> String {
        let retries = crate::config::MAX_RETRIES;
        let timeout = crate::config::default_timeout();
        format!("Connecting with {retries} retries, {timeout}s timeout")
    }

    pub mod http {
        pub fn get(url: &str) -> String {
            // Absolute path from crate root:
            let timeout = crate::config::default_timeout();
            format!("GET {url} (timeout: {timeout}s)")
        }
    }
}

fn main() {
    // --- 1. Using absolute paths ---
    println!("MAX_RETRIES = {}", crate::config::MAX_RETRIES);
    println!("timeout = {}", crate::config::default_timeout());

    // --- 2. Module using crate:: internally ---
    println!("{}", network::connect());
    println!("{}", network::http::get("https://example.com"));

    // --- 3. When to use crate:: ---
    // Use crate:: when:
    // - You want an absolute/unambiguous path
    // - You're deep in a module hierarchy and want to reach the root
    // - You want to distinguish local modules from external crates
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_prefix() {
        assert_eq!(crate::config::MAX_RETRIES, 3);
        assert_eq!(crate::config::default_timeout(), 30);
    }

    #[test]
    fn test_network() {
        let msg = crate::network::connect();
        assert!(msg.contains("3 retries"));
    }
}
