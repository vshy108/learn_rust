// LeetCode 489: Robot Room Cleaner [Hard]
// https://leetcode.com/problems/robot-room-cleaner/
//
// Given a robot cleaner in a room modeled as an m x n grid (0 = wall,
// 1 = empty), design an algorithm to clean the entire room using only the
// Robot API. The robot starts at an unknown position facing up. You have
// no direct access to the grid.
//
// Robot API:
//   move()       -> bool  (attempt to move forward; returns false if blocked)
//   turn_left()           (rotate 90 degrees counter-clockwise)
//   turn_right()          (rotate 90 degrees clockwise)
//   clean()               (clean the current cell)
//
// We simulate the Robot locally since there is no online judge interface.
//
// Examples:
//   room = [[1,1,1,1,1,0,1,1],
//           [1,1,1,1,1,0,1,1],
//           [1,0,1,1,1,1,1,1],
//           [0,0,0,1,0,0,0,0],
//           [1,1,1,1,1,1,1,1]]
//   row = 1, col = 3  ->  all reachable empty cells cleaned
//
// Constraints:
//   1 <= m <= 100, 1 <= n <= 200
//   room[i][j] is 0 or 1
//   robot starts on an empty cell facing up
//
// Run:  cargo run --example lc_0489_robot_room_cleaner
// Test: cargo test --example lc_0489_robot_room_cleaner
// Bench: cargo test --example lc_0489_robot_room_cleaner bench -- --nocapture
//
// First benchmark (50K iterations, 2026-04-08):
//   My solution: avg 40.738us
//   AI solution: avg 43.364us
//   Ratio: 0.94x (similar DFS backtracking)

use std::collections::HashSet;

/// Simulated Robot that wraps a grid, tracks position, direction, and
/// which cells have been cleaned.
struct Robot {
    room: Vec<Vec<i32>>,
    row: i32,
    col: i32,
    /// Direction index: 0=up, 1=right, 2=down, 3=left
    dir: usize,
    /// Set of (row, col) cells that have been cleaned (absolute coords)
    cleaned: HashSet<(i32, i32)>,
}

impl Robot {
    /// Create a new Robot placed at (row, col) facing up.
    ///
    /// # Arguments
    /// * `room` - 2D grid where 1 = empty, 0 = wall
    /// * `row`  - Starting row (absolute)
    /// * `col`  - Starting column (absolute)
    fn new(room: Vec<Vec<i32>>, row: i32, col: i32) -> Self {
        Self {
            room,
            row,
            col,
            dir: 0,
            cleaned: HashSet::new(),
        }
    }

    /// Direction vectors indexed by dir: up, right, down, left
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    /// Attempt to move one step forward. Returns `true` if the cell
    /// ahead is in-bounds and empty (1); updates position accordingly.
    fn move_robot(&mut self) -> bool {
        let (dr, dc) = Self::DIRS[self.dir];
        let nr = self.row + dr;
        let nc = self.col + dc;
        let rows = self.room.len() as i32;
        let cols = self.room[0].len() as i32;
        if nr >= 0 && nr < rows && nc >= 0 && nc < cols && self.room[nr as usize][nc as usize] == 1
        {
            self.row = nr;
            self.col = nc;
            true
        } else {
            false
        }
    }

    /// Rotate 90 degrees counter-clockwise.
    fn turn_left(&mut self) {
        // (dir + 3) % 4 rotates left in our up/right/down/left scheme
        self.dir = (self.dir + 3) % 4;
    }

    /// Rotate 90 degrees clockwise.
    fn turn_right(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }

    /// Clean the current cell and record it.
    fn clean(&mut self) {
        self.cleaned.insert((self.row, self.col));
    }
}

/// Time: O(m*n), Space: O(m*n)
/// DFS backtracking with direction order: up, right, down, left.
/// At each cell clean it, then try all 4 directions. If the next cell
/// is unvisited and reachable, recurse into it, then backtrack
/// (turn 180, move, turn 180 to restore orientation).
#[allow(dead_code)]
mod my_solution {
    use std::collections::HashSet;

    use super::Robot;

    /// Clean all reachable empty cells starting from the robot's position.
    ///
    /// # Arguments
    /// * `robot` - Mutable reference to the simulated Robot
    pub fn clean_room(robot: &mut Robot) {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        dfs(robot, 0, 0, 0, &mut visited);
    }

