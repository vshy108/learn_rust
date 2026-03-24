// Demonstrates last, nth, nth_back, and position methods.
//
// Run: cargo run --example s21_last_nth_position

fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // --- 1. last ---
    let last = v.iter().last();
    println!("last: {last:?}"); // Some(&50)

    // --- 2. nth (0-indexed) ---
    let second = v.iter().nth(1);
    let tenth = v.iter().nth(10);
    println!("nth(1): {second:?}"); // Some(&20)
    println!("nth(10): {tenth:?}"); // None

    // --- 3. nth_back (from the end) ---
    let back = v.iter().nth_back(0); // last element
    let back1 = v.iter().nth_back(1); // second to last
    println!("nth_back(0): {back:?}"); // Some(&50)
    println!("nth_back(1): {back1:?}"); // Some(&40)

    // --- 4. position (find index of match) ---
    let pos = v.iter().position(|&n| n == 30);
    println!("position of 30: {pos:?}"); // Some(2)

    let pos = v.iter().position(|&n| n == 99);
    println!("position of 99: {pos:?}"); // None

    // --- 5. rposition (from the end) ---
    let nums = vec![1, 2, 3, 2, 1];
    let rpos = nums.iter().rposition(|&n| n == 2);
    println!("rposition of 2: {rpos:?}"); // Some(3)

    // --- 6. Practical: find index of first negative ---
    let data = vec![5, 3, -1, 7, -2];
    let neg_pos = data.iter().position(|&n| n < 0);
    println!("first negative at: {neg_pos:?}"); // Some(2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_last() {
        assert_eq!(vec![1, 2, 3].iter().last(), Some(&3));
        assert_eq!(Vec::<i32>::new().iter().last(), None);
    }

    #[test]
    fn test_nth() {
        assert_eq!(vec![10, 20, 30].iter().nth(1), Some(&20));
        assert_eq!(vec![10, 20, 30].iter().nth(5), None);
    }

    #[test]
    fn test_position() {
        let v = vec![1, 2, 3];
        assert_eq!(v.iter().position(|&n| n == 2), Some(1));
        assert_eq!(v.iter().position(|&n| n == 5), None);
    }
}
