//! Popover component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Popover(
    open: Signal<bool>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    use_context_provider(|| open);
    let classes = format!("relative inline-block {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn PopoverTrigger(#[props(default = String::new())] class: String, children: Element) -> Element {
    let mut open = use_context::<Signal<bool>>();
    rsx! {
        div {
            class: "{class}",
            onclick: move |_| open.set(!open()),
            aria_expanded: if open() { "true" } else { "false" },
            aria_haspopup: "dialog",
            {children}
        }
    }
}

#[component]
pub fn PopoverContent(#[props(default = String::new())] class: String, children: Element) -> Element {
    let open = use_context::<Signal<bool>>();
    let classes = format!("absolute z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none top-full mt-2 {class}");

    if !open() {
        return rsx! {};
    }

    rsx! { div { class: "{classes}", role: "dialog", {children} } }
}
