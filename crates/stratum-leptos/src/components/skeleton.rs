//! Skeleton loading placeholder for Leptos.

use leptos::prelude::*;

/// An animated placeholder indicating loading state.
#[component]
pub fn Skeleton(
    /// CSS classes controlling width, height, and shape.
    #[prop(optional, default = String::from("h-4 w-full"))]
    class: String,
) -> impl IntoView {
    let classes = format!("animate-pulse rounded-md bg-primary/10 {}", class);
    view! { <div class=classes aria-busy="true"></div> }
}
