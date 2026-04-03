//! Kbd (keyboard shortcut) component for Leptos.

use leptos::prelude::*;

/// Displays a keyboard shortcut indicator.
#[component]
pub fn Kbd(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!(
        "pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100 {}",
        class
    );
    view! { <kbd class=classes>{children()}</kbd> }
}
