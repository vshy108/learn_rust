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
<!-- Updated: 2026-10-19T08:20:05.310Z -->
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
rustfinity/rf_file_metadata.rs:200  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:209  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:217  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:225  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:233  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:242  # TODO: Implement this function
rustfinity/rf_file_metadata.rs:252  # TODO: Implement this function
rustfinity/rf_find_the_first_palindrome.rs:88  # TODO: Implement the function here
rustfinity/rf_fizz_buzz.rs:48  # TODO: Implement the FizzBuzz challenge
rustfinity/rf_fold_and_scan.rs:174  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:188  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:201  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:213  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:225  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:240  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:253  # TODO: Implement this function
rustfinity/rf_fold_and_scan.rs:265  # TODO: Implement this function
rustfinity/rf_from_into_traits.rs:209  # TODO: Return the inner value
rustfinity/rf_generics.rs:40  # TODO: Define a generic struct with a single field `item: T`.
rustfinity/rf_graceful_error_handling.rs:33  # TODO: Implement the function here
```

## changes (last 10 commits — 5 hours ago)
```
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
.github/copilot-instructions.md               +here
```

## .github

### .github/copilot-instructions.md
```
h1 Agent Guidelines for Rust Code Quality
h2 Core Principles
h2 Preferred Tools
h2 Audience
h2 Code Style and Formatting
h2 Documentation
h2 Type System
h2 Error Handling
h2 Function Design
h2 Struct and Enum Design
h2 Testing
h2 Imports and Dependencies
h2 Rust Best Practices
h2 Memory and Performance
h2 Concurrency
h2 Security
h2 Version Control
h2 Tools
h2 Before Committing
h2 Auto-generated signatures
h1 Code signatures
h2 SigMap commands
h2 todos
h2 changes (last 10 commits — 5 days ago)
h2 examples
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

### rustfinity/rf_file_metadata.rs
```
pub fn get_file_size(path: &Path) → io::Result<u64>  :199-202
pub fn get_file_type(path: &Path) → io::Result<String>  :208-211
pub fn is_readonly(path: &Path) → io::Result<bool>  :216-219
pub fn get_modified_time(path: &Path) → io::Result<SystemTime>  :224-227
pub fn was_modified_within(path: &Path, duration: Duration) → io::Result<bool>  :232-235
pub fn is_executable(path: &Path) → io::Result<bool>  :241-244
pub fn compare_modified_times(path1: &Path, path2: &Path) → io::Result<Ordering>  :251-254
pub fn main()  :256-303
```

### rustfinity/rf_find_the_first_palindrome.rs
```
pub fn find_first_palindrome(start: i32, end: i32) → Option<i32>  :87-91
```

### rustfinity/rf_finite_state_automaton.rs
```
pub fn recognize_pattern(input: &str) → bool  :61-65
```

### rustfinity/rf_first_async_function.rs
```
pub async fn greet(name: &str) → String  :108-111
pub async fn greet_and_count(calls: &AtomicUsize, name: &str) → String  :114-117
pub async fn main()  :121-126
```

### rustfinity/rf_fizz_buzz.rs
```
pub fn fizz_buzz(num: u32) → String  :47-50
```

### rustfinity/rf_floating_point.rs
```
pub fn is_valid_number(x: f64) → bool  :176-179
pub fn classify_float(x: f64) → &'static str  :184-187
pub fn safe_divide(a: f64, b: f64) → Option<f64>  :195-198
pub fn round_to_places(x: f64, places: u32) → f64  :201-204
pub fn approx_equal(a: f64, b: f64, epsilon: f64) → bool  :209-212
pub fn clamp_to_range(x: f64, min: f64, max: f64) → Option<f64>  :219-222
pub fn safe_sqrt(x: f64) → Option<f64>  :229-232
pub fn sum_finite(numbers: &[f64]) → f64  :237-240
pub fn main()  :243-278
```

### rustfinity/rf_fold_and_scan.rs
```
pub fn sum_with_fold(numbers: &[i32]) → i32  :173-176
pub fn product_with_fold(numbers: &[i32]) → i64  :187-190
pub fn concat_strings(strings: &[&str], separator: &str) → String  :200-203
pub fn running_sum(numbers: &[i32]) → Vec<i32>  :212-215
pub fn running_max(numbers: &[i32]) → Vec<i32>  :224-227
pub fn take_while_sum_under(numbers: &[i32], limit: i32) → Vec<i32>  :239-242
pub fn count_occurrences(items: &[T], target: &T) → usize  :252-255
pub fn running_average(numbers: &[f64]) → Vec<f64>  :264-267
pub fn main()  :270-302
```

### rustfinity/rf_from_into_traits.rs
```
pub struct Celsius  :129-129
pub struct Fahrenheit  :133-133
pub struct Rgb  :149-153
pub struct HexColor  :157-157
pub struct Email  :167-167
pub struct Point2D  :183-186
pub struct Point3D  :190-194
pub struct Wrapper  :204-212
impl Wrapper  :206-212
  pub fn into_inner(self) → T  :208-208
impl From  :214-218
pub fn main()  :220-254
```

### rustfinity/rf_functions.rs
```
pub fn add(a: i32, b: i32) → i32  :44-47
pub fn subtract(a: i32, b: i32) → i32  :49-51
pub fn multiply(a: i32, b: i32) → i32  :53-55
```

