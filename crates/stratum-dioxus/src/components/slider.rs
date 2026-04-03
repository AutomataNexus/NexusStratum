//! Slider component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Slider(
    #[props(optional)] value: Option<Signal<f64>>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<f64>>,
) -> Element {
    let mut val = value.unwrap_or_else(|| Signal::new(min));
    let classes = format!("relative flex w-full touch-none select-none items-center {class}");

    rsx! {
        div { class: "{classes}",
            input {
                r#type: "range",
                class: "w-full h-2 rounded-full appearance-none cursor-pointer bg-primary/20 accent-primary disabled:cursor-not-allowed disabled:opacity-50",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                disabled: disabled,
                value: "{val()}",
                role: "slider",
                aria_valuemin: "{min}",
                aria_valuemax: "{max}",
                aria_valuenow: "{val()}",
                oninput: move |evt: Event<FormData>| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        val.set(v);
                        if let Some(handler) = &on_change { handler.call(v); }
                    }
                },
            }
        }
    }
}
