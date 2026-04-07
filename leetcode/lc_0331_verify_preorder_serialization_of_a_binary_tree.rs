// @lc app=leetcode id=331 lang=rust
// #[Medium] - Verify Preorder Serialization of a Binary Tree
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.116us
// ai_solution  | avg: 0.215us
// Performance: My 1.85x faster

/// # 331. Verify Preorder Serialization of a Binary Tree
///
/// Given a comma-separated string `preorder`, return `true` if it is a
/// correct preorder traversal serialization of a binary tree. `#` means null.
///
/// Constraints:
/// - 1 <= preorder.length <= 10^4
/// - Values are integers in [0, 100] or '#'

/// ## My Solution — Slot counting
///
/// A valid binary tree has exactly n+1 nulls for n non-null nodes.
/// Track available "slots": start with 1; each non-null consumes 1 and
/// adds 2; each null consumes 1. Slots must never go negative.
///
/// - Time: O(n)
/// - Space: O(1)
mod my_solution {
    pub fn is_valid_serialization(preorder: &str) -> bool {
        let mut slots: i32 = 1; // start with one slot for root
        for token in preorder.split(',') {
            // Consume a slot
            slots -= 1;
            if slots < 0 {
                return false; // more nodes than available slots
            }
            if token != "#" {
                // Non-null node creates 2 child slots
                slots += 2;
            }
        }
        // All slots must be consumed
        slots == 0
    }
}

/// ## AI Solution — Stack reduction
///
/// Push tokens onto a stack. Whenever the top two are "#" and the one
/// below them is not "#", replace all three with a single "#".
/// At the end, the stack should contain exactly one "#".
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    pub fn is_valid_serialization(preorder: &str) -> bool {
        let mut stk: Vec<&str> = Vec::new();
        for token in preorder.split(',') {
            stk.push(token);
            // Reduce: if top two are "#" and third is non-"#", collapse
            while stk.len() >= 3
                && stk[stk.len() - 1] == "#"
                && stk[stk.len() - 2] == "#"
                && stk[stk.len() - 3] != "#"
            {
                stk.truncate(stk.len() - 3);
                stk.push("#");
            }
        }
        stk.len() == 1 && stk[0] == "#"
    }
}

fn run_tests(f: fn(&str) -> bool) {
    assert!(f("9,3,4,#,#,1,#,#,2,#,6,#,#"));
    assert!(!f("1,#"));
    assert!(!f("9,#,#,1"));
    assert!(f("#")); // single null = empty tree
    assert!(!f("1,#,#,#")); // extra null
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::is_valid_serialization);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::is_valid_serialization);
    }
}

fn main() {
    let preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#";
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::is_valid_serialization(
            std::hint::black_box(preorder),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::is_valid_serialization(
            std::hint::black_box(preorder),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::is_valid_serialization(
            std::hint::black_box(preorder),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::is_valid_serialization(
            std::hint::black_box(preorder),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 331 - Verify Preorder Serialization of a Binary Tree");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
