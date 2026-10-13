//! # Rustfinity: Procedural Macros
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/procedural-macros
//!
//! Implement a `Describe` derive macro that generates a `describe` method for structs with named
//! fields. The generated description should include the struct name and each field name and value.
//!
//! Run its integration tests with:
//!
//! ```sh
//! cargo test --manifest-path rustfinity/rf_procedural_macros/Cargo.toml
//! ```
//!
//! Proc macros must live in their own crate, and their integration tests must be separate from the
//! defining crate. See `tests/describe.rs` for the test cases.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;

    quote! {
        impl Describe for #type_name {
            fn describe(&self) -> String {
                String::new()
            }
        }
    }
    .into()
}
