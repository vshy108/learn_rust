// LeetCode 83: Remove Duplicates from Sorted List [Easy]
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 17.627us
//   AI solution avg: 13.934us
//   Ratio: AI 1.27x faster
//
// Given the head of a sorted linked list, delete all duplicates such that
// each element appears only once. Return the sorted list.
//
// Constraints: [0, 300] nodes, -100 <= Node.val <= 100, sorted ascending
//
// Run:  cargo run --example lc_0083_remove_duplicates_sorted_list
// Test: cargo test --example lc_0083_remove_duplicates_sorted_list
// Bench: cargo test --example lc_0083_remove_duplicates_sorted_list bench -- --nocapture

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution — collect, dedup, rebuild.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::{ListNode, build_list, list_to_vec};

    pub struct Solution;

    impl Solution {
        /// Remove duplicates so each value appears once.
        ///
        /// # Arguments
        /// * `head` - sorted linked list
        ///
        /// # Returns
        /// Deduplicated list
        pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut vals = list_to_vec(head);
            vals.dedup(); // remove consecutive duplicates
            build_list(&vals)
        }
    }
}

/// AI solution — in-place traversal skipping duplicates.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Remove duplicates so each value appears once.
        ///
        /// # Arguments
        /// * `head` - sorted linked list
        ///
        /// # Returns
        /// Deduplicated list
        pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut current = &mut head;
            while let Some(node) = current {
                // Skip all next nodes with the same value
                while node.next.is_some() && node.next.as_ref().expect("checked").val == node.val {
                    node.next = node.next.as_mut().expect("checked").next.take();
                }
                current = &mut node.next;
            }
            head
        }
    }
}

fn main() {
    let list = build_list(&[1, 1, 2, 3, 3]);
    let my = my_solution::Solution::delete_duplicates(list.clone());
    let ai = ai_solution::Solution::delete_duplicates(list);
    println!("my: {:?}", list_to_vec(my));
    println!("ai: {:?}", list_to_vec(ai));
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_delete_duplicates {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_list, list_to_vec};

                #[test]
                fn test_example1() {
                    let list = build_list(&[1, 1, 2]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![1, 2]);
                }

                #[test]
                fn test_example2() {
                    let list = build_list(&[1, 1, 2, 3, 3]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![1, 2, 3]);
                }

                #[test]
                fn test_no_dups() {
                    let list = build_list(&[1, 2, 3]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![1, 2, 3]);
                }

                #[test]
                fn test_all_same() {
                    let list = build_list(&[1, 1, 1]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![1]);
                }

                #[test]
                fn test_empty() {
                    let result = list_to_vec(<$solution>::delete_duplicates(None));
                    assert_eq!(result, vec![]);
                }
            }
        };
    }

    test_delete_duplicates!(my, crate::my_solution::Solution);
    test_delete_duplicates!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = {
            let mut v = Vec::new();
            for i in 0..50 {
                for _ in 0..3 {
                    v.push(i);
                }
            }
            v
        };

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::my_solution::Solution::delete_duplicates(
                std::hint::black_box(list),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::ai_solution::Solution::delete_duplicates(
                std::hint::black_box(list),
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
