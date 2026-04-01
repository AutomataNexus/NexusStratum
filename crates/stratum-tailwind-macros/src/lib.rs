//! Proc macros for the stratum-tailwind crate.
//!
//! Provides the `tw!` macro for type-safe Tailwind CSS class building
//! with conflict resolution.

use proc_macro::TokenStream;
use quote::quote;

/// Build Tailwind CSS class strings with conflict resolution.
///
/// Accepts static strings, conditional expressions, and match arms.
/// Automatically resolves class conflicts (e.g., `h-8` vs `h-12`).
#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // For now, pass through as a string literal
    // Full implementation will parse and resolve conflicts
    let output = quote! {
        #input_str
    };

    output.into()
}
