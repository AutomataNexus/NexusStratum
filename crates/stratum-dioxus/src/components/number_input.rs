//! NumberInput for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn NumberInput(
    #[props(optional)] value: Option<Signal<f64>>,
    #[props(default = 0.0)] default_value: f64,
    #[props(optional)] min: Option<f64>,
    #[props(optional)] max: Option<f64>,
    #[props(default = 1.0)] step: f64,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] placeholder: String,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<f64>>,
) -> Element {
    let mut val = value.unwrap_or_else(|| Signal::new(default_value));
    let classes = format!("flex items-center {class}");
    let input_cls = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm text-center focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let btn_cls = "inline-flex items-center justify-center rounded-md border border-input bg-background h-9 w-9 text-sm hover:bg-accent disabled:opacity-50";

    let clamp = move |v: f64| -> f64 {
        let mut c = v;
        if let Some(lo) = min { c = c.max(lo); }
        if let Some(hi) = max { c = c.min(hi); }
        c
    };

    rsx! {
        div { class: "{classes}",
            button { class: "{btn_cls}", disabled: disabled, aria_label: "Decrease", onclick: move |_| {
                if !disabled { let nv = clamp(val() - step); val.set(nv); if let Some(h) = &on_change { h.call(nv); } }
            }, "-" }
            input {
                r#type: "number",
                class: "{input_cls}",
                disabled: disabled,
                placeholder: "{placeholder}",
                value: "{val()}",
                role: "spinbutton",
                aria_valuemin: min.map(|v| format!("{v}")),
                aria_valuemax: max.map(|v| format!("{v}")),
                aria_valuenow: "{val()}",
                oninput: move |evt: Event<FormData>| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        let nv = clamp(v); val.set(nv); if let Some(h) = &on_change { h.call(nv); }
                    }
                },
            }
            button { class: "{btn_cls}", disabled: disabled, aria_label: "Increase", onclick: move |_| {
                if !disabled { let nv = clamp(val() + step); val.set(nv); if let Some(h) = &on_change { h.call(nv); } }
            }, "+" }
        }
    }
}
