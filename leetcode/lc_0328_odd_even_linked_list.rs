// @lc app=leetcode id=328 lang=rust
// #[Medium] - Odd Even Linked List
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.156us
// ai_solution  | avg: 0.025us
// Performance: AI 6.14x faster

/// # 328. Odd Even Linked List
///
/// Given the head of a singly linked list, group all nodes with odd indices
/// together followed by nodes with even indices. Return the reordered list.
///
/// The first node is odd, the second is even, etc.
/// Must solve in O(1) extra space and O(n) time.
///
/// Constraints:
/// - Number of nodes in [0, 10^4]
/// - -10^6 <= Node.val <= 10^6

/// Linked list represented as Vec<i32> for benchmarking convenience.
/// The algorithm simulates pointer manipulation on indices.

/// ## My Solution — Two-pointer index split
///
/// Collect odd-indexed and even-indexed values separately, then
/// concatenate. Uses direct index arithmetic.
///
/// - Time: O(n)
/// - Space: O(n) — for output vec
mod my_solution {
    pub fn odd_even_list(head: &[i32]) -> Vec<i32> {
        if head.is_empty() {
            return vec![];
        }
        let mut odds: Vec<i32> = Vec::with_capacity((head.len() + 1) / 2);
        let mut evens: Vec<i32> = Vec::with_capacity(head.len() / 2);
        for (i, &val) in head.iter().enumerate() {
            // 0-based: index 0 => 1st (odd), index 1 => 2nd (even), etc.
            if i % 2 == 0 {
                odds.push(val);
            } else {
                evens.push(val);
            }
        }
        odds.extend_from_slice(&evens);
        odds
    }
}

/// ## AI Solution — In-place swap simulation
///
/// Simulate the linked-list pointer rewiring on a Vec. Walk with two
/// cursors (odd/even) and build result by pulling from alternating positions.
///
/// - Time: O(n)
/// - Space: O(n) — for output vec
mod ai_solution {
    pub fn odd_even_list(head: &[i32]) -> Vec<i32> {
        let n = head.len();
        if n == 0 {
            return vec![];
        }
        let mut result = Vec::with_capacity(n);
        // Collect odd-indexed first (step by 2 from 0)
        let mut i = 0;
        while i < n {
            result.push(head[i]);
            i += 2;
        }
        // Then even-indexed (step by 2 from 1)
        i = 1;
        while i < n {
            result.push(head[i]);
            i += 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[i32]) -> Vec<i32>) {
        // Example 1: [1,2,3,4,5] => odds=[1,3,5], evens=[2,4]
        assert_eq!(f(&[1, 2, 3, 4, 5]), vec![1, 3, 5, 2, 4]);
        // Example 2
        assert_eq!(f(&[2, 1, 3, 5, 6, 4, 7]), vec![2, 3, 6, 7, 1, 5, 4]);
        // Empty
        assert_eq!(f(&[]), Vec::<i32>::new());
        // Single element
        assert_eq!(f(&[42]), vec![42]);
        // Two elements
        assert_eq!(f(&[1, 2]), vec![1, 2]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::odd_even_list);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::odd_even_list);
    }
}

fn main() {
    let head = vec![1, 2, 3, 4, 5];
    let iterations = 100_000;

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(my_solution::odd_even_list(std::hint::black_box(&head)));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::odd_even_list(std::hint::black_box(&head)));
    }
    let my_dur = start.elapsed();

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(ai_solution::odd_even_list(std::hint::black_box(&head)));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::odd_even_list(std::hint::black_box(&head)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 328 - Odd Even Linked List");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
