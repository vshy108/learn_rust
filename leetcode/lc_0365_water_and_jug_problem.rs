// @lc app=leetcode id=365 lang=rust
// #[Medium] - Water and Jug Problem
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.003us
// ai_solution  | avg: 1.224us
// Performance: My 371.33x faster

/// # 365. Water and Jug Problem
///
/// Given two jugs with capacities x and y liters, return whether the total amount
/// of water in both jugs may reach `target` using fill, empty, or pour operations.
///
/// Constraints:
/// - 1 <= x, y, target <= 1000

/// ## My Solution — Math (Bezout's identity)
///
/// target is achievable iff target <= x + y and target % gcd(x, y) == 0.
/// By Bezout's identity, any multiple of gcd(x,y) up to x+y is reachable.
///
/// - Time: O(log(min(x, y))) for GCD
/// - Space: O(1)
mod my_solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        if target > x + y {
            return false;
        }
        target % gcd(x, y) == 0
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
}

/// ## AI Solution — BFS state exploration
///
/// States are (water_in_jug1, water_in_jug2). From each state, try all 6 operations.
/// Use a HashSet to avoid revisiting states.
///
/// - Time: O(x * y)
/// - Space: O(x * y)
mod ai_solution {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0i32, 0i32));
        visited.insert((0, 0));

        while let Some((a, b)) = queue.pop_front() {
            if a == target || b == target || a + b == target {
                return true;
            }
            // Generate all 6 possible next states
            let next_states = [
                (x, b), // fill jug1
                (a, y), // fill jug2
                (0, b), // empty jug1
                (a, 0), // empty jug2
                // Pour jug1 -> jug2
                {
                    let pour = a.min(y - b);
                    (a - pour, b + pour)
                },
                // Pour jug2 -> jug1
                {
                    let pour = b.min(x - a);
                    (a + pour, b - pour)
                },
            ];
            for state in next_states {
                if visited.insert(state) {
                    queue.push_back(state);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32, i32, i32) -> bool) {
        assert!(f(3, 5, 4));
        assert!(!f(2, 6, 5));
        assert!(f(1, 2, 3));
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::can_measure_water);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::can_measure_water);
    }
}

fn main() {
    let (x, y, target) = (3, 5, 4);
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::can_measure_water(x, y, target));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::can_measure_water(x, y, target));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::can_measure_water(x, y, target));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::can_measure_water(x, y, target));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 365 - Water and Jug Problem");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
