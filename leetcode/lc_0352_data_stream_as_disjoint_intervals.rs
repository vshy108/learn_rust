// @lc app=leetcode id=352 lang=rust
// #[Hard] - Data Stream as Disjoint Intervals
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 4.789us
// ai_solution  | avg: 4.246us
// Performance: AI 1.13x faster

/// # 352. Data Stream as Disjoint Intervals
///
/// Given a data stream of non-negative integers, summarize as disjoint intervals.
///
/// Constraints:
/// - 0 <= value <= 10^4
/// - At most 3*10^4 calls to addNum, 10^2 calls to getIntervals

/// ## My Solution — BTreeMap: O(log n) addNum
///
/// Use a BTreeMap keyed by interval start. On addNum, find neighbors and merge.
///
/// - Time: O(log n) per addNum, O(n) per getIntervals
/// - Space: O(n)
mod my_solution {
    use std::collections::BTreeMap;

    /// Process a stream of operations and return interval snapshots.
    /// ops: slice of (true=addNum(val), false=getIntervals)
    pub fn process(ops: &[(bool, i32)]) -> Vec<Vec<[i32; 2]>> {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new(); // start -> end
        let mut results = Vec::new();

        for &(is_add, val) in ops {
            if is_add {
                // Find the interval whose start <= val (predecessor)
                let left = map.range(..=val).next_back().map(|(&s, &e)| (s, e));
                // Find the interval whose start > val (successor)
                let right = map.range((val + 1)..).next().map(|(&s, &e)| (s, e));

                let mut new_start = val;
                let mut new_end = val;

                // Check if val is already covered by left interval
                if let Some((ls, le)) = left {
                    if le >= val {
                        // Already contained in [ls, le]
                        continue;
                    }
                    if le + 1 == val {
                        // Extend left interval
                        new_start = ls;
                    }
                }

                // Check if val merges with right interval
                if let Some((rs, re)) = right {
                    if val + 1 == rs {
                        new_end = re;
                        map.remove(&rs);
                    }
                }

                // If we extended from left, remove old left entry
                if let Some((ls, le)) = left {
                    if le + 1 == val {
                        map.remove(&ls);
                    }
                }

                map.insert(new_start, new_end);
            } else {
                results.push(map.iter().map(|(&s, &e)| [s, e]).collect());
            }
        }
        results
    }
}

/// ## AI Solution — Boolean array + scan
///
/// Since values are 0..10^4, use a boolean array. On getIntervals, scan for runs.
///
/// - Time: O(1) addNum, O(max_val) getIntervals
/// - Space: O(max_val)
mod ai_solution {
    /// Process a stream of operations and return interval snapshots.
    pub fn process(ops: &[(bool, i32)]) -> Vec<Vec<[i32; 2]>> {
        let mut present = [false; 10001];
        let mut results = Vec::new();

        for &(is_add, val) in ops {
            if is_add {
                present[val as usize] = true;
            } else {
                // Scan for contiguous runs
                let mut intervals = Vec::new();
                let mut i = 0;
                while i <= 10000 {
                    if present[i] {
                        let start = i;
                        while i <= 10000 && present[i] {
                            i += 1;
                        }
                        intervals.push([start as i32, (i - 1) as i32]);
                    } else {
                        i += 1;
                    }
                }
                results.push(intervals);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[(bool, i32)]) -> Vec<Vec<[i32; 2]>>) {
        // Example: addNum(1), get, addNum(3), get, addNum(7), get, addNum(2), get, addNum(6), get
        let ops = vec![
            (true, 1),
            (false, 0), // getIntervals
            (true, 3),
            (false, 0),
            (true, 7),
            (false, 0),
            (true, 2),
            (false, 0),
            (true, 6),
            (false, 0),
        ];
        let result = f(&ops);
        assert_eq!(result[0], vec![[1, 1]]);
        assert_eq!(result[1], vec![[1, 1], [3, 3]]);
        assert_eq!(result[2], vec![[1, 1], [3, 3], [7, 7]]);
        assert_eq!(result[3], vec![[1, 3], [7, 7]]);
        assert_eq!(result[4], vec![[1, 3], [6, 7]]);
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
    // Benchmark: add many values then get intervals
    let mut ops: Vec<(bool, i32)> = (0..100).map(|v| (true, v * 3)).collect();
    ops.push((false, 0));
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::process(std::hint::black_box(&ops)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::process(std::hint::black_box(&ops)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::process(std::hint::black_box(&ops)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::process(std::hint::black_box(&ops)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 352 - Data Stream as Disjoint Intervals");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
