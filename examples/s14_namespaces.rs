// Demonstrates the benefit of namespaces via modules.
//
// Modules prevent name collisions by providing separate namespaces.
// Two modules can each have a function called `connect()` without conflict.
//
// Run: cargo run --example s14_namespaces

mod database {
    /// Connects to the database.
    pub fn connect() -> String {
        "Connected to database".to_string()
    }

    /// Executes a query.
    pub fn query(sql: &str) -> String {
        format!("DB query: {sql}")
    }
}

mod cache {
    /// Connects to the cache server.
    pub fn connect() -> String {
        "Connected to cache".to_string()
    }

    /// Gets a cached value.
    pub fn get(key: &str) -> Option<String> {
        if key == "user:1" {
            Some("cached_alice".to_string())
        } else {
            None
        }
    }
}

mod api {
    /// Connects to the external API.
    pub fn connect() -> String {
        "Connected to API".to_string()
    }
}

fn main() {
    // --- 1. No ambiguity — each connect() is namespaced ---
    println!("{}", database::connect());
    println!("{}", cache::connect());
    println!("{}", api::connect());

    // --- 2. Using full paths for clarity ---
    println!("{}", database::query("SELECT * FROM users"));
    println!("cache lookup: {:?}", cache::get("user:1"));
    println!("cache miss:   {:?}", cache::get("user:2"));

    // --- 3. use to bring one into scope ---
    use database::query;
    println!("{}", query("INSERT INTO logs VALUES (1)"));

    // --- 4. Aliasing to avoid conflicts ---
    use cache::connect as cache_connect;
    use database::connect as db_connect;
    println!("{}", db_connect());
    println!("{}", cache_connect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_name_collision() {
        let db = database::connect();
        let cache_conn = cache::connect();
        let api_conn = api::connect();
        assert_ne!(db, cache_conn);
        assert_ne!(cache_conn, api_conn);
    }
}
