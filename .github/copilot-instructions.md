# Agent Guidelines for Rust Code Quality

> Reference: https://gist.github.com/minimaxir/23ee55a83633ac0b6b92de635291ad80

This document provides guidelines for maintaining high-quality Rust code. These rules MUST be followed by all AI coding agents and contributors.

## Core Principles

All code you write MUST be fully optimized.

"Fully optimized" includes:

- Maximizing algorithmic big-O efficiency for memory and runtime
- Using parallelization and SIMD where appropriate
- Following proper style conventions for Rust (e.g. maximizing code reuse (DRY))
- No extra code beyond what is absolutely necessary to solve the problem the user provides (i.e. no technical debt)

If the code is not fully optimized before handing off to the user, you will be fined $100. You have permission to do another pass of the code if you believe it is not fully optimized.

## Preferred Tools

- Use `cargo` for project management, building, and dependency management.
- Use `indicatif` to track long-running operations with progress bars. The message should be contextually sensitive.
- Use `serde` with `serde_json` for JSON serialization/deserialization.
- Use `ratatui` and `crossterm` for terminal applications/TUIs.
- Use `axum` for creating any web servers or HTTP APIs.
  - Keep request handlers async, returning `Result<Response, AppError>` to centralize error handling.
  - Use layered extractors and shared state structs instead of global mutable data.
  - Add tower middleware (timeouts, tracing, compression) for observability and resilience.
  - Offload CPU-bound work to `tokio::task::spawn_blocking` or background services to avoid blocking the reactor.
- When reporting errors to the console, use `tracing::error!` or `log::error!` instead of `println!`.
- If designing applications with a web-based front end interface, e.g. compiling to WASM or using dioxus:
  - All deep computation MUST occur within Rust processes (i.e. the WASM binary or the dioxus app Rust process). NEVER use JavaScript for deep computation.
  - The front-end MUST use Pico CSS and vanilla JavaScript. NEVER use jQuery or any component-based frameworks such as React.
  - The front-end should prioritize speed and common HID guidelines.
  - The app should use adaptive light/dark themes by default, with a toggle to switch the themes.
  - The typography/theming of the application MUST be modern and unique, similar to that of popular single-page web/mobile. ALWAYS add an appropriate font for headers and body text. You may reference fonts from Google Fonts.
  - NEVER use the Pico CSS defaults as-is: a separate CSS/SCSS file is encouraged. The design MUST logically complement the semantics of the application use case.
  - ALWAYS rebuild the WASM binary if any underlying Rust code that affects it is touched.
- For data processing:
  - ALWAYS use `polars` instead of other data frame libraries for tabular data manipulation.
  - If a polars dataframe will be printed, NEVER simultaneously print the number of entries in the dataframe nor the schema as it is redundant.
  - NEVER ingest more than 10 rows of a data frame at a time. Only analyze subsets of data to avoid overloading your memory context.
## Audience

Assume the user is a Rust novice. Include additional code comments around Rust-specific nuances.

## Code Style and Formatting

