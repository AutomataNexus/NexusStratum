//! Carousel for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Carousel(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("relative {class}");
    rsx! { div { class: "{classes}", role: "region", aria_roledescription: "carousel", {children} } }
}

#[component]
pub fn CarouselContent(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("flex overflow-hidden {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn CarouselItem(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("min-w-0 shrink-0 grow-0 basis-full {class}");
    rsx! { div { class: "{classes}", role: "group", aria_roledescription: "slide", {children} } }
}

#[component]
pub fn CarouselPrevious(#[props(optional)] on_click: Option<EventHandler<()>>, #[props(default = String::new())] class: String) -> Element {
    let classes = format!("absolute left-2 top-1/2 -translate-y-1/2 inline-flex items-center justify-center rounded-full border bg-background h-8 w-8 shadow-sm hover:bg-accent {class}");
    rsx! {
        button { class: "{classes}", aria_label: "Previous slide", onclick: move |_| { if let Some(h) = &on_click { h.call(()); } },
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "h-4 w-4", path { d: "m15 18-6-6 6-6" } }
        }
    }
}

#[component]
pub fn CarouselNext(#[props(optional)] on_click: Option<EventHandler<()>>, #[props(default = String::new())] class: String) -> Element {
    let classes = format!("absolute right-2 top-1/2 -translate-y-1/2 inline-flex items-center justify-center rounded-full border bg-background h-8 w-8 shadow-sm hover:bg-accent {class}");
    rsx! {
        button { class: "{classes}", aria_label: "Next slide", onclick: move |_| { if let Some(h) = &on_click { h.call(()); } },
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "h-4 w-4", path { d: "m9 18 6-6-6-6" } }
        }
    }
}
