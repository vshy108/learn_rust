// Demonstrates `unreachable!()` and the wildcard `_` pattern in match.
//
// `unreachable!()` is a macro that panics with "internal error: entered
// unreachable code". Use it to mark code paths that should never execute.
// The compiler trusts you — if the path IS reached, the program panics.
//
// `_` in match is the wildcard (catch-all) pattern. Combined together,
// `_ => unreachable!()` says "this arm should never match."
//
// Run: cargo run --example s05_unreachable_match

/// Classify a score into a letter grade.
///
/// Precondition: score must be 0..=100.
/// Violating this precondition triggers `unreachable!()`.
fn grade(score: u8) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        // u8 can be 0..=255, so the compiler requires handling 101..=255.
        // We know valid scores are 0..=100, so this should never happen.
        _ => unreachable!("score must be 0..=100, got {score}"),
    }
}

/// Demonstrate that match must be exhaustive.
///
/// For enums, Rust checks at compile time. For integers, `_` is needed.
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn describe_direction(dir: &Direction) -> &'static str {
    match dir {
        Direction::North => "heading north",
        Direction::South => "heading south",
        Direction::East => "heading east",
        Direction::West => "heading west",
        // No `_` needed — all variants are covered!
        // Adding a new variant without updating this match = compile error.
    }
}

fn main() {
    // --- 1. unreachable! in match ---
    println!("85 => {}", grade(85));
    println!("92 => {}", grade(92));
    println!("45 => {}", grade(45));

    // Calling grade(150) would panic:
    //   "internal error: entered unreachable code: score must be 0..=100, got 150"

    // --- 2. Exhaustive enum matching (no _ needed) ---
    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for dir in &dirs {
        println!("{dir:?} => {}", describe_direction(dir));
    }

    // --- 3. Using _ to ignore specific values ---
    let x = 42;
    match x {
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("something else: {x}"),
    }

    // --- 4. Using _ in destructuring ---
    // `_` can ignore parts of a pattern:
    let point = (10, 20, 30);
    let (x, _, z) = point; // ignore the y coordinate
    println!("x={x}, z={z}");

    // --- 5. unreachable! vs todo! vs unimplemented! ---
    // `unreachable!()` — code should never reach here; panics if it does.
    // `todo!()`         — placeholder for unfinished code; panics if reached.
    // `unimplemented!()` — like todo but signals "deliberately not done".
    // All three return `!` (the never type) so they satisfy any return type.

    // --- 6. Gotcha: unreachable! is NOT checked at compile time ---
    // The compiler does NOT verify that `unreachable!()` is truly unreachable.
    // It's a runtime assertion that always panics.
    // If your logic has a bug and the arm IS reached, the program panics.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_a() {
        assert_eq!(grade(95), "A");
        assert_eq!(grade(100), "A");
    }

    #[test]
    fn test_grade_b() {
        assert_eq!(grade(85), "B");
    }

    #[test]
    fn test_grade_f() {
        assert_eq!(grade(0), "F");
        assert_eq!(grade(59), "F");
    }

    #[test]
    #[should_panic(expected = "score must be 0..=100")]
    fn test_grade_out_of_range_panics() {
        grade(150);
    }

    #[test]
    fn test_direction_exhaustive() {
        // All four directions should produce non-empty descriptions.
        let dirs = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        for dir in &dirs {
            let desc = describe_direction(dir);
            assert!(!desc.is_empty());
        }
    }
}
