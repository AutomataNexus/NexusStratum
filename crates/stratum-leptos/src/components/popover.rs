//! Popover component for Leptos.

use leptos::prelude::*;

/// An anchored floating panel for interactive content.
#[component]
pub fn Popover(
    /// Whether the popover is open.
    open: RwSignal<bool>,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    provide_context(open);
    view! { <div class=format!("relative inline-block {}", class)>{children()}</div> }
}

/// The element that triggers the popover.
#[component]
pub fn PopoverTrigger(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("PopoverTrigger must be inside Popover");

    view! {
        <div
            class=class
            on:click=move |_| open.update(|v| *v = !*v)
            aria-expanded=move || if open.get() { "true" } else { "false" }
            aria-haspopup="dialog"
        >
            {children()}
        </div>
    }
}

/// The floating content panel.
#[component]
pub fn PopoverContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("PopoverContent must be inside Popover");

    let classes = format!(
        "absolute z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none top-full mt-2 {}",
        class
    );

    let rendered = children();

    view! {
        <div
            class=classes
            role="dialog"
            style=move || if open.get() { "" } else { "display:none" }
        >
            {rendered}
        </div>
    }
}
