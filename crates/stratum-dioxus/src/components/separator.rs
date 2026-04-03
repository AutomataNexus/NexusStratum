//! Separator component for Dioxus.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn Separator(
    #[props(default = Orientation::Horizontal)] orientation: Orientation,
    #[props(default = true)] decorative: bool,
    #[props(default = String::new())] class: String,
) -> Element {
    let dir_cls = match orientation {
        Orientation::Horizontal => "h-[1px] w-full",
        Orientation::Vertical => "h-full w-[1px]",
    };
    let classes = format!("shrink-0 bg-border {dir_cls} {class}");

    rsx! {
        div {
            class: "{classes}",
            role: if decorative { "none" } else { "separator" },
        }
    }
}
