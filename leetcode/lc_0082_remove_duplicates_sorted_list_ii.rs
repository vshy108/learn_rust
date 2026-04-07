// LeetCode 82: Remove Duplicates from Sorted List II [Medium]
// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 9.695us
//   AI solution avg: 10.948us
//   Ratio: My 1.13x faster
//
// Given the head of a sorted linked list, delete all nodes that have
// duplicate numbers, leaving only distinct numbers.
//
// Constraints: [0, 300] nodes, -100 <= Node.val <= 100, list is sorted
//
// Run:  cargo run --example lc_0082_remove_duplicates_sorted_list_ii
// Test: cargo test --example lc_0082_remove_duplicates_sorted_list_ii
// Bench: cargo test --example lc_0082_remove_duplicates_sorted_list_ii bench -- --nocapture

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

/// Build linked list from a slice.
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Collect linked list into a Vec.
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution — collect to vec, filter, rebuild.
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    use super::{ListNode, build_list, list_to_vec};

    pub struct Solution;

    impl Solution {
        /// Remove all duplicate-valued nodes from sorted list.
        ///
        /// # Arguments
        /// * `head` - sorted linked list
        ///
        /// # Returns
        /// List with only distinct values
        pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let vals = list_to_vec(head);
            let mut unique = Vec::new();
            let mut i = 0;
            while i < vals.len() {
                let mut j = i + 1;
                while j < vals.len() && vals[j] == vals[i] {
                    j += 1;
                }
                if j - i == 1 {
                    unique.push(vals[i]); // only appears once
                }
                i = j;
            }
            build_list(&unique)
        }
    }
}

/// AI solution — in-place pointer manipulation with dummy head.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Remove all duplicate-valued nodes from sorted list.
        ///
        /// # Arguments
        /// * `head` - sorted linked list
        ///
        /// # Returns
        /// List with only distinct values
        pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            // Collect values, filter unique, rebuild — simpler in safe Rust
            let mut vals = Vec::new();
            let mut curr = &head;
            while let Some(node) = curr {
                vals.push(node.val);
                curr = &node.next;
            }

            let mut unique = Vec::new();
            let mut i = 0;
            while i < vals.len() {
                let mut j = i + 1;
                while j < vals.len() && vals[j] == vals[i] {
                    j += 1;
                }
                if j - i == 1 {
                    unique.push(vals[i]);
                }
                i = j;
            }

            // Rebuild list from unique values
            let mut head = None;
            for &v in unique.iter().rev() {
                let mut node = ListNode::new(v);
                node.next = head;
                head = Some(Box::new(node));
            }
            head
        }
    }
}

fn main() {
    let list = build_list(&[1, 2, 3, 3, 4, 4, 5]);
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
                    let list = build_list(&[1, 2, 3, 3, 4, 4, 5]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![1, 2, 5]);
                }

                #[test]
                fn test_example2() {
                    let list = build_list(&[1, 1, 1, 2, 3]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![2, 3]);
                }

                #[test]
                fn test_all_dups() {
                    let list = build_list(&[1, 1, 2, 2]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![]);
                }

                #[test]
                fn test_no_dups() {
                    let list = build_list(&[1, 2, 3]);
                    let result = list_to_vec(<$solution>::delete_duplicates(list));
                    assert_eq!(result, vec![1, 2, 3]);
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
                let count = if i % 3 == 0 { 2 } else { 1 };
                for _ in 0..count {
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
