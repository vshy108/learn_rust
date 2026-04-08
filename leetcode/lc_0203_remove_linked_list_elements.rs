// LeetCode 203: Remove Linked List Elements [Easy]
// https://leetcode.com/problems/remove-linked-list-elements/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 6.356us
//   AI solution avg: 5.648us
//   Ratio: AI 1.13x faster
//
// Given the head of a linked list and an integer val, remove all nodes
// with Node.val == val and return the new head.
//
// Constraints: nodes in [0, 10^4], 1 <= Node.val <= 50, 0 <= val <= 50
//
// Run:  cargo run --example lc_0203_remove_linked_list_elements
// Test: cargo test --example lc_0203_remove_linked_list_elements
// Bench: cargo test --example lc_0203_remove_linked_list_elements bench -- --nocapture

/// Singly-linked list node definition.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// Build a linked list from a slice.
fn from_slice(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Convert linked list to Vec for easy comparison.
fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Remove all nodes with the given value.
        pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            // Recursive approach
            match head {
                None => None,
                Some(mut node) => {
                    node.next = Self::remove_elements(node.next, val);
                    if node.val == val {
                        node.next
                    } else {
                        Some(node)
                    }
                }
            }
        }
    }
}

/// AI solution — iterative with dummy head to handle head removal.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Iterative approach using a dummy head node.
        pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            let mut dummy = Box::new(ListNode { val: 0, next: head });
            let mut current = &mut dummy;

            // Walk through the list, skipping nodes that match val
            while current.next.is_some() {
                if current.next.as_ref().expect("checked").val == val {
                    // Remove the next node by skipping it
                    let removed = current.next.take().expect("checked");
                    current.next = removed.next;
                } else {
                    current = current.next.as_mut().expect("checked");
                }
            }
            dummy.next
        }
    }
}

fn main() {
    let list = from_slice(&[1, 2, 6, 3, 4, 5, 6]);
    let my = to_vec(my_solution::Solution::remove_elements(list.clone(), 6));
    let ai = to_vec(ai_solution::Solution::remove_elements(list, 6));
    println!("removed 6: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_remove {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example() {
                    let list = from_slice(&[1, 2, 6, 3, 4, 5, 6]);
                    assert_eq!(
                        to_vec(<$solution>::remove_elements(list, 6)),
                        vec![1, 2, 3, 4, 5],
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(
                        to_vec(<$solution>::remove_elements(None, 1)),
                        Vec::<i32>::new(),
                    );
                }

                #[test]
                fn test_all_same() {
                    let list = from_slice(&[7, 7, 7, 7]);
                    assert_eq!(
                        to_vec(<$solution>::remove_elements(list, 7)),
                        Vec::<i32>::new(),
                    );
                }

                #[test]
                fn test_head_removal() {
                    let list = from_slice(&[1, 1, 2, 3]);
                    assert_eq!(to_vec(<$solution>::remove_elements(list, 1)), vec![2, 3],);
                }

                #[test]
                fn test_no_removal() {
                    let list = from_slice(&[1, 2, 3]);
                    assert_eq!(to_vec(<$solution>::remove_elements(list, 5)), vec![1, 2, 3],);
                }
            }
        };
    }

    test_remove!(my, crate::my_solution::Solution);
    test_remove!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (0..100)
            .map(|i| if i % 5 == 0 { 3 } else { i % 10 })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = from_slice(&vals);
            std::hint::black_box(crate::my_solution::Solution::remove_elements(
                std::hint::black_box(list),
                3,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = from_slice(&vals);
            std::hint::black_box(crate::ai_solution::Solution::remove_elements(
                std::hint::black_box(list),
                3,
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