    /// Recursive DFS using relative coordinates from the starting cell.
    ///
    /// # Arguments
    /// * `robot`   - The robot reference
    /// * `r`       - Current relative row
    /// * `c`       - Current relative col
    /// * `dir`     - Robot's current facing direction (0=up,1=right,2=down,3=left)
    /// * `visited` - Set of visited relative positions
    fn dfs(robot: &mut Robot, r: i32, c: i32, dir: usize, visited: &mut HashSet<(i32, i32)>) {
        robot.clean();
        visited.insert((r, c));

        // Direction deltas: up, right, down, left (matching Robot::DIRS)
        let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for i in 0..4 {
            // Compute the actual direction we want to try
            let new_dir = (dir + i) % 4;
            let (dr, dc) = dirs[new_dir];
            let nr = r + dr;
            let nc = c + dc;

            if !visited.contains(&(nr, nc)) && robot.move_robot() {
                dfs(robot, nr, nc, new_dir, visited);
                // Backtrack: turn 180, move back, turn 180 to restore facing
                robot.turn_right();
                robot.turn_right();
                robot.move_robot();
                robot.turn_right();
                robot.turn_right();
            }
            // Turn right to try the next direction on next iteration
            robot.turn_right();
        }
    }
}

/// Time: O(m*n), Space: O(m*n)
/// Same DFS backtracking but explores in clockwise spiral order:
/// right, down, left, up. Produces a different traversal sequence but
/// identical asymptotic complexity and correctness.
#[allow(dead_code)]
mod ai_solution {
    use std::collections::HashSet;

    use super::Robot;

    /// Clean all reachable empty cells starting from the robot's position.
    ///
    /// # Arguments
    /// * `robot` - Mutable reference to the simulated Robot
    pub fn clean_room(robot: &mut Robot) {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        dfs(robot, 0, 0, 0, &mut visited);
    }

    /// Recursive DFS starting with right-first spiral order.
    ///
    /// Direction mapping: 0=up, 1=right, 2=down, 3=left.
    /// We begin exploring from direction index `(dir + 1) % 4` (i.e.
    /// right-first when initially facing up), giving a clockwise spiral.
    fn dfs(robot: &mut Robot, r: i32, c: i32, dir: usize, visited: &mut HashSet<(i32, i32)>) {
        robot.clean();
        visited.insert((r, c));

        let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for i in 0..4 {
            // Start from (dir + 1) to produce right-first spiral order
            let new_dir = (dir + 1 + i) % 4;
            let (dr, dc) = dirs[new_dir];
            let nr = r + dr;
            let nc = c + dc;

            if !visited.contains(&(nr, nc)) {
                // Orient robot to face new_dir by turning right from
                // current facing `dir` the appropriate number of times.
                let right_turns = (new_dir + 4 - dir) % 4;
                for _ in 0..right_turns {
                    robot.turn_right();
                }

                if robot.move_robot() {
                    dfs(robot, nr, nc, new_dir, visited);
                    // Backtrack: turn 180, move, turn 180
                    robot.turn_right();
                    robot.turn_right();
                    robot.move_robot();
                    robot.turn_right();
                    robot.turn_right();
                }

                // Turn back to original facing `dir`
                for _ in 0..right_turns {
                    robot.turn_left();
                }
            }
        }
    }
}

