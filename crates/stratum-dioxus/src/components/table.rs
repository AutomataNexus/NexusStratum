//! Table components for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Table(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("w-full caption-bottom text-sm {class}");
    rsx! { div { class: "relative w-full overflow-auto", table { class: "{classes}", {children} } } }
}

#[component]
pub fn TableHeader(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("[&_tr]:border-b {class}");
    rsx! { thead { class: "{classes}", {children} } }
}

#[component]
pub fn TableBody(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("[&_tr:last-child]:border-0 {class}");
    rsx! { tbody { class: "{classes}", {children} } }
}

#[component]
pub fn TableFooter(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("border-t bg-muted/50 font-medium {class}");
    rsx! { tfoot { class: "{classes}", {children} } }
}

#[component]
pub fn TableRow(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("border-b transition-colors hover:bg-muted/50 {class}");
    rsx! { tr { class: "{classes}", {children} } }
}

#[component]
pub fn TableHead(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("h-10 px-2 text-left align-middle font-medium text-muted-foreground {class}");
    rsx! { th { class: "{classes}", {children} } }
}

#[component]
pub fn TableCell(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("p-2 align-middle {class}");
    rsx! { td { class: "{classes}", {children} } }
}

#[component]
pub fn TableCaption(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("mt-4 text-sm text-muted-foreground {class}");
    rsx! { caption { class: "{classes}", {children} } }
}
