// Demonstrates multiple impl blocks for the same struct.
//
// Rust allows splitting methods across multiple `impl` blocks.
// All blocks for the same type are logically combined by the compiler.
//
// This is useful for:
// - Organizing methods by category.
// - Conditional compilation (`#[cfg(...)]`).
// - Implementing traits in separate blocks (required by Rust).
//
// Run: cargo run --example s09_multiple_impl_blocks

use std::fmt;

#[derive(Debug, Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

// --- Block 1: Constructors ---
impl Matrix {
    /// Creates a matrix filled with zeros.
    fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Creates an identity matrix.
    fn identity(size: usize) -> Self {
        let mut m = Self::zeros(size, size);
        for i in 0..size {
            m.data[i * size + i] = 1.0;
        }
        m
    }
}

// --- Block 2: Accessors ---
impl Matrix {
    /// Gets the element at (row, col).
    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    /// Sets the element at (row, col).
    fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row * self.cols + col] = val;
    }

    /// Returns the dimensions as (rows, cols).
    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

// --- Block 3: Operations ---
impl Matrix {
    /// Multiplies every element by a scalar.
    fn scale(&mut self, factor: f64) {
        for val in &mut self.data {
            *val *= factor;
        }
    }

    /// Returns the sum of all elements.
    fn sum(&self) -> f64 {
        self.data.iter().sum()
    }
}

// --- Block 4: Display trait (must be a separate impl block) ---
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            let vals: Vec<String> = self.data[start..end]
                .iter()
                .map(|v| format!("{v:6.2}"))
                .collect();
            writeln!(f, "[ {} ]", vals.join(", "))?;
        }
        Ok(())
    }
}

fn main() {
    // --- 1. Using constructors (block 1) ---
    let id = Matrix::identity(3);
    println!("Identity 3x3:\n{id}");

    // --- 2. Using accessors (block 2) ---
    let mut m = Matrix::zeros(2, 3);
    m.set(0, 0, 1.0);
    m.set(0, 2, 3.0);
    m.set(1, 1, 5.0);
    println!("Custom matrix ({:?}):\n{m}", m.shape());

    // --- 3. Using operations (block 3) ---
    println!("Sum: {}", m.sum());
    m.scale(2.0);
    println!("After scale(2.0):\n{m}");

    // All methods work together seamlessly despite being in different blocks.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let m = Matrix::zeros(2, 3);
        assert_eq!(m.shape(), (2, 3));
        assert!((m.sum() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_identity() {
        let m = Matrix::identity(3);
        assert!((m.get(0, 0) - 1.0).abs() < f64::EPSILON);
        assert!((m.get(0, 1) - 0.0).abs() < f64::EPSILON);
        assert!((m.get(1, 1) - 1.0).abs() < f64::EPSILON);
        assert!((m.sum() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_set() {
        let mut m = Matrix::zeros(2, 2);
        m.set(1, 0, 42.0);
        assert!((m.get(1, 0) - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scale() {
        let mut m = Matrix::identity(2);
        m.scale(5.0);
        assert!((m.get(0, 0) - 5.0).abs() < f64::EPSILON);
        assert!((m.get(0, 1) - 0.0).abs() < f64::EPSILON);
    }
}
