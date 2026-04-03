//! Spinner component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Spinner(
    #[props(default = 24)] size: u32,
    #[props(default = String::from("Loading"))] aria_label: String,
    #[props(default = String::new())] class: String,
) -> Element {
    let sz = format!("{size}px");
    let classes = format!("animate-spin {class}");
    rsx! {
        span { role: "status", aria_label: "{aria_label}",
            svg {
                class: "{classes}",
                width: "{sz}",
                height: "{sz}",
                view_box: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                circle { class: "opacity-25", cx: "12", cy: "12", r: "10", stroke: "currentColor", stroke_width: "4" }
                path { class: "opacity-75", fill: "currentColor", d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" }
            }
        }
    }
}
