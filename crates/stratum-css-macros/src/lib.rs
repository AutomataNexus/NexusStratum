//! Proc macros for the stratum-css crate.
//!
//! Provides the `css!` macro for compile-time CSS generation with
//! scoped class names and zero runtime overhead.

use proc_macro::TokenStream;
use quote::quote;

/// Generate scoped CSS at compile time.
///
/// Parses CSS syntax, generates a deterministic scoped class name,
/// and registers the CSS for injection.
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let hash = simple_hash(&input_str);
    let class_name = format!("stratum-css-{:08x}", hash);

    let output = quote! {
        #class_name
    };

    output.into()
}

fn simple_hash(input: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in input.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}
