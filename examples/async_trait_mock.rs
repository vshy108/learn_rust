// Demonstrates the "happy path" for async traits + mocking in modern Rust.
//
// Before Rust 1.75, async fn in traits required the `#[async_trait]` macro,
// which boxed the returned future. That macro didn't play well with `mockall`.
// Now, async fn in traits is native — no macro needed, and mockall supports it.
//
// The happy path:
//   1. Write a struct              → be happy
//   2. Extract a trait              → dependency inject
//   3. Mock with mockall            → test easily
//   4. Make it async                → no extra macros needed!

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Step 1 & 2: Define a trait with an async method (native, no macro needed!)
// ---------------------------------------------------------------------------
trait UserRepository: Send + Sync {
    /// Fetch a user by ID. Returns None if not found.
    async fn find_by_id(&self, id: u64) -> Result<Option<String>, String>;
}

// ---------------------------------------------------------------------------
// Step 3: A real implementation (e.g., backed by an in-memory HashMap)
// ---------------------------------------------------------------------------
struct InMemoryUserRepo {
    users: HashMap<u64, String>,
}

impl UserRepository for InMemoryUserRepo {
    async fn find_by_id(&self, id: u64) -> Result<Option<String>, String> {
        // In real code this could be an HTTP call or DB query — all async.
        Ok(self.users.get(&id).cloned())
    }
}

// ---------------------------------------------------------------------------
// Step 4: Business logic that depends on the trait (easy to test!)
// ---------------------------------------------------------------------------
async fn greet_user(repo: &impl UserRepository, id: u64) -> String {
    match repo.find_by_id(id).await {
        Ok(Some(name)) => format!("Hello, {}!", name),
        Ok(None) => "User not found.".to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

// ---------------------------------------------------------------------------
// main: run the real implementation
// ---------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    let mut users = HashMap::new();
    users.insert(1, "Alice".to_string());
    users.insert(2, "Bob".to_string());

    let repo = InMemoryUserRepo { users };

    println!("{}", greet_user(&repo, 1).await); // Hello, Alice!
    println!("{}", greet_user(&repo, 99).await); // User not found.
}

// ---------------------------------------------------------------------------
// Step 5: Tests with mockall — async mocks just work!
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use mockall::automock;

    // Re-declare the trait with #[automock] for test builds.
    // (In a real project you'd put #[cfg_attr(test, automock)] on the original.)
    #[automock]
    trait UserRepository: Send + Sync {
        async fn find_by_id(&self, id: u64) -> Result<Option<String>, String>;
    }

    // Helper that accepts the mock's trait (same signature as the real one)
    async fn greet_user_mock(repo: &MockUserRepository, id: u64) -> String {
        match repo.find_by_id(id).await {
            Ok(Some(name)) => format!("Hello, {}!", name),
            Ok(None) => "User not found.".to_string(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_greet_known_user() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .with(mockall::predicate::eq(1))
            .returning(|_| Ok(Some("Alice".to_string())));

        assert_eq!(greet_user_mock(&mock, 1).await, "Hello, Alice!");
    }

    #[tokio::test]
    async fn test_greet_unknown_user() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id().returning(|_| Ok(None));

        assert_eq!(greet_user_mock(&mock, 42).await, "User not found.");
    }

    #[tokio::test]
    async fn test_greet_error() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .returning(|_| Err("connection timeout".to_string()));

        assert_eq!(greet_user_mock(&mock, 1).await, "Error: connection timeout");
    }
}
