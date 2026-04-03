//! AspectRatio component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn AspectRatio(
    #[props(default = 1.0)] ratio: f64,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let padding = format!("{}%", (1.0 / ratio) * 100.0);
    rsx! {
        div { class: "{class}", style: "position:relative;width:100%;padding-bottom:{padding}",
            div { style: "position:absolute;inset:0", {children} }
        }
    }
}