fn main() {
    println!("LeetCode 489: Robot Room Cleaner");
    let room = vec![
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let row = 1i32;
    let col = 3i32;

    // Count reachable empty cells via flood fill from (row, col)
    let reachable = count_reachable(&room, row, col);

    let mut robot = Robot::new(room.clone(), row, col);
    my_solution::clean_room(&mut robot);
    println!(
        "  my_solution cleaned {} / {} reachable cells",
        robot.cleaned.len(),
        reachable
    );

    let mut robot = Robot::new(room, row, col);
    ai_solution::clean_room(&mut robot);
    println!(
        "  ai_solution cleaned {} / {} reachable cells",
        robot.cleaned.len(),
        reachable
    );
}

/// Count cells reachable from (start_r, start_c) via DFS on the grid
/// (only traversing cells with value 1).
fn count_reachable(room: &[Vec<i32>], start_r: i32, start_c: i32) -> usize {
    let rows = room.len() as i32;
    let cols = room[0].len() as i32;
    let mut visited = HashSet::new();
    let mut stack = vec![(start_r, start_c)];
    visited.insert((start_r, start_c));
    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0
                && nr < rows
                && nc >= 0
                && nc < cols
                && room[nr as usize][nc as usize] == 1
                && !visited.contains(&(nr, nc))
            {
                visited.insert((nr, nc));
                stack.push((nr, nc));
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Count reachable empty cells from (start_r, start_c).
    fn reachable_cells(room: &[Vec<i32>], start_r: i32, start_c: i32) -> HashSet<(i32, i32)> {
        let rows = room.len() as i32;
        let cols = room[0].len() as i32;
        let mut visited = HashSet::new();
        let mut stack = vec![(start_r, start_c)];
        visited.insert((start_r, start_c));
        while let Some((r, c)) = stack.pop() {
            for (dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r + dr;
                let nc = c + dc;
                if nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && room[nr as usize][nc as usize] == 1
                    && !visited.contains(&(nr, nc))
                {
                    visited.insert((nr, nc));
                    stack.push((nr, nc));
                }
            }
        }
        visited
    }

    /// Helper: run a clean_room function and assert all reachable cells
    /// were cleaned.
    fn assert_cleans_all(room: Vec<Vec<i32>>, row: i32, col: i32, clean_fn: fn(&mut Robot)) {
        let expected = reachable_cells(&room, row, col);
        let mut robot = Robot::new(room, row, col);
        clean_fn(&mut robot);
        assert_eq!(
            robot.cleaned, expected,
            "cleaned {:?} but expected {:?}",
            robot.cleaned, expected
        );
    }

    // -- Example room from the problem description --

    fn example_room() -> Vec<Vec<i32>> {
        vec![
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ]
    }

    #[test]
    fn my_example() {
        assert_cleans_all(example_room(), 1, 3, my_solution::clean_room);
    }

    #[test]
    fn ai_example() {
        assert_cleans_all(example_room(), 1, 3, ai_solution::clean_room);
    }

    // -- Fully open room --

    #[test]
    fn my_open_room() {
        let room = vec![vec![1; 5]; 5];
        assert_cleans_all(room, 2, 2, my_solution::clean_room);
    }

    #[test]
    fn ai_open_room() {
        let room = vec![vec![1; 5]; 5];
        assert_cleans_all(room, 2, 2, ai_solution::clean_room);
    }

    // -- Single cell --

    #[test]
    fn my_single_cell() {
        let room = vec![vec![1]];
        assert_cleans_all(room, 0, 0, my_solution::clean_room);
    }

    #[test]
    fn ai_single_cell() {
        let room = vec![vec![1]];
        assert_cleans_all(room, 0, 0, ai_solution::clean_room);
    }

    // -- Narrow corridor --

    #[test]
    fn my_corridor() {
        let room = vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]];
        assert_cleans_all(room, 0, 5, my_solution::clean_room);
    }

    #[test]
    fn ai_corridor() {
        let room = vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]];
        assert_cleans_all(room, 0, 5, ai_solution::clean_room);
    }

    // -- L-shaped room --

    #[test]
    fn my_l_shape() {
        let room = vec![vec![1, 1, 0], vec![1, 1, 0], vec![1, 1, 1]];
        assert_cleans_all(room, 0, 0, my_solution::clean_room);
    }

    #[test]
    fn ai_l_shape() {
        let room = vec![vec![1, 1, 0], vec![1, 1, 0], vec![1, 1, 1]];
        assert_cleans_all(room, 0, 0, ai_solution::clean_room);
    }

    // -- Disconnected region (robot can't reach right side) --

    #[test]
    fn my_disconnected() {
        let room = vec![vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1]];
        // Robot at (0,0) can only reach the left column
        assert_cleans_all(room, 0, 0, my_solution::clean_room);
    }

    #[test]
    fn ai_disconnected() {
        let room = vec![vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1]];
        assert_cleans_all(room, 0, 0, ai_solution::clean_room);
    }
}

#[cfg(test)]
fn bench_solution(name: &str, clean_fn: fn(&mut Robot)) {
    let iterations = 50_000;
    // Build a 20x20 room with scattered walls for non-trivial traversal
    let size = 20;
    let mut room = vec![vec![1i32; size]; size];
    for i in (0..size).step_by(3) {
        for j in (1..size - 1).step_by(4) {
            room[i][j] = 0;
        }
    }
    // Ensure start cell is open
    room[size / 2][size / 2] = 1;

    let start_time = std::time::Instant::now();
    for _ in 0..iterations {
        let mut robot = Robot::new(
            std::hint::black_box(room.clone()),
            (size / 2) as i32,
            (size / 2) as i32,
        );
        std::hint::black_box(clean_fn(&mut robot));
    }
    let elapsed = start_time.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::clean_room);
    bench_solution("AI solution", ai_solution::clean_room);
}
