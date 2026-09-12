// Demonstrates declarative macros with repetition.

macro_rules! say_hello {
    () => {
        println!("hello from macro_rules!");
    };
}

macro_rules! sum {
    ($($value:expr),* $(,)?) => {
        0 $(+ $value)*
    };
}

fn compute_demo_sum() -> i32 {
    sum!(1, 2, 3, 4)
}

fn main() {
    say_hello!();

    let total = compute_demo_sum();
    println!("sum = {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_macro_handles_values() {
        let result = sum!(10, 20, 30);
        assert_eq!(result, 60);
    }

    #[test]
    fn sum_macro_supports_trailing_comma() {
        let result = sum!(1, 2, 3,);
        assert_eq!(result, 6);
    }

    #[test]
    fn compute_demo_sum_matches_expected_total() {
        assert_eq!(compute_demo_sum(), 10);
    }
}
