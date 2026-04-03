//! Collapsible component for Leptos.

use leptos::prelude::*;

/// A simple show/hide content area.
#[component]
pub fn Collapsible(
    /// Whether open (controlled).
    open: RwSignal<bool>,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    provide_context(open);
    view! { <div class=class>{children()}</div> }
}

/// The trigger that toggles the collapsible.
#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("CollapsibleTrigger must be inside Collapsible");
    view! {
        <button
            class=class
            on:click=move |_| open.update(|v| *v = !*v)
            aria-expanded=move || if open.get() { "true" } else { "false" }
        >
            {children()}
        </button>
    }
}

/// The content that shows/hides.
#[component]
pub fn CollapsibleContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open =
        use_context::<RwSignal<bool>>().expect("CollapsibleContent must be inside Collapsible");
    let rendered = children();
    view! {
        <div class=class style=move || if open.get() { "" } else { "display:none" }>
            {rendered}
        </div>
    }
}
