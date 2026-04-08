// LeetCode 147: Insertion Sort List [Medium]
// https://leetcode.com/problems/insertion-sort-list/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 5.878us
//   AI solution avg: 2.825us
//   Ratio: AI 2.08x faster
//
// Given the head of a singly linked list, sort it using insertion sort.
//
// Constraints: [0, 5000] nodes, -5000 <= Node.val <= 5000
//
// Run:  cargo run --example lc_0147_insertion_sort_list
// Test: cargo test --example lc_0147_insertion_sort_list
// Bench: cargo test --example lc_0147_insertion_sort_list bench -- --nocapture

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

/// My solution — collect values, sort, rebuild list.
///
/// Time: O(n log n) due to sort.
/// Space: O(n).
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Sort the linked list.
        pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut vals = to_vec(&head);
            vals.sort_unstable();
            build_list(&vals)
        }
    }
}

/// AI solution — true insertion sort on the linked list.
///
/// Time: O(n^2) worst case (insertion sort).
/// Space: O(1) extra.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Sort using actual insertion sort on linked list.
        pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut sorted: Option<Box<ListNode>> = None;
            let mut current = head;

            while let Some(mut node) = current {
                current = node.next.take();
                // Insert node into the correct position in sorted list
                sorted = Self::sorted_insert(sorted, node);
            }
            sorted
        }

        fn sorted_insert(
            mut head: Option<Box<ListNode>>,
            mut node: Box<ListNode>,
        ) -> Option<Box<ListNode>> {
            // If list is empty or node should go at the front
            if head.is_none() || node.val <= head.as_ref().expect("checked").val {
                node.next = head;
                return Some(node);
            }

            // Walk the sorted list to find insertion point
            let mut cur = head.as_mut().expect("non-empty");
            while cur.next.is_some() && cur.next.as_ref().expect("checked").val < node.val {
                cur = cur.next.as_mut().expect("checked");
            }
            node.next = cur.next.take();
            cur.next = Some(node);
            head
        }
    }
}

fn main() {
    let my = my_solution::Solution::insertion_sort_list(build_list(&[4, 2, 1, 3]));
    let ai = ai_solution::Solution::insertion_sort_list(build_list(&[4, 2, 1, 3]));
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
                    let result = <$solution>::insertion_sort_list(build_list(&[4, 2, 1, 3]));
                    assert_eq!(to_vec(&result), vec![1, 2, 3, 4]);
                }

                #[test]
                fn test_example2() {
                    let result = <$solution>::insertion_sort_list(build_list(&[-1, 5, 3, 4, 0]));
                    assert_eq!(to_vec(&result), vec![-1, 0, 3, 4, 5]);
                }

                #[test]
                fn test_empty() {
                    let result = <$solution>::insertion_sort_list(None);
                    assert!(result.is_none());
                }

                #[test]
                fn test_single() {
                    let result = <$solution>::insertion_sort_list(build_list(&[5]));
                    assert_eq!(to_vec(&result), vec![5]);
                }

                #[test]
                fn test_already_sorted() {
                    let result = <$solution>::insertion_sort_list(build_list(&[1, 2, 3, 4, 5]));
                    assert_eq!(to_vec(&result), vec![1, 2, 3, 4, 5]);
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
            std::hint::black_box(crate::my_solution::Solution::insertion_sort_list(
                std::hint::black_box(build_list(&vals)),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::insertion_sort_list(
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
