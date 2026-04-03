//! Avatar component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Avatar(
    #[props(optional)] src: Option<String>,
    #[props(default = String::new())] alt: String,
    #[props(default = String::new())] fallback: String,
    #[props(default = String::new())] class: String,
) -> Element {
    let classes = format!("relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full {class}");
    rsx! {
        span { class: "{classes}",
            if let Some(url) = &src {
                img { class: "aspect-square h-full w-full", src: "{url}", alt: "{alt}" }
            } else {
                span { class: "flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium",
                    "{fallback}"
                }
            }
        }
    }
}