### rustfinity/rf_generics.rs
```
pub struct ItemContainer  :41-43
impl ItemContainer  :45-49
  pub fn get_item(&self) → &T  :46-46
pub fn main()  :52-61
```

### rustfinity/rf_graceful_error_handling_2.rs
```
pub enum ParsePercentageError  :40-43
impl ParsePercentageError  :45-49
impl ParsePercentageError  :51-51
pub fn parse_percentage(input: &str) → Result<u8, ParsePercentageE...  :53-55
pub fn main()  :58-67
```

### rustfinity/rf_graceful_error_handling.rs
```
pub fn parse_percentage(input: &str) → Result<u8, String>  :32-35
pub fn main()  :38-47
```

### rustfinity/rf_handling_thread_panics.rs
```
pub fn describe_panic(job: Box<dyn FnOnce() → Option<String>  :88-91
pub fn divide_in_thread(a: i32, b: i32) → Result<i32, String>  :94-98
pub fn main()  :101-107
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
pub fn create_with_capacity(capacity: usize) → HashMap<String, i32>  :148-151
pub fn reserve_additional(map: &mut HashMap<String, i32>, additional: usize)  :154-157
pub fn shrink_map(map: &mut HashMap<String, i32>)  :160-163
pub fn bulk_insert(items: &[(&str, i32) → HashMap<String, i32>  :166-169
pub fn get_capacity_stats(map: &HashMap<String, i32>) → (usize, usize)  :172-175
pub fn clear_and_shrink(map: &mut HashMap<String, i32>)  :178-181
pub fn group_by_key(items: &[(String, i32) → HashMap<String, Vec<i32>> w...  :184-190
pub fn merge_with_capacity(maps: Vec<HashMap<String, i32>>) → HashMap<String, i32>  :194-197
pub fn main()  :200-248
```

### rustfinity/rf_hashmaps.rs
```
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String)  :54-57
pub fn get_value(map: &HashMap<String, String>, key: String) → Option<String>  :60-63
pub fn main()  :66-82
```

### rustfinity/rf_hashset_operations.rs
```
pub fn unique_elements(items: &[i32]) → HashSet<i32>  :136-139
pub fn count_unique(items: &[i32]) → usize  :142-145
pub fn find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :148-151
pub fn find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :154-157
pub fn find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :160-163
pub fn find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) → HashSet<i32>  :166-169
pub fn is_subset(potential_subset: &HashSet<i32>, potential_superset: &HashSet<i32>) → bool  :172-175
pub fn main()  :177-213
```

### rustfinity/rf_if_else.rs
```
pub fn is_even(n: i32) → bool  :35-38
```

### rustfinity/rf_if_let_enums.rs
```
pub enum Message  :61-66
pub fn process_text_message(message: &Message) → String  :68-72
pub fn main()  :74-85
```

### rustfinity/rf_integer_parsing.rs
```
pub fn parse_decimal(s: &str) → Option<i32>  :121-124
pub fn parse_binary(s: &str) → Option<u32>  :130-133
pub fn parse_hex(s: &str) → Option<u32>  :140-143
pub fn parse_octal(s: &str) → Option<u32>  :149-152
pub fn parse_with_radix(s: &str, radix: u32) → Option<u32>  :158-161
pub fn parse_multiple(s: &str) → Vec<i32>  :167-170
pub fn try_parse_u8(s: &str) → Result<u8, String>  :176-179
pub fn detect_and_parse(s: &str) → Option<i64>  :189-192
pub fn main()  :195-234
```

### rustfinity/rf_interior_mutability.rs
```
pub fn push(data: Rc<RefCell<Vec<T>>>, element: T)  :54-56
pub fn iterate_and_print_shared_data(data: Rc<RefCell<Vec<T>>>)  :58-60
pub fn plus_one(data: Rc<RefCell<i32>>)  :62-64
pub fn main()  :67-81
```

### rustfinity/rf_is_prime.rs
```
pub fn is_prime(n: u32) → bool  :46-49
```

### rustfinity/rf_iterator_combinators.rs
```
pub fn chain_sequences(first: &[T], second: &[T]) → Vec<T>  :148-151
pub fn zip_pairs(first: &[T], second: &[U]) → Vec<(T, U)>  :163-166
pub fn take_first(items: &[T], n: usize) → Vec<T>  :178-181
pub fn skip_first(items: &[T], n: usize) → Vec<T>  :193-196
pub fn reverse_sequence(items: &[T]) → Vec<T>  :205-208
pub fn interleave(first: &[T], second: &[T]) → Vec<T>  :221-224
pub fn sliding_pairs(items: &[T]) → Vec<(T, T)>  :235-238
pub fn main()  :240-268
```

### rustfinity/rf_iterator_filtering.rs
```
pub fn filter_even(numbers: &[i32]) → Vec<i32>  :178-181
pub fn filter_by_predicate(numbers: &[i32], predicate: F) → Vec<i32> where F: Fn(&i32) ...  :186-192
pub fn parse_valid_numbers(strings: &[&str]) → Vec<i32>  :197-200
pub fn filter_map_with(items: &[T], f: F) → Vec<U> where T: Clone, F: F...  :205-212
pub fn take_while_positive(numbers: &[i32]) → Vec<i32>  :217-220
pub fn skip_while_negative(numbers: &[i32]) → Vec<i32>  :225-228
pub fn filter_in_range(numbers: &[i32], min: i32, max: i32) → Vec<i32>  :231-234
pub fn first_matching(items: &[T], predicate: F) → Option<T> where F: Fn(&T) -...  :239-245
pub fn main()  :248-294
```

