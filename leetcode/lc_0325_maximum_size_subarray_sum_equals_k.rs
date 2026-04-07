// @lc app=leetcode id=325 lang=rust
// #[Medium] - Maximum Size Subarray Sum Equals k
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.187us
// ai_solution  | avg: 0.137us
// Performance: AI 1.36x faster

/// # 325. Maximum Size Subarray Sum Equals k
///
/// Given an integer array `nums` and an integer `k`, return the maximum
/// length of a subarray that sums to `k`. If none exists, return `0`.
///
/// Constraints:
/// - 1 <= nums.length <= 2 * 10^5
/// - -10^4 <= nums[i] <= 10^4
/// - -10^9 <= k <= 10^9

/// ## My Solution — Prefix Sum + HashMap (first occurrence)
///
/// Track running prefix sum. For each index, check if `prefix_sum - k`
/// was seen before. Store only the *first* occurrence of each prefix sum
/// so the subarray length is maximized.
///
/// - Time: O(n)
/// - Space: O(n)
mod my_solution {
    use std::collections::HashMap;

    pub fn max_sub_array_len(nums: &[i32], k: i32) -> i32 {
        // Map from prefix_sum -> earliest index where it occurred
        let mut first_seen: HashMap<i64, i32> = HashMap::new();
        // prefix_sum 0 occurs "before" index 0, i.e. at virtual index -1
        first_seen.insert(0, -1);
        let mut ans = 0i32;
        let mut prefix = 0i64;
        for (i, &x) in nums.iter().enumerate() {
            prefix += x as i64;
            // If prefix - k was seen at index j, then sum(j+1..=i) == k
            if let Some(&j) = first_seen.get(&(prefix - k as i64)) {
                ans = ans.max(i as i32 - j);
            }
            // Only store the first occurrence to maximize length
            first_seen.entry(prefix).or_insert(i as i32);
        }
        ans
    }
}

/// ## AI Solution — Prefix Sum + Vec-based open-addressing hash
///
/// Same prefix-sum logic but uses a fixed-size Vec with open addressing
/// to avoid HashMap overhead. Keys are i64 prefix sums hashed to slots.
///
/// - Time: O(n) amortized
/// - Space: O(n)
mod ai_solution {
    /// Simple open-addressing hash map for i64 -> i32 with linear probing.
    struct PrefixMap {
        keys: Vec<i64>,
        vals: Vec<i32>,
        occupied: Vec<bool>,
        mask: usize,
    }

    impl PrefixMap {
        fn new(cap: usize) -> Self {
            // Round up to next power of two
            let cap = cap.next_power_of_two();
            Self {
                keys: vec![0; cap],
                vals: vec![0; cap],
                occupied: vec![false; cap],
                mask: cap - 1,
            }
        }

        #[inline]
        fn slot(&self, key: i64) -> usize {
            // Mix bits with a multiply-shift hash
            let h = (key as u64).wrapping_mul(0x9E3779B97F4A7C15);
            (h >> 32) as usize & self.mask
        }

        /// Insert only if key is absent (first-occurrence semantics).
        fn insert_first(&mut self, key: i64, val: i32) {
            let mut idx = self.slot(key);
            loop {
                if !self.occupied[idx] {
                    self.keys[idx] = key;
                    self.vals[idx] = val;
                    self.occupied[idx] = true;
                    return;
                }
                if self.keys[idx] == key {
                    return; // already stored — keep first
                }
                idx = (idx + 1) & self.mask;
            }
        }

        fn get(&self, key: i64) -> Option<i32> {
            let mut idx = self.slot(key);
            loop {
                if !self.occupied[idx] {
                    return None;
                }
                if self.keys[idx] == key {
                    return Some(self.vals[idx]);
                }
                idx = (idx + 1) & self.mask;
            }
        }
    }

    pub fn max_sub_array_len(nums: &[i32], k: i32) -> i32 {
        // Capacity: ~2x the number of entries to keep load factor low
        let mut map = PrefixMap::new((nums.len() + 2) * 2);
        map.insert_first(0, -1);
        let mut ans = 0i32;
        let mut prefix = 0i64;
        for (i, &x) in nums.iter().enumerate() {
            prefix += x as i64;
            if let Some(j) = map.get(prefix - k as i64) {
                ans = ans.max(i as i32 - j);
            }
            map.insert_first(prefix, i as i32);
        }
        ans
    }
}

fn run_tests(f: fn(&[i32], i32) -> i32) {
    // Example 1: [1,-1,5,-2] sums to 3, length 4
    assert_eq!(f(&[1, -1, 5, -2, 3], 3), 4);
    // Example 2: [-1,2] sums to 1, length 2
    assert_eq!(f(&[-2, -1, 2, 1], 1), 2);
    // No subarray sums to target
    assert_eq!(f(&[1, 2, 3], 100), 0);
    // Single element match
    assert_eq!(f(&[5], 5), 1);
    // Entire array sums to k
    assert_eq!(f(&[1, 1, 1], 3), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::max_sub_array_len);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::max_sub_array_len);
    }
}

fn main() {
    let nums = vec![1, -1, 5, -2, 3];
    let k = 3;
    let iterations = 100_000;

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(my_solution::max_sub_array_len(
            std::hint::black_box(&nums),
            std::hint::black_box(k),
        ));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::max_sub_array_len(
            std::hint::black_box(&nums),
            std::hint::black_box(k),
        ));
    }
    let my_dur = start.elapsed();

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(ai_solution::max_sub_array_len(
            std::hint::black_box(&nums),
            std::hint::black_box(k),
        ));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::max_sub_array_len(
            std::hint::black_box(&nums),
            std::hint::black_box(k),
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

    println!("LC 325 - Maximum Size Subarray Sum Equals k");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
