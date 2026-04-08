// @lc app=leetcode id=347 lang=rust
// #[Medium] - Top K Frequent Elements
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.355us
// ai_solution  | avg: 0.205us
// Performance: AI 1.73x faster

/// # 347. Top K Frequent Elements
///
/// Given an integer array and k, return the k most frequent elements.
///
/// Constraints:
/// - 1 <= nums.length <= 10^5
/// - -10^4 <= nums[i] <= 10^4
/// - k in [1, unique elements count]
/// - Answer is unique

/// ## My Solution — Bucket sort O(n)
///
/// Count frequencies, then use bucket sort: index = frequency, value = list of elements.
/// Iterate buckets from high to low to collect top k.
///
/// - Time: O(n)
/// - Space: O(n)
mod my_solution {
    use std::collections::HashMap;

    pub fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
        let mut cnt: HashMap<i32, usize> = HashMap::new();
        for &x in nums {
            *cnt.entry(x).or_insert(0) += 1;
        }
        // Bucket sort: buckets[freq] = list of elements with that frequency
        let n = nums.len();
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
        for (&val, &freq) in &cnt {
            buckets[freq].push(val);
        }
        let mut result = Vec::with_capacity(k);
        // Collect from highest frequency bucket down
        for freq in (1..=n).rev() {
            for &val in &buckets[freq] {
                result.push(val);
                if result.len() == k {
                    return result;
                }
            }
        }
        result
    }
}

/// ## AI Solution — Min-heap of size k
///
/// Count frequencies, maintain a min-heap of size k.
/// Push each (freq, val); pop when size exceeds k.
///
/// - Time: O(n log k)
/// - Space: O(n)
mod ai_solution {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    pub fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
        let mut cnt: HashMap<i32, usize> = HashMap::new();
        for &x in nums {
            *cnt.entry(x).or_insert(0) += 1;
        }
        // Min-heap keyed by frequency; keep only top k
        let mut heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::with_capacity(k + 1);
        for (&val, &freq) in &cnt {
            heap.push(Reverse((freq, val)));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|Reverse((_, v))| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[i32], usize) -> Vec<i32>) {
        let mut r = f(&[1, 1, 1, 2, 2, 3], 2);
        r.sort();
        assert_eq!(r, vec![1, 2]);

        assert_eq!(f(&[1], 1), vec![1]);

        let mut r = f(&[1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2);
        r.sort();
        assert_eq!(r, vec![1, 2]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::top_k_frequent);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::top_k_frequent);
    }
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::top_k_frequent(std::hint::black_box(&nums), k));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::top_k_frequent(std::hint::black_box(&nums), k));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::top_k_frequent(std::hint::black_box(&nums), k));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::top_k_frequent(std::hint::black_box(&nums), k));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 347 - Top K Frequent Elements");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