### rustfinity/rf_iterator_flattening.rs
```
pub fn flatten_nested(nested: Vec<Vec<i32>>) → Vec<i32>  :167-170
pub fn flatten_options(options: Vec<Option<i32>>) → Vec<i32>  :181-184
pub fn flatten_results(results: Vec<Result<i32, &str>>) → Vec<i32>  :195-198
pub fn chars_from_words(words: &[&str]) → Vec<char>  :209-212
pub fn expand_ranges(ranges: &[(i32, i32) → Vec<i32>  :223-226
pub fn flatten_to_depth_one(nested: Vec<Vec<Vec<i32>>>) → Vec<Vec<i32>>  :237-240
pub fn words_from_lines(lines: &[&str]) → Vec<String>  :251-254
pub fn flatten_and_filter(nested: Vec<Vec<T>>, predicate: F) → Vec<T> where T: Clone, F: F...  :266-273
pub fn main()  :276-301
```

### rustfinity/rf_iterator_inspection.rs
```
pub fn indexed_elements(items: &[T]) → Vec<(usize, T)>  :180-183
pub fn find_index(items: &[T], target: &T) → Option<usize>  :193-196
pub fn elements_with_next(items: &[T]) → Vec<(T, Option<T>)>  :207-210
pub fn group_consecutive_duplicates(items: &[T]) → Vec<(T, usize)>  :221-224
pub fn find_first_repeated(items: &[T]) → Option<T>  :235-238
pub fn collect_with_trace(items: &[T], trace: &mut Vec<String>,) → Vec<T>  :250-256
pub fn sum_with_running_total(numbers: &[i32], totals: &mut Vec<i32>) → i32  :268-271
pub fn main()  :273-308
```

### rustfinity/rf_iterators.rs
```
pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) → Vec<i32>  :58-60
pub fn uppercase_strings(items: I) → Vec<String> where I: Iterat...  :62-68
pub fn main()  :74-84
```

### rustfinity/rf_joining_futures.rs
```
pub struct Profile  :107-111
pub async fn load_name(id: u32) → String  :114-117
pub async fn load_email(id: u32) → String  :120-123
pub async fn load_posts(id: u32) → usize  :126-129
pub async fn load_quota(id: u32) → Result<u32, String>  :132-140
pub async fn load_rate_limit(id: u32) → Result<u32, String>  :143-151
pub async fn load_profile(id: u32) → Profile  :154-157
pub async fn load_limits(id: u32) → Result<(u32, u32), String>  :160-163
pub async fn main()  :167-170
```

### rustfinity/rf_joinset_concurrent_tasks.rs
```
pub async fn work(id: u64) → u64  :75-78
pub async fn results_in_completion_order(ids: Vec<u64>) → Vec<u64>  :82-85
pub async fn first_n_to_finish(ids: Vec<u64>, n: usize) → Vec<u64>  :89-92
pub async fn with_limit(ids: Vec<u64>, limit: usize) → Vec<u64>  :97-100
pub async fn main()  :104-108
```

### rustfinity/rf_lifetimes.rs
```
pub fn longest(first: &'a str, second: &'a str) → &'a str  :43-45
pub fn main()  :48-61
```

### rustfinity/rf_linkedlist_basics.rs
```
pub fn create_list(items: &[i32]) → LinkedList<i32>  :124-127
pub fn add_front(list: &mut LinkedList<T>, item: T)  :130-133
pub fn add_back(list: &mut LinkedList<T>, item: T)  :136-139
pub fn remove_front(list: &mut LinkedList<T>) → Option<T>  :142-145
pub fn remove_back(list: &mut LinkedList<T>) → Option<T>  :148-151
pub fn peek_front(list: &LinkedList<T>) → Option<&T>  :154-157
pub fn peek_back(list: &LinkedList<T>) → Option<&T>  :160-163
pub fn move_to_front(list: &mut LinkedList<T>, value: &T) → bool  :167-170
pub fn concat_lists(list1: LinkedList<T>, list2: LinkedList<T>) → LinkedList<T>  :173-176
```

### rustfinity/rf_macros_2.rs
```
pub struct ConnectionTimeout  :55-55
pub struct MaxConnections  :56-56
pub struct RetryAttempts  :57-57
pub struct PostgresPort  :58-58
pub struct MySQLPort  :59-59
pub struct MongoPort  :60-60
pub struct RedisPort  :61-61
pub trait ConfigDefault  :50-52
pub fn main()  :83-92
```

### rustfinity/rf_macros.rs
```
pub fn main()  :73-78
```

### rustfinity/rf_mathematical_operations.rs
```
pub fn math_operations(a: i32, b: i32) → (i32, i32, i32, i32)  :43-46
```

### rustfinity/rf_maze_solver.rs
```
pub fn solve_maze(maze: Vec<Vec<char>>, start: (usize, usize) → Vec<(usize, usize)>  :81-87
```

### rustfinity/rf_median_and_mode.rs
```
pub fn median(numbers: &mut Vec<i32>) → f32  :79-82
pub fn mode(numbers: &Vec<i32>) → Vec<i32>  :84-87
```

### rustfinity/rf_methods_on_enums.rs
```
pub enum VehicleStatus  :46-50
impl VehicleStatus  :52-60
  pub fn is_operational(&self) → bool  :53-53
  pub fn description(&self) → String  :57-57
pub fn main()  :63-78
```

