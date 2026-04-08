// @lc app=leetcode id=359 lang=rust
// #[Easy] - Logger Rate Limiter
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.191us
// ai_solution  | avg: 0.105us
// Performance: AI 1.82x faster

/// # 359. Logger Rate Limiter
///
/// Design a logger that allows printing a message at most every 10 seconds.
///
/// Constraints:
/// - 0 <= timestamp <= 10^9 (chronological order)
/// - 1 <= message.length <= 30
/// - At most 10^4 calls

/// ## My Solution — HashMap storing next-allowed timestamp
///
/// For each message, store the earliest timestamp it can next be printed.
///
/// - Time: O(1) per call
/// - Space: O(m) where m = distinct messages
mod my_solution {
    use std::collections::HashMap;

    /// Process log entries and return whether each should print.
    pub fn process(entries: &[(i32, &str)]) -> Vec<bool> {
        let mut map: HashMap<&str, i32> = HashMap::new();
        let mut results = Vec::with_capacity(entries.len());
        for &(ts, msg) in entries {
            let next = map.get(msg).copied().unwrap_or(0);
            if ts >= next {
                map.insert(msg, ts + 10);
                results.push(true);
            } else {
                results.push(false);
            }
        }
        results
    }
}

/// ## AI Solution — Vec-based with linear scan (no HashMap)
///
/// Store (message, next_ts) pairs in a Vec. Linear scan to find match.
/// Simpler but O(m) per call.
///
/// - Time: O(m) per call
/// - Space: O(m)
mod ai_solution {
    /// Process log entries and return whether each should print.
    pub fn process(entries: &[(i32, &str)]) -> Vec<bool> {
        let mut store: Vec<(&str, i32)> = Vec::new();
        let mut results = Vec::with_capacity(entries.len());
        for &(ts, msg) in entries {
            // Linear search for existing entry
            let mut found = false;
            for entry in store.iter_mut() {
                if entry.0 == msg {
                    if ts >= entry.1 {
                        entry.1 = ts + 10;
                        results.push(true);
                    } else {
                        results.push(false);
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                store.push((msg, ts + 10));
                results.push(true);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[(i32, &str)]) -> Vec<bool>) {
        let entries = vec![
            (1, "foo"),
            (2, "bar"),
            (3, "foo"),  // false: 3 < 11
            (8, "bar"),  // false: 8 < 12
            (10, "foo"), // false: 10 < 11
            (11, "foo"), // true: 11 >= 11
        ];
        assert_eq!(f(&entries), vec![true, true, false, false, false, true]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::process);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::process);
    }
}

fn main() {
    let entries: Vec<(i32, &str)> = vec![
        (1, "foo"),
        (2, "bar"),
        (3, "foo"),
        (8, "bar"),
        (10, "foo"),
        (11, "foo"),
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::process(std::hint::black_box(&entries)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::process(std::hint::black_box(&entries)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::process(std::hint::black_box(&entries)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::process(std::hint::black_box(&entries)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 359 - Logger Rate Limiter");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
