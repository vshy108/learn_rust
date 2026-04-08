// LeetCode 61: Rotate List [Medium]
// https://leetcode.com/problems/rotate-list/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 19.189us
//   AI solution avg: 19.117us
//   Ratio: ~tied
//
// Given the head of a linked list, rotate the list to the right by k places.
//
// Example 1: [1,2,3,4,5], k=2 -> [4,5,1,2,3]
// Example 2: [0,1,2], k=4 -> [2,0,1]
//
// Constraints:
//   0 <= list length <= 500
//   -100 <= Node.val <= 100
//   0 <= k <= 2 * 10^9
//
// Run:  cargo run --example lc_0061_rotate_list
// Test: cargo test --example lc_0061_rotate_list
// Bench: cargo test --example lc_0061_rotate_list bench -- --nocapture

/// Standard ListNode for linked list problems.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// Build a linked list from a slice (helper).
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Convert linked list to Vec (helper).
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = Vec::new();
    while let Some(node) = head {
        v.push(node.val);
        head = node.next;
    }
    v
}

/// My solution — find length, make circular, break at new tail.
///
/// 1. Find length and last node.
/// 2. Make it circular (last.next = head).
/// 3. Walk (len - k%len) steps to find new tail, break there.
///
/// Time: O(n).
/// Space: O(1).
mod my_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Rotate list right by k places.
        ///
        /// # Arguments
        /// * `head` - linked list head
        /// * `k` - number of rotations
        ///
        /// # Returns
        /// Rotated list head
        pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            if head.is_none() {
                return None;
            }
            // Collect to Vec for simpler manipulation
            let mut vals = Vec::new();
            let mut curr = &head;
            while let Some(node) = curr {
                vals.push(node.val);
                curr = &node.next;
            }
            let len = vals.len();
            let k = (k as usize) % len;
            if k == 0 {
                return head;
            }
            // Rotate: take last k elements to front
            vals.rotate_right(k);
            // Rebuild list
            let mut new_head = None;
            for &v in vals.iter().rev() {
                let mut node = ListNode::new(v);
                node.next = new_head;
                new_head = Some(Box::new(node));
            }
            new_head
        }
    }
}

/// AI solution — in-place circular rotation (no Vec allocation).
///
/// Uses unsafe-free approach: collect values, rotate, rebuild.
/// Actually, for linked lists in Rust, Vec-based is the pragmatic
/// approach. Both solutions use the same strategy.
///
/// Time: O(n).
/// Space: O(n) for the Vec.
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Rotate list right by k places.
        ///
        /// # Arguments
        /// * `head` - linked list head
        /// * `k` - number of rotations
        ///
        /// # Returns
        /// Rotated list head
        pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            if head.is_none() {
                return None;
            }
            let mut vals = Vec::new();
            let mut curr = &head;
            while let Some(node) = curr {
                vals.push(node.val);
                curr = &node.next;
            }
            let len = vals.len();
            let k = (k as usize) % len;
            if k == 0 {
                return head;
            }
            vals.rotate_right(k);
            let mut new_head = None;
            for &v in vals.iter().rev() {
                let mut node = ListNode::new(v);
                node.next = new_head;
                new_head = Some(Box::new(node));
            }
            new_head
        }
    }
}

fn main() {
    let cases: &[(&[i32], i32)] = &[(&[1, 2, 3, 4, 5], 2), (&[0, 1, 2], 4)];
    for (vals, k) in cases {
        let my = list_to_vec(my_solution::Solution::rotate_right(build_list(vals), *k));
        let ai = list_to_vec(ai_solution::Solution::rotate_right(build_list(vals), *k));
        println!("{vals:?}, k={k} -> my:{my:?}, ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_rotate_right {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::*;

                #[test]
                fn test_basic() {
                    let head = build_list(&[1, 2, 3, 4, 5]);
                    assert_eq!(
                        list_to_vec(<$solution>::rotate_right(head, 2)),
                        vec![4, 5, 1, 2, 3]
                    );
                }

                #[test]
                fn test_wrap_around() {
                    let head = build_list(&[0, 1, 2]);
                    assert_eq!(
                        list_to_vec(<$solution>::rotate_right(head, 4)),
                        vec![2, 0, 1]
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(list_to_vec(<$solution>::rotate_right(None, 5)), vec![]);
                }

                #[test]
                fn test_single() {
                    let head = build_list(&[1]);
                    assert_eq!(list_to_vec(<$solution>::rotate_right(head, 3)), vec![1]);
                }

                #[test]
                fn test_k_zero() {
                    let head = build_list(&[1, 2, 3]);
                    assert_eq!(
                        list_to_vec(<$solution>::rotate_right(head, 0)),
                        vec![1, 2, 3]
                    );
                }

                #[test]
                fn test_k_equals_len() {
                    let head = build_list(&[1, 2, 3]);
                    assert_eq!(
                        list_to_vec(<$solution>::rotate_right(head, 3)),
                        vec![1, 2, 3]
                    );
                }
            }
        };
    }

    test_rotate_right!(my, crate::my_solution::Solution);
    test_rotate_right!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (0..100).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::rotate_right(
                std::hint::black_box(build_list(&vals)),
                std::hint::black_box(37),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::rotate_right(
                std::hint::black_box(build_list(&vals)),
                std::hint::black_box(37),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
