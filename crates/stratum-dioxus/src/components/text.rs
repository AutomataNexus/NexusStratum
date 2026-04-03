//! Text and Link components for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Text(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("leading-7 {class}");
    rsx! { p { class: "{classes}", {children} } }
}

#[component]
pub fn Muted(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("text-sm text-muted-foreground {class}");
    rsx! { p { class: "{classes}", {children} } }
}

#[component]
pub fn InlineCode(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!(
        "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold {class}"
    );
    rsx! { code { class: "{classes}", {children} } }
}

#[component]
pub fn Link(
    href: String,
    #[props(default = false)] external: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!(
        "font-medium text-primary underline underline-offset-4 hover:text-primary/80 {class}"
    );
    rsx! {
        a {
            href: "{href}",
            class: "{classes}",
            target: if external { "_blank" } else { "" },
            rel: if external { "noopener noreferrer" } else { "" },
            {children}
        }
    }
}
