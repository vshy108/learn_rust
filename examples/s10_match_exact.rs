// Demonstrates match with exact values and bindings.
//
// You can match on exact literals and bind the matched value to a variable.
//
// Run: cargo run --example s10_match_exact

#[derive(Debug)]
enum HttpStatus {
    Ok,
    NotFound,
    ServerError,
    Custom(u16),
}

impl HttpStatus {
    /// Returns the numeric status code.
    fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::ServerError => 500,
            HttpStatus::Custom(code) => *code, // bind the inner value
        }
    }

    /// Returns a description string.
    fn description(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::ServerError => "Internal Server Error",
            HttpStatus::Custom(_) => "Custom Status",
        }
    }
}

fn main() {
    // --- 1. Match on exact enum variants ---
    let statuses = [
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::ServerError,
        HttpStatus::Custom(301),
        HttpStatus::Custom(418),
    ];

    for status in &statuses {
        println!("{} — {}", status.code(), status.description());
    }

    // --- 2. Match exact integer values ---
    let http_code: u16 = 404;
    let msg = match http_code {
        200 => "OK",
        301 => "Moved Permanently",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };
    println!("{http_code}: {msg}");

    // --- 3. Match with variable binding using @ ---
    let code: u16 = 418;
    match code {
        200 => println!("Success"),
        c @ 400..=499 => println!("Client error: {c}"),
        c @ 500..=599 => println!("Server error: {c}"),
        other => println!("Other: {other}"),
    }

    // --- 4. Matching exact strings ---
    let command = "quit";
    match command {
        "help" => println!("Showing help"),
        "quit" | "exit" => println!("Goodbye!"),
        "version" => println!("v1.0"),
        _ => println!("Unknown command: {command}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes() {
        assert_eq!(HttpStatus::Ok.code(), 200);
        assert_eq!(HttpStatus::NotFound.code(), 404);
        assert_eq!(HttpStatus::Custom(301).code(), 301);
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(HttpStatus::Ok.description(), "OK");
        assert_eq!(HttpStatus::Custom(999).description(), "Custom Status");
    }

    #[test]
    fn test_at_binding() {
        let code: u16 = 403;
        let is_client_error = matches!(code, 400..=499);
        assert!(is_client_error);
    }
}