### rustfinity/rf_methods_on_structs.rs
```
pub struct Counter  :68-70
impl Counter  :72-88
  pub fn new() → Self  :73-73
  pub fn increment(&mut self)  :77-77
  pub fn decrement(&mut self)  :81-81
  pub fn get_count(&self) → i32  :85-85
pub fn main()  :91-107
```

### rustfinity/rf_move_closures_threads.rs
```
pub fn count_chars_in_thread(words: Vec<String>) → usize  :80-83
pub fn greet_each(prefix: String, names: Vec<String>) → Vec<String>  :86-89
pub fn main()  :92-98
```

### rustfinity/rf_mpsc_fan_in.rs
```
pub fn collect_all(chunks: Vec<Vec<u64>>) → Vec<u64>  :111-115
pub fn sum_by_worker(chunks: Vec<Vec<u64>>) → Vec<u64>  :118-122
pub fn main()  :125-130
```

### rustfinity/rf_mutable_references.rs
```
pub fn append_suffix(s: &mut String, suffix: &str)  :55-57
```

### rustfinity/rf_mutable_slices.rs
```
pub fn transform_even_odd(slice: &mut [i32])  :56-58
```

### rustfinity/rf_mutable_struct_references.rs
```
pub struct MutableTextFinder  :26-28
impl MutableTextFinder  :30-46
  pub fn new(content: &'a mut String) → Self  :31-31
  pub fn find_first(&self, keyword: &str) → Option<&str>  :35-35
  pub fn replace_lines(&mut self, keyword: &str, replacement: &str)  :39-39
  pub fn get_text(&self) → &str  :43-43
pub fn main()  :49-58
```

### rustfinity/rf_mutable_variables.rs
```
pub fn mutating_variables() → String  :61-68
pub fn mutates_value(value: &mut String)  :71-73
```

### rustfinity/rf_mutex_poisoning.rs
```
pub struct ResilientCounter  :98-100
impl ResilientCounter  :102-128
  pub fn new(start: i64) → Self  :104-104
  pub fn with(&self, f: impl FnOnce(&mut i64) → R) -> R  :112-112
  pub fn get(&self) → i64  :118-118
  pub fn is_poisoned(&self) → bool  :124-124
pub fn main()  :131-135
```

### rustfinity/rf_number_conversions.rs
```
pub fn safe_i32_to_i16(value: i32) → Option<i16>  :132-135
pub fn safe_u64_to_u32(value: u64) → Option<u32>  :140-143
pub fn safe_i64_to_usize(value: i64) → Option<usize>  :148-151
pub fn checked_multiply(a: i32, b: i32) → Option<i32>  :156-159
pub fn checked_power(base: u32, exp: u32) → Option<u32>  :164-167
pub fn saturating_sum(numbers: &[i32]) → i32  :173-176
pub fn wrapping_factorial(n: u32) → u32  :182-185
pub fn safe_average(numbers: &[i64]) → Option<i64>  :190-193
pub fn main()  :196-221
```

### rustfinity/rf_number_formatting.rs
```
pub fn format_padded(num: i32, width: usize) → String  :28-47
pub fn format_aligned(num: i32, width: usize) → String  :53-63
pub fn format_base_less_than_ten(num: u32, base: u32) → Result<String, &'static str>  :66-101
pub fn format_binary(num: u32) → String  :106-112
pub fn format_binary_prefixed(num: u32) → String  :117-120
pub fn format_hex_lower(num: u32) → String  :140-162
pub fn format_hex_upper_prefixed(num: u32) → String  :182-204
pub fn format_octal(num: u32) → String  :207-214
pub fn format_float_precision(num: f64, precision: usize) → String  :217-293
pub fn format_scientific(num: f64) → String  :296-300
pub fn format_currency(amount: f64) → String  :303-307
```

### rustfinity/rf_oncelock_lazy_init.rs
```
pub struct Settings  :107-110
pub fn settings() → &'static Settings  :129-132
pub fn build_count() → usize  :135-138
pub fn main()  :144-147
```

### rustfinity/rf_option_propagation.rs
```
pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) → Option<i32>  :28-35
pub fn main()  :38-48
```

### rustfinity/rf_option_to_result.rs
```
pub fn get_first_element(numbers: Vec<i32>, min_value: i32) → Result<i32, String>  :43-47
pub fn main()  :50-63
```

### rustfinity/rf_optional_values.rs
```
pub fn find_first_even(numbers: &[i32]) → Option<i32>  :30-33
pub fn main()  :36-42
```

### rustfinity/rf_osstring_basics.rs
```
pub fn to_os_string(s: &str) → OsString  :116-119
pub fn os_str_to_str(os: &OsStr) → Option<&str>  :123-126
pub fn os_string_to_string_lossy(os: &OsStr) → String  :129-132
pub fn get_file_extension(path: &Path) → Option<String>  :136-139
pub fn join_path_components(components: &[&str]) → OsString  :143-146
pub fn is_valid_utf8(os: &OsStr) → bool  :149-152
pub fn main()  :154-193
```

### rustfinity/rf_ownership_rules.rs
```
pub fn calculate_and_modify() → (String, usize)  :70-81
```

### rustfinity/rf_ownership.rs
```
pub fn calculate_length(s: &String) → usize  :84-87
```

### rustfinity/rf_panic.rs
```
pub fn get_database_url() → String  :40-43
pub fn main()  :56-67
```

