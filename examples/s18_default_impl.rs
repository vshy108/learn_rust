// Demonstrates default implementations in traits.
//
// A trait method CAN have a body — that's the default implementation.
// Types may override it or use the default.
//
// Run: cargo run --example s18_default_impl

/// A trait with both required and default methods.
trait Summary {
    /// Required: each type MUST provide this.
    fn title(&self) -> &str;

    /// Default implementation — types can override or keep it.
    fn summarize(&self) -> String {
        format!("{} (read more...)", self.title())
    }

    /// Another default that calls the required method.
    fn preview(&self) -> String {
        format!("[Preview] {}", self.title())
    }
}

struct Article {
    title: String,
    content: String,
}

struct Tweet {
    username: String,
    #[allow(dead_code)] // present for realism, not used in trait methods
    text: String,
}

impl Summary for Article {
    fn title(&self) -> &str {
        &self.title
    }

    // Override the default summarize:
    fn summarize(&self) -> String {
        let preview = if self.content.len() > 30 {
            &self.content[..30]
        } else {
            &self.content
        };
        format!("{}: {}...", self.title, preview)
    }
}

impl Summary for Tweet {
    fn title(&self) -> &str {
        &self.username
    }
    // Uses default summarize() and preview()
}

fn main() {
    let article = Article {
        title: "Rust 2024 Released".into(),
        content: "The Rust team announced the 2024 edition with many improvements.".into(),
    };

    let tweet = Tweet {
        username: "@rustlang".into(),
        text: "Rust is great!".into(),
    };

    // Article overrides summarize:
    println!("{}", article.summarize());
    println!("{}", article.preview());

    // Tweet uses default summarize:
    println!("{}", tweet.summarize());
    println!("{}", tweet.preview());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_overrides() {
        let a = Article {
            title: "T".into(),
            content: "C".into(),
        };
        assert!(a.summarize().contains("T"));
        assert!(a.summarize().contains("C"));
    }

    #[test]
    fn test_tweet_defaults() {
        let t = Tweet {
            username: "u".into(),
            text: "t".into(),
        };
        assert!(t.summarize().contains("read more"));
        assert!(t.preview().contains("[Preview]"));
    }
}
