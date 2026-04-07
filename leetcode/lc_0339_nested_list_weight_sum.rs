// @lc app=leetcode id=339 lang=rust
// #[Medium] - Nested List Weight Sum
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.010us
// ai_solution  | avg: 0.046us
// Performance: My 4.45x faster

/// # 339. Nested List Weight Sum
///
/// Given a nested list of integers, return the sum of each integer
/// multiplied by its depth. Depth starts at 1.
///
/// Constraints:
/// - 1 <= nestedList.length <= 50
/// - Values in [-100, 100]
/// - Max depth <= 50
///
/// We represent the nested list as an enum for Rust.

/// Enum representing a nested list element.
#[derive(Debug, Clone)]
enum NestedInt {
    Int(i32),
    List(Vec<NestedInt>),
}

/// ## My Solution — Recursive DFS
///
/// Recurse into each element, tracking depth. Multiply integers by depth.
///
/// - Time: O(n) total elements
/// - Space: O(d) recursion depth
mod my_solution {
    use super::NestedInt;

    pub fn depth_sum(nested_list: &[NestedInt]) -> i32 {
        dfs(nested_list, 1)
    }

    fn dfs(list: &[NestedInt], depth: i32) -> i32 {
        let mut sum = 0;
        for item in list {
            match item {
                NestedInt::Int(v) => sum += v * depth,
                NestedInt::List(inner) => sum += dfs(inner, depth + 1),
            }
        }
        sum
    }
}

/// ## AI Solution — Iterative BFS (level-by-level)
///
/// Use a queue. Process layer by layer, incrementing depth each level.
///
/// - Time: O(n)
/// - Space: O(n) for queue
mod ai_solution {
    use super::NestedInt;
    use std::collections::VecDeque;

    pub fn depth_sum(nested_list: &[NestedInt]) -> i32 {
        let mut queue: VecDeque<(&NestedInt, i32)> = VecDeque::new();
        for item in nested_list {
            queue.push_back((item, 1));
        }
        let mut total = 0;
        while let Some((item, depth)) = queue.pop_front() {
            match item {
                NestedInt::Int(v) => total += v * depth,
                NestedInt::List(inner) => {
                    for child in inner {
                        queue.push_back((child, depth + 1));
                    }
                }
            }
        }
        total
    }
}

fn run_tests(f: fn(&[NestedInt]) -> i32) {
    use NestedInt::{Int, List};
    // Example 1: [[1,1],2,[1,1]] => 4*1*2 + 1*2*1 = 10
    assert_eq!(
        f(&[List(vec![Int(1), Int(1)]), Int(2), List(vec![Int(1), Int(1)])]),
        10
    );
    // Example 2: [1,[4,[6]]] => 1*1 + 4*2 + 6*3 = 27
    assert_eq!(
        f(&[Int(1), List(vec![Int(4), List(vec![Int(6)])])]),
        27
    );
    // Example 3: [0]
    assert_eq!(f(&[Int(0)]), 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::depth_sum);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::depth_sum);
    }
}

fn main() {
    use NestedInt::{Int, List};
    let nested = vec![
        List(vec![Int(1), Int(1)]),
        Int(2),
        List(vec![Int(1), Int(1)]),
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::depth_sum(
            std::hint::black_box(&nested),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::depth_sum(
            std::hint::black_box(&nested),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::depth_sum(
            std::hint::black_box(&nested),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::depth_sum(
            std::hint::black_box(&nested),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 339 - Nested List Weight Sum");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
