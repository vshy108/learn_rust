// LeetCode 514: Freedom Trail [Hard]
// https://leetcode.com/problems/freedom-trail/
//
// In the video game Fallout 4, the quest "Road to Freedom" requires players
// to reach a metal dial called the "Freedom Trail Ring" and use the dial to
// spell a specific keyword to open the door.
//
// Given a string `ring` that represents the code engraved on the outer ring
// and another string `key` that represents the keyword that needs to be
// spelled, return the minimum number of steps to spell all the characters
// in the keyword.
//
// Initially, the first character of ring is aligned at the "12:00" direction.
// You should spell all the characters in key one by one by rotating ring
// clockwise or anticlockwise to align each character of key at "12:00" and
// then pressing the center button.
//
// Each rotation from one character to an adjacent character counts as 1 step.
// Each press of the center button counts as 1 step.
//
// Examples:
//   ring = "godding", key = "gd"      -> 4
//   ring = "godding", key = "godding" -> 13
//
// Constraints:
//   - 1 <= ring.length, key.length <= 100
//   - ring and key consist of only lowercase English letters
//   - It is guaranteed that key could always be spelled by rotating ring
//
// Run:  cargo run --example lc_0514_freedom_trail
// Test: cargo test --example lc_0514_freedom_trail
// Bench: cargo test --example lc_0514_freedom_trail bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum steps to spell the key using the ring.
        ///
        /// # Arguments
        ///
        /// * `ring` - The ring string
        /// * `key` - The keyword to spell
        ///
        /// # Returns
        ///
        /// Minimum number of steps (rotations + button presses)
        pub fn find_rotate_steps(ring: String, key: String) -> i32 {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum steps to spell the key using the ring.
        ///
        /// # Arguments
        ///
        /// * `ring` - The ring string
        /// * `key` - The keyword to spell
        ///
        /// # Returns
        ///
        /// Minimum number of steps (rotations + button presses)
        pub fn find_rotate_steps(ring: String, key: String) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, &str, i32)> = vec![("godding", "gd", 4), ("godding", "godding", 13)];

    println!("--- My Solution ---");
    for (ring, key, expected) in &test_cases {
        let result = my_solution::Solution::find_rotate_steps(ring.to_string(), key.to_string());
        println!("  ring=\"{ring}\", key=\"{key}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (ring, key, expected) in &test_cases {
        let result = ai_solution::Solution::find_rotate_steps(ring.to_string(), key.to_string());
        println!("  ring=\"{ring}\", key=\"{key}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_find_rotate_steps {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
                        4
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
                        13
                    );
                }

                #[test]
                fn test_single_char_ring() {
                    assert_eq!(
                        Solution::find_rotate_steps("a".to_string(), "a".to_string()),
                        1
                    );
                }

                #[test]
                fn test_repeated_char_ring() {
                    assert_eq!(
                        Solution::find_rotate_steps("aaa".to_string(), "a".to_string()),
                        1
                    );
                }

                #[test]
                fn test_two_char_ring() {
                    assert_eq!(
                        Solution::find_rotate_steps("ab".to_string(), "b".to_string()),
                        2
                    );
                }

                #[test]
                fn test_key_same_as_first() {
                    assert_eq!(
                        Solution::find_rotate_steps("abcde".to_string(), "a".to_string()),
                        1
                    );
                }

                #[test]
                fn test_multiple_occurrences() {
                    assert_eq!(
                        Solution::find_rotate_steps("edcba".to_string(), "abcde".to_string()),
                        10
                    );
                }
            }
        };
    }

    test_find_rotate_steps!(my_tests, crate::my_solution::Solution);
    test_find_rotate_steps!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(&str, &str)> =
            vec![("godding", "gd"), ("godding", "godding"), ("abcde", "a")];

        let start = Instant::now();
        for _ in 0..iterations {
            for (ring, key) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::find_rotate_steps(
                    std::hint::black_box(ring.to_string()),
                    std::hint::black_box(key.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (ring, key) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::find_rotate_steps(
                    std::hint::black_box(ring.to_string()),
                    std::hint::black_box(key.to_string()),
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
