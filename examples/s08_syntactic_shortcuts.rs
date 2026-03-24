// Demonstrates syntactic shortcuts for slicing.
//
// Rust provides shorthand for common slice range patterns:
//
// | Syntax      | Meaning              | Equivalent to      |
// |-------------|----------------------|--------------------|
// | `&s[..end]` | From start to `end`  | `&s[0..end]`       |
// | `&s[start..]` | From `start` to end | `&s[start..s.len()]` |
// | `&s[..]`    | The entire thing     | `&s[0..s.len()]`   |
// | `&s[..=end]` | Inclusive end        | `&s[0..end+1]`     |
//
// These work on both String slices (&str) and array/vec slices (&[T]).
//
// Run: cargo run --example s08_syntactic_shortcuts

fn main() {
    let s = String::from("hello world");

    // --- 1. From beginning: &s[..end] ---
    let hello = &s[..5]; // same as &s[0..5]
    println!("hello = \"{hello}\"");

    // --- 2. To the end: &s[start..] ---
    let world = &s[6..]; // same as &s[6..11]
    println!("world = \"{world}\"");

    // --- 3. Entire string: &s[..] ---
    let full = &s[..]; // same as &s[0..11]
    println!("full = \"{full}\"");

    // --- 4. Inclusive range: &s[start..=end] ---
    let hell = &s[0..=3]; // includes byte at index 3
    println!("inclusive = \"{hell}\""); // "hell"

    // --- 5. Same shortcuts on arrays ---
    let arr = [10, 20, 30, 40, 50];
    let first_three = &arr[..3]; // [10, 20, 30]
    let last_two = &arr[3..]; // [40, 50]
    let all = &arr[..]; // [10, 20, 30, 40, 50]
    let inclusive = &arr[1..=3]; // [20, 30, 40]
    println!("first_three = {first_three:?}");
    println!("last_two = {last_two:?}");
    println!("all = {all:?}");
    println!("inclusive = {inclusive:?}");

    // --- 6. Vectors too ---
    let v = vec!['a', 'b', 'c', 'd'];
    let middle = &v[1..3]; // ['b', 'c']
    println!("middle = {middle:?}");

    // --- 7. Empty slices ---
    let empty_str = &s[0..0]; // ""
    let empty_arr = &arr[2..2]; // []
    println!("empty_str = \"{empty_str}\", empty_arr = {empty_arr:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_start() {
        let s = String::from("abcde");
        assert_eq!(&s[..3], "abc");
    }

    #[test]
    fn test_to_end() {
        let s = String::from("abcde");
        assert_eq!(&s[2..], "cde");
    }

    #[test]
    fn test_full() {
        let s = String::from("abcde");
        assert_eq!(&s[..], "abcde");
    }

    #[test]
    fn test_inclusive_range() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(&arr[1..=3], &[2, 3, 4]);
    }

    #[test]
    fn test_empty_slice() {
        let arr = [1, 2, 3];
        let empty = &arr[1..1];
        assert!(empty.is_empty());
    }
}
