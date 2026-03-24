// Demonstrates match catching multiple values with `|` (OR pattern).
//
// You can match several patterns in one arm using `|`:
//
//   match x {
//       1 | 2 | 3 => "small",
//       4..=6 => "medium",   // range pattern
//       _ => "large",
//   }
//
// Run: cargo run --example s10_match_multiple

#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// Returns a temperature description for the season.
fn temperature(season: &Season) -> &'static str {
    match season {
        // `|` combines multiple patterns in one arm
        Season::Spring | Season::Autumn => "mild",
        Season::Summer => "hot",
        Season::Winter => "cold",
    }
}

fn main() {
    // --- 1. Multiple enum variants in one arm ---
    let seasons = [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ];
    for s in &seasons {
        println!("{s:?} → {}", temperature(s));
    }

    // --- 2. Multiple integer values ---
    for x in 0..=10 {
        let label = match x {
            0 => "zero",
            1 | 2 | 3 => "small",
            4..=6 => "medium", // inclusive range pattern
            7 | 8 | 9 | 10 => "large",
            _ => "out of range",
        };
        println!("{x} → {label}");
    }

    // --- 3. Multiple char values ---
    let ch = 'e';
    let is_vowel = matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u');
    println!("'{ch}' is vowel: {is_vowel}");

    // --- 4. Combining with guards ---
    let num: i32 = -3;
    let desc = match num {
        0 => "zero",
        n if n > 0 => "positive",
        _ => "negative",
    };
    println!("{num} is {desc}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature() {
        assert_eq!(temperature(&Season::Spring), "mild");
        assert_eq!(temperature(&Season::Autumn), "mild");
        assert_eq!(temperature(&Season::Summer), "hot");
        assert_eq!(temperature(&Season::Winter), "cold");
    }

    #[test]
    fn test_vowel_check() {
        for ch in ['a', 'e', 'i', 'o', 'u'] {
            assert!(matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u'));
        }
        assert!(!matches!('x', 'a' | 'e' | 'i' | 'o' | 'u'));
    }
}
