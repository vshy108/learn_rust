// @lc app=leetcode id=341 lang=rust
// #[Medium] - Flatten Nested List Iterator
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.112us
// ai_solution  | avg: 0.181us
// Performance: My 1.61x faster

/// # 341. Flatten Nested List Iterator
///
/// Given a nested list of integers, implement an iterator to flatten it.
/// Each element is either an integer or a list of elements.
///
/// Constraints:
/// - 1 <= nestedList.length <= 500
/// - Values in [-10^6, 10^6]

/// Reuse NestedInt enum from LC 339.
#[derive(Debug, Clone)]
enum NestedInt {
    Int(i32),
    List(Vec<NestedInt>),
}

/// ## My Solution — Eager flatten via recursive DFS
///
/// Flatten the entire structure into a Vec<i32> up front, then iterate.
///
/// - Time: O(n) construction, O(1) per next/hasNext
/// - Space: O(n)
mod my_solution {
    use super::NestedInt;

    pub fn flatten(nested: &[NestedInt]) -> Vec<i32> {
        let mut result = Vec::new();
        dfs(nested, &mut result);
        result
    }

    fn dfs(list: &[NestedInt], out: &mut Vec<i32>) {
        for item in list {
            match item {
                NestedInt::Int(v) => out.push(*v),
                NestedInt::List(inner) => dfs(inner, out),
            }
        }
    }
}

/// ## AI Solution — Flatten via iterative stack
///
/// Use a stack of slices. Pop from stack; if it's an integer, emit it.
/// If it's a list, push its elements in reverse order. This simulates
/// lazy evaluation but we collect all results for benchmarking.
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    use super::NestedInt;

    pub fn flatten(nested: &[NestedInt]) -> Vec<i32> {
        let mut result = Vec::new();
        // Stack of references to NestedInt items
        let mut stack: Vec<&NestedInt> = nested.iter().rev().collect();
        while let Some(item) = stack.pop() {
            match item {
                NestedInt::Int(v) => result.push(*v),
                NestedInt::List(inner) => {
                    // Push children in reverse so leftmost is on top
                    for child in inner.iter().rev() {
                        stack.push(child);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[NestedInt]) -> Vec<i32>) {
        use NestedInt::{Int, List};
        // Example 1: [[1,1],2,[1,1]] => [1,1,2,1,1]
        assert_eq!(
            f(&[
                List(vec![Int(1), Int(1)]),
                Int(2),
                List(vec![Int(1), Int(1)])
            ]),
            vec![1, 1, 2, 1, 1]
        );
        // Example 2: [1,[4,[6]]] => [1,4,6]
        assert_eq!(
            f(&[Int(1), List(vec![Int(4), List(vec![Int(6)])])]),
            vec![1, 4, 6]
        );
        // Empty nested list
        assert_eq!(f(&[List(vec![])]), Vec::<i32>::new());
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::flatten);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::flatten);
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
        std::hint::black_box(my_solution::flatten(std::hint::black_box(&nested)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::flatten(std::hint::black_box(&nested)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::flatten(std::hint::black_box(&nested)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::flatten(std::hint::black_box(&nested)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 341 - Flatten Nested List Iterator");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
