// LeetCode 143: Reorder List [Medium]
// https://leetcode.com/problems/reorder-list/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 6.087us
//   AI solution avg: 2.938us
//   Ratio: AI 2.07x faster
//
// Given the head of a linked list, reorder it in-place:
// L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...
//
// Constraints: [1, 5*10^4] nodes, 1 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0143_reorder_list
// Test: cargo test --example lc_0143_reorder_list
// Bench: cargo test --example lc_0143_reorder_list bench -- --nocapture

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

/// My solution — collect to vec, then rebuild using two-pointer interleave.
///
/// Time: O(n).
/// Space: O(n) for the vec.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Reorder the list in-place.
        pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
            let vals = to_vec(head);
            if vals.len() <= 2 { return; }

            // Interleave front and back using two pointers
            let mut reordered = Vec::with_capacity(vals.len());
            let (mut l, mut r) = (0, vals.len() - 1);
            while l <= r {
                reordered.push(vals[l]);
                if l != r {
                    reordered.push(vals[r]);
                }
                l += 1;
                if r == 0 { break; }
                r -= 1;
            }

            // Rebuild linked list from reordered values
            *head = build_list(&reordered);
        }
    }
}

/// AI solution — find middle, reverse second half, merge two halves.
///
/// Time: O(n).
/// Space: O(1) extra.
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Reorder in-place: split, reverse, merge.
        pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
            if head.is_none() {
                return;
            }

            // Step 1: find the middle using slow/fast pointers (via length)
            let len = {
                let mut count = 0;
                let mut cur = head.as_ref();
                while let Some(node) = cur {
                    count += 1;
                    cur = node.next.as_ref();
                }
                count
            };
            if len <= 2 { return; }

            // Split at the midpoint — keep first ceil(n/2) nodes in first half
            let mid = len / 2;
            let mut cur = head.as_mut().expect("non-empty");
            for _ in 0..mid {
                cur = cur.next.as_mut().expect("within bounds");
            }
            let second_half = cur.next.take();

            // Step 2: reverse the second half
            let mut prev: Option<Box<ListNode>> = None;
            let mut curr = second_half;
            while let Some(mut node) = curr {
                curr = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
            let mut reversed = prev;

            // Step 3: merge the two halves
            let mut cur = head.as_mut();
            while let Some(node) = cur {
                if let Some(mut r_node) = reversed.take() {
                    reversed = r_node.next.take();
                    r_node.next = node.next.take();
                    node.next = Some(r_node);
                    cur = node.next.as_mut()
                        .and_then(|n| n.next.as_mut());
                } else {
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut list = build_list(&[1, 2, 3, 4]);
    my_solution::Solution::reorder_list(&mut list);
    println!("my: {:?}", to_vec(&list));

    let mut list2 = build_list(&[1, 2, 3, 4]);
    ai_solution::Solution::reorder_list(&mut list2);
    println!("ai: {:?}", to_vec(&list2));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_reorder {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let mut list = build_list(&[1, 2, 3, 4]);
                    <$solution>::reorder_list(&mut list);
                    assert_eq!(to_vec(&list), vec![1, 4, 2, 3]);
                }

                #[test]
                fn test_example2() {
                    let mut list = build_list(&[1, 2, 3, 4, 5]);
                    <$solution>::reorder_list(&mut list);
                    assert_eq!(to_vec(&list), vec![1, 5, 2, 4, 3]);
                }

                #[test]
                fn test_single() {
                    let mut list = build_list(&[1]);
                    <$solution>::reorder_list(&mut list);
                    assert_eq!(to_vec(&list), vec![1]);
                }

                #[test]
                fn test_two() {
                    let mut list = build_list(&[1, 2]);
                    <$solution>::reorder_list(&mut list);
                    assert_eq!(to_vec(&list), vec![1, 2]);
                }

                #[test]
                fn test_three() {
                    let mut list = build_list(&[1, 2, 3]);
                    <$solution>::reorder_list(&mut list);
                    assert_eq!(to_vec(&list), vec![1, 3, 2]);
                }
            }
        };
    }

    test_reorder!(my, crate::my_solution::Solution);
    test_reorder!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let vals: Vec<i32> = (1..=50).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut list = build_list(&vals);
            crate::my_solution::Solution::reorder_list(
                std::hint::black_box(&mut list),
            );
            std::hint::black_box(&list);
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut list = build_list(&vals);
            crate::ai_solution::Solution::reorder_list(
                std::hint::black_box(&mut list),
            );
            std::hint::black_box(&list);
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
