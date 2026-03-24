// Demonstrates creating struct instances.
//
// Field init shorthand: if a variable has the same name as a struct field,
// you can omit the `: value` part.
//
// Run: cargo run --example s09_create_instance

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u64,
}

/// Creates a new User with the given details.
///
/// # Arguments
///
/// * `username` - The user's display name
/// * `email` - The user's email address
///
/// # Returns
///
/// A new User instance with active=true and login_count=1
fn create_user(username: String, email: String) -> User {
    // Field init shorthand: `username` matches the field name.
    User {
        username, // shorthand for `username: username`
        email,    // shorthand for `email: email`
        active: true,
        login_count: 1,
    }
}

fn main() {
    // --- 1. Explicit field values ---
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        login_count: 0,
    };
    println!("user1: {user1:?}");

    // --- 2. Using a constructor function ---
    let user2 = create_user(String::from("bob"), String::from("bob@example.com"));
    println!("user2: {user2:?}");

    // --- 3. Field init shorthand ---
    let username = String::from("charlie");
    let email = String::from("charlie@example.com");
    let user3 = User {
        username, // shorthand
        email,    // shorthand
        active: false,
        login_count: 5,
    };
    println!("user3: {user3:?}");

    // --- 4. Order of fields doesn't matter ---
    let user4 = User {
        login_count: 100,
        email: String::from("d@example.com"),
        active: true,
        username: String::from("dave"),
    };
    println!("user4: {user4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let u = create_user(String::from("test"), String::from("t@t.com"));
        assert_eq!(u.username, "test");
        assert_eq!(u.email, "t@t.com");
        assert!(u.active);
        assert_eq!(u.login_count, 1);
    }

    #[test]
    fn test_field_init_shorthand() {
        let username = String::from("user");
        let email = String::from("email");
        let u = User {
            username,
            email,
            active: true,
            login_count: 0,
        };
        assert_eq!(u.username, "user");
    }
}