### rustfinity/rf_partialeq_eq.rs
```
pub struct Point  :151-154
pub struct CaseInsensitiveString  :161-161
pub struct ApproximateFloat  :188-188
pub struct UserId  :203-203
pub struct Person  :210-213
pub enum Status  :227-231
impl CaseInsensitiveString  :163-173
  pub fn new(s: S) → Self  :165-165
  pub fn value(&self) → &str  :170-170
impl CaseInsensitiveString  :175-179
impl CaseInsensitiveString  :181-181
impl ApproximateFloat  :193-197
impl Person  :215-219
impl Person  :221-221
pub fn are_all_equal(items: &[T]) → bool  :239-242
pub fn count_matches(items: &[T], target: &T) → usize  :247-250
pub fn find_first_match(items: &[T], target: &T) → Option<usize>  :255-258
pub fn main()  :261-300
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
pub fn join_paths(base: &str, parts: &[&str]) → PathBuf  :152-155
pub fn get_extension(path: &str) → Option<String>  :166-169
pub fn get_file_name(path: &str) → Option<String>  :180-183
pub fn get_file_stem(path: &str) → Option<String>  :194-197
pub fn get_parent(path: &str) → Option<PathBuf>  :208-211
pub fn change_extension(path: &str, new_ext: &str) → PathBuf  :223-226
pub fn is_absolute(path: &str) → bool  :237-240
pub fn normalize_path(path: &str) → PathBuf  :251-254
pub fn main()  :257-281
```

### rustfinity/rf_playing_cards.rs
```
pub enum Card  :44-49
pub fn card_description(card: &Card) → String  :51-54
```

### rustfinity/rf_plugin_system.rs
```
pub struct PluginManager  :56-58
pub struct MyPlugin  :79-79
pub trait Plugin  :51-54
impl PluginManager  :60-76
  pub fn new() → Self  :61-61
  pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>)  :65-65
  pub fn remove_plugin(&mut self, name: &str) → Option<Box<dyn Plugin>>  :69-69
  pub fn execute_all(&self)  :73-73
impl MyPlugin  :81-88
impl MyPlugin  :90-94
pub fn main()  :96-101
```

### rustfinity/rf_primitive_data_types.rs
```
pub fn data_types() → (u8, f64, bool, char)  :45-56
```

### rustfinity/rf_printing_hello_world.rs
```
pub fn hello_world()  :42-44
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
pub fn get_process_id() → u32  :122-125
pub fn exit_code_success() → ExitCode  :130-133
pub fn exit_code_failure() → ExitCode  :138-141
pub fn exit_code_from_u8(code: u8) → ExitCode  :146-149
pub fn is_success_code(code: u8) → bool  :154-157
pub fn is_failure_code(code: u8) → bool  :162-165
pub fn describe_exit_code(code: u8) → String  :178-181
pub fn validate_exit_code(code: Option<u8>) → Result<u8, String>  :187-190
pub fn main()  :193-225
```

### rustfinity/rf_racing_with_select.rs
```
pub async fn first_of(a: A, b: B) → String where A: Future<Outp...  :103-110
pub async fn collect_until_deadline(rx: &mut mpsc::Receiver<i32>, limit: Duration) → Vec<i32>  :113-116
pub async fn main()  :120-141
```

### rustfinity/rf_rc_reference_counting.rs
```
pub struct SharedBuffer  :161-163
pub struct Node  :223-226
pub struct Observable  :269-272
pub trait Observer  :261-264
impl SharedBuffer  :165-195
  pub fn new(data: Vec<u8>) → Rc<Self>  :167-167
  pub fn len(&self) → usize  :173-173
  pub fn is_empty(&self) → bool  :179-179
  pub fn get(&self, index: usize) → Option<u8>  :185-185
  pub fn as_slice(&self) → &[u8]  :191-191
impl Node  :228-258
  pub fn new(value: T) → Rc<Self>  :230-230
  pub fn value(&self) → &T  :236-236
  pub fn add_child(&self, child: Rc<Node<T>>)  :242-242
  pub fn children(&self) → Vec<Rc<Node<T>>>  :248-248
  pub fn children_count(&self) → usize  :254-254
impl Observable  :274-316
  pub fn new(value: T) → Self  :276-276
  pub fn get(&self) → &T  :282-282
  pub fn set(&mut self, value: T)  :288-288
  pub fn subscribe(&mut self, observer: Rc<dyn Observer<T>>)  :296-296
  pub fn notify(&mut self)  :304-304
  pub fn observer_count(&self) → usize  :312-312
pub fn create_shared(value: T) → Rc<T>  :134-137
pub fn clone_shared(rc: &Rc<T>) → Rc<T>  :142-145
```

### rustfinity/rf_rc.rs
```
pub fn use_shared_data(data: Rc<Vec<T>>)  :43-45
pub fn share_data_to_other_functions(mut take_item: F, items: Vec<String>)  :47-53
pub fn main()  :56-74
```

### rustfinity/rf_reading_files.rs
```
pub fn read_entire_file(path: &str) → Result<String, io::Error>  :107-110
pub fn count_lines(path: &str) → Result<usize, io::Error>  :113-116
pub fn count_words(path: &str) → Result<usize, io::Error>  :120-123
pub fn read_lines(path: &str) → Result<Vec<String>, io::Error>  :126-129
pub fn first_n_lines(path: &str, n: usize) → Result<Vec<String>, io::Error>  :132-135
pub fn main()  :137-174
```

