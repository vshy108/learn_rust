// Demonstrates the #[should_panic] attribute.
//
// `#[should_panic]` marks a test that is expected to panic.
// Optionally specify the expected panic message.
//
// Run: cargo test --example s22_should_panic

/// Divide two numbers, panics on divide by zero.
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("division by zero");
    }
    a / b
}

/// Get element at index, panics if out of bounds.
fn get_element(v: &[i32], index: usize) -> i32 {
    if index >= v.len() {
        panic!("index {} out of bounds for length {}", index, v.len());
    }
    v[index]
}

fn main() {
    println!("divide(10, 3) = {}", divide(10.0, 3.0));
    println!("get_element([1,2,3], 1) = {}", get_element(&[1, 2, 3], 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10.0, 2.0), 5.0);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10.0, 0.0); // should panic
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_divide_by_zero_message() {
        // The panic message must contain "division by zero"
        divide(1.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_out_of_bounds() {
        get_element(&[1, 2, 3], 10);
    }

    #[test]
    fn test_valid_index() {
        assert_eq!(get_element(&[10, 20, 30], 1), 20);
    }
}
