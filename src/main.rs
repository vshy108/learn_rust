// `fn` declares a function. `main` is the entry point of every Rust binary.
// The curly braces `{}` define the function's block (body).
fn main() {
    // `println!` is a macro (indicated by the `!`), not a regular function.
    // It prints the given string to the standard output, followed by a newline.
    // Constants are `'static` by default in Rust, so `&str` is sufficient here
    // (no need to write `&'static str` explicitly — clippy will flag it).
    const HELLO_WORLD: &str = "Hello, world!";
    println!("{HELLO_WORLD}");

    // `overflowing_add` returns a tuple (result, overflowed): wraps on overflow
    // and returns `true` if overflow occurred. Here 250 + 10 = 260 wraps to 4
    // in u8 (260 - 256 = 4), so this prints `(4, true)`.
    // Use `{:?}` (Debug format) because tuples don't implement `Display`.
    println!("{:?}", 250u8.overflowing_add(10));
}
