// LeetCode 160: Intersection of Two Linked Lists [Easy]
// https://leetcode.com/problems/intersection-of-two-linked-lists/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.577us
//   AI solution avg: 0.337us
//   Ratio: AI 1.71x faster
//
// Given the heads of two singly linked-lists headA and headB, return the node
// at which the two lists intersect. If they don't intersect, return null.
//
// Constraints: listA has m nodes, listB has n nodes,
//   1 <= m, n <= 3*10^4, 1 <= Node.val <= 10^5
//
// Run:  cargo run --example lc_0160_intersection_two_lists
// Test: cargo test --example lc_0160_intersection_two_lists
// Bench: cargo test --example lc_0160_intersection_two_lists bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Rc<RefCell<_>> for shared ownership at the intersection point.
#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

type Link = Option<Rc<RefCell<ListNode>>>;

/// Build two lists that intersect at a given point.
/// Returns (headA, headB).
fn build_intersecting(
    prefix_a: &[i32],
    prefix_b: &[i32],
    shared: &[i32],
) -> (Link, Link) {
    // Build shared tail
    let mut shared_head: Link = None;
    for &v in shared.iter().rev() {
        let node = Rc::new(RefCell::new(ListNode::new(v)));
        node.borrow_mut().next = shared_head;
        shared_head = Some(node);
    }

    // Build prefix A -> shared
    let mut head_a = shared_head.clone();
    for &v in prefix_a.iter().rev() {
        let node = Rc::new(RefCell::new(ListNode::new(v)));
        node.borrow_mut().next = head_a;
        head_a = Some(node);
    }

    // Build prefix B -> shared
    let mut head_b = shared_head;
    for &v in prefix_b.iter().rev() {
        let node = Rc::new(RefCell::new(ListNode::new(v)));
        node.borrow_mut().next = head_b;
        head_b = Some(node);
    }

    (head_a, head_b)
}

fn list_len(head: &Link) -> usize {
    let mut count = 0;
    let mut cur = head.clone();
    while let Some(node) = cur {
        count += 1;
        cur = node.borrow().next.clone();
    }
    count
}

/// My solution — align by length difference, then walk in lockstep.
///
/// Time: O(m + n).
/// Space: O(1).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the intersection node (by Rc pointer equality), or None.
        pub fn get_intersection_node(head_a: &Link, head_b: &Link) -> Link {
            let len_a = list_len(head_a);
            let len_b = list_len(head_b);

            let mut a = head_a.clone();
            let mut b = head_b.clone();

            // Advance the longer list
            if len_a > len_b {
                for _ in 0..len_a - len_b {
                    a = a.and_then(|n| n.borrow().next.clone());
                }
            } else {
                for _ in 0..len_b - len_a {
                    b = b.and_then(|n| n.borrow().next.clone());
                }
            }

            // Walk both in lockstep
            while let (Some(na), Some(nb)) = (&a, &b) {
                if Rc::ptr_eq(na, nb) {
                    return a;
                }
                let next_a = na.borrow().next.clone();
                let next_b = nb.borrow().next.clone();
                a = next_a;
                b = next_b;
            }
            None
        }
    }
}

/// AI solution — two-pointer trick: switch to other head on reaching end.
///
/// Time: O(m + n).
/// Space: O(1).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return intersection using the two-pointer switch technique.
        pub fn get_intersection_node(head_a: &Link, head_b: &Link) -> Link {
            let mut a = head_a.clone();
            let mut b = head_b.clone();
            // After one switch each, both pointers have traveled m+n nodes,
            // so they're aligned regardless of length difference.
            let mut switched_a = false;
            let mut switched_b = false;

            loop {
                match (&a, &b) {
                    (Some(na), Some(nb)) => {
                        if Rc::ptr_eq(na, nb) {
                            return a;
                        }
                        let next_a = na.borrow().next.clone();
                        let next_b = nb.borrow().next.clone();
                        a = next_a;
                        b = next_b;
                    }
                    (None, Some(_)) => {
                        if switched_a { return None; }
                        a = head_b.clone();
                        switched_a = true;
                    }
                    (Some(_), None) => {
                        if switched_b { return None; }
                        b = head_a.clone();
                        switched_b = true;
                    }
                    (None, None) => return None,
                }
            }
        }
    }
}

fn main() {
    let (a, b) = build_intersecting(&[4, 1], &[5, 6, 1], &[8, 4, 5]);
    let my = my_solution::Solution::get_intersection_node(&a, &b);
    let ai = ai_solution::Solution::get_intersection_node(&a, &b);
    println!(
        "intersection: my={}, ai={}",
        my.as_ref().map(|n| n.borrow().val).unwrap_or(-1),
        ai.as_ref().map(|n| n.borrow().val).unwrap_or(-1),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_intersection {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_intersect() {
                    let (a, b) = build_intersecting(
                        &[4, 1], &[5, 6, 1], &[8, 4, 5],
                    );
                    let result = <$solution>::get_intersection_node(&a, &b);
                    assert_eq!(
                        result.as_ref().map(|n| n.borrow().val),
                        Some(8),
                    );
                }

                #[test]
                fn test_no_intersection() {
                    let (a, b) = build_intersecting(
                        &[2, 6, 4], &[1, 5], &[],
                    );
                    let result = <$solution>::get_intersection_node(&a, &b);
                    assert!(result.is_none());
                }

                #[test]
                fn test_intersect_at_start() {
                    let (a, b) = build_intersecting(
                        &[], &[], &[1, 2, 3],
                    );
                    let result = <$solution>::get_intersection_node(&a, &b);
                    assert_eq!(
                        result.as_ref().map(|n| n.borrow().val),
                        Some(1),
                    );
                }

                #[test]
                fn test_single_shared() {
                    let (a, b) = build_intersecting(
                        &[1], &[2], &[3],
                    );
                    let result = <$solution>::get_intersection_node(&a, &b);
                    assert_eq!(
                        result.as_ref().map(|n| n.borrow().val),
                        Some(3),
                    );
                }
            }
        };
    }

    test_intersection!(my, crate::my_solution::Solution);
    test_intersection!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let prefix_a: Vec<i32> = (1..=20).collect();
        let prefix_b: Vec<i32> = (100..=130).collect();
        let shared: Vec<i32> = (200..=220).collect();
        let (a, b) = build_intersecting(&prefix_a, &prefix_b, &shared);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::get_intersection_node(
                    std::hint::black_box(&a),
                    std::hint::black_box(&b),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::get_intersection_node(
                    std::hint::black_box(&a),
                    std::hint::black_box(&b),
                ),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
