//! Accordion component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Accordion(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("divide-y divide-border {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn AccordionItem(
    #[props(default = false)] default_open: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let is_open = use_signal(|| default_open);
    use_context_provider(|| is_open);
    let classes = format!("border-b {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn AccordionTrigger(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let mut is_open = use_context::<Signal<bool>>();
    let classes = format!("flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline {class}");
    let rotate = if is_open() {
        "transform: rotate(180deg)"
    } else {
        ""
    };

    rsx! {
        button {
            class: "{classes}",
            aria_expanded: if is_open() { "true" } else { "false" },
            onclick: move |_| is_open.set(!is_open()),
            {children}
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24", height: "24",
                view_box: "0 0 24 24",
                fill: "none", stroke: "currentColor",
                stroke_width: "2", stroke_linecap: "round",
                class: "h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200",
                style: "{rotate}",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let is_open = use_context::<Signal<bool>>();
    if !is_open() {
        return rsx! {};
    }
    let classes = format!("overflow-hidden text-sm pb-4 pt-0 {class}");
    rsx! { div { class: "{classes}", role: "region", {children} } }
}
