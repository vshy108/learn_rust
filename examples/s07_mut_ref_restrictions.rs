// Demonstrates mutable reference restrictions.
//
// Rust enforces strict rules around `&mut T`:
//
// 1. **Only one `&mut T` at a time** — no aliased mutable access.
// 2. **No `&T` while `&mut T` exists** — no reading during mutation.
// 3. **Non-Lexical Lifetimes (NLL)**: borrows end at their LAST USE,
//    not at the end of the scope. This makes the rules more ergonomic.
//
// These rules prevent data races at compile time — no runtime cost!
//
// Run: cargo run --example s07_mut_ref_restrictions

fn main() {
    // --- 1. Only one &mut at a time ---
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" world");
    // let r2 = &mut s; // error: can't borrow `s` as mutable more than once
    println!("{r1}");
    // After r1's last use, we can take a new &mut:
    let r2 = &mut s;
    r2.push_str("!");
    println!("{r2}");

    // --- 2. Can't mix &T and &mut T ---
    let mut v = vec![1, 2, 3];
    let immut = &v;
    println!("immut: {immut:?}");
    // immut's last use is above, so borrow ends here (NLL).
    // Now we can take &mut:
    let mutable = &mut v;
    mutable.push(4);
    println!("after push: {mutable:?}");

    // --- 3. NLL in action ---
    let mut data = String::from("data");
    let r = &data; // immutable borrow starts
    println!("read: {r}"); // last use of `r` — borrow ends
    data.push_str(" modified"); // OK: no active immutable borrow
    println!("modified: {data}");

    // --- 4. Why? Preventing iterator invalidation ---
    // In C++, you can push to a vector while iterating → UB.
    // Rust prevents this:
    let mut nums = vec![1, 2, 3];
    // for n in &nums {
    //     nums.push(*n); // error: can't borrow `nums` as mutable
    // }
    // Instead, collect changes and apply after:
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    nums.extend(doubled);
    println!("extended: {nums:?}");

    // --- 5. Reborrowing: a subtle but useful feature ---
    let mut x: i32 = 10;
    let r = &mut x;
    increment(r); // `r` is "reborrowed" here, not moved
    increment(r); // can use `r` again because it was reborrowed
    println!("x = {}", *r); // 12

    // --- 6. Scoped borrows ---
    let mut val = String::from("scoped");
    {
        let r = &mut val;
        r.push_str("!");
    } // r goes out of scope — borrow ends
    println!("{val}"); // OK: no active borrow
}

/// Increments a value through a mutable reference.
///
/// # Arguments
///
/// * `x` - Mutable reference to an i32
fn increment(x: &mut i32) {
    *x += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut val: i32 = 0;
        increment(&mut val);
        assert_eq!(val, 1);
        increment(&mut val);
        assert_eq!(val, 2);
    }

    #[test]
    fn test_nll_allows_sequential_borrows() {
        let mut s = String::from("test");
        let r = &s;
        let _ = r.len(); // last use of immutable borrow
        // Now mutable borrow is OK:
        s.push_str("!");
        assert_eq!(s, "test!");
    }

    #[test]
    fn test_scoped_borrow() {
        let mut v = vec![1, 2];
        {
            let r = &mut v;
            r.push(3);
        }
        assert_eq!(v, vec![1, 2, 3]);
    }
}
