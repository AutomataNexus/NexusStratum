//! Kbd component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Kbd(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!(
        "pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100 {class}"
    );
    rsx! { kbd { class: "{classes}", {children} } }
}
