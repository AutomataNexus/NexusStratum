//! VisuallyHidden component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn VisuallyHidden(children: Element) -> Element {
    rsx! {
        span {
            style: "position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0",
            {children}
        }
    }
}
