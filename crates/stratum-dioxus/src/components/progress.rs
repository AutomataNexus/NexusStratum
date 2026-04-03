//! Progress bar component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Progress(
    #[props(optional)] value: Option<f64>,
    #[props(default = 100.0)] max: f64,
    #[props(default = String::new())] class: String,
) -> Element {
    let pct = value.map(|v| ((v / max) * 100.0).clamp(0.0, 100.0));
    let classes = format!("relative h-2 w-full overflow-hidden rounded-full bg-primary/20 {class}");

    let transform = match pct {
        Some(p) => format!("transform: translateX(-{}%)", 100.0 - p),
        None => "animation: indeterminate 1s ease infinite; transform-origin: 0% 50%".to_string(),
    };

    rsx! {
        div {
            class: "{classes}",
            role: "progressbar",
            aria_valuemin: "0",
            aria_valuemax: "{max}",
            aria_valuenow: pct.map(|v| format!("{v}")),
            div {
                class: "h-full w-full flex-1 bg-primary transition-all",
                style: "{transform}",
            }
        }
    }
}
