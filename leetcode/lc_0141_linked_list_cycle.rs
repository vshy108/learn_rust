// LeetCode 141: Linked List Cycle [Easy]
// https://leetcode.com/problems/linked-list-cycle/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.389us
//   AI solution avg: 0.420us
//   Ratio: ~tied
//
// Given head, determine if the linked list has a cycle in it.
// A cycle exists if some node can be reached again by following next.
//
// Constraints: [0, 10^4] nodes, -10^5 <= Node.val <= 10^5
//   pos is -1 or a valid index. Can you solve it using O(1) memory?
//
// Run:  cargo run --example lc_0141_linked_list_cycle
// Test: cargo test --example lc_0141_linked_list_cycle
// Bench: cargo test --example lc_0141_linked_list_cycle bench -- --nocapture

use std::cell::RefCell;
use std::rc::Rc;

/// Use Rc<RefCell<_>> since we need shared ownership for cycles.
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

/// Build list with optional cycle: pos = index where tail connects back (-1 = no cycle).
fn build_cyclic_list(vals: &[i32], pos: i32) -> Link {
    if vals.is_empty() {
        return None;
    }
    let nodes: Vec<Rc<RefCell<ListNode>>> = vals
        .iter()
        .map(|&v| Rc::new(RefCell::new(ListNode::new(v))))
        .collect();
    for i in 0..nodes.len() - 1 {
        nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
    }
    if pos >= 0 {
        nodes.last().expect("non-empty").borrow_mut().next = Some(Rc::clone(&nodes[pos as usize]));
    }
    Some(Rc::clone(&nodes[0]))
}

/// My solution — Floyd's slow/fast pointer (tortoise and hare).
///
/// Time: O(n).
/// Space: O(1).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return true if the list has a cycle.
        pub fn has_cycle(head: &Link) -> bool {
            let mut slow = head.clone();
            let mut fast = head.clone();

            loop {
                // Move slow one step
                slow = slow.and_then(|n| n.borrow().next.clone());
                // Move fast two steps
                fast = fast
                    .and_then(|n| n.borrow().next.clone())
                    .and_then(|n| n.borrow().next.clone());

                match (&slow, &fast) {
                    (None, _) | (_, None) => return false,
                    (Some(s), Some(f)) => {
                        // Compare Rc pointers to detect cycle
                        if Rc::ptr_eq(s, f) {
                            return true;
                        }
                    }
                }
            }
        }
    }
}

/// AI solution — same Floyd's cycle detection (canonical O(1) space approach).
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return true if the list has a cycle (slightly different loop style).
        pub fn has_cycle(head: &Link) -> bool {
            let mut slow = head.clone();
            let mut fast = head.clone();

            while let Some(f) = fast {
                let f_next = f.borrow().next.clone();
                fast = f_next.as_ref().and_then(|n| n.borrow().next.clone());
                slow = slow.and_then(|s| s.borrow().next.clone());

                if let (Some(s), Some(f2)) = (&slow, &fast) {
                    if Rc::ptr_eq(s, f2) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn main() {
    let list = build_cyclic_list(&[3, 2, 0, -4], 1);
    let my = my_solution::Solution::has_cycle(&list);
    let ai = ai_solution::Solution::has_cycle(&list);
    println!("has cycle: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cycle {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_cycle_at_1() {
                    let list = build_cyclic_list(&[3, 2, 0, -4], 1);
                    assert!(<$solution>::has_cycle(&list));
                }

                #[test]
                fn test_cycle_at_0() {
                    let list = build_cyclic_list(&[1, 2], 0);
                    assert!(<$solution>::has_cycle(&list));
                }

                #[test]
                fn test_no_cycle() {
                    let list = build_cyclic_list(&[1], -1);
                    assert!(!<$solution>::has_cycle(&list));
                }

                #[test]
                fn test_empty() {
                    assert!(!<$solution>::has_cycle(&None));
                }

                #[test]
                fn test_two_no_cycle() {
                    let list = build_cyclic_list(&[1, 2], -1);
                    assert!(!<$solution>::has_cycle(&list));
                }
            }
        };
    }

    test_cycle!(my, crate::my_solution::Solution);
    test_cycle!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let list = build_cyclic_list(
            &(0..100).collect::<Vec<i32>>(),
            50, // cycle to middle
        );

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::has_cycle(
                std::hint::black_box(&list),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::has_cycle(
                std::hint::black_box(&list),
            ));
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
