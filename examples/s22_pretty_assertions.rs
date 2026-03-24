// Demonstrates the pretty_assertions crate.
//
// `pretty_assertions` replaces `assert_eq!` with a colorful diff output,
// making it much easier to see what went wrong (especially for large structs).
//
// Run: cargo test --example s22_pretty_assertions

// In tests, we import from the crate:
// use pretty_assertions::assert_eq;

#[derive(Debug, PartialEq)]
struct Config {
    name: String,
    version: u32,
    debug: bool,
    features: Vec<String>,
}

impl Config {
    fn default_config() -> Self {
        Self {
            name: "my_app".to_string(),
            version: 1,
            debug: false,
            features: vec!["logging".to_string(), "metrics".to_string()],
        }
    }
}

fn main() {
    let config = Config::default_config();
    println!("config: {config:?}");
    println!("Use `cargo test --example s22_pretty_assertions` to see pretty diffs.");
}

#[cfg(test)]
mod tests {
    use super::*;
    // This replaces the built-in assert_eq! with a colorful one
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_config() {
        let config = Config::default_config();
        assert_eq!(config.name, "my_app");
        assert_eq!(config.version, 1);
        assert_eq!(config.debug, false);
    }

    #[test]
    fn test_features() {
        let config = Config::default_config();
        assert_eq!(
            config.features,
            vec!["logging".to_string(), "metrics".to_string()]
        );
    }

    #[test]
    fn test_full_config() {
        let expected = Config {
            name: "my_app".to_string(),
            version: 1,
            debug: false,
            features: vec!["logging".to_string(), "metrics".to_string()],
        };
        assert_eq!(Config::default_config(), expected);
    }
}
