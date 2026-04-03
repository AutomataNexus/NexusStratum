//! Dialog component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Dialog(
    open: Signal<bool>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    if !open() {
        return rsx! {};
    }

    let classes = format!(
        "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg sm:rounded-lg {class}"
    );

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 z-50 bg-black/80",
            onclick: move |_| {
                if let Some(handler) = &on_close {
                    handler.call(());
                }
            },
        }
        // Content
        div {
            class: "{classes}",
            role: "dialog",
            aria_modal: "true",
            onkeydown: move |evt| {
                if evt.key() == Key::Escape {
                    if let Some(handler) = &on_close {
                        handler.call(());
                    }
                }
            },
            {children}
        }
    }
}

#[component]
pub fn DialogHeader(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("flex flex-col space-y-1.5 text-center sm:text-left {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn DialogTitle(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("text-lg font-semibold leading-none tracking-tight {class}");
    rsx! { h2 { class: "{classes}", {children} } }
}

#[component]
pub fn DialogDescription(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("text-sm text-muted-foreground {class}");
    rsx! { p { class: "{classes}", {children} } }
}

#[component]
pub fn DialogFooter(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {class}");
    rsx! { div { class: "{classes}", {children} } }
}
