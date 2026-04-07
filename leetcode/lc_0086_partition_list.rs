// LeetCode 86: Partition List [Medium]
// https://leetcode.com/problems/partition-list/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 11.060us
//   AI solution avg: 5.313us
//   Ratio: AI 2.08x faster
//
// Given the head of a linked list and a value x, partition it such that
// all nodes less than x come before nodes >= x. Preserve relative order.
//
// Constraints: [0, 200] nodes, -100 <= Node.val <= 100, -200 <= x <= 200
//
// Run:  cargo run --example lc_0086_partition_list
// Test: cargo test --example lc_0086_partition_list
// Bench: cargo test --example lc_0086_partition_list bench -- --nocapture

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

/// My solution — collect to vec, partition, rebuild.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::{ListNode, build_list, list_to_vec};

    pub struct Solution;

    impl Solution {
        /// Partition list around value x.
        ///
        /// # Arguments
        /// * `head` - linked list
        /// * `x` - partition value
        ///
        /// # Returns
        /// Partitioned list
        pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
            let vals = list_to_vec(head);
            let mut less: Vec<i32> = vals.iter().copied().filter(|&v| v < x).collect();
            let geq: Vec<i32> = vals.iter().copied().filter(|&v| v >= x).collect();
            less.extend(geq);
            build_list(&less)
        }
    }
}

/// AI solution — two-pointer approach with dummy nodes.
///
/// Build two separate lists (less, greater-or-equal), then concatenate.
///
/// Time: O(n).
/// Space: O(1) extra (reuse existing nodes).
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Partition list around value x.
        ///
        /// # Arguments
        /// * `head` - linked list
        /// * `x` - partition value
        ///
        /// # Returns
        /// Partitioned list
        pub fn partition(
            mut head: Option<Box<ListNode>>,
            x: i32,
        ) -> Option<Box<ListNode>> {
            let mut less_head = Box::new(ListNode::new(0));
            let mut geq_head = Box::new(ListNode::new(0));
            let mut less = &mut less_head;
            let mut geq = &mut geq_head;

            while let Some(mut node) = head {
                head = node.next.take();
                if node.val < x {
                    less.next = Some(node);
                    less = less.next.as_mut().expect("just assigned");
                } else {
                    geq.next = Some(node);
                    geq = geq.next.as_mut().expect("just assigned");
                }
            }
            // Concatenate less list with geq list
            less.next = geq_head.next;
            less_head.next
        }
    }
}

fn main() {
    let list = build_list(&[1, 4, 3, 2, 5, 2]);
    let my = my_solution::Solution::partition(list.clone(), 3);
    let ai = ai_solution::Solution::partition(list, 3);
    println!("my: {:?}", list_to_vec(my));
    println!("ai: {:?}", list_to_vec(ai));
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_partition {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::{build_list, list_to_vec};

                #[test]
                fn test_example1() {
                    let list = build_list(&[1, 4, 3, 2, 5, 2]);
                    let result = list_to_vec(<$solution>::partition(list, 3));
                    assert_eq!(result, vec![1, 2, 2, 4, 3, 5]);
                }

                #[test]
                fn test_example2() {
                    let list = build_list(&[2, 1]);
                    let result = list_to_vec(<$solution>::partition(list, 2));
                    assert_eq!(result, vec![1, 2]);
                }

                #[test]
                fn test_empty() {
                    let result = list_to_vec(<$solution>::partition(None, 0));
                    assert_eq!(result, vec![]);
                }

                #[test]
                fn test_all_less() {
                    let list = build_list(&[1, 2, 3]);
                    let result = list_to_vec(<$solution>::partition(list, 5));
                    assert_eq!(result, vec![1, 2, 3]);
                }

                #[test]
                fn test_all_geq() {
                    let list = build_list(&[5, 4, 3]);
                    let result = list_to_vec(<$solution>::partition(list, 1));
                    assert_eq!(result, vec![5, 4, 3]);
                }
            }
        };
    }

    test_partition!(my, crate::my_solution::Solution);
    test_partition!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (0..50).map(|i| if i % 2 == 0 { i } else { 50 - i }).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::my_solution::Solution::partition(
                std::hint::black_box(list), 25,
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::ai_solution::Solution::partition(
                std::hint::black_box(list), 25,
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
