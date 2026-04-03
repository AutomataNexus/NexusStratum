//! Pagination component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Pagination(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("mx-auto flex w-full justify-center {class}");
    rsx! { nav { role: "navigation", aria_label: "pagination", class: "{classes}", {children} } }
}

#[component]
pub fn PaginationContent(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("flex flex-row items-center gap-1 {class}");
    rsx! { ul { class: "{classes}", {children} } }
}

#[component]
pub fn PaginationItem(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    rsx! { li { class: "{class}", {children} } }
}

#[component]
pub fn PaginationLink(
    href: String,
    #[props(default = false)] is_active: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let base = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 w-9";
    let active_cls = if is_active {
        "border border-input bg-background shadow-sm"
    } else {
        ""
    };
    let classes = format!("{base} {active_cls} {class}");
    rsx! { a { href: "{href}", class: "{classes}", aria_current: if is_active { "page" } else { "" }, {children} } }
}

#[component]
pub fn PaginationPrevious(
    href: String,
    #[props(default = String::new())] class: String,
) -> Element {
    let classes = format!(
        "gap-1 pl-2.5 inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 {class}"
    );
    rsx! {
        a { href: "{href}", class: "{classes}", aria_label: "Go to previous page",
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", class: "h-4 w-4", path { d: "m15 18-6-6 6-6" } }
            "Previous"
        }
    }
}

#[component]
pub fn PaginationNext(href: String, #[props(default = String::new())] class: String) -> Element {
    let classes = format!(
        "gap-1 pr-2.5 inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 {class}"
    );
    rsx! {
        a { href: "{href}", class: "{classes}", aria_label: "Go to next page",
            "Next"
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", class: "h-4 w-4", path { d: "m9 18 6-6-6-6" } }
        }
    }
}
