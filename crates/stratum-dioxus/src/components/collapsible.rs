//! Collapsible component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Collapsible(
    open: Signal<bool>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    use_context_provider(|| open);
    rsx! { div { class: "{class}", {children} } }
}

#[component]
pub fn CollapsibleTrigger(#[props(default = String::new())] class: String, children: Element) -> Element {
    let mut open = use_context::<Signal<bool>>();
    rsx! {
        button {
            class: "{class}",
            onclick: move |_| open.set(!open()),
            aria_expanded: if open() { "true" } else { "false" },
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(#[props(default = String::new())] class: String, children: Element) -> Element {
    let open = use_context::<Signal<bool>>();
    if !open() {
        return rsx! {};
    }
    rsx! { div { class: "{class}", {children} } }
}
