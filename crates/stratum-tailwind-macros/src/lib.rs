//! Proc macros for the stratum-tailwind crate.
//!
//! Provides the `tw!` macro for building Tailwind CSS class strings.
//! The macro concatenates its arguments at compile time and delegates
//! conflict resolution to `stratum_tailwind::merge_classes` at runtime.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Expr, Token, punctuated::Punctuated};
use syn::parse::{Parse, ParseStream};

/// A comma-separated list of expressions that evaluate to `&str` or `String`.
struct TwInput {
    exprs: Punctuated<Expr, Token![,]>,
}

impl Parse for TwInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::parse_terminated(input)?;
        Ok(TwInput { exprs })
    }
}

/// Build a Tailwind CSS class string from multiple expressions.
///
/// Each argument should evaluate to something that implements `AsRef<str>`.
/// The resulting strings are joined and passed through `merge_classes`
/// for conflict resolution at runtime.
///
/// # Examples
///
/// ```ignore
/// use stratum_tailwind::tw;
///
/// // Static strings
/// let cls = tw!("flex items-center", "gap-4 p-2");
///
/// // Conditional
/// let active = true;
/// let cls = tw!(
///     "flex items-center",
///     if active { "bg-primary" } else { "bg-secondary" }
/// );
/// ```
#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as TwInput);

    let exprs: Vec<&Expr> = parsed.exprs.iter().collect();

    if exprs.is_empty() {
        return quote! { String::new() }.into();
    }

    // Build a Vec of &str at runtime, then call merge_classes
    let push_stmts: Vec<TokenStream2> = exprs
        .iter()
        .map(|expr| {
            quote! {
                {
                    let __val: &str = &(#expr);
                    if !__val.is_empty() {
                        __parts.push(__val);
                    }
                }
            }
        })
        .collect();

    let output = quote! {
        {
            let mut __parts: Vec<&str> = Vec::new();
            #(#push_stmts)*
            stratum_tailwind::merge_classes(&__parts)
        }
    };

    output.into()
}
