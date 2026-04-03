//! Label component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Label(
    #[props(optional)] for_id: Option<String>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {class}");
    rsx! { label { class: "{classes}", r#for: for_id, {children} } }
}