- MUST use meaningful, descriptive variable and function names
- MUST follow Rust API Guidelines and idiomatic Rust conventions
- MUST use 4 spaces for indentation (never tabs)
- NEVER use emoji, or unicode that emulates emoji (e.g. checkmarks, crosses). The only exception is when writing tests and testing the impact of multibyte characters.
- Use `snake_case` for functions/variables/modules, `PascalCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants
- Limit line length to 100 characters (rustfmt default)

## Documentation

- MUST include doc comments for all public functions, structs, enums, and methods
- MUST document function parameters, return values, and errors
- Keep comments up-to-date with code changes
- Include examples in doc comments for complex functions

Example doc comment:

```rust
/// Calculate the total cost of items including tax.
///
/// # Arguments
///
/// * `items` - Slice of item structs with price fields
/// * `tax_rate` - Tax rate as decimal (e.g., 0.08 for 8%)
///
/// # Returns
///
/// Total cost including tax
///
/// # Errors
///
/// Returns `CalculationError::EmptyItems` if items is empty
/// Returns `CalculationError::InvalidTaxRate` if tax_rate is negative
///
/// # Examples
///
/// ```
/// let items = vec![Item { price: 10.0 }, Item { price: 20.0 }];
/// let total = calculate_total(&items, 0.08)?;
/// assert_eq!(total, 32.40);
/// ```
pub fn calculate_total(items: &[Item], tax_rate: f64) -> Result<f64, CalculationError> {
```

## Type System

- MUST leverage Rust's type system to prevent bugs at compile time
- NEVER use `.unwrap()` in library code; use `.expect()` only for invariant violations with a descriptive message
- MUST use meaningful custom error types with `thiserror`
- Use newtypes to distinguish semantically different values of the same underlying type
- Prefer `Option<T>` over sentinel values

## Error Handling

- NEVER use `.unwrap()` in production code paths
- MUST use `Result<T, E>` for fallible operations
- MUST use `thiserror` for defining error types and `anyhow` for application-level errors
- MUST propagate errors with `?` operator where appropriate
- Provide meaningful error messages with context using `.context()` from `anyhow`

## Function Design

- MUST keep functions focused on a single responsibility
- MUST prefer borrowing (`&T`, `&mut T`) over ownership when possible
- Limit function parameters to 5 or fewer; use a config struct for more
- Return early to reduce nesting
- Use iterators and combinators over explicit loops where clearer
- NEVER use mutable objects as default argument values
- MUST use type hints for all function signatures (parameters and return values)
- NEVER use `Any` type unless absolutely necessary

## Struct and Enum Design

- MUST keep types focused on a single responsibility
- MUST derive common traits: `Debug`, `Clone`, `PartialEq` where appropriate
- Use `#[derive(Default)]` when a sensible default exists
- Prefer composition over inheritance-like patterns
- Use builder pattern for complex struct construction
- Make fields private by default; provide accessor methods when needed

## Testing

- MUST write unit tests for all new functions and types
- MUST mock external dependencies (APIs, databases, file systems)
- MUST use the built-in `#[test]` attribute and `cargo test`
- Follow the Arrange-Act-Assert pattern
- Do not commit commented-out tests
- Use `#[cfg(test)]` modules for test code

## Imports and Dependencies

- MUST avoid wildcard imports (`use module::*`) except for preludes, test modules (`use super::*`), and prelude re-exports
- MUST document dependencies in `Cargo.toml` with version constraints
- Use `cargo` for dependency management
- Organize imports: standard library, external crates, local modules
- Use `rustfmt` to automate import formatting

## Rust Best Practices

- NEVER use `unsafe` unless absolutely necessary; document safety invariants when used
- MUST call `.clone()` explicitly on non-`Copy` types; avoid hidden clones in closures and iterators
- MUST use pattern matching exhaustively; avoid catch-all `_` patterns when possible
- MUST use `format!` macro for string formatting
- Use iterators and iterator adapters over manual loops
- Use `enumerate()` instead of manual counter variables
- Prefer `if let` and `while let` for single-pattern matching

## Memory and Performance

- MUST avoid unnecessary allocations; prefer `&str` over `String` when possible
- MUST use `Cow<'_, str>` when ownership is conditionally needed
- Use `Vec::with_capacity()` when the size is known
- Prefer stack allocation over heap when appropriate
- Use `Arc` and `Rc` judiciously; prefer borrowing

## Concurrency

- MUST use `Send` and `Sync` bounds appropriately
- MUST prefer `tokio` for async runtime in async applications
- MUST use `rayon` for CPU-bound parallelism
- Avoid `Mutex` when `RwLock` or lock-free alternatives are appropriate
- Use channels (`mpsc`, `crossbeam`) for message passing

## Security

- NEVER store secrets, API keys, or passwords in code. Only store them in `.env`.
- Ensure `.env` is declared in `.gitignore`.
- MUST use environment variables for sensitive configuration via `dotenvy` or `std::env`
- NEVER log sensitive information (passwords, tokens, PII)
- Use `secrecy` crate for sensitive data types

## Version Control

- MUST write clear, descriptive commit messages
- NEVER commit commented-out code; delete it
- NEVER commit debug `println!` statements or `dbg!` macros
- NEVER commit credentials or sensitive data

## Tools

- MUST use `rustfmt` for code formatting
- MUST use `clippy` for linting and follow its suggestions
- MUST ensure code compiles with no warnings (use `-D warnings` flag in CI, not `#![deny(warnings)]` in source)
- Use `cargo` for building, testing, and dependency management
- Use `cargo test` for running tests
- Use `cargo doc` for generating documentation
## Before Committing

- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings (`cargo build`)
- [ ] Clippy passes (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] If the project creates a WASM package and Rust code is touched, rebuild the WASM package (`wasm-pack build --target web --out-dir web/pkg`)
- [ ] All public items have doc comments
- [ ] No commented-out code or debug statements
- [ ] No hardcoded credentials

Remember: Prioritize clarity and maintainability over cleverness.


## Auto-generated signatures
<!-- Updated by gen-context.js -->
<!-- Generated by SigMap gen-context.js v8.18.0 -->
<!-- Updated: 2026-10-18T07:26:56.089Z -->
<!-- sigmap: version=8.18.0 -->
<!-- Do not edit below — regenerate with: node gen-context.js -->

# Code signatures
## SigMap commands

| When | Command |
|------|---------|
| Before answering a question about code | `sigmap ask "<your question>"` |
| To rank files by topic | `sigmap --query "<topic>"` |
| After changing config or source dirs | `sigmap validate` |
| To verify an AI answer is grounded | `sigmap judge --response <file>` |

Always run `sigmap ask` (or `sigmap --query`) before searching for files relevant to a task.

## todos
```
rustfinity/rf_error_propagation.rs:77  # TODO: Implement this function
rustfinity/rf_fibonacci.rs:39  # TODO: Implement the Fibonacci sequence
rustfinity/rf_file_metadata.rs:201  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:210  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:218  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:226  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:234  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:243  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:253  # TODO: Implement this function
rustfinity/rf_find_the_first_palindrome.rs:89  # TODO: Implement the function here
rustfinity/rf_fizz_buzz.rs:49  # TODO: Implement the FizzBuzz challenge
rustfinity/rf_fold_and_scan.rs:175  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:189  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:202  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:214  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:226  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:241  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:254  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:266  # TODO: Implement this function
rustfinity/rf_from_into_traits.rs:210  # TODO: Return the inner value
```

## changes (last 10 commits — 5 days ago)
```
rustfinity/rf_error_propagation.rs            +that  +can
rustfinity/rf_factorial_calculator.rs         +to  +that  +should
rustfinity/rf_fibonacci.rs                    +meeting  +should
rustfinity/rf_find_the_first_palindrome.rs    +should  +here
rustfinity/rf_finite_state_automaton.rs       +should
rustfinity/rf_fizz_buzz.rs                    +should
rustfinity/rf_functions.rs                    +should  +name  +body  +signatures
rustfinity/rf_graceful_error_handling_2.rs    +remains  +to
rustfinity/rf_graceful_error_handling.rs      +that  +should  +here
rustfinity/rf_hash_trait.rs                   +to
rustfinity/rf_if_else.rs                      +should
rustfinity/rf_if_let_enums.rs                 +should
rustfinity/rf_integer_parsing.rs              +on
rustfinity/rf_is_prime.rs                     +to  +should
rustfinity/rf_iterator_filtering.rs           +that  +returning
rustfinity/rf_iterator_flattening.rs          +that
rustfinity/rf_iterators.rs                    +carefully  +signature  +here
rustfinity/rf_lifetimes.rs                    +will  +signature
rustfinity/rf_maze_solver.rs                  +should  +using
rustfinity/rf_median_and_mode.rs              +in
rustfinity/rf_methods_on_structs.rs           +that
rustfinity/rf_move_closures_threads.rs        +that
rustfinity/rf_mutable_references.rs           +should
rustfinity/rf_mutable_variables.rs            +where
rustfinity/rf_option_propagation.rs           +retrieves  +returns
rustfinity/rf_optional_values.rs              +should  +so
rustfinity/rf_ownership_rules.rs              +by
rustfinity/rf_ownership.rs                    +call  +to  +should
rustfinity/rf_panic.rs                        +that  +must  +should  +to
rustfinity/rf_path_operations.rs              +parameters
rustfinity/rf_printing_hello_world.rs         +in
rustfinity/rf_rc.rs                           +prints  +execution
rustfinity/rf_result_to_option.rs             +that
rustfinity/rf_return_impl_trait.rs            +will  +named
rustfinity/rf_return_trait_objects.rs         +returns  +that
rustfinity/rf_shared_state_concurrency.rs     +for  +should  +to
rustfinity/rf_slice_manipulation.rs           +that  +should
rustfinity/rf_slices.rs                       +that  +should  +can
rustfinity/rf_spawning_threads.rs             +that  +body
rustfinity/rf_sum_of_array.rs                 +to  +should  +here
rustfinity/rf_sum_of_even_numbers.rs          +should
rustfinity/rf_temperature_converter.rs        +that  +should  +here
rustfinity/rf_tempfile_operations.rs          +returns
rustfinity/rf_the_as_ref_trait.rs             +that  +should
rustfinity/rf_the_unit_type.rs                +that  +should  +here
rustfinity/rf_thread_local_state.rs           +call
rustfinity/rf_trait_bounds.rs                 +can  +works
rustfinity/rf_tuples.rs                       +that  +should  +here
rustfinity/rf_unique_items.rs                 +that  +signature  +should
rustfinity/rf_unit_structs.rs                 +that  +for
rustfinity/rf_unwrap_and_expect.rs            +takes  +retrieves
rustfinity/rf_validate_user_input.rs          +that  +should  +early
rustfinity/rf_vectors.rs                      +takes
rustfinity/rf_weekday_from_number.rs          +should  +here
examples/s29_edition2024.rs                   +with
```

## examples

### examples/s29_edition2024.rs
```
impl User  :37-45
impl Cache  :132-138
```

### examples/s30_rust_190_192_features.rs
```
pub fn move_index_checked(index: u32, offset: i32) → Option<u32>  :17-19
pub fn move_index_saturating(index: u32, offset: i32) → u32  :21-23
pub fn move_index_wrapping(index: u32, offset: i32) → u32  :25-27
pub fn page_count(total_items: NonZeroU32, page_size: NonZeroU32) → NonZeroU32  :31-33
pub fn insert_or_replace_score(scores: &mut BTreeMap<&'name str, u32>, name: &'name str, score: u32,) → (String, u32)  :37-46
```

### examples/s31_advanced_type_system.rs
```
impl Counter  :22-29
impl Quantity  :38-45
```

## rustfinity

### rustfinity/rf_enums.rs
```
pub enum TrafficLight  :40-44
pub fn light_action(light: &TrafficLight) → &'static str  :46-49
```

### rustfinity/rf_error_propagation.rs
```
pub fn sum_integers_from_file(file_path: &str) → Result<i32, io::Error>  :75-80
pub fn main()  :83-90
```

### rustfinity/rf_factorial_calculator.rs
```
pub fn factorial(n: u32) → u128  :47-50
```

### rustfinity/rf_fibonacci.rs
```
pub fn fibonacci(n: u32) → u32  :38-41
```

### rustfinity/rf_file_metadata.rs
```
pub fn get_file_size(path: &Path) → io::Result<u64>  :200-203
pub fn get_file_type(path: &Path) → io::Result<String>  :209-212
pub fn is_readonly(path: &Path) → io::Result<bool>  :217-220
pub fn get_modified_time(path: &Path) → io::Result<SystemTime>  :225-228
pub fn was_modified_within(path: &Path, duration: Duration) → io::Result<bool>  :233-236
pub fn is_executable(path: &Path) → io::Result<bool>  :242-245
pub fn compare_modified_times(path1: &Path, path2: &Path) → io::Result<Ordering>  :252-255
pub fn main()  :257-304
```

### rustfinity/rf_find_the_first_palindrome.rs
```
pub fn find_first_palindrome(start: i32, end: i32) → Option<i32>  :88-92
```

### rustfinity/rf_finite_state_automaton.rs
```
pub fn recognize_pattern(input: &str) → bool  :62-66
```

### rustfinity/rf_first_async_function.rs
```
pub async fn greet(name: &str) → String  :109-112
pub async fn greet_and_count(calls: &AtomicUsize, name: &str) → String  :115-118
pub async fn main()  :122-127
```

### rustfinity/rf_fizz_buzz.rs
```
pub fn fizz_buzz(num: u32) → String  :48-51
```

### rustfinity/rf_floating_point.rs
```
pub fn is_valid_number(x: f64) → bool  :177-180
pub fn classify_float(x: f64) → &'static str  :185-188
pub fn safe_divide(a: f64, b: f64) → Option<f64>  :196-199
pub fn round_to_places(x: f64, places: u32) → f64  :202-205
pub fn approx_equal(a: f64, b: f64, epsilon: f64) → bool  :210-213
pub fn clamp_to_range(x: f64, min: f64, max: f64) → Option<f64>  :220-223
pub fn safe_sqrt(x: f64) → Option<f64>  :230-233
pub fn sum_finite(numbers: &[f64]) → f64  :238-241
pub fn main()  :244-279
```

### rustfinity/rf_fold_and_scan.rs
```
pub fn sum_with_fold(numbers: &[i32]) → i32  :174-177
pub fn product_with_fold(numbers: &[i32]) → i64  :188-191
pub fn concat_strings(strings: &[&str], separator: &str) → String  :201-204
pub fn running_sum(numbers: &[i32]) → Vec<i32>  :213-216
pub fn running_max(numbers: &[i32]) → Vec<i32>  :225-228
pub fn take_while_sum_under(numbers: &[i32], limit: i32) → Vec<i32>  :240-243
pub fn count_occurrences(items: &[T], target: &T) → usize  :253-256
pub fn running_average(numbers: &[f64]) → Vec<f64>  :265-268
pub fn main()  :271-303
```

### rustfinity/rf_from_into_traits.rs
```
pub struct Celsius  :130-130
pub struct Fahrenheit  :134-134
pub struct Rgb  :150-154
pub struct HexColor  :158-158
pub struct Email  :168-168
pub struct Point2D  :184-187
pub struct Point3D  :191-195
pub struct Wrapper  :205-213
impl Wrapper  :207-213
  pub fn into_inner(self) → T  :209-209
impl From  :215-219
pub fn main()  :221-255
```

### rustfinity/rf_functions.rs
```
pub fn add(a: i32, b: i32) → i32  :45-48
pub fn subtract(a: i32, b: i32) → i32  :50-52
pub fn multiply(a: i32, b: i32) → i32  :54-56
```

### rustfinity/rf_generics.rs
```
pub struct ItemContainer  :42-44
impl ItemContainer  :46-50
  pub fn get_item(&self) → &T  :47-47
pub fn main()  :53-62
```

### rustfinity/rf_graceful_error_handling_2.rs
```
pub enum ParsePercentageError  :41-44
impl ParsePercentageError  :46-50
impl ParsePercentageError  :52-52
pub fn parse_percentage(input: &str) → Result<u8, ParsePercentageE...  :54-56
pub fn main()  :59-68
```

### rustfinity/rf_graceful_error_handling.rs
```
pub fn parse_percentage(input: &str) → Result<u8, String>  :33-36
pub fn main()  :39-48
```

### rustfinity/rf_handling_thread_panics.rs
```
pub fn describe_panic(job: Box<dyn FnOnce() → Option<String>  :89-92
pub fn divide_in_thread(a: i32, b: i32) → Result<i32, String>  :95-99
pub fn main()  :102-108
```

### rustfinity/rf_hash_trait.rs
```
pub struct Point  :126-129
pub struct UserId  :135-135
pub struct CaseInsensitiveString  :143-143
pub struct Document  :179-183
pub struct Rgb  :216-220
impl CaseInsensitiveString  :145-155
  pub fn new(s: S) → Self  :147-147
  pub fn value(&self) → &str  :152-152
impl CaseInsensitiveString  :157-162
impl CaseInsensitiveString  :164-164
impl CaseInsensitiveString  :166-171
impl Document  :185-194
  pub fn new(id: u64, title: S, content: S) → Self  :187-187
impl Document  :196-201
impl Document  :203-203
impl Document  :205-210
impl Rgb  :222-227
  pub fn new(r: u8, g: u8, b: u8) → Self  :224-224
pub fn count_unique(items: &[T]) → usize  :232-235
pub fn find_duplicates(items: &[T]) → Vec<T>  :241-244
pub fn main()  :259-288
```

### rustfinity/rf_hashmap_advanced.rs
```
pub fn create_with_capacity(capacity: usize) → HashMap<String, i32>  :149-152
pub fn reserve_additional(map: &mut HashMap<String, i32>, additional: usize)  :155-158
pub fn shrink_map(map: &mut HashMap<String, i32>)  :161-164
pub fn bulk_insert(items: &[(&str, i32) → HashMap<String, i32>  :167-170
pub fn get_capacity_stats(map: &HashMap<String, i32>) → (usize, usize)  :173-176
pub fn clear_and_shrink(map: &mut HashMap<String, i32>)  :179-182
pub fn group_by_key(items: &[(String, i32) → HashMap<String, Vec<i32>> w...  :185-191
pub fn merge_with_capacity(maps: Vec<HashMap<String, i32>>) → HashMap<String, i32>  :195-198
pub fn main()  :201-249
```

### rustfinity/rf_hashmaps.rs
```
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String)  :55-58
pub fn get_value(map: &HashMap<String, String>, key: String) → Option<String>  :61-64
pub fn main()  :67-83
```

### rustfinity/rf_hashset_operations.rs
```
pub fn unique_elements(items: &[i32]) → HashSet<i32>  :137-140
pub fn count_unique(items: &[i32]) → usize  :143-146
pub fn find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :149-152
pub fn find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :155-158
pub fn find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :161-164
pub fn find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :167-170
pub fn is_subset(potential_subset: &HashSet<i32>, potential_superset: &HashSet<i32>) → bool  :173-176
pub fn main()  :178-214
```

### rustfinity/rf_if_else.rs
```
pub fn is_even(n: i32) → bool  :36-39
```

### rustfinity/rf_if_let_enums.rs
```
pub enum Message  :62-67
pub fn process_text_message(message: &Message) → String  :69-73
pub fn main()  :75-86
```

### rustfinity/rf_integer_parsing.rs
```
pub fn parse_decimal(s: &str) → Option<i32>  :122-125
pub fn parse_binary(s: &str) → Option<u32>  :131-134
pub fn parse_hex(s: &str) → Option<u32>  :141-144
pub fn parse_octal(s: &str) → Option<u32>  :150-153
pub fn parse_with_radix(s: &str, radix: u32) → Option<u32>  :159-162
pub fn parse_multiple(s: &str) → Vec<i32>  :168-171
pub fn try_parse_u8(s: &str) → Result<u8, String>  :177-180
pub fn detect_and_parse(s: &str) → Option<i64>  :190-193
pub fn main()  :196-235
```

### rustfinity/rf_interior_mutability.rs
```
pub fn push(data: Rc<RefCell<Vec<T>>>, element: T)  :55-57
pub fn iterate_and_print_shared_data(data: Rc<RefCell<Vec<T>>>)  :59-61
pub fn plus_one(data: Rc<RefCell<i32>>)  :63-65
pub fn main()  :68-82
```

### rustfinity/rf_is_prime.rs
```
pub fn is_prime(n: u32) → bool  :47-50
```

### rustfinity/rf_iterator_combinators.rs
```
pub fn chain_sequences(first: &[T], second: &[T]) → Vec<T>  :149-152
pub fn zip_pairs(first: &[T], second: &[U]) → Vec<(T, U)>  :164-167
pub fn take_first(items: &[T], n: usize) → Vec<T>  :179-182
pub fn skip_first(items: &[T], n: usize) → Vec<T>  :194-197
pub fn reverse_sequence(items: &[T]) → Vec<T>  :206-209
pub fn interleave(first: &[T], second: &[T]) → Vec<T>  :222-225
pub fn sliding_pairs(items: &[T]) → Vec<(T, T)>  :236-239
pub fn main()  :241-269
```

### rustfinity/rf_iterator_filtering.rs
```
pub fn filter_even(numbers: &[i32]) → Vec<i32>  :179-182
pub fn filter_by_predicate(numbers: &[i32], predicate: F) → Vec<i32> where F: Fn(&i32) ...  :187-193
pub fn parse_valid_numbers(strings: &[&str]) → Vec<i32>  :198-201
pub fn filter_map_with(items: &[T], f: F) → Vec<U> where T: Clone, F: F...  :206-213
pub fn take_while_positive(numbers: &[i32]) → Vec<i32>  :218-221
pub fn skip_while_negative(numbers: &[i32]) → Vec<i32>  :226-229
pub fn filter_in_range(numbers: &[i32], min: i32, max: i32) → Vec<i32>  :232-235
pub fn first_matching(items: &[T], predicate: F) → Option<T> where F: Fn(&T) -...  :240-246
pub fn main()  :249-295
```

### rustfinity/rf_iterator_flattening.rs
```
pub fn flatten_nested(nested: Vec<Vec<i32>>) → Vec<i32>  :168-171
pub fn flatten_options(options: Vec<Option<i32>>) → Vec<i32>  :182-185
pub fn flatten_results(results: Vec<Result<i32, &str>>) → Vec<i32>  :196-199
pub fn chars_from_words(words: &[&str]) → Vec<char>  :210-213
pub fn expand_ranges(ranges: &[(i32, i32) → Vec<i32>  :224-227
pub fn flatten_to_depth_one(nested: Vec<Vec<Vec<i32>>>) → Vec<Vec<i32>>  :238-241
pub fn words_from_lines(lines: &[&str]) → Vec<String>  :252-255
pub fn flatten_and_filter(nested: Vec<Vec<T>>, predicate: F) → Vec<T> where T: Clone, F: F...  :267-274
pub fn main()  :277-302
```

### rustfinity/rf_iterator_inspection.rs
```
pub fn indexed_elements(items: &[T]) → Vec<(usize, T)>  :181-184
pub fn find_index(items: &[T], target: &T) → Option<usize>  :194-197
pub fn elements_with_next(items: &[T]) → Vec<(T, Option<T>)>  :208-211
pub fn group_consecutive_duplicates(items: &[T]) → Vec<(T, usize)>  :222-225
pub fn find_first_repeated(items: &[T]) → Option<T>  :236-239
pub fn collect_with_trace(items: &[T], trace: &mut Vec<String>,) → Vec<T>  :251-257
pub fn sum_with_running_total(numbers: &[i32], totals: &mut Vec<i32>) → i32  :269-272
pub fn main()  :274-309
```

### rustfinity/rf_iterators.rs
```
pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) → Vec<i32>  :59-61
pub fn uppercase_strings(items: I) → Vec<String> where I: Iterat...  :63-69
pub fn main()  :75-85
```

### rustfinity/rf_joining_futures.rs
```
pub struct Profile  :108-112
pub async fn load_name(id: u32) → String  :115-118
pub async fn load_email(id: u32) → String  :121-124
pub async fn load_posts(id: u32) → usize  :127-130
pub async fn load_quota(id: u32) → Result<u32, String>  :133-141
pub async fn load_rate_limit(id: u32) → Result<u32, String>  :144-152
pub async fn load_profile(id: u32) → Profile  :155-158
pub async fn load_limits(id: u32) → Result<(u32, u32), String>  :161-164
pub async fn main()  :168-171
```

### rustfinity/rf_joinset_concurrent_tasks.rs
```
pub async fn work(id: u64) → u64  :76-79
pub async fn results_in_completion_order(ids: Vec<u64>) → Vec<u64>  :83-86
pub async fn first_n_to_finish(ids: Vec<u64>, n: usize) → Vec<u64>  :90-93
pub async fn with_limit(ids: Vec<u64>, limit: usize) → Vec<u64>  :98-101
pub async fn main()  :105-109
```

### rustfinity/rf_lifetimes.rs
```
pub fn longest(first: &'a str, second: &'a str) → &'a str  :44-46
pub fn main()  :49-62
```

### rustfinity/rf_linkedlist_basics.rs
```
pub fn create_list(items: &[i32]) → LinkedList<i32>  :125-128
pub fn add_front(list: &mut LinkedList<T>, item: T)  :131-134
pub fn add_back(list: &mut LinkedList<T>, item: T)  :137-140
pub fn remove_front(list: &mut LinkedList<T>) → Option<T>  :143-146
pub fn remove_back(list: &mut LinkedList<T>) → Option<T>  :149-152
pub fn peek_front(list: &LinkedList<T>) → Option<&T>  :155-158
pub fn peek_back(list: &LinkedList<T>) → Option<&T>  :161-164
pub fn move_to_front(list: &mut LinkedList<T>, value: &T) → bool  :168-171
pub fn concat_lists(list1: LinkedList<T>, list2: LinkedList<T>) → LinkedList<T>  :174-177
```

### rustfinity/rf_macros_2.rs
```
pub struct ConnectionTimeout  :56-56
pub struct MaxConnections  :57-57
pub struct RetryAttempts  :58-58
pub struct PostgresPort  :59-59
pub struct MySQLPort  :60-60
pub struct MongoPort  :61-61
pub struct RedisPort  :62-62
pub trait ConfigDefault  :51-53
pub fn main()  :84-93
```

### rustfinity/rf_macros.rs
```
pub fn main()  :74-79
```

### rustfinity/rf_mathematical_operations.rs
```
pub fn math_operations(a: i32, b: i32) → (i32, i32, i32, i32)  :44-47
```

### rustfinity/rf_maze_solver.rs
```
pub fn solve_maze(maze: Vec<Vec<char>>, start: (usize, usize) → Vec<(usize, usize)>  :82-88
```

### rustfinity/rf_median_and_mode.rs
```
pub fn median(numbers: &mut Vec<i32>) → f32  :80-83
pub fn mode(numbers: &Vec<i32>) → Vec<i32>  :85-88
```

### rustfinity/rf_methods_on_enums.rs
```
pub enum VehicleStatus  :47-51
impl VehicleStatus  :53-61
  pub fn is_operational(&self) → bool  :54-54
  pub fn description(&self) → String  :58-58
pub fn main()  :64-79
```

### rustfinity/rf_methods_on_structs.rs
```
pub struct Counter  :69-71
impl Counter  :73-89
  pub fn new() → Self  :74-74
  pub fn increment(&mut self)  :78-78
  pub fn decrement(&mut self)  :82-82
  pub fn get_count(&self) → i32  :86-86
pub fn main()  :92-108
```

### rustfinity/rf_move_closures_threads.rs
```
pub fn count_chars_in_thread(words: Vec<String>) → usize  :81-84
pub fn greet_each(prefix: String, names: Vec<String>) → Vec<String>  :87-90
pub fn main()  :93-99
```

### rustfinity/rf_mpsc_fan_in.rs
```
pub fn collect_all(chunks: Vec<Vec<u64>>) → Vec<u64>  :112-116
pub fn sum_by_worker(chunks: Vec<Vec<u64>>) → Vec<u64>  :119-123
pub fn main()  :126-131
```

### rustfinity/rf_mutable_references.rs
```
pub fn append_suffix(s: &mut String, suffix: &str)  :56-58
```

### rustfinity/rf_mutable_slices.rs
```
pub fn transform_even_odd(slice: &mut [i32])  :57-59
```

### rustfinity/rf_mutable_struct_references.rs
```
pub struct MutableTextFinder  :27-29
impl MutableTextFinder  :31-47
  pub fn new(content: &'a mut String) → Self  :32-32
  pub fn find_first(&self, keyword: &str) → Option<&str>  :36-36
  pub fn replace_lines(&mut self, keyword: &str, replacement: &str)  :40-40
  pub fn get_text(&self) → &str  :44-44
pub fn main()  :50-59
```

### rustfinity/rf_mutable_variables.rs
```
pub fn mutating_variables() → String  :62-69
pub fn mutates_value(value: &mut String)  :72-74
```

### rustfinity/rf_mutex_poisoning.rs
```
pub struct ResilientCounter  :99-101
impl ResilientCounter  :103-129
  pub fn new(start: i64) → Self  :105-105
  pub fn with(&self, f: impl FnOnce(&mut i64) → R) -> R  :113-113
  pub fn get(&self) → i64  :119-119
  pub fn is_poisoned(&self) → bool  :125-125
pub fn main()  :132-136
```

### rustfinity/rf_number_conversions.rs
```
pub fn safe_i32_to_i16(value: i32) → Option<i16>  :133-136
pub fn safe_u64_to_u32(value: u64) → Option<u32>  :141-144
pub fn safe_i64_to_usize(value: i64) → Option<usize>  :149-152
pub fn checked_multiply(a: i32, b: i32) → Option<i32>  :157-160
pub fn checked_power(base: u32, exp: u32) → Option<u32>  :165-168
pub fn saturating_sum(numbers: &[i32]) → i32  :174-177
pub fn wrapping_factorial(n: u32) → u32  :183-186
pub fn safe_average(numbers: &[i64]) → Option<i64>  :191-194
pub fn main()  :197-222
```

### rustfinity/rf_number_formatting.rs
```
pub fn format_padded(num: i32, width: usize) → String  :28-47
pub fn format_aligned(num: i32, width: usize) → String  :53-63
pub fn format_base_less_than_ten(num: u32, base: u32) → Result<String, &'static str>  :66-99
pub fn format_binary(num: u32) → String  :104-109
pub fn format_binary_prefixed(num: u32) → String  :114-117
pub fn format_hex_lower(num: u32) → String  :136-157
pub fn format_hex_upper_prefixed(num: u32) → String  :176-197
pub fn format_octal(num: u32) → String  :200-206
pub fn format_float_precision(num: f64, precision: usize) → String  :209-239
pub fn format_scientific(num: f64) → String  :242-246
pub fn format_currency(amount: f64) → String  :249-253
```

### rustfinity/rf_oncelock_lazy_init.rs
```
pub struct Settings  :108-111
pub fn settings() → &'static Settings  :130-133
pub fn build_count() → usize  :136-139
pub fn main()  :145-148
```

### rustfinity/rf_option_propagation.rs
```
pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) → Option<i32>  :29-36
pub fn main()  :39-49
```

### rustfinity/rf_option_to_result.rs
```
pub fn get_first_element(numbers: Vec<i32>, min_value: i32) → Result<i32, String>  :44-48
pub fn main()  :51-64
```

### rustfinity/rf_optional_values.rs
```
pub fn find_first_even(numbers: &[i32]) → Option<i32>  :31-34
pub fn main()  :37-43
```

### rustfinity/rf_osstring_basics.rs
```
pub fn to_os_string(s: &str) → OsString  :117-120
pub fn os_str_to_str(os: &OsStr) → Option<&str>  :124-127
pub fn os_string_to_string_lossy(os: &OsStr) → String  :130-133
pub fn get_file_extension(path: &Path) → Option<String>  :137-140
pub fn join_path_components(components: &[&str]) → OsString  :144-147
pub fn is_valid_utf8(os: &OsStr) → bool  :150-153
pub fn main()  :155-194
```

### rustfinity/rf_ownership_rules.rs
```
pub fn calculate_and_modify() → (String, usize)  :71-82
```

### rustfinity/rf_ownership.rs
```
pub fn calculate_length(s: &String) → usize  :85-88
```

### rustfinity/rf_panic.rs
```
pub fn get_database_url() → String  :40-43
pub fn main()  :56-67
```

### rustfinity/rf_partialeq_eq.rs
```
pub struct Point  :152-155
pub struct CaseInsensitiveString  :162-162
pub struct ApproximateFloat  :189-189
pub struct UserId  :204-204
pub struct Person  :211-214
pub enum Status  :228-232
impl CaseInsensitiveString  :164-174
  pub fn new(s: S) → Self  :166-166
  pub fn value(&self) → &str  :171-171
impl CaseInsensitiveString  :176-180
impl CaseInsensitiveString  :182-182
impl ApproximateFloat  :194-198
impl Person  :216-220
impl Person  :222-222
pub fn are_all_equal(items: &[T]) → bool  :240-243
pub fn count_matches(items: &[T], target: &T) → usize  :248-251
pub fn find_first_match(items: &[T], target: &T) → Option<usize>  :256-259
pub fn main()  :262-301
```

### rustfinity/rf_partialord_ord.rs
```
pub struct Score  :100-100
pub struct Version  :107-111
pub struct Player  :184-187
pub enum Temperature  :142-145
pub enum Priority  :172-177
impl Version  :113-122
  pub fn new(major: u32, minor: u32, patch: u32) → Self  :115-115
impl Version  :124-128
impl Version  :130-134
impl Temperature  :147-153
  pub fn to_celsius(&self) → f64  :149-149
impl Temperature  :155-159
impl Temperature  :161-165
impl Player  :189-197
  pub fn new(name: &str, score: u32) → Self  :191-191
impl Player  :199-203
impl Player  :205-209
pub fn find_min(items: &[T]) → Option<&T>  :214-217
pub fn find_max(items: &[T]) → Option<&T>  :222-225
pub fn is_sorted(items: &[T]) → bool  :231-234
pub fn clamp(value: &'a T, min: &'a T, max: &'a T) → &'a T  :241-244
pub fn main()  :247-280
```

### rustfinity/rf_path_operations.rs
```
pub fn join_paths(base: &str, parts: &[&str]) → PathBuf  :153-156
pub fn get_extension(path: &str) → Option<String>  :167-170
pub fn get_file_name(path: &str) → Option<String>  :181-184
pub fn get_file_stem(path: &str) → Option<String>  :195-198
pub fn get_parent(path: &str) → Option<PathBuf>  :209-212
pub fn change_extension(path: &str, new_ext: &str) → PathBuf  :224-227
pub fn is_absolute(path: &str) → bool  :238-241
pub fn normalize_path(path: &str) → PathBuf  :252-255
pub fn main()  :258-282
```

### rustfinity/rf_playing_cards.rs
```
pub enum Card  :45-50
pub fn card_description(card: &Card) → String  :52-55
```

### rustfinity/rf_plugin_system.rs
```
pub struct PluginManager  :57-59
pub struct MyPlugin  :80-80
pub trait Plugin  :52-55
impl PluginManager  :61-77
  pub fn new() → Self  :62-62
  pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>)  :66-66
  pub fn remove_plugin(&mut self, name: &str) → Option<Box<dyn Plugin>>  :70-70
  pub fn execute_all(&self)  :74-74
impl MyPlugin  :82-89
impl MyPlugin  :91-95
pub fn main()  :97-102
```

### rustfinity/rf_primitive_data_types.rs
```
pub fn data_types() → (u8, f64, bool, char)  :46-57
```

### rustfinity/rf_printing_hello_world.rs
```
pub fn hello_world()  :43-45
```

### rustfinity/rf_procedural_macros/Cargo.toml
```
table [package]
table [lib]
table [dependencies]
table [workspace]
key name
key version
key edition
key publish
key proc-macro
key quote
key syn
```

### rustfinity/rf_procedural_macros/src/lib.rs
```
pub fn derive_describe(input: TokenStream) → TokenStream  :22-34
```

### rustfinity/rf_process_exit.rs
```
pub fn get_process_id() → u32  :123-126
pub fn exit_code_success() → ExitCode  :131-134
pub fn exit_code_failure() → ExitCode  :139-142
pub fn exit_code_from_u8(code: u8) → ExitCode  :147-150
pub fn is_success_code(code: u8) → bool  :155-158
pub fn is_failure_code(code: u8) → bool  :163-166
pub fn describe_exit_code(code: u8) → String  :179-182
pub fn validate_exit_code(code: Option<u8>) → Result<u8, String>  :188-191
pub fn main()  :194-226
```

### rustfinity/rf_racing_with_select.rs
```
pub async fn first_of(a: A, b: B) → String where A: Future<Outp...  :104-111
pub async fn collect_until_deadline(rx: &mut mpsc::Receiver<i32>, limit: Duration) → Vec<i32>  :114-117
pub async fn main()  :121-142
```

### rustfinity/rf_rc_reference_counting.rs
```
pub struct SharedBuffer  :162-164
pub struct Node  :224-227
pub struct Observable  :270-273
pub trait Observer  :262-265
impl SharedBuffer  :166-196
  pub fn new(data: Vec<u8>) → Rc<Self>  :168-168
  pub fn len(&self) → usize  :174-174
  pub fn is_empty(&self) → bool  :180-180
  pub fn get(&self, index: usize) → Option<u8>  :186-186
  pub fn as_slice(&self) → &[u8]  :192-192
impl Node  :229-259
  pub fn new(value: T) → Rc<Self>  :231-231
  pub fn value(&self) → &T  :237-237
  pub fn add_child(&self, child: Rc<Node<T>>)  :243-243
  pub fn children(&self) → Vec<Rc<Node<T>>>  :249-249
  pub fn children_count(&self) → usize  :255-255
impl Observable  :275-317
  pub fn new(value: T) → Self  :277-277
  pub fn get(&self) → &T  :283-283
  pub fn set(&mut self, value: T)  :289-289
  pub fn subscribe(&mut self, observer: Rc<dyn Observer<T>>)  :297-297
  pub fn notify(&mut self)  :305-305
  pub fn observer_count(&self) → usize  :313-313
pub fn create_shared(value: T) → Rc<T>  :135-138
pub fn clone_shared(rc: &Rc<T>) → Rc<T>  :143-146
```

### rustfinity/rf_rc.rs
```
pub fn use_shared_data(data: Rc<Vec<T>>)  :43-45
pub fn share_data_to_other_functions(mut take_item: F, items: Vec<String>)  :47-53
pub fn main()  :56-74
```

### rustfinity/rf_reading_files.rs
```
pub fn read_entire_file(path: &str) → Result<String, io::Error>  :108-111
pub fn count_lines(path: &str) → Result<usize, io::Error>  :114-117
pub fn count_words(path: &str) → Result<usize, io::Error>  :121-124
pub fn read_lines(path: &str) → Result<Vec<String>, io::Error>  :127-130
pub fn first_n_lines(path: &str, n: usize) → Result<Vec<String>, io::Error>  :133-136
pub fn main()  :138-175
```

### rustfinity/rf_result_to_option.rs
```
pub fn read_file(file_path: &str) → Option<String>  :27-31
pub fn main()  :34-41
```

### rustfinity/rf_return_impl_trait.rs
```
pub fn filter_starts_with(items: &'a [String], prefix: &'a str,) → impl Iterator<Item = &'a St...  :38-44
pub fn main()  :47-56
```

### rustfinity/rf_return_trait_objects.rs
```
pub struct Dog  :37-40
pub struct Robot  :48-51
pub trait Speakable  :33-35
impl Dog  :42-46
impl Robot  :53-57
pub fn get_speaker(kind: &str) → Box<dyn Speakable>  :59-65
pub fn main()  :68-74
```

### rustfinity/rf_rwlock_readers_writers.rs
```
pub struct WordCounts  :110-112
impl WordCounts  :114-146
  pub fn new() → Self  :116-116
  pub fn record(&self, word: &str)  :122-122
  pub fn count(&self, word: &str) → usize  :128-128
  pub fn total(&self) → usize  :134-134
  pub fn most_common(&self) → Option<(String, usize)>  :142-142
pub fn main()  :149-155
```

### rustfinity/rf_scoped_threads.rs
```
pub fn parallel_max(data: &[i32]) → Option<i32>  :73-77
pub fn split_evens_odds(data: &[i32]) → (Vec<i32>, Vec<i32>)  :80-87
pub fn main()  :90-93
```

### rustfinity/rf_send_and_sync_bounds.rs
```
pub fn parallel_map(items: Vec<T>, f: F) → Vec<U> where T: Send + 'sta...  :93-100
pub fn run_jobs(jobs: Vec<F>) → Vec<T> where T: Send + 'sta...  :103-109
pub fn main()  :112-119
```

### rustfinity/rf_shared_state_concurrency.rs
```
pub fn create_shared_data(initial: T) → Arc<Mutex<T>>  :61-64
pub fn increment_counter(counter: Arc<Mutex<i32>>, threads: usize, increments: usize,) → Vec<JoinHandle<()>>  :66-73
pub fn modify_shared_data(data: Arc<Mutex<T>>, modifier: fn(&mut T) → JoinHandle<()>  :75-81
pub fn main()  :84-96
```

### rustfinity/rf_slice_manipulation.rs
```
pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32)  :55-57
```

### rustfinity/rf_slices.rs
```
pub fn find_largest_in_slice(slice: &[i32]) → Option<i32>  :42-45
pub fn main()  :48-57
```

### rustfinity/rf_spawn_blocking.rs
```
pub fn checksum(data: &str) → u64  :71-77
pub async fn checksum_one(data: String) → u64  :80-83
pub async fn checksum_all(items: Vec<String>) → Vec<u64>  :86-89
pub async fn main()  :93-98
```

### rustfinity/rf_spawning_async_tasks.rs
```
pub async fn run_all(delays_ms: Vec<u64>) → Vec<u64>  :120-123
pub async fn checked_divide(a: i64, b: i64) → Result<i64, String>  :126-129
pub async fn main()  :133-136
```

### rustfinity/rf_spawning_threads.rs
```
pub fn sum_in_thread(values: Vec<u64>) → u64  :76-79
pub fn spawn_counter(start: u64, steps: u64) → JoinHandle<u64>  :84-87
pub fn main()  :90-95
```

### rustfinity/rf_stdio_operations.rs
```
pub fn read_line_from_reader(mut reader: R) → io::Result<String>  :154-157
pub fn read_all_lines_from_reader(reader: R) → io::Result<Vec<String>>  :161-164
pub fn write_to_writer(writer: &mut W, message: &str) → io::Result<()>  :168-171
pub fn writeln_to_writer(writer: &mut W, message: &str) → io::Result<()>  :174-177
pub fn write_and_flush(writer: &mut W, message: &str) → io::Result<()>  :181-184
pub fn write_error_to_writer(writer: &mut W, error: &str) → io::Result<()>  :187-190
pub fn main()  :192-233
```

### rustfinity/rf_string_basics.rs
```
pub fn to_owned_string(s: &str) → String  :104-107
pub fn count_chars(s: &str) → usize  :110-113
pub fn count_bytes(s: &str) → usize  :116-119
pub fn is_ascii_only(s: &str) → bool  :122-125
pub fn first_char(s: &str) → Option<char>  :128-131
pub fn main()  :133-142
```

### rustfinity/rf_string_building.rs
```
pub struct Person  :163-166
impl Person  :170-175
pub fn build_greeting(name: &str, age: u32) → String  :148-152
pub fn build_list(items: &[&str]) → String  :155-159
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) → String  :186-194
pub fn concat_with_separator(parts: &[&str], sep: &str) → String  :197-201
pub fn main()  :203-232
```

### rustfinity/rf_string_iterators.rs
```
pub fn chars_to_vec(s: &str) → Vec<char>  :135-138
pub fn words_to_vec(s: &str) → Vec<String>  :141-144
pub fn lines_to_vec(s: &str) → Vec<String>  :147-150
pub fn count_words(s: &str) → usize  :153-156
pub fn reverse_words(s: &str) → String  :159-162
pub fn capitalize_words(s: &str) → String  :165-168
pub fn main()  :170-183
```

### rustfinity/rf_string_manipulation.rs
```
pub fn clean_string(s: &str) → String  :129-132
pub fn contains_word(text: &str, word: &str) → bool  :135-138
pub fn replace_word(text: &str, from: &str, to: &str) → String  :141-144
pub fn split_and_trim(s: &str, delimiter: char) → Vec<String>  :147-150
pub fn normalize_whitespace(s: &str) → String  :153-156
pub fn main()  :158-182
```

### rustfinity/rf_string_parsing.rs
```
pub struct Color  :150-154
impl Color  :158-165
pub fn parse_int(s: &str) → Result<i32, String>  :130-133
pub fn parse_bool(s: &str) → Result<bool, String>  :137-140
pub fn parse_key_value(s: &str) → Result<(String, String), St...  :143-146
pub fn parse_list(s: &str, delimiter: char) → Result<Vec<T>, String>  :168-171
pub fn main()  :173-201
```

### rustfinity/rf_string_patterns.rs
```
pub fn has_prefix(s: &str, prefix: &str) → bool  :151-154
pub fn has_suffix(s: &str, suffix: &str) → bool  :157-160
pub fn find_first(s: &str, pattern: &str) → Option<usize>  :164-167
pub fn find_last(s: &str, pattern: &str) → Option<usize>  :171-174
pub fn count_occurrences(s: &str, pattern: &str) → usize  :177-180
pub fn find_all_indices(s: &str, pattern: &str) → Vec<usize>  :183-186
pub fn extract_between(s: &str, start: &str, end: &str) → Option<String>  :190-193
pub fn main()  :195-222
```

### rustfinity/rf_structs_and_references.rs
```
pub struct TextFinder  :44-46
impl TextFinder  :48-60
  pub fn new(text: &'a str) → Self  :49-49
  pub fn find_first(&self, keyword: &str) → Option<&'a str>  :53-53
  pub fn find_many(&self, keyword: &str) → Vec<&'a str>  :57-57
pub fn main()  :63-72
```

### rustfinity/rf_structs.rs
```
pub struct Person  :78-81
pub fn is_adult(person: &Person) → bool  :83-85
```

### rustfinity/rf_student_grades_tracker_2.rs
```
pub struct Student  :30-33
pub struct StudentGrades  :47-49
impl Student  :35-45
  pub fn add_grade(&mut self, grade: u8)  :36-36
  pub fn average_grade(&self) → f64  :41-41
impl StudentGrades  :51-74
  pub fn new() → Self  :52-52
  pub fn add_student(&mut self, name: &str)  :58-58
  pub fn add_grade(&mut self, name: &str, grade: u8)  :65-65
  pub fn get_grades(&self, name: &str) → &[u8]  :71-71
pub fn main()  :76-92
```

### rustfinity/rf_student_grades_tracker.rs
```
pub struct Student  :41-44
pub struct StudentGrades  :46-48
impl StudentGrades  :50-69
  pub fn new() → Self  :51-51
  pub fn add_student(&mut self, name: &str)  :58-58
  pub fn add_grade(&mut self, name: &str, grade: u8)  :62-62
  pub fn get_grades(&self, name: &str) → &[u8]  :66-66
pub fn main()  :72-84
```

### rustfinity/rf_sum_of_array.rs
```
pub fn sum_array(arr: &[i32]) → i32  :39-42
```

### rustfinity/rf_sum_of_even_numbers.rs
```
pub fn sum_of_evens(start: i32, end: i32) → i32  :40-43
```

### rustfinity/rf_supertraits.rs
```
pub struct Undergraduate  :54-58
pub trait Person  :45-47
pub trait Student  :49-49
impl Undergraduate  :60-64
impl Undergraduate  :66-73
pub fn main()  :76-86
```

### rustfinity/rf_sync_channel_backpressure.rs
```
pub fn drain(items: Vec<u64>, capacity: usize) → Vec<u64>  :101-105
pub fn fill_without_receiving(items: Vec<u64>, capacity: usize) → usize  :109-113
pub fn main()  :116-119
```

### rustfinity/rf_systemtime_usage.rs
```
pub fn current_unix_timestamp() → u64  :151-154
pub fn from_unix_timestamp(timestamp: u64) → SystemTime  :157-160
pub fn to_unix_timestamp(time: SystemTime) → Option<u64>  :164-167
pub fn seconds_between(earlier: SystemTime, later: SystemTime) → Option<u64>  :171-174
pub fn is_in_past(time: SystemTime) → bool  :177-180
pub fn is_in_future(time: SystemTime) → bool  :183-186
pub fn add_seconds(time: SystemTime, seconds: u64) → SystemTime  :189-192
pub fn time_until(deadline: SystemTime) → Option<Duration>  :196-199
```

### rustfinity/rf_temperature_converter.rs
```
pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) → Result<f64, String>  :56-59
```

### rustfinity/rf_tempfile_operations.rs
```
pub struct TempFile  :198-200
impl TempFile  :202-226
  pub fn new(prefix: &str, suffix: &str) → io::Result<Self>  :204-204
  pub fn path(&self) → &Path  :210-210
  pub fn write(&self, content: &str) → io::Result<()>  :216-216
  pub fn read(&self) → io::Result<String>  :222-222
impl TempFile  :228-232
pub fn get_temp_dir() → PathBuf  :174-177
pub fn create_temp_file(prefix: &str, suffix: &str) → io::Result<PathBuf>  :182-185
pub fn create_temp_file_with_content(prefix: &str, suffix: &str, content: &str,) → io::Result<PathBuf>  :188-195
pub fn create_temp_dir(prefix: &str) → io::Result<PathBuf>  :235-238
pub fn cleanup_temp_files(dir: &Path, prefix: &str) → io::Result<usize>  :242-245
pub fn main()  :248-284
```

### rustfinity/rf_the_as_ref_trait.rs
```
pub fn print_message(message: T)  :34-36
pub fn main()  :39-46
```

### rustfinity/rf_the_default_trait.rs
```
pub struct AppConfig  :74-83
impl AppConfig  :85-89
pub fn main()  :92-103
```

### rustfinity/rf_the_drop_trait.rs
```
pub struct TempFile  :41-43
impl TempFile  :45-49
  pub fn new(name: N) → std::io::Result<Self>  :46-46
impl TempFile  :51-56
pub fn main()  :59-74
```

### rustfinity/rf_the_from_trait.rs
```
pub struct Minutes  :46-46
pub struct Hours  :47-47
pub struct Days  :48-48
pub fn main()  :76-80
```

### rustfinity/rf_the_unit_type.rs
```
pub fn print_message() → ()  :33-35
```

### rustfinity/rf_thread_local_state.rs
```
pub fn bump() → u64  :117-120
pub fn record(event: &str)  :123-126
pub fn events() → Vec<String>  :129-132
pub fn counts_per_thread(bumps: &[u64]) → Vec<u64>  :136-140
pub fn main()  :143-148
```

### rustfinity/rf_timeouts_and_sleep.rs
```
pub async fn with_timeout(limit: Duration, work: F) → Option<F::Output> where F: ...  :108-114
pub async fn retry(attempts: usize, gap: Duration, op: impl FnMut() → bool) -> Option<usize>  :117-120
pub async fn main()  :124-140
```

### rustfinity/rf_tostring_display.rs
```
pub struct Point  :197-200
pub struct Money  :244-247
pub struct Person  :259-262
pub enum Color  :212-217
pub enum Temperature  :229-232
impl Point  :203-208
impl Color  :220-225
impl Temperature  :235-240
impl Money  :250-255
impl Person  :265-270
pub fn list_to_string(items: &[T]) → String  :278-281
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) → String  :295-298
pub fn main()  :301-345
```

### rustfinity/rf_trait_bounds.rs
```
pub fn compare_and_display(first: T, second: T) → T where T: std::fmt::Displa...  :74-79
pub fn main()  :82-88
```

### rustfinity/rf_trait_objects.rs
```
pub struct Circle  :67-69
pub struct Rectangle  :71-74
pub struct Canvas  :88-90
pub trait Renderable  :63-65
impl Circle  :76-80
impl Rectangle  :82-86
impl Canvas  :92-104
  pub fn new() → Self  :93-93
  pub fn add_shape(&mut self, shape: Box<dyn Renderable>)  :97-97
  pub fn render_all(&self) → Vec<String>  :101-101
pub fn main()  :107-118
```

### rustfinity/rf_traits.rs
```
pub struct Person  :62-65
pub struct Book  :67-70
pub trait Describable  :58-60
impl Person  :72-76
impl Book  :78-82
pub fn main()  :85-98
```

### rustfinity/rf_tryfrom_tryinto.rs
```
pub struct PositiveNumber  :138-138
pub struct Percentage  :149-149
pub struct NonEmptyString  :160-160
pub struct EvenNumber  :178-178
pub struct AsciiChar  :189-189
pub fn main()  :206-244
```

### rustfinity/rf_tuple_structs.rs
```
pub struct Rectangle  :66-66
pub fn area(rect: &Rectangle) → f32  :68-74
```

### rustfinity/rf_tuples.rs
```
pub fn create_tuple(a: i32, b: f64, c: &str) → (i32, f64, String)  :38-41
```

### rustfinity/rf_unicode_operations.rs
```
pub fn char_count(s: &str) → usize  :156-159
pub fn byte_count(s: &str) → usize  :162-165
pub fn safe_substring(s: &str, start: usize, end: usize) → Option<String>  :170-173
pub fn char_at(s: &str, index: usize) → Option<char>  :178-181
pub fn is_single_char(s: &str) → bool  :184-187
pub fn main()  :189-207
```

### rustfinity/rf_unique_items.rs
```
pub fn unique_items(items: I) → Vec<String> where I: Iterat...  :54-60
pub fn main()  :63-76
```

### rustfinity/rf_unit_structs.rs
```
pub struct Logger  :53-53
impl Logger  :55-59
  pub fn log_message(message: &str)  :56-56
pub fn main()  :62-64
```

### rustfinity/rf_unwrap_and_expect.rs
```
pub fn read_file_to_string(path: &str) → String  :44-47
pub fn get_env_variable(key: &str) → String  :49-52
pub fn main()  :55-69
```

### rustfinity/rf_validate_user_input.rs
```
pub fn validate_user(age: i32, email: &str) → Result<(), String>  :57-60
```

### rustfinity/rf_vecdeque_queue.rs
```
pub fn create_queue(items: &[T]) → VecDeque<T>  :121-124
pub fn enqueue(queue: &mut VecDeque<T>, item: T)  :127-130
pub fn dequeue(queue: &mut VecDeque<T>) → Option<T>  :135-138
pub fn peek_front(queue: &VecDeque<T>) → Option<&T>  :141-144
pub fn peek_back(queue: &VecDeque<T>) → Option<&T>  :147-150
pub fn rotate_left(queue: &mut VecDeque<T>, n: usize)  :156-160
pub fn rotate_right(queue: &mut VecDeque<T>, n: usize)  :166-170
pub fn main()  :173-196
```

### rustfinity/rf_vectors.rs
```
pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32])  :37-40
pub fn remove_element(vec: &mut Vec<i32>, index: usize)  :42-45
pub fn get_element(vec: &Vec<i32>, index: usize) → Option<i32>  :47-50
pub fn main()  :53-63
```

### rustfinity/rf_weekday_from_number.rs
```
pub fn weekday_from_number(day: u8) → &'static str  :50-53
```

### rustfinity/rf_writing_files.rs
```
pub fn write_string(path: &str, content: &str) → Result<(), io::Error>  :154-157
pub fn write_bytes(path: &str, data: &[u8]) → Result<(), io::Error>  :160-163
pub fn append_string(path: &str, content: &str) → Result<(), io::Error>  :166-169
pub fn write_lines(path: &str, lines: &[&str]) → Result<(), io::Error>  :172-175
pub fn write_with_buffer(path: &str, chunks: &[&str]) → Result<(), io::Error>  :178-181
pub fn main()  :183-232
```
