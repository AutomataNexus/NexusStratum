//! Breadcrumb navigation for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Breadcrumb(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!(
        "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 {class}"
    );
    rsx! { nav { aria_label: "breadcrumb", ol { class: "{classes}", {children} } } }
}

#[component]
pub fn BreadcrumbItem(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("inline-flex items-center gap-1.5 {class}");
    rsx! { li { class: "{classes}", {children} } }
}

#[component]
pub fn BreadcrumbLink(
    href: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("transition-colors hover:text-foreground {class}");
    rsx! { a { href: "{href}", class: "{classes}", {children} } }
}

#[component]
pub fn BreadcrumbSeparator(#[props(default = String::new())] class: String) -> Element {
    let classes = format!("[&>svg]:size-3.5 {class}");
    rsx! {
        li { role: "presentation", aria_hidden: "true", class: "{classes}",
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", class: "h-3.5 w-3.5",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

#[component]
pub fn BreadcrumbPage(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("font-normal text-foreground {class}");
    rsx! { span { role: "link", aria_disabled: "true", aria_current: "page", class: "{classes}", {children} } }
}