### rustfinity/rf_result_to_option.rs
```
pub fn read_file(file_path: &str) → Option<String>  :26-30
pub fn main()  :33-40
```

### rustfinity/rf_return_impl_trait.rs
```
pub fn filter_starts_with(items: &'a [String], prefix: &'a str,) → impl Iterator<Item = &'a St...  :37-43
pub fn main()  :46-55
```

### rustfinity/rf_return_trait_objects.rs
```
pub struct Dog  :36-39
pub struct Robot  :47-50
pub trait Speakable  :32-34
impl Dog  :41-45
impl Robot  :52-56
pub fn get_speaker(kind: &str) → Box<dyn Speakable>  :58-64
pub fn main()  :67-73
```

### rustfinity/rf_rwlock_readers_writers.rs
```
pub struct WordCounts  :109-111
impl WordCounts  :113-145
  pub fn new() → Self  :115-115
  pub fn record(&self, word: &str)  :121-121
  pub fn count(&self, word: &str) → usize  :127-127
  pub fn total(&self) → usize  :133-133
  pub fn most_common(&self) → Option<(String, usize)>  :141-141
pub fn main()  :148-154
```

### rustfinity/rf_scoped_threads.rs
```
pub fn parallel_max(data: &[i32]) → Option<i32>  :72-76
pub fn split_evens_odds(data: &[i32]) → (Vec<i32>, Vec<i32>)  :79-86
pub fn main()  :89-92
```

### rustfinity/rf_send_and_sync_bounds.rs
```
pub fn parallel_map(items: Vec<T>, f: F) → Vec<U> where T: Send + 'sta...  :93-100
pub fn run_jobs(jobs: Vec<F>) → Vec<T> where T: Send + 'sta...  :103-109
pub fn main()  :112-119
```

### rustfinity/rf_shared_state_concurrency.rs
```
pub fn create_shared_data(initial: T) → Arc<Mutex<T>>  :60-63
pub fn increment_counter(counter: Arc<Mutex<i32>>, threads: usize, increments: usize,) → Vec<JoinHandle<()>>  :65-72
pub fn modify_shared_data(data: Arc<Mutex<T>>, modifier: fn(&mut T) → JoinHandle<()>  :74-80
pub fn main()  :83-95
```

### rustfinity/rf_slice_manipulation.rs
```
pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32)  :54-56
```

### rustfinity/rf_slices.rs
```
pub fn find_largest_in_slice(slice: &[i32]) → Option<i32>  :41-44
pub fn main()  :47-56
```

### rustfinity/rf_spawn_blocking.rs
```
pub fn checksum(data: &str) → u64  :70-76
pub async fn checksum_one(data: String) → u64  :79-82
pub async fn checksum_all(items: Vec<String>) → Vec<u64>  :85-88
pub async fn main()  :92-97
```

### rustfinity/rf_spawning_async_tasks.rs
```
pub async fn run_all(delays_ms: Vec<u64>) → Vec<u64>  :119-122
pub async fn checked_divide(a: i64, b: i64) → Result<i64, String>  :125-128
pub async fn main()  :132-135
```

### rustfinity/rf_spawning_threads.rs
```
pub fn sum_in_thread(values: Vec<u64>) → u64  :75-78
pub fn spawn_counter(start: u64, steps: u64) → JoinHandle<u64>  :83-86
pub fn main()  :89-94
```

### rustfinity/rf_stdio_operations.rs
```
pub fn read_line_from_reader(mut reader: R) → io::Result<String>  :153-156
pub fn read_all_lines_from_reader(reader: R) → io::Result<Vec<String>>  :160-163
pub fn write_to_writer(writer: &mut W, message: &str) → io::Result<()>  :167-170
pub fn writeln_to_writer(writer: &mut W, message: &str) → io::Result<()>  :173-176
pub fn write_and_flush(writer: &mut W, message: &str) → io::Result<()>  :180-183
pub fn write_error_to_writer(writer: &mut W, error: &str) → io::Result<()>  :186-189
pub fn main()  :191-232
```

### rustfinity/rf_string_basics.rs
```
pub fn to_owned_string(s: &str) → String  :103-106
pub fn count_chars(s: &str) → usize  :109-112
pub fn count_bytes(s: &str) → usize  :115-118
pub fn is_ascii_only(s: &str) → bool  :121-124
pub fn first_char(s: &str) → Option<char>  :127-130
pub fn main()  :132-141
```

### rustfinity/rf_string_building.rs
```
pub struct Person  :162-165
impl Person  :169-174
pub fn build_greeting(name: &str, age: u32) → String  :147-151
pub fn build_list(items: &[&str]) → String  :154-158
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) → String  :185-193
pub fn concat_with_separator(parts: &[&str], sep: &str) → String  :196-200
pub fn main()  :202-231
```

### rustfinity/rf_string_iterators.rs
```
pub fn chars_to_vec(s: &str) → Vec<char>  :134-137
pub fn words_to_vec(s: &str) → Vec<String>  :140-143
pub fn lines_to_vec(s: &str) → Vec<String>  :146-149
pub fn count_words(s: &str) → usize  :152-155
pub fn reverse_words(s: &str) → String  :158-161
pub fn capitalize_words(s: &str) → String  :164-167
pub fn main()  :169-182
```

