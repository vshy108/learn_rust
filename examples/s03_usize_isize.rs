// Demonstrates `usize` and `isize` — pointer-sized integer types.
//
// `usize`: unsigned, same size as a pointer on the target platform
//          (8 bytes on 64-bit, 4 bytes on 32-bit)
// `isize`: signed, same size as a pointer
//
// These are used for indexing collections and representing sizes/lengths.
//
// Run: cargo run --example s03_usize_isize

fn main() {
    // --- 1. usize is the index type ---
    // Array/vector indexing REQUIRES usize. You can't use i32 directly.
    let arr = [10, 20, 30, 40, 50];
    let idx: usize = 2;
    println!("arr[{idx}] = {}", arr[idx]);

    // Trying to index with i32 won't compile:
    // let i: i32 = 2;
    // arr[i]  // error[E0277]: the type `[i32]` cannot be indexed by `i32`

    // --- 2. .len() returns usize ---
    // All collection lengths are usize.
    let v = vec![1, 2, 3];
    let len: usize = v.len();
    println!("vec len = {len}");

    // --- 3. Platform-dependent size ---
    println!("size of usize: {} bytes", std::mem::size_of::<usize>());
    println!("size of isize: {} bytes", std::mem::size_of::<isize>());
    // On 64-bit: 8 bytes. On 32-bit: 4 bytes.

    println!("usize::MAX = {}", usize::MAX);
    // On 64-bit: 18_446_744_073_709_551_615

    // --- 4. isize for signed offsets ---
    // `isize` is used for pointer arithmetic (rare in safe Rust).
    let offset: isize = -3;
    println!("offset = {offset}");

    // --- 5. Converting between usize and other integers ---
    let x: i32 = 42;

    // i32 -> usize: use `as` (but beware of negative values!)
    let u: usize = x as usize;
    println!("i32 {x} as usize = {u}");

    // Safer: use TryFrom to catch errors
    let safe_u: Result<usize, _> = usize::try_from(x);
    println!("try_from: {safe_u:?}");

    // Negative -> usize wraps silently with `as`:
    let neg: i32 = -1;
    let wrapped = neg as usize;
    println!("-1i32 as usize = {wrapped}"); // Very large number!
    // This is why `TryFrom` is preferred for user input.

    // --- 6. usize in for loops ---
    // Range `0..n` produces usize values when `n` is usize.
    for i in 0..arr.len() {
        print!("arr[{i}]={} ", arr[i]);
    }
    println!();

    // --- Gotcha: usize subtraction can overflow ---
    // `0usize - 1` panics in debug mode (wraps in release).
    // Always check before subtracting from usize.
    let a: usize = 5;
    let b: usize = 3;
    if a >= b {
        println!("{a} - {b} = {}", a - b);
    }
    // Safer: use `checked_sub` or `saturating_sub`
    println!("0usize.saturating_sub(1) = {}", 0usize.saturating_sub(1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usize_is_pointer_sized() {
        assert_eq!(
            std::mem::size_of::<usize>(),
            std::mem::size_of::<*const u8>()
        );
    }

    #[test]
    fn test_try_from_negative_fails() {
        let result = usize::try_from(-1i32);
        assert!(result.is_err());
    }

    #[test]
    fn test_saturating_sub_at_zero() {
        assert_eq!(0usize.saturating_sub(1), 0);
    }

    #[test]
    fn test_len_is_usize() {
        let v = vec![1, 2, 3];
        let _len: usize = v.len(); // Confirms .len() returns usize
        assert_eq!(_len, 3);
    }
}
