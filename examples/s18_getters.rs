// Demonstrates getters in traits.
//
// Getters provide read-only access to a type's data through a trait.
// This is how you define an interface for reading properties.
//
// Run: cargo run --example s18_getters

/// A trait defining read-only access to user properties.
trait UserInfo {
    fn username(&self) -> &str;
    fn email(&self) -> &str;
    fn age(&self) -> u32;

    /// Default method built on getters:
    fn summary(&self) -> String {
        format!("{} ({}, age {})", self.username(), self.email(), self.age())
    }
}

struct DbUser {
    name: String,
    email: String,
    age: u32,
}

struct ApiUser {
    handle: String,
    contact: String,
    birth_year: u32,
}

impl UserInfo for DbUser {
    fn username(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }
    fn age(&self) -> u32 {
        self.age
    }
}

impl UserInfo for ApiUser {
    fn username(&self) -> &str {
        &self.handle
    }
    fn email(&self) -> &str {
        &self.contact
    }
    fn age(&self) -> u32 {
        // Computed getter:
        2025 - self.birth_year
    }
}

/// Works with any UserInfo implementor.
fn print_user(user: &impl UserInfo) {
    println!("{}", user.summary());
}

fn main() {
    let db = DbUser {
        name: "alice".into(),
        email: "alice@db.com".into(),
        age: 30,
    };
    let api = ApiUser {
        handle: "bob_api".into(),
        contact: "bob@api.com".into(),
        birth_year: 1990,
    };

    print_user(&db);
    print_user(&api);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_user_getters() {
        let u = DbUser {
            name: "x".into(),
            email: "y".into(),
            age: 25,
        };
        assert_eq!(u.username(), "x");
        assert_eq!(u.email(), "y");
        assert_eq!(u.age(), 25);
    }

    #[test]
    fn test_api_user_computed_age() {
        let u = ApiUser {
            handle: "h".into(),
            contact: "c".into(),
            birth_year: 2000,
        };
        assert_eq!(u.age(), 25);
    }

    #[test]
    fn test_summary() {
        let u = DbUser {
            name: "z".into(),
            email: "z@z".into(),
            age: 1,
        };
        assert!(u.summary().contains("z"));
    }
}
