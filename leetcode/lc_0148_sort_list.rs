// LeetCode 148: Sort List [Medium]
// https://leetcode.com/problems/sort-list/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 5.926us
//   AI solution avg: 4.124us
//   Ratio: AI 1.44x faster
//
// Given the head of a linked list, return the list sorted in ascending order.
// Can you sort in O(n log n) time and O(1) memory?
//
// Constraints: [0, 5*10^4] nodes, -10^5 <= Node.val <= 10^5
//
// Run:  cargo run --example lc_0148_sort_list
// Test: cargo test --example lc_0148_sort_list
// Bench: cargo test --example lc_0148_sort_list bench -- --nocapture

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        result.push(node.val);
        cur = &node.next;
    }
    result
}

/// My solution — collect to vec, sort, rebuild.
///
/// Time: O(n log n).
/// Space: O(n).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Sort the linked list.
        pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut vals = to_vec(&head);
            vals.sort_unstable();
            build_list(&vals)
        }
    }
}

/// AI solution — merge sort on the linked list (true O(log n) space via recursion).
///
/// Time: O(n log n).
/// Space: O(log n) recursion stack.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Sort using top-down merge sort on the list.
        pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_ref().expect("checked").next.is_none() {
                return head;
            }

            // Split list in half
            let (left, right) = Self::split(head);
            let left = Self::sort_list(left);
            let right = Self::sort_list(right);
            Self::merge(left, right)
        }

        /// Split linked list into two halves using slow/fast pointers.
        fn split(
            mut head: Option<Box<ListNode>>,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            // Count length, split at midpoint
            let len = {
                let mut count = 0;
                let mut cur = &head;
                while let Some(node) = cur {
                    count += 1;
                    cur = &node.next;
                }
                count
            };

            let mid = len / 2;
            let mut cur = &mut head;
            for _ in 0..mid - 1 {
                cur = &mut cur.as_mut().expect("within bounds").next;
            }
            let right = cur.as_mut().expect("within bounds").next.take();
            (head, right)
        }

        /// Merge two sorted linked lists.
        fn merge(
            mut l1: Option<Box<ListNode>>,
            mut l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut dummy = Box::new(ListNode::new(0));
            let mut tail = &mut dummy;

            while l1.is_some() && l2.is_some() {
                if l1.as_ref().expect("checked").val <= l2.as_ref().expect("checked").val {
                    let next = l1.as_mut().expect("checked").next.take();
                    tail.next = l1;
                    l1 = next;
                } else {
                    let next = l2.as_mut().expect("checked").next.take();
                    tail.next = l2;
                    l2 = next;
                }
                tail = tail.next.as_mut().expect("just assigned");
            }
            tail.next = if l1.is_some() { l1 } else { l2 };
            dummy.next
        }
    }
}

fn main() {
    let my = my_solution::Solution::sort_list(build_list(&[4, 2, 1, 3]));
    let ai = ai_solution::Solution::sort_list(build_list(&[4, 2, 1, 3]));
    println!("sorted: my={:?}, ai={:?}", to_vec(&my), to_vec(&ai));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sort {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let result = <$solution>::sort_list(build_list(&[4, 2, 1, 3]));
                    assert_eq!(to_vec(&result), vec![1, 2, 3, 4]);
                }

                #[test]
                fn test_example2() {
                    let result = <$solution>::sort_list(build_list(&[-1, 5, 3, 4, 0]));
                    assert_eq!(to_vec(&result), vec![-1, 0, 3, 4, 5]);
                }

                #[test]
                fn test_empty() {
                    let result = <$solution>::sort_list(None);
                    assert!(result.is_none());
                }

                #[test]
                fn test_single() {
                    let result = <$solution>::sort_list(build_list(&[1]));
                    assert_eq!(to_vec(&result), vec![1]);
                }

                #[test]
                fn test_duplicates() {
                    let result = <$solution>::sort_list(build_list(&[3, 1, 2, 3, 1]));
                    assert_eq!(to_vec(&result), vec![1, 1, 2, 3, 3]);
                }
            }
        };
    }

    test_sort!(my, crate::my_solution::Solution);
    test_sort!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (0..50).rev().collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::sort_list(
                std::hint::black_box(build_list(&vals)),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::sort_list(
                std::hint::black_box(build_list(&vals)),
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
