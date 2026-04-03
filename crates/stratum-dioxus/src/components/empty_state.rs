//! EmptyState for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn EmptyState(
    #[props(default = String::from("No content"))] title: String,
    #[props(optional)] description: Option<String>,
    #[props(default = String::new())] class: String,
    #[props(optional)] children: Option<Element>,
) -> Element {
    let classes = format!("flex flex-col items-center justify-center py-12 text-center {class}");
    rsx! {
        div { class: "{classes}",
            h3 { class: "mt-2 text-sm font-semibold text-foreground", "{title}" }
            if let Some(d) = &description { p { class: "mt-1 text-sm text-muted-foreground", "{d}" } }
            if let Some(c) = children { div { class: "mt-6", {c} } }
        }
    }
}
