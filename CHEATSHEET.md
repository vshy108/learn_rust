# Rust Cheatsheet

## Run

```bash
cargo run --example s01_println_macro
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all --check
```

## Ownership

```rust
let value = String::from("hello");
let moved = value;
// value is no longer usable here.

let borrowed = &moved;
println!("{borrowed}");
```

## Result And Option

```rust
fn parse_count(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse()
}

let maybe_first = [1, 2, 3].first().copied();
```

## Iterators

```rust
let doubled: Vec<_> = [1, 2, 3]
    .into_iter()
    .map(|n| n * 2)
    .collect();
```

## Traits

```rust
trait Describe {
    fn describe(&self) -> String;
}
```

## Tests

```rust
#[test]
fn adds_numbers() {
    assert_eq!(2 + 2, 4);
}
```