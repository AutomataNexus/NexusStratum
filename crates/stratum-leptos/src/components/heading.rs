//! Heading component for Leptos.

use leptos::prelude::*;

/// Heading level (h1-h6).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// A semantic heading element.
#[component]
pub fn Heading(
    /// Heading level.
    #[prop(optional, default = HeadingLevel::H2)]
    level: HeadingLevel,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let size_cls = match level {
        HeadingLevel::H1 => "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        HeadingLevel::H2 => {
            "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0"
        }
        HeadingLevel::H3 => "scroll-m-20 text-2xl font-semibold tracking-tight",
        HeadingLevel::H4 => "scroll-m-20 text-xl font-semibold tracking-tight",
        HeadingLevel::H5 => "scroll-m-20 text-lg font-semibold tracking-tight",
        HeadingLevel::H6 => "scroll-m-20 text-base font-semibold tracking-tight",
    };

    let classes = format!("{} {}", size_cls, class);
    let content = children();

    match level {
        HeadingLevel::H1 => view! { <h1 class=classes>{content}</h1> }.into_any(),
        HeadingLevel::H2 => view! { <h2 class=classes>{content}</h2> }.into_any(),
        HeadingLevel::H3 => view! { <h3 class=classes>{content}</h3> }.into_any(),
        HeadingLevel::H4 => view! { <h4 class=classes>{content}</h4> }.into_any(),
        HeadingLevel::H5 => view! { <h5 class=classes>{content}</h5> }.into_any(),
        HeadingLevel::H6 => view! { <h6 class=classes>{content}</h6> }.into_any(),
    }
}
