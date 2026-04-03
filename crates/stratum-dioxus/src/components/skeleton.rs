//! Skeleton component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Skeleton(#[props(default = String::from("h-4 w-full"))] class: String) -> Element {
    let classes = format!("animate-pulse rounded-md bg-primary/10 {class}");
    rsx! { div { class: "{classes}", aria_busy: "true" } }
}
