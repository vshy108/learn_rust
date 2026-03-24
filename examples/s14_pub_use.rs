#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates using `pub use` to re-export names.
//
// `pub use` lets you re-export items from submodules,
// creating a cleaner public API. Users don't need to know
// your internal module structure.
//
// Run: cargo run --example s14_pub_use

mod models {
    mod user {
        #[derive(Debug)]
        pub struct User {
            pub name: String,
            pub email: String,
        }

        impl User {
            pub fn new(name: &str, email: &str) -> Self {
                Self {
                    name: name.to_string(),
                    email: email.to_string(),
                }
            }
        }
    }

    mod product {
        #[derive(Debug)]
        pub struct Product {
            pub name: String,
            pub price: f64,
        }
    }

    // --- Re-export so users access `models::User` not `models::user::User` ---
    pub use product::Product;
    pub use user::User;
}

fn main() {
    // --- 1. Without pub use, you'd need: models::user::User ---
    // With pub use, the flat path works:
    let user = models::User::new("Alice", "alice@example.com");
    println!("user: {user:?}");

    let product = models::Product {
        name: "Widget".into(),
        price: 9.99,
    };
    println!("product: {product:?}");

    // --- 2. Common pattern: prelude modules ---
    // Many libraries create a `prelude` module:
    mod my_lib {
        mod types {
            #[derive(Debug)]
            pub struct Config {
                pub name: String,
            }
        }
        mod utils {
            pub fn version() -> &'static str {
                "1.0"
            }
        }

        pub mod prelude {
            pub use super::types::Config;
            pub use super::utils::version;
        }
    }

    // Users just do: use my_lib::prelude::*;
    use my_lib::prelude::*;
    let cfg = Config { name: "app".into() };
    println!("{cfg:?}, version: {}", version());
}

#[cfg(test)]
mod tests {
    use super::models::{Product, User};

    #[test]
    fn test_pub_use_user() {
        let u = User::new("Bob", "bob@test.com");
        assert_eq!(u.name, "Bob");
    }

    #[test]
    fn test_pub_use_product() {
        let p = Product {
            name: "Gadget".into(),
            price: 19.99,
        };
        assert_eq!(p.name, "Gadget");
    }
}