### rustfinity/rf_string_manipulation.rs
```
pub fn clean_string(s: &str) → String  :128-131
pub fn contains_word(text: &str, word: &str) → bool  :134-137
pub fn replace_word(text: &str, from: &str, to: &str) → String  :140-143
pub fn split_and_trim(s: &str, delimiter: char) → Vec<String>  :146-149
pub fn normalize_whitespace(s: &str) → String  :152-155
pub fn main()  :157-181
```

### rustfinity/rf_string_parsing.rs
```
pub struct Color  :149-153
impl Color  :157-164
pub fn parse_int(s: &str) → Result<i32, String>  :129-132
pub fn parse_bool(s: &str) → Result<bool, String>  :136-139
pub fn parse_key_value(s: &str) → Result<(String, String), St...  :142-145
pub fn parse_list(s: &str, delimiter: char) → Result<Vec<T>, String>  :167-170
pub fn main()  :172-200
```

### rustfinity/rf_string_patterns.rs
```
pub fn has_prefix(s: &str, prefix: &str) → bool  :150-153
pub fn has_suffix(s: &str, suffix: &str) → bool  :156-159
pub fn find_first(s: &str, pattern: &str) → Option<usize>  :163-166
pub fn find_last(s: &str, pattern: &str) → Option<usize>  :170-173
pub fn count_occurrences(s: &str, pattern: &str) → usize  :176-179
pub fn find_all_indices(s: &str, pattern: &str) → Vec<usize>  :182-185
pub fn extract_between(s: &str, start: &str, end: &str) → Option<String>  :189-192
pub fn main()  :194-221
```

### rustfinity/rf_structs_and_references.rs
```
pub struct TextFinder  :43-45
impl TextFinder  :47-59
  pub fn new(text: &'a str) → Self  :48-48
  pub fn find_first(&self, keyword: &str) → Option<&'a str>  :52-52
  pub fn find_many(&self, keyword: &str) → Vec<&'a str>  :56-56
pub fn main()  :62-71
```

### rustfinity/rf_structs.rs
```
pub struct Person  :77-80
pub fn is_adult(person: &Person) → bool  :82-84
```

### rustfinity/rf_student_grades_tracker_2.rs
```
pub struct Student  :29-32
pub struct StudentGrades  :46-48
impl Student  :34-44
  pub fn add_grade(&mut self, grade: u8)  :35-35
  pub fn average_grade(&self) → f64  :40-40
impl StudentGrades  :50-73
  pub fn new() → Self  :51-51
  pub fn add_student(&mut self, name: &str)  :57-57
  pub fn add_grade(&mut self, name: &str, grade: u8)  :64-64
  pub fn get_grades(&self, name: &str) → &[u8]  :70-70
pub fn main()  :75-91
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
pub fn sum_array(arr: &[i32]) → i32  :38-41
```

### rustfinity/rf_sum_of_even_numbers.rs
```
pub fn sum_of_evens(start: i32, end: i32) → i32  :39-42
```

### rustfinity/rf_supertraits.rs
```
pub struct Undergraduate  :53-57
pub trait Person  :44-46
pub trait Student  :48-48
impl Undergraduate  :59-63
impl Undergraduate  :65-72
pub fn main()  :75-85
```

### rustfinity/rf_sync_channel_backpressure.rs
```
pub fn drain(items: Vec<u64>, capacity: usize) → Vec<u64>  :100-104
pub fn fill_without_receiving(items: Vec<u64>, capacity: usize) → usize  :108-112
pub fn main()  :115-118
```

### rustfinity/rf_systemtime_usage.rs
```
pub fn current_unix_timestamp() → u64  :150-153
pub fn from_unix_timestamp(timestamp: u64) → SystemTime  :156-159
pub fn to_unix_timestamp(time: SystemTime) → Option<u64>  :163-166
pub fn seconds_between(earlier: SystemTime, later: SystemTime) → Option<u64>  :170-173
pub fn is_in_past(time: SystemTime) → bool  :176-179
pub fn is_in_future(time: SystemTime) → bool  :182-185
pub fn add_seconds(time: SystemTime, seconds: u64) → SystemTime  :188-191
pub fn time_until(deadline: SystemTime) → Option<Duration>  :195-198
```

### rustfinity/rf_temperature_converter.rs
```
pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) → Result<f64, String>  :55-58
```

### rustfinity/rf_tempfile_operations.rs
```
pub struct TempFile  :197-199
impl TempFile  :201-225
  pub fn new(prefix: &str, suffix: &str) → io::Result<Self>  :203-203
  pub fn path(&self) → &Path  :209-209
  pub fn write(&self, content: &str) → io::Result<()>  :215-215
  pub fn read(&self) → io::Result<String>  :221-221
impl TempFile  :227-231
pub fn get_temp_dir() → PathBuf  :173-176
pub fn create_temp_file(prefix: &str, suffix: &str) → io::Result<PathBuf>  :181-184
pub fn create_temp_file_with_content(prefix: &str, suffix: &str, content: &str,) → io::Result<PathBuf>  :187-194
pub fn create_temp_dir(prefix: &str) → io::Result<PathBuf>  :234-237
pub fn cleanup_temp_files(dir: &Path, prefix: &str) → io::Result<usize>  :241-244
pub fn main()  :247-283
```

### rustfinity/rf_the_as_ref_trait.rs
```
pub fn print_message(message: T)  :33-35
pub fn main()  :38-45
```

### rustfinity/rf_the_default_trait.rs
```
pub struct AppConfig  :73-82
impl AppConfig  :84-88
pub fn main()  :91-102
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
pub struct Minutes  :45-45
pub struct Hours  :46-46
pub struct Days  :47-47
pub fn main()  :75-79
```

