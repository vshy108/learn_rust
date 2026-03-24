#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates creating aliases with the `as` keyword.
//
// `as` renames imports to avoid collisions or improve clarity.
//
// Run: cargo run --example s14_as_alias

// Two modules with same-named types:
mod io_module {
    #[derive(Debug)]
    pub struct Result {
        pub message: String,
    }
}

mod network_module {
    #[derive(Debug)]
    pub struct Result {
        pub status: u16,
        pub body: String,
    }
}

// --- Aliasing to avoid collision ---
use io_module::Result as IoResult;
use network_module::Result as NetworkResult;

fn main() {
    // --- 1. Use aliased types ---
    let io = IoResult {
        message: "file read ok".into(),
    };
    let net = NetworkResult {
        status: 200,
        body: "OK".into(),
    };
    println!("io: {io:?}");
    println!("net: {net:?}");

    // --- 2. Standard library example ---
    // std::io::Result and std::fmt::Result both exist.
    use std::fmt::Result as FmtResult;
    use std::io::Result as StdIoResult;
    let _f: FmtResult = Ok(());
    let _i: StdIoResult<()> = Ok(());

    // --- 3. Aliasing module imports ---
    use std::collections::HashMap as Map;
    let mut m = Map::new();
    m.insert("key", 42);
    println!("map: {m:?}");

    // --- 4. Aliasing functions ---
    use std::thread::sleep as wait;
    use std::time::Duration;
    println!("waiting 10ms...");
    wait(Duration::from_millis(10));
    println!("done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_result() {
        let r = IoResult {
            message: "ok".into(),
        };
        assert_eq!(r.message, "ok");
    }

    #[test]
    fn test_network_result() {
        let r = NetworkResult {
            status: 404,
            body: "not found".into(),
        };
        assert_eq!(r.status, 404);
    }
}
