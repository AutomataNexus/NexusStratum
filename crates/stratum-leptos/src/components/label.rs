//! Label component for Leptos.

use leptos::prelude::*;

/// A form label.
#[component]
pub fn Label(
    /// ID of the input this label is for.
    #[prop(optional)]
    for_id: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {}",
        class
    );
    view! {
        <label class=classes for=for_id>
            {children()}
        </label>
    }
}
