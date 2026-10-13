# Rustfinity Challenges

Standard Rustfinity challenges live in this directory as one Cargo example per challenge. Each
`rf_<slug>.rs` file keeps the challenge description, starter code, example `main`, and unit tests
together. The upstream snapshot is not stored in this repository; add new challenges from your
authorized source material.

## Add a Challenge

1. Create `rf_<slug>.rs`, converting the challenge slug to snake case. For example,
   `number-formatting` becomes `rf_number_formatting.rs`.
2. Put the prompt and source URL in crate-level `//!` documentation at the top. Include the
   requirements, examples, hints, and the commands to run and test the example.
3. Add the starter function or types, then a `fn main()` demonstration. Cargo examples are binary
   targets, so `main` must return `()` or another type implementing `Termination`. Give a
   value-returning exercise function a different name, such as `main_value`.
4. Put the challenge tests in a `#[cfg(test)] mod tests` in the same file. Use `use crate::*;` to
   access the example's items. Give tests descriptive names so Cargo's test filter can select them.
5. Register the file in the root `Cargo.toml`:

   ```toml
   [[example]]
   name = "rf_example_slug"
   path = "rustfinity/rf_example_slug.rs"
   ```

6. Add a dependency to the root `Cargo.toml` only if the challenge needs one. Avoid test helpers
   that spawn `cargo` from inside a test: they run from the repository root and can interfere with
   other targets. Write a local assertion or a dedicated test harness instead.

While a starter is incomplete, `#![allow(unused)]` can keep expected TODO-related warnings out of
Problems. Remove or narrow that allowance as you implement the challenge so new warnings in your
solution are visible.

## Run and Test

From the repository root:

```sh
cargo run --example rf_example_slug
cargo test --example rf_example_slug
cargo test --example rf_example_slug test_name_filter
cargo test --no-run --example rf_example_slug
```

`cargo test --example` runs only that challenge's tests. The final command checks that its test
target compiles without executing TODO bodies.

## Procedural Macros

A procedural macro cannot be defined in a normal `[[example]]` target. Give it a dedicated Cargo
crate under `rustfinity/`, with integration tests in that crate's `tests/` directory. The current
example is `rf_procedural_macros/`; test it from the repository root with:

```sh
cargo test --manifest-path rustfinity/rf_procedural_macros/Cargo.toml
```
