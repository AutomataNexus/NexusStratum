//! ScrollArea component for Leptos.

use leptos::prelude::*;

/// A scrollable container with custom scrollbar styling.
#[component]
pub fn ScrollArea(
    /// Maximum height (e.g., "300px", "50vh").
    #[prop(optional, default = String::from("100%"))]
    max_height: String,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "relative overflow-auto scrollbar-thin scrollbar-thumb-border scrollbar-track-transparent {}",
        class
    );
    view! {
        <div class=classes style=format!("max-height: {}", max_height)>
            {children()}
        </div>
    }
}
