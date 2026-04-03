//! Tooltip component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Tooltip(
    content: String,
    #[props(default = String::from("top"))] side: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let mut visible = use_signal(|| false);

    let position_cls = match side.as_str() {
        "bottom" => "top-full left-1/2 -translate-x-1/2 mt-2",
        "left" => "right-full top-1/2 -translate-y-1/2 mr-2",
        "right" => "left-full top-1/2 -translate-y-1/2 ml-2",
        _ => "bottom-full left-1/2 -translate-x-1/2 mb-2",
    };

    let tooltip_classes = format!(
        "absolute z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground animate-in fade-in-0 zoom-in-95 {position_cls} {class}"
    );

    rsx! {
        div {
            class: "relative inline-flex",
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            onfocusin: move |_| visible.set(true),
            onfocusout: move |_| visible.set(false),
            {children}
            if visible() {
                div {
                    role: "tooltip",
                    class: "{tooltip_classes}",
                    "{content}"
                }
            }
        }
    }
}
