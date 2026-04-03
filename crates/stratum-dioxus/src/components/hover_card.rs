//! HoverCard component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn HoverCard(#[props(default = String::new())] class: String, children: Element) -> Element {
    let mut visible = use_signal(|| false);
    use_context_provider(|| visible);
    let classes = format!("relative inline-block {class}");
    rsx! {
        div { class: "{classes}",
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            {children}
        }
    }
}

#[component]
pub fn HoverCardTrigger(#[props(default = String::new())] class: String, children: Element) -> Element {
    rsx! { div { class: "{class}", {children} } }
}

#[component]
pub fn HoverCardContent(#[props(default = String::new())] class: String, children: Element) -> Element {
    let visible = use_context::<Signal<bool>>();
    if !visible() { return rsx! {}; }
    let classes = format!("absolute z-50 w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none top-full mt-2 {class}");
    rsx! { div { class: "{classes}", {children} } }
}
