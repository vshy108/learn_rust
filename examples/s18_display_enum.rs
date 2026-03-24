// Demonstrates implementing Display on an enum.
//
// Each variant can have its own display format.
//
// Run: cargo run --example s18_display_enum

use std::fmt;

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `match` with `write!` for each variant:
        match self {
            Season::Spring => write!(f, "Spring"),
            Season::Summer => write!(f, "Summer"),
            Season::Autumn => write!(f, "Autumn"),
            Season::Winter => write!(f, "Winter"),
        }
    }
}

/// An enum with data in variants:
enum HttpStatus {
    Ok,
    NotFound,
    ServerError(String),
    Custom(u16, String),
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
            HttpStatus::ServerError(msg) => write!(f, "500 Server Error: {msg}"),
            HttpStatus::Custom(code, msg) => write!(f, "{code} {msg}"),
        }
    }
}

/// Enum with a generic variant:
enum LogLevel {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        };
        write!(f, "[{label}]")
    }
}

fn main() {
    for season in [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ] {
        println!("Season: {season}");
    }

    println!();
    let statuses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::ServerError("disk full".into()),
        HttpStatus::Custom(418, "I'm a teapot".into()),
    ];
    for s in &statuses {
        println!("HTTP {s}");
    }

    println!();
    for level in [LogLevel::Info, LogLevel::Warning, LogLevel::Error] {
        println!("{level} Something happened");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_season_display() {
        assert_eq!(Season::Spring.to_string(), "Spring");
    }

    #[test]
    fn test_http_status() {
        assert_eq!(HttpStatus::Ok.to_string(), "200 OK");
        assert_eq!(HttpStatus::NotFound.to_string(), "404 Not Found");
    }

    #[test]
    fn test_custom_status() {
        let s = HttpStatus::Custom(418, "I'm a teapot".into());
        assert!(s.to_string().contains("418"));
    }

    #[test]
    fn test_log_level() {
        assert_eq!(LogLevel::Error.to_string(), "[ERROR]");
    }
}
