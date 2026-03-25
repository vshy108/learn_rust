// Section 1, Topic 5: fmt, build, and run
//
// In Python, you might use: black (formatter), mypy (type-check), python file.py (run).
// In Rust, the equivalent workflow uses `cargo` subcommands:
//
//   cargo fmt    — auto-formats code (like Python's `black` or `ruff format`)
//   cargo build  — compiles the project (type-checks + generates binary)
//   cargo run    — builds AND runs in one step
//   cargo check  — fast type-check without producing a binary (like mypy)
//   cargo clippy — linter with style/performance suggestions (like ruff/pylint)
//
// This example demonstrates code that `cargo fmt` would reformat,
// and shows how to programmatically check formatting rules.

/// Describe the core Cargo commands for a Rust workflow.
///
/// # Returns
///
/// A vector of (command, description) pairs.
fn cargo_commands() -> Vec<(&'static str, &'static str)> {
    vec![
        ("cargo fmt", "Auto-format code (like black/ruff format)"),
        (
            "cargo check",
            "Fast type-check, no binary output (like mypy)",
        ),
        ("cargo build", "Compile to binary (type-check + codegen)"),
        ("cargo run", "Build + run in one step (like python main.py)"),
        (
            "cargo clippy",
            "Lint for style/perf issues (like ruff/pylint)",
        ),
        ("cargo test", "Run all tests (like pytest)"),
    ]
}

/// Check if a line exceeds the recommended line length (100 chars).
///
/// # Arguments
///
/// * `line` - The source code line to check.
///
/// # Returns
///
/// `true` if the line is within the 100-character limit.
fn is_within_line_limit(line: &str) -> bool {
    // rustfmt default max line width is 100 characters.
    // In Python, PEP 8 recommends 79 (or 88 for black).
    line.len() <= 100
}

/// Check if indentation uses spaces (not tabs).
///
/// # Arguments
///
/// * `line` - The source code line to check.
///
/// # Returns
///
/// `true` if the line does not start with a tab character.
fn uses_spaces_not_tabs(line: &str) -> bool {
    // Rust convention: 4 spaces for indentation (enforced by rustfmt).
    // Unlike Python (which also uses spaces), tabs will cause rustfmt to
    // reformat your code.
    !line.starts_with('\t')
}

fn main() {
    println!("=== Rust Development Workflow: fmt, build, and run ===\n");

    println!("Core Cargo commands (Python equivalents in parentheses):\n");
    for (command, description) in cargo_commands() {
        // {command:20} is a width format specifier:
        //   - Prints `command` in a field at least 20 characters wide.
        //   - If `command` is shorter than 20 chars, it is padded with spaces
        //     on the right (strings are left-aligned by default).
        //   - If `command` is 20+ chars, no truncation occurs; it prints fully.
        //   - This keeps the descriptions aligned in a neat column.
        //   - Other alignments: {:>20} (right), {:^20} (center).
        //   - Custom fill char: {:-<20} pads with dashes instead of spaces.
        println!("  {command:20} {description}");
    }

    println!("\n--- Formatting Rules ---\n");

    let good_line = "    let x = 42; // 4-space indent, short line";
    let tab_line = "\tlet x = 42; // oops, tab indent";
    let long_line = "x".repeat(101);

    println!(
        "  Good line within limit: {}",
        is_within_line_limit(good_line)
    );
    println!(
        "  Long line within limit: {}",
        is_within_line_limit(&long_line)
    );
    println!(
        "  Good line uses spaces:  {}",
        uses_spaces_not_tabs(good_line)
    );
    println!(
        "  Tab line uses spaces:   {}",
        uses_spaces_not_tabs(tab_line)
    );

    println!("\nTIP: Run `cargo fmt` before every commit to auto-fix formatting.");
    println!("TIP: Run `cargo clippy -- -D warnings` to catch lint issues.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_commands_has_six_entries() {
        assert_eq!(cargo_commands().len(), 6);
    }

    #[test]
    fn test_line_limit_accepts_short_lines() {
        assert!(is_within_line_limit("let x = 42;"));
    }

    #[test]
    fn test_line_limit_accepts_exactly_100_chars() {
        let line = "x".repeat(100);
        assert!(is_within_line_limit(&line));
    }

    #[test]
    fn test_line_limit_rejects_101_chars() {
        let line = "x".repeat(101);
        assert!(!is_within_line_limit(&line));
    }

    #[test]
    fn test_spaces_accepted() {
        assert!(uses_spaces_not_tabs("    let x = 1;"));
    }

    #[test]
    fn test_tabs_rejected() {
        assert!(!uses_spaces_not_tabs("\tlet x = 1;"));
    }

    #[test]
    fn test_empty_line_is_valid() {
        assert!(is_within_line_limit(""));
        assert!(uses_spaces_not_tabs(""));
    }
}