### rustfinity/rf_the_unit_type.rs
```
pub fn print_message() → ()  :32-34
```

### rustfinity/rf_thread_local_state.rs
```
pub fn bump() → u64  :116-119
pub fn record(event: &str)  :122-125
pub fn events() → Vec<String>  :128-131
pub fn counts_per_thread(bumps: &[u64]) → Vec<u64>  :135-139
pub fn main()  :142-147
```

### rustfinity/rf_timeouts_and_sleep.rs
```
pub async fn with_timeout(limit: Duration, work: F) → Option<F::Output> where F: ...  :107-113
pub async fn retry(attempts: usize, gap: Duration, op: impl FnMut() → bool) -> Option<usize>  :116-119
pub async fn main()  :123-139
```

### rustfinity/rf_tostring_display.rs
```
pub struct Point  :196-199
pub struct Money  :243-246
pub struct Person  :258-261
pub enum Color  :211-216
pub enum Temperature  :228-231
impl Point  :202-207
impl Color  :219-224
impl Temperature  :234-239
impl Money  :249-254
impl Person  :264-269
pub fn list_to_string(items: &[T]) → String  :277-280
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) → String  :294-297
pub fn main()  :300-344
```

### rustfinity/rf_trait_bounds.rs
```
pub fn compare_and_display(first: T, second: T) → T where T: std::fmt::Displa...  :73-78
pub fn main()  :81-87
```

### rustfinity/rf_trait_objects.rs
```
pub struct Circle  :66-68
pub struct Rectangle  :70-73
pub struct Canvas  :87-89
pub trait Renderable  :62-64
impl Circle  :75-79
impl Rectangle  :81-85
impl Canvas  :91-103
  pub fn new() → Self  :92-92
  pub fn add_shape(&mut self, shape: Box<dyn Renderable>)  :96-96
  pub fn render_all(&self) → Vec<String>  :100-100
pub fn main()  :106-117
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
pub struct PositiveNumber  :137-137
pub struct Percentage  :148-148
pub struct NonEmptyString  :159-159
pub struct EvenNumber  :177-177
pub struct AsciiChar  :188-188
pub fn main()  :205-243
```

### rustfinity/rf_tuple_structs.rs
```
pub struct Rectangle  :65-65
pub fn area(rect: &Rectangle) → f32  :67-73
```

### rustfinity/rf_tuples.rs
```
pub fn create_tuple(a: i32, b: f64, c: &str) → (i32, f64, String)  :37-40
```

### rustfinity/rf_unicode_operations.rs
```
pub fn char_count(s: &str) → usize  :155-158
pub fn byte_count(s: &str) → usize  :161-164
pub fn safe_substring(s: &str, start: usize, end: usize) → Option<String>  :169-172
pub fn char_at(s: &str, index: usize) → Option<char>  :177-180
pub fn is_single_char(s: &str) → bool  :183-186
pub fn main()  :188-206
```

### rustfinity/rf_unique_items.rs
```
pub fn unique_items(items: I) → Vec<String> where I: Iterat...  :53-59
pub fn main()  :62-75
```

### rustfinity/rf_unit_structs.rs
```
pub struct Logger  :52-52
impl Logger  :54-58
  pub fn log_message(message: &str)  :55-55
pub fn main()  :61-63
```

### rustfinity/rf_unwrap_and_expect.rs
```
pub fn read_file_to_string(path: &str) → String  :44-47
pub fn get_env_variable(key: &str) → String  :49-52
pub fn main()  :55-69
```

### rustfinity/rf_validate_user_input.rs
```
pub fn validate_user(age: i32, email: &str) → Result<(), String>  :56-59
```

### rustfinity/rf_vecdeque_queue.rs
```
pub fn create_queue(items: &[T]) → VecDeque<T>  :120-123
pub fn enqueue(queue: &mut VecDeque<T>, item: T)  :126-129
pub fn dequeue(queue: &mut VecDeque<T>) → Option<T>  :134-137
pub fn peek_front(queue: &VecDeque<T>) → Option<&T>  :140-143
pub fn peek_back(queue: &VecDeque<T>) → Option<&T>  :146-149
pub fn rotate_left(queue: &mut VecDeque<T>, n: usize)  :155-159
pub fn rotate_right(queue: &mut VecDeque<T>, n: usize)  :165-169
pub fn main()  :172-195
```

### rustfinity/rf_vectors.rs
```
pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32])  :36-39
pub fn remove_element(vec: &mut Vec<i32>, index: usize)  :41-44
pub fn get_element(vec: &Vec<i32>, index: usize) → Option<i32>  :46-49
pub fn main()  :52-62
```

### rustfinity/rf_weekday_from_number.rs
```
pub fn weekday_from_number(day: u8) → &'static str  :49-52
```

### rustfinity/rf_writing_files.rs
```
pub fn write_string(path: &str, content: &str) → Result<(), io::Error>  :153-156
pub fn write_bytes(path: &str, data: &[u8]) → Result<(), io::Error>  :159-162
pub fn append_string(path: &str, content: &str) → Result<(), io::Error>  :165-168
pub fn write_lines(path: &str, lines: &[&str]) → Result<(), io::Error>  :171-174
pub fn write_with_buffer(path: &str, chunks: &[&str]) → Result<(), io::Error>  :177-180
pub fn main()  :182-231
```
