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
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  2209.71ms (avg 22.09us)
// AI solution:  678.06ms (avg 6.78us)
// -> AI solution is 3.26x faster
// Reason: rolling 1D DP with last-position tracking prunes unnecessary
//   iterations; i16 reduces memory traffic; same-char shortcut avoids
//   rebuilding the DP row for repeated characters.

/// My solution module — top-down DP with memoization.
///
/// Time:  O(m * n^2) where m = key.len(), n = ring.len()
/// Space: O(m * n) for the memo table
mod my_solution {
    use std::collections::HashMap;

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
            let ring = ring.as_bytes();
            let key = key.as_bytes();
            let n = ring.len();
            // Precompute positions for each character in ring
            let mut positions: HashMap<u8, Vec<usize>> = HashMap::new();
            for (i, &b) in ring.iter().enumerate() {
                positions.entry(b).or_default().push(i);
            }
            let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
            Self::dp(0, 0, ring, key, n, &positions, &mut memo)
        }

        // Clippy flags `ring`, `key`, `n`, `positions` as "only used in recursion"
        // because they're passed unchanged to recursive calls. But they ARE read
        // in each call (key[key_idx], positions[&target], ring_idx.abs_diff, n - diff)
        // — clippy's analysis doesn't track indexing into these slices/maps.
        #[allow(clippy::only_used_in_recursion)]
        fn dp(
            ring_idx: usize,
            key_idx: usize,
            ring: &[u8],
            key: &[u8],
            n: usize,
            positions: &HashMap<u8, Vec<usize>>,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if key_idx == key.len() {
                return 0;
            }
            if let Some(&cached) = memo.get(&(ring_idx, key_idx)) {
                return cached;
            }
            let target = key[key_idx];
            let mut min_steps = i32::MAX;
            // Try every position of the target character on the ring
            for &pos in &positions[&target] {
                let diff = pos.abs_diff(ring_idx);
                // Minimum of clockwise vs counterclockwise, plus 1 for button press
                let step = diff.min(n - diff) as i32 + 1;
                let rest = Self::dp(pos, key_idx + 1, ring, key, n, positions, memo);
                min_steps = min_steps.min(step + rest);
            }
            memo.insert((ring_idx, key_idx), min_steps);
            min_steps
        }
    }
}

/// AI solution module — rolling 1D DP with last-position tracking.
///
/// Only tracks positions from the previous character (not all ring
/// positions), and skips DP when consecutive key chars match.
/// Uses `i16` for smaller memory footprint.
///
/// Time:  O(m * n^2)
/// Space: O(n) — only one DP row at a time
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
            const LARGE: i16 = 10000;
            let nr = ring.len();
            let nri = nr as i16;
            let mut dp0 = vec![LARGE; nr];
            dp0[0] = 0;
            let pos = {
                let mut pos = vec![Vec::new(); 26];
                for (i, k) in ring.bytes().enumerate() {
                    pos[usize::from(k - b'a')].push(i);
                }
                pos
            };
            let mut last = vec![0];
            let mut last_c = ring.as_bytes()[0];
            for k in key.bytes() {
                if last_c == k {
                    // Same char as last: just add 1 button press, no rotation
                    for &p in &last {
                        dp0[p] += 1;
                    }
                } else {
                    let mut dp1 = vec![LARGE; nr];
                    let pos = pos[usize::from(k - b'a')].clone();
                    for &p1 in &pos {
                        let mut res = LARGE;
                        for &p0 in &last {
                            let d = {
                                let p1 = p1 as i16;
                                let p0 = p0 as i16;
                                let d = (p0 - p1).abs();
                                d.min(nri - d)
                            };
                            let t = dp0[p0] + d + 1;
                            res = res.min(t);
                        }
                        dp1[p1] = res;
                    }
                    dp0 = dp1;
                    last = pos;
                    last_c = k;
                }
            }
            let mut ans = LARGE;
            for p in last {
                ans = ans.min(dp0[p]);
            }
            i32::from(ans)
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
