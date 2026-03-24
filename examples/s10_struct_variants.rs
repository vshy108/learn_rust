// Demonstrates struct variants in enums.
//
// Enum variants can have named fields, just like structs:
//
//   enum Event {
//       Click { x: i32, y: i32 },
//       KeyPress { key: char, ctrl: bool },
//   }
//
// This is called a "struct variant" — it's like embedding a struct
// inside the enum variant.
//
// Run: cargo run --example s10_struct_variants

#[derive(Debug)]
enum HttpRequest {
    Get {
        url: String,
    },
    Post {
        url: String,
        body: String,
        content_type: String,
    },
    Delete {
        url: String,
        id: u64,
    },
}

impl HttpRequest {
    /// Returns the URL of the request.
    fn url(&self) -> &str {
        match self {
            HttpRequest::Get { url }
            | HttpRequest::Post { url, .. }
            | HttpRequest::Delete { url, .. } => url,
        }
    }

    /// Returns a human-readable description.
    fn describe(&self) -> String {
        match self {
            HttpRequest::Get { url } => format!("GET {url}"),
            HttpRequest::Post {
                url, content_type, ..
            } => {
                format!("POST {url} [{content_type}]")
            }
            HttpRequest::Delete { url, id } => {
                format!("DELETE {url} (id={id})")
            }
        }
    }
}

fn main() {
    let requests = vec![
        HttpRequest::Get {
            url: "/api/users".to_string(),
        },
        HttpRequest::Post {
            url: "/api/users".to_string(),
            body: r#"{"name":"Alice"}"#.to_string(),
            content_type: "application/json".to_string(),
        },
        HttpRequest::Delete {
            url: "/api/users".to_string(),
            id: 42,
        },
    ];

    for req in &requests {
        println!("{}", req.describe());
        println!("  url: {}", req.url());
    }

    // --- Destructure struct variant ---
    if let HttpRequest::Post {
        body, content_type, ..
    } = &requests[1]
    {
        println!("\nPOST body ({content_type}): {body}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_get() {
        let r = HttpRequest::Get {
            url: "/test".into(),
        };
        assert_eq!(r.url(), "/test");
    }

    #[test]
    fn test_url_post() {
        let r = HttpRequest::Post {
            url: "/api".into(),
            body: "data".into(),
            content_type: "text/plain".into(),
        };
        assert_eq!(r.url(), "/api");
    }

    #[test]
    fn test_describe_get() {
        let r = HttpRequest::Get {
            url: "/path".into(),
        };
        assert_eq!(r.describe(), "GET /path");
    }

    #[test]
    fn test_describe_delete() {
        let r = HttpRequest::Delete {
            url: "/x".into(),
            id: 5,
        };
        assert_eq!(r.describe(), "DELETE /x (id=5)");
    }
}
