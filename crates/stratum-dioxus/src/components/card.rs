//! Card component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Card(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("rounded-xl border bg-card text-card-foreground shadow {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn CardHeader(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("flex flex-col space-y-1.5 p-6 {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn CardTitle(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("font-semibold leading-none tracking-tight {class}");
    rsx! { h3 { class: "{classes}", {children} } }
}

#[component]
pub fn CardDescription(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("text-sm text-muted-foreground {class}");
    rsx! { p { class: "{classes}", {children} } }
}

#[component]
pub fn CardContent(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("p-6 pt-0 {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn CardFooter(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("flex items-center p-6 pt-0 {class}");
    rsx! { div { class: "{classes}", {children} } }
}
