// LeetCode 281: Zigzag Iterator [Medium] (Premium)
// https://leetcode.com/problems/zigzag-iterator/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.542us
//   AI solution avg: 0.601us
//   Ratio: My 1.11x faster
//
// Given two vectors v1 and v2, implement an iterator that returns their
// elements alternately: v1[0], v2[0], v1[1], v2[1], ...
// When one is exhausted, continue with the remaining elements of the other.
//
// Example 1: v1 = [1,2], v2 = [3,4,5,6] -> [1,3,2,4,5,6]
// Example 2: v1 = [1], v2 = [] -> [1]
// Example 3: v1 = [], v2 = [1] -> [1]
//
// Constraints:
//   - 0 <= v1.length, v2.length <= 1000
//   - 1 <= v1.length + v2.length <= 2000
//   - -2^31 <= v1[i], v2[i] <= 2^31 - 1
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0281.Zigzag%20Iterator/README_EN.md
//
// Run:  cargo run --example lc_0281_zigzag_iterator
// Test: cargo test --example lc_0281_zigzag_iterator
// Bench: cargo test --example lc_0281_zigzag_iterator bench -- --nocapture

/// My solution — Index-based cyclic iterator (from doocs Python/Java).
///
/// Store both vectors and per-vector index pointers. A `cur` variable
/// tracks which vector to pull from next. `has_next` skips exhausted
/// vectors by advancing `cur` cyclically. Generalizes to k vectors.
///
/// Time:  O(1) amortized per next/has_next call
/// Space: O(1) auxiliary (vectors stored by value)
mod my_solution {
    pub struct ZigzagIterator {
        vectors: Vec<Vec<i32>>,
        indexes: Vec<usize>,
        cur: usize,
        size: usize,
    }

    impl ZigzagIterator {
        pub fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
            Self {
                size: 2,
                indexes: vec![0; 2],
                vectors: vec![v1, v2],
                cur: 0,
            }
        }

        pub fn next(&mut self) -> i32 {
            let idx = self.indexes[self.cur];
            let val = self.vectors[self.cur][idx];
            self.indexes[self.cur] += 1;
            self.cur = (self.cur + 1) % self.size;
            val
        }

        pub fn has_next(&mut self) -> bool {
            let start = self.cur;
            // Skip exhausted vectors
            loop {
                if self.indexes[self.cur] < self.vectors[self.cur].len() {
                    return true;
                }
                self.cur = (self.cur + 1) % self.size;
                if self.cur == start {
                    return false;
                }
            }
        }
    }
}

/// AI solution — VecDeque of iterators (queue-based approach).
///
/// Store iterators in a VecDeque. On `next`, pop the front iterator,
/// take one element, and if it has more elements push it to the back.
/// Naturally handles k vectors and exhausted iterators.
///
/// Time:  O(1) per next/has_next call
/// Space: O(k) for the queue of iterators
mod ai_solution {
    use std::collections::VecDeque;

    pub struct ZigzagIterator {
        /// Each entry: (vector, current_index).
        queue: VecDeque<(Vec<i32>, usize)>,
    }

    impl ZigzagIterator {
        pub fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
            let mut queue = VecDeque::new();
            // Only add non-empty vectors
            if !v1.is_empty() {
                queue.push_back((v1, 0));
            }
            if !v2.is_empty() {
                queue.push_back((v2, 0));
            }
            Self { queue }
        }

        pub fn next(&mut self) -> i32 {
            // Pop front iterator, take one element
            let (vec, idx) = self.queue.pop_front().expect("called next on empty");
            let val = vec[idx];
            // If more elements remain, push back to queue
            if idx + 1 < vec.len() {
                self.queue.push_back((vec, idx + 1));
            }
            val
        }

        pub fn has_next(&self) -> bool {
            !self.queue.is_empty()
        }
    }
}

/// Collect all elements from a zigzag iterator into a Vec.
fn collect_my(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut iter = my_solution::ZigzagIterator::new(v1, v2);
    let mut result = Vec::new();
    while iter.has_next() {
        result.push(iter.next());
    }
    result
}

fn collect_ai(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut iter = ai_solution::ZigzagIterator::new(v1, v2);
    let mut result = Vec::new();
    while iter.has_next() {
        result.push(iter.next());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_zigzag {
        ($name:ident, $v1:expr, $v2:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let expected: Vec<i32> = $expected.to_vec();
                assert_eq!(
                    collect_my($v1.to_vec(), $v2.to_vec()),
                    expected,
                    "my_solution failed"
                );
                assert_eq!(
                    collect_ai($v1.to_vec(), $v2.to_vec()),
                    expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Example 1
    test_zigzag!(test_example_1, [1, 2], [3, 4, 5, 6], [1, 3, 2, 4, 5, 6]);
    // Example 2: v2 empty
    test_zigzag!(test_example_2, [1], [], [1]);
    // Example 3: v1 empty
    test_zigzag!(test_example_3, [], [1], [1]);
    // Equal length
    test_zigzag!(test_equal, [1, 2, 3], [4, 5, 6], [1, 4, 2, 5, 3, 6]);
    // v1 longer
    test_zigzag!(test_v1_longer, [1, 2, 3, 4], [5], [1, 5, 2, 3, 4]);
    // v2 longer
    test_zigzag!(test_v2_longer, [1], [2, 3, 4, 5], [1, 2, 3, 4, 5]);
    // Single element each
    test_zigzag!(test_single_each, [1], [2], [1, 2]);
    // Same values
    test_zigzag!(test_same_values, [1, 1], [1, 1], [1, 1, 1, 1]);
    // Negative values
    test_zigzag!(test_negatives, [-1, -2], [-3, -4], [-1, -3, -2, -4]);
    // Large difference in length
    test_zigzag!(
        test_large_diff,
        [1],
        [2, 3, 4, 5, 6, 7, 8],
        [1, 2, 3, 4, 5, 6, 7, 8]
    );
}

fn main() {
    println!("LeetCode 281: Zigzag Iterator");
    let result = collect_my(vec![1, 2], vec![3, 4, 5, 6]);
    println!("my_solution: [1,2] + [3,4,5,6] -> {:?}", result);
    let result = collect_ai(vec![1, 2], vec![3, 4, 5, 6]);
    println!("ai_solution: [1,2] + [3,4,5,6] -> {:?}", result);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let v1: Vec<i32> = (0..50).collect();
        let v2: Vec<i32> = (50..100).collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(collect_my(v1.clone(), v2.clone()));
            std::hint::black_box(collect_ai(v1.clone(), v2.clone()));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(collect_my(v1.clone(), v2.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(collect_ai(v1.clone(), v2.clone()));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
