//! Avatar component for Leptos.

use leptos::prelude::*;

/// A user avatar with image and fallback.
#[component]
pub fn Avatar(
    /// Image URL.
    #[prop(optional)]
    src: Option<String>,
    /// Alt text.
    #[prop(optional, default = String::new())]
    alt: String,
    /// Fallback text (e.g., initials).
    #[prop(optional, default = String::new())]
    fallback: String,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
) -> impl IntoView {
    let classes = format!(
        "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full {}",
        class
    );

    view! {
        <span class=classes>
            {match src {
                Some(url) => {
                    view! {
                        <img class="aspect-square h-full w-full" src=url alt=alt.clone() />
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <span class="flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium">
                            {fallback.clone()}
                        </span>
                    }
                        .into_any()
                }
            }}
        </span>
    }
}
