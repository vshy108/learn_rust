# Learn Rust

A comprehensive Rust learning project with **324 standalone examples** across **27 sections**, progressing from basics to smart pointers. Each topic is a self-contained file with `fn main()`, inline comments explaining Rust-specific nuances, and unit tests.

The topic list and section structure are based on the Udemy course [Learn to Code with Rust](https://www.udemy.com/course/learn-to-code-with-rust/). Thanks to the course author for the excellent curriculum.

## Quick Start

```bash
# Run any example
cargo run --example s01_println_macro

# Run a specific example's tests
cargo test --example s18_clone_trait

# Run all tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all --check
```

## Project Structure

```
learn_rust/
├── Cargo.toml                       # Workspace root
├── examples/
│   ├── s01_*.rs  ..  s27_*.rs       # 324 standalone example files
├── crates/
│   ├── s14_project_structure/       # Multi-file module demos
│   └── s21_wordle/                  # Wordle game project
├── src/main.rs
├── PLAN.md                          # Full roadmap with all 339 topics
└── README.md
```

**Convention**: one file per topic, named `s{NN}_{topic}.rs`. Cargo auto-discovers them as runnable examples.

## Sections

| # | Section | Examples | Key Topics |
|---|---------|----------|------------|
| 1 | Basics | 7 | `println!`, comments, interpolation, `cargo fmt/build/run` |
| 2 | Variables and Mutability | 8 | shadowing, scopes, constants, type aliases, compiler directives |
| 3 | Data Types | 19 | integers, floats, `as`, bool/char sizes, arrays, tuples, ranges, generics intro |
| 4 | Functions | 2 | parameters vs arguments, unit type `()` |
| 5 | Control Flow | 5 | `if let`, `while let`, `match`, recursion |
| 6 | Ownership | 18 | stack vs heap, `Copy`/`Clone`/`Drop`, `String`, references, borrowing |
| 7 | References and Borrowing | 5 | mutable/immutable refs, borrow rules, ownership with arrays/tuples |
| 8 | Slices | 6 | string slices, array slices, deref coercion |
| 9 | Structs | 17 | methods, `self`, associated fns, builder pattern, `Debug` |
| 10 | Enums | 10 | associated values, memory layout, `match`, `if let`, `let else` |
| 11 | Generics | 5 | turbofish, multi-generics, impl blocks, enums |
| 12 | Option and Result | 17 | `Option<T>`, `Result<T,E>`, `unwrap`, `?`, `while let`, `'static` |
| 13 | Vectors | 7 | create, add/remove, `get`, ownership, capacity |
| 14 | Project Structure | 16 + crate | modules, `pub`, `use`, `super`, `as`, doc comments, lib/bin crates |
| 15 | Strings | 5 | concatenation, `format!`, methods, user input |
| 16 | HashMaps | 8 | create, remove, `entry`, ownership, `HashSet` |
| 17 | Error Handling | 11 | `panic!`, file I/O, `?` operator, error propagation |
| 18 | Traits | 35 | defining/implementing, bounds, `dyn`, Display/Debug/Clone/Copy/Drop/PartialEq/Ord, associated types, async mock |
| 19 | Lifetimes | 13 | concrete lifetimes, NLL, elision rules, struct lifetimes, `'static` |
| 20 | Closures | 14 | `Fn`/`FnMut`/`FnOnce`, captures, `move`, `retain` |
| 21 | Iterators | 33 + crate | `map`, `filter`, `fold`, `collect`, `flat_map`, `enumerate`, `zip`, sorting, Wordle project |
| 22 | Testing | 21 | `assert_*!`, `#[should_panic]`, `Result` in tests, doc-tests, `rstest`, TDD |
| 23 | Randomness (`rand`) | 3 | `thread_rng`, `gen_range`, `shuffle` |
| 24 | Datetimes (`chrono`) | 8 | `NaiveDate`, `DateTime`, timezones, parsing, formatting |
| 25 | Regex (`regex`) | 13 | `find`, `captures`, character classes, anchors, `replace_all` |
| 26 | Smart Pointers I: Box | 16 | raw pointers, `Box`, linked list, BST, `Deref`/`Drop`, trait objects, `thiserror` |
| 27 | Smart Pointers II | 2 | `Cell`, `RefCell`, interior mutability |

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1 | Async runtime (full features) |
| `rand` | 0.9 | Random number generation |
| `chrono` | 0.4 | Date and time |
| `regex` | 1 | Regular expressions |
| `thiserror` | 2 | Custom error types |
| `mockall` | 0.13 | Mocking (dev) |
| `pretty_assertions` | 1 | Colored test diffs (dev) |
| `rstest` | 0.23 | Test fixtures (dev) |

## Requirements

- Rust edition 2024 (rustc 1.85+)
- `cargo`, `rustfmt`, `clippy`

## License

This is a personal learning project.
