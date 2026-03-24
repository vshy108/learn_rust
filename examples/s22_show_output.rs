// Demonstrates showing printed output from passing tests.
//
// By default, `cargo test` captures stdout for passing tests.
// Use `cargo test -- --show-output` to see prints from passing tests.
//
// Run: cargo test --example s22_show_output -- --show-output

fn process_data(data: &[i32]) -> Vec<i32> {
    println!("Processing {} items", data.len());
    let result: Vec<i32> = data
        .iter()
        .map(|&n| {
            println!("  transforming {n} -> {}", n * 2);
            n * 2
        })
        .collect();
    println!("Done! Result has {} items", result.len());
    result
}

fn main() {
    let data = vec![1, 2, 3];
    let result = process_data(&data);
    println!("final: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        // The println! calls inside process_data will only be visible
        // if you run: cargo test -- --show-output
        let result = process_data(&[1, 2, 3]);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_empty() {
        println!("Testing empty input");
        let result = process_data(&[]);
        assert!(result.is_empty());
    }
}
