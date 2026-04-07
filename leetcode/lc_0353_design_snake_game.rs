// @lc app=leetcode id=353 lang=rust
// #[Medium] - Design Snake Game
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.241us
// ai_solution  | avg: 0.134us
// Performance: AI 1.79x faster

/// # 353. Design Snake Game
///
/// Design a Snake game on a height x width grid.
/// Snake starts at (0,0) with length 1. Eating food increases length and score.
/// Game over if snake hits wall or itself.
///
/// Constraints:
/// - 1 <= width, height <= 10^4
/// - 1 <= food.length <= 50
/// - At most 10^4 calls to move

/// ## My Solution — VecDeque + HashSet (encode position)
///
/// Use a VecDeque for the snake body (head at front, tail at back).
/// Use a HashSet of encoded positions (row*width+col) to check self-collision in O(1).
///
/// - Time: O(1) per move
/// - Space: O(snake_length)
mod my_solution {
    use std::collections::{HashSet, VecDeque};

    /// Simulate the snake game. Returns the result of each move.
    pub fn play(
        width: usize,
        height: usize,
        food: &[[usize; 2]],
        moves: &[u8],
    ) -> Vec<i32> {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut vis: HashSet<usize> = HashSet::new();
        q.push_back(0); // start at (0,0) encoded as 0
        vis.insert(0);
        let mut score = 0usize;
        let mut food_idx = 0;
        let mut results = Vec::with_capacity(moves.len());

        for &dir in moves {
            let head = *q.front().expect("snake must have a head");
            let r = head / width;
            let c = head % width;

            let (nr, nc) = match dir {
                b'U' => (r.wrapping_sub(1), c),
                b'D' => (r + 1, c),
                b'L' => (r, c.wrapping_sub(1)),
                b'R' => (r, c + 1),
                _ => unreachable!(),
            };

            // Check wall collision (wrapping_sub gives usize::MAX on underflow)
            if nr >= height || nc >= width {
                results.push(-1);
                continue;
            }

            let new_pos = nr * width + nc;

            // Check if eating food
            if food_idx < food.len() && nr == food[food_idx][0] && nc == food[food_idx][1] {
                score += 1;
                food_idx += 1;
                // Don't remove tail — snake grows
            } else {
                // Remove tail
                let tail = q.pop_back().expect("snake must have a tail");
                vis.remove(&tail);
            }

            // Check self-collision (after removing tail, before adding new head)
            if vis.contains(&new_pos) {
                results.push(-1);
                continue;
            }

            q.push_front(new_pos);
            vis.insert(new_pos);
            results.push(score as i32);
        }
        results
    }
}

/// ## AI Solution — Vec-based deque without HashSet
///
/// Use a Vec as a deque (push front via insert(0,..)). Check collision by
/// linear scan. Simpler but O(n) per move for collision check.
///
/// - Time: O(snake_length) per move
/// - Space: O(snake_length)
mod ai_solution {
    /// Simulate the snake game. Returns the result of each move.
    pub fn play(
        width: usize,
        height: usize,
        food: &[[usize; 2]],
        moves: &[u8],
    ) -> Vec<i32> {
        let mut snake: Vec<usize> = vec![0]; // head at index 0
        let mut score = 0usize;
        let mut food_idx = 0;
        let mut results = Vec::with_capacity(moves.len());

        for &dir in moves {
            let head = snake[0];
            let r = head / width;
            let c = head % width;

            let (nr, nc) = match dir {
                b'U' => (r.wrapping_sub(1), c),
                b'D' => (r + 1, c),
                b'L' => (r, c.wrapping_sub(1)),
                b'R' => (r, c + 1),
                _ => unreachable!(),
            };

            if nr >= height || nc >= width {
                results.push(-1);
                continue;
            }

            let new_pos = nr * width + nc;

            if food_idx < food.len() && nr == food[food_idx][0] && nc == food[food_idx][1] {
                score += 1;
                food_idx += 1;
            } else {
                snake.pop(); // remove tail
            }

            // Linear scan for self-collision
            if snake.contains(&new_pos) {
                results.push(-1);
                continue;
            }

            snake.insert(0, new_pos);
            results.push(score as i32);
        }
        results
    }
}

fn run_tests(f: fn(usize, usize, &[[usize; 2]], &[u8]) -> Vec<i32>) {
    // Example: width=3, height=2, food=[[1,2],[0,1]]
    // Moves: R, D, R, U, L, U
    let result = f(3, 2, &[[1, 2], [0, 1]], b"RDRULU");
    assert_eq!(result, vec![0, 0, 1, 1, 2, -1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::play);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::play);
    }
}

fn main() {
    let food = [[1, 2], [0, 1]];
    let moves = b"RDRULU";
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::play(3, 2, std::hint::black_box(&food), moves));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::play(3, 2, std::hint::black_box(&food), moves));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::play(3, 2, std::hint::black_box(&food), moves));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::play(3, 2, std::hint::black_box(&food), moves));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 353 - Design Snake Game");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
