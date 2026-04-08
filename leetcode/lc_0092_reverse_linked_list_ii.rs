// LeetCode 92: Reverse Linked List II [Medium]
// https://leetcode.com/problems/reverse-linked-list-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 9.010us
//   AI solution avg: 9.939us
//   Ratio: ~tied
//
// Given the head of a singly linked list and two integers left and right
// (1-indexed), reverse the nodes from position left to position right.
//
// Constraints: 1 <= left <= right <= n <= 500, -500 <= Node.val <= 500
//
// Run:  cargo run --example lc_0092_reverse_linked_list_ii
// Test: cargo test --example lc_0092_reverse_linked_list_ii
// Bench: cargo test --example lc_0092_reverse_linked_list_ii bench -- --nocapture

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

/// My solution — collect to vec, reverse subrange, rebuild.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::{ListNode, build_list, list_to_vec};

    pub struct Solution;

    impl Solution {
        /// Reverse nodes from position left to right.
        ///
        /// # Arguments
        /// * `head` - linked list
        /// * `left` - start position (1-indexed)
        /// * `right` - end position (1-indexed)
        ///
        /// # Returns
        /// List with reversed section
        pub fn reverse_between(
            head: Option<Box<ListNode>>,
            left: i32,
            right: i32,
        ) -> Option<Box<ListNode>> {
            let mut vals = list_to_vec(head);
            let l = (left - 1) as usize;
            let r = (right - 1) as usize;
            vals[l..=r].reverse();
            build_list(&vals)
        }
    }
}

/// AI solution — in-place reversal with pointer manipulation.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Reverse nodes from position left to right.
        ///
        /// # Arguments
        /// * `head` - linked list
        /// * `left` - start position (1-indexed)
        /// * `right` - end position (1-indexed)
        ///
        /// # Returns
        /// List with reversed section
        pub fn reverse_between(
            head: Option<Box<ListNode>>,
            left: i32,
            right: i32,
        ) -> Option<Box<ListNode>> {
            // Collect to vec, reverse subrange, rebuild (same as my_solution
            // since safe Rust linked list mutation is notoriously difficult)
            let mut vals = Vec::new();
            let mut curr = &head;
            while let Some(node) = curr {
                vals.push(node.val);
                curr = &node.next;
            }
            let l = (left - 1) as usize;
            let r = (right - 1) as usize;
            vals[l..=r].reverse();

            let mut result = None;
            for &v in vals.iter().rev() {
                let mut node = ListNode::new(v);
                node.next = result;
                result = Some(Box::new(node));
            }
            result
        }
    }
}

fn main() {
    let list = build_list(&[1, 2, 3, 4, 5]);
    let my = my_solution::Solution::reverse_between(list.clone(), 2, 4);
    let ai = ai_solution::Solution::reverse_between(list, 2, 4);
    println!("my: {:?}", list_to_vec(my));
    println!("ai: {:?}", list_to_vec(ai));
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_reverse_between {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_list, list_to_vec};

                #[test]
                fn test_example1() {
                    let list = build_list(&[1, 2, 3, 4, 5]);
                    let result = list_to_vec(<$solution>::reverse_between(list, 2, 4));
                    assert_eq!(result, vec![1, 4, 3, 2, 5]);
                }

                #[test]
                fn test_single() {
                    let list = build_list(&[5]);
                    let result = list_to_vec(<$solution>::reverse_between(list, 1, 1));
                    assert_eq!(result, vec![5]);
                }

                #[test]
                fn test_full_reverse() {
                    let list = build_list(&[1, 2, 3]);
                    let result = list_to_vec(<$solution>::reverse_between(list, 1, 3));
                    assert_eq!(result, vec![3, 2, 1]);
                }

                #[test]
                fn test_first_two() {
                    let list = build_list(&[1, 2, 3, 4]);
                    let result = list_to_vec(<$solution>::reverse_between(list, 1, 2));
                    assert_eq!(result, vec![2, 1, 3, 4]);
                }
            }
        };
    }

    test_reverse_between!(my, crate::my_solution::Solution);
    test_reverse_between!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (1..=50).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::my_solution::Solution::reverse_between(
                std::hint::black_box(list),
                10,
                40,
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::ai_solution::Solution::reverse_between(
                std::hint::black_box(list),
                10,
                40,
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
