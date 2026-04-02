// LeetCode 13: Roman to Integer [Easy]
// https://leetcode.com/problems/roman-to-integer/
//
// Roman numerals: I=1, V=5, X=10, L=50, C=100, D=500, M=1000.
// Subtraction rule: when a smaller value precedes a larger one, subtract it.
//   IV=4, IX=9, XL=40, XC=90, CD=400, CM=900.
//
// Given a roman numeral, convert it to an integer.
//
// Examples:
//   s = "III"     -> 3
//   s = "LVIII"   -> 58
//   s = "MCMXCIV" -> 1994
//
// Constraints:
//   - 1 <= s.length <= 15
//   - s contains only ('I', 'V', 'X', 'L', 'C', 'D', 'M')
//   - s is a valid roman numeral in the range [1, 3999]
//
// Run:  cargo run --example lc_0013_roman_to_integer
// Test: cargo test --example lc_0013_roman_to_integer
// Bench: cargo test --example lc_0013_roman_to_integer bench -- --nocapture
//
// Benchmark (100_000 iterations, 2026-03-27):
//   My solution: 1.706000542s (avg 17.06µs)
//   AI solution: 79.084958ms  (avg 790ns)
//   -> AI solution is 21.57x faster
//   Reason: HashMap::from([...]) heap-allocates on every call — my solution pays that cost 100k times;
//   the match-based AI solution compiles to a zero-allocation jump table (one branch per character).
//
// --- Rust concepts used ---
// - HashMap::from([...]) inline init: convenient but heap-allocates on every call;
//   prefer match or const for hot paths (see CHEATSHEET: HashMap section)
// - map[&key] Index trait: requires &K reference, not K directly
// - match as zero-cost lookup: compiles to a jump table, no heap allocation
// - Closure as local function: `let char_value = |c: char| -> i32 { match c { ... } }`
// - unreachable!() macro: marks impossible branches; panics with message in debug
// - Subtraction trick: `total += value - 2 * prev_value` corrects for the
//   previously-added smaller value in one step (no need to undo + redo)

/// My solution module.
///
/// Time:  O(n) — single pass over the string (n <= 15, so effectively O(1))
/// Space: O(1) — HashMap holds exactly 7 fixed entries; result is a single integer
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert a Roman numeral string to an integer.
        ///
        /// # Arguments
        ///
        /// * `s` - A valid Roman numeral string
        ///
        /// # Returns
        ///
        /// The integer value
        pub fn roman_to_int(s: String) -> i32 {
            // HashMap::from([...]) builds the map from an array of (key, value) tuples inline.
            // Gotcha: HashMap has no guaranteed iteration order, but that's fine here
            // because we look up one character at a time rather than iterating the map.
            // A: use HashMap::from([(), ()]) to provide initialization value
            // A: can't define a HashMap as a const in Rust
            // A: HashMap requires heap allocation and its constructor is not const fn.
            let roman_map: std::collections::HashMap<char, i32> =
                std::collections::HashMap::from([
                    ('I', 1),
                    ('V', 5),
                    ('X', 10),
                    ('L', 50),
                    ('C', 100),
                    ('D', 500),
                    ('M', 1000),
                ]);

            let mut total = 0;
            let mut prev_value = 0;

            for char in s.chars() {
                // `&char` is required because HashMap's index operator is defined as Index<&K>,
                // so it always takes a reference to the key. Passing `char` directly (without &)
                // would not match the trait bound and would fail to compile.
                // For Copy types like char the cost is zero; the same API also lets you index a
                // HashMap<String, _> with a &str without giving up ownership of the key.
                // A: pass key need with &
                let value = roman_map[&char];

                if prev_value < value {
                    total += value - 2 * prev_value;
                } else {
                    total += value;
                }

                prev_value = value;
            }

            total
        }
    }
}

/// AI solution module.
///
/// Time:  O(n) — single pass over the string (n <= 15, so effectively O(1))
/// Space: O(1) — no allocations; `match` replaces the HashMap with zero heap cost
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert a Roman numeral string to an integer.
        ///
        /// # Arguments
        ///
        /// * `s` - A valid Roman numeral string
        ///
        /// # Returns
        ///
        /// The integer value
        pub fn roman_to_int(s: String) -> i32 {
            // `match` replaces HashMap: the compiler turns it into a jump table,
            // so lookup is O(1) with zero heap allocation per call.
            let char_value = |c: char| -> i32 {
                match c {
                    'I' => 1,
                    'V' => 5,
                    'X' => 10,
                    'L' => 50,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    // The constraint guarantees only valid characters, so this is unreachable.
                    _ => unreachable!("invalid Roman numeral character: {c}"),
                }
            };

            let mut total = 0;
            let mut prev_value = 0;

            for ch in s.chars() {
                let value = char_value(ch);

                if prev_value < value {
                    total += value - 2 * prev_value;
                } else {
                    total += value;
                }

                prev_value = value;
            }

            total
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> = vec![("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::roman_to_int(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::roman_to_int(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_roman_to_int {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_iii() {
                    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
                }

                #[test]
                fn test_lviii() {
                    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
                }

                #[test]
                fn test_mcmxciv() {
                    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
                }

                #[test]
                fn test_iv() {
                    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
                }

                #[test]
                fn test_ix() {
                    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
                }

                #[test]
                fn test_single() {
                    assert_eq!(Solution::roman_to_int("M".to_string()), 1000);
                }

                #[test]
                fn test_3999() {
                    assert_eq!(Solution::roman_to_int("MMMCMXCIX".to_string()), 3999);
                }
            }
        };
    }

    test_roman_to_int!(my_tests, crate::my_solution::Solution);
    test_roman_to_int!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec!["III", "LVIII", "MCMXCIV", "IV", "IX", "MMMCMXCIX"];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::roman_to_int(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::roman_to_int(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
