// Demonstrates implementing the Debug trait.
//
// `Debug` provides programmer-facing output with `{:?}` and `{:#?}`.
// Usually derived, but you can implement it manually for custom formatting.
//
// Run: cargo run --example s18_debug_trait

use std::fmt;

// --- 1. Derived Debug (most common) ---
#[derive(Debug)]
#[allow(dead_code)] // fields used only via Debug formatting
struct AutoDebug {
    name: String,
    value: i32,
}

// --- 2. Manual Debug for custom formatting ---
struct Password {
    user: String,
    // We store the hash but intentionally hide it from Debug output.
    #[allow(dead_code)]
    hash: String,
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NEVER show the actual hash in debug output!
        f.debug_struct("Password")
            .field("user", &self.user)
            .field("hash", &"[REDACTED]")
            .finish()
    }
}

// --- 3. Debug for enums ---
enum Command {
    Start,
    Stop,
    SetSpeed(u32),
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Start => write!(f, "CMD:START"),
            Command::Stop => write!(f, "CMD:STOP"),
            Command::SetSpeed(s) => write!(f, "CMD:SPEED({s})"),
        }
    }
}

fn main() {
    let a = AutoDebug {
        name: "test".into(),
        value: 42,
    };
    println!("Derived:  {a:?}");
    println!("Pretty:   {a:#?}");

    println!();
    let p = Password {
        user: "admin".into(),
        hash: "abc123secret".into(),
    };
    println!("Password: {p:?}");
    // Notice: hash is redacted!

    println!();
    let cmds = vec![Command::Start, Command::SetSpeed(100), Command::Stop];
    for cmd in &cmds {
        println!("{cmd:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_debug() {
        let a = AutoDebug {
            name: "x".into(),
            value: 1,
        };
        let s = format!("{a:?}");
        assert!(s.contains("AutoDebug"));
    }

    #[test]
    fn test_password_redacted() {
        let p = Password {
            user: "u".into(),
            hash: "secret".into(),
        };
        let s = format!("{p:?}");
        assert!(s.contains("REDACTED"));
        assert!(!s.contains("secret"));
    }

    #[test]
    fn test_command_debug() {
        assert_eq!(format!("{:?}", Command::Start), "CMD:START");
        assert_eq!(format!("{:?}", Command::SetSpeed(50)), "CMD:SPEED(50)");
    }
}
