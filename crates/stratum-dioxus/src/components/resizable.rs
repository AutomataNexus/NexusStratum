//! Resizable panels for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn ResizablePanelGroup(
    #[props(default = String::from("horizontal"))] direction: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let dir_cls = if direction == "vertical" {
        "flex-col"
    } else {
        "flex-row"
    };
    let classes = format!("flex h-full w-full {dir_cls} {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn ResizablePanel(
    #[props(default = 50)] default_size: u32,
    #[props(default = 10)] min_size: u32,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("flex-1 overflow-auto {class}");
    rsx! { div { class: "{classes}", style: "flex-basis:{default_size}%;min-width:{min_size}%", {children} } }
}

#[component]
pub fn ResizableHandle(#[props(default = String::new())] class: String) -> Element {
    let classes = format!(
        "relative flex w-px items-center justify-center bg-border cursor-col-resize {class}"
    );
    rsx! {
        div { class: "{classes}", role: "separator", tabindex: "0",
            div { class: "z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-border",
                svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "h-2.5 w-2.5",
                    circle { cx: "12", cy: "9", r: "1" }
                    circle { cx: "12", cy: "15", r: "1" }
                }
            }
        }
    }
}
