//! ScrollArea component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn ScrollArea(
    #[props(default = String::from("100%"))] max_height: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("relative overflow-auto scrollbar-thin scrollbar-thumb-border scrollbar-track-transparent {class}");
    rsx! { div { class: "{classes}", style: "max-height: {max_height}", {children} } }
}
