//! EmptyState placeholder for Leptos.

use leptos::prelude::*;

/// A placeholder for empty content areas.
#[component]
pub fn EmptyState(
    #[prop(optional, default = String::from("No content"))] title: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = format!(
        "flex flex-col items-center justify-center py-12 text-center {}",
        class
    );
    view! {
        <div class=classes>
            <h3 class="mt-2 text-sm font-semibold text-foreground">{title}</h3>
            {description.map(|d| view! { <p class="mt-1 text-sm text-muted-foreground">{d}</p> })}
            {children.map(|c| view! { <div class="mt-6">{c()}</div> })}
        </div>
    }
}
