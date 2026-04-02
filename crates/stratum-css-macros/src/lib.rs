//! Proc macros for the stratum-css crate.
//!
//! Provides the `css!` macro for compile-time CSS generation with
//! scoped class names and zero runtime overhead.

use proc_macro::TokenStream;
use quote::quote;

/// Generate scoped CSS at compile time.
///
/// Accepts CSS syntax as token input, generates a deterministic scoped
/// class name from a content hash, and registers the CSS with the global
/// `StyleRegistry` on first use.
///
/// Returns a `&'static str` class name like `"stratum-css-a7f3b20012345678"`.
///
/// # Example
///
/// ```ignore
/// use stratum_css::css;
///
/// let class = css!(
///     display: flex;
///     align-items: center;
///     gap: 8px;
/// );
/// assert!(class.starts_with("stratum-css-"));
/// ```
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let css_text = input.to_string();

    // Normalize whitespace for deterministic hashing AND for the scoped rule.
    // This ensures the hash and the emitted CSS are derived from the same content.
    let normalized: String = css_text
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let hash = fnv1a_hash(normalized.as_bytes());
    // Use full 64-bit hash (16 hex digits) to minimize collision probability
    let class_name = format!("stratum-css-{:016x}", hash);

    // Build the scoped CSS rule using normalized content
    let scoped_css = format!(".{} {{ {} }}", class_name, normalized);

    let output = quote! {
        {
            static __CSS_REGISTERED: ::std::sync::Once = ::std::sync::Once::new();
            __CSS_REGISTERED.call_once(|| {
                stratum_css::StyleRegistry::global().register(#class_name, #scoped_css);
            });
            #class_name
        }
    };

    output.into()
}

/// FNV-1a hash for deterministic class name generation.
/// Same CSS content always produces the same class name,
/// making SSR + hydration correct.
fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
