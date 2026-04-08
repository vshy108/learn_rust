// @lc app=leetcode id=358 lang=rust
// #[Hard] - Rearrange String k Distance Apart
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.154us
// ai_solution  | avg: 0.064us
// Performance: AI 2.42x faster

/// # 358. Rearrange String k Distance Apart
///
/// Rearrange string s so same chars are at least k apart. Return "" if impossible.
///
/// Constraints:
/// - 1 <= s.length <= 3*10^5
/// - s: lowercase English letters
/// - 0 <= k <= s.length

/// ## My Solution — Max-heap + cooldown queue
///
/// Greedily pick the most frequent char. After placing it, put it in a
/// cooldown queue. When queue reaches size k, release the front back to the heap.
///
/// - Time: O(n log 26) = O(n)
/// - Space: O(26) = O(1)
mod my_solution {
    use std::collections::{BinaryHeap, VecDeque};

    pub fn rearrange_string(s: &str, k: usize) -> String {
        if k <= 1 {
            return s.to_string();
        }
        let mut cnt = [0u32; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        // Max-heap: (count, char_index)
        let mut heap: BinaryHeap<(u32, u8)> = BinaryHeap::new();
        for (i, &c) in cnt.iter().enumerate() {
            if c > 0 {
                heap.push((c, i as u8));
            }
        }

        let mut result = Vec::with_capacity(s.len());
        // Cooldown queue: (remaining_count, char_index)
        let mut q: VecDeque<(u32, u8)> = VecDeque::new();

        while let Some((count, ch)) = heap.pop() {
            result.push(b'a' + ch);
            q.push_back((count - 1, ch));
            // Release from cooldown when queue reaches k
            if q.len() >= k {
                let (rc, rch) = q.pop_front().expect("queue should not be empty");
                if rc > 0 {
                    heap.push((rc, rch));
                }
            }
        }

        if result.len() < s.len() {
            String::new()
        } else {
            // SAFETY: result contains only b'a'..=b'z'
            unsafe { String::from_utf8_unchecked(result) }
        }
    }
}

/// ## AI Solution — Greedy with sorted frequency + round-robin fill
///
/// Sort characters by frequency descending. Place the most frequent first
/// at positions 0, k, 2k, ... then next most frequent fills the gaps.
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    pub fn rearrange_string(s: &str, k: usize) -> String {
        if k <= 1 {
            return s.to_string();
        }
        let n = s.len();
        let mut cnt = [(0u32, 0u8); 26];
        for (i, c) in cnt.iter_mut().enumerate() {
            c.1 = i as u8;
        }
        for b in s.bytes() {
            cnt[(b - b'a') as usize].0 += 1;
        }
        // Sort by count descending
        cnt.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        // Check feasibility: max_freq chars need (max_freq-1)*k + count_of_max slots
        let max_freq = cnt[0].0 as usize;
        let max_count = cnt.iter().take_while(|c| c.0 as usize == max_freq).count();
        if (max_freq - 1) * k + max_count > n {
            return String::new();
        }

        // Fill result array in round-robin: place each char at next available position
        let mut result = vec![0u8; n];
        let mut pos = 0;
        for &(count, ch) in &cnt {
            if count == 0 {
                break;
            }
            for _ in 0..count {
                result[pos] = b'a' + ch;
                pos += k;
                if pos >= n {
                    // Move to next column
                    pos %= k;
                    // Find the first empty column position
                    while pos < n && result[pos] != 0 {
                        pos += 1;
                    }
                }
            }
        }
        // SAFETY: result contains only b'a'..=b'z'
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
fn is_valid(s: &str, k: usize) -> bool {
    if k <= 1 {
        return true;
    }
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        for j in (i + 1)..bytes.len().min(i + k) {
            if bytes[i] == bytes[j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&str, usize) -> String) {
        let r = f("aabbcc", 3);
        assert_eq!(r.len(), 6);
        assert!(is_valid(&r, 3), "invalid: {r}");

        assert_eq!(f("aaabc", 3), "");

        let r = f("aaadbbcc", 2);
        assert_eq!(r.len(), 8);
        assert!(is_valid(&r, 2), "invalid: {r}");

        // k=0 means no constraint
        assert_eq!(f("abc", 0), "abc");
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::rearrange_string);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::rearrange_string);
    }
}

fn main() {
    let s = "aabbcc";
    let k = 3;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::rearrange_string(std::hint::black_box(s), k));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::rearrange_string(std::hint::black_box(s), k));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::rearrange_string(std::hint::black_box(s), k));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::rearrange_string(std::hint::black_box(s), k));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 358 - Rearrange String k Distance Apart");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
