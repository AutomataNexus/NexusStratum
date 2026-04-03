//! NumberInput (numeric input with increment/decrement) for Leptos.

use leptos::prelude::*;

#[component]
pub fn NumberInput(
    #[prop(optional)] value: Option<RwSignal<f64>>,
    #[prop(optional, default = 0.0)] default_value: f64,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional, default = 1.0)] step: f64,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = String::new())] placeholder: String,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] on_change: Option<Callback<f64>>,
) -> impl IntoView {
    let val = value.unwrap_or_else(|| RwSignal::new(default_value));

    let update = move |delta: f64| {
        if !disabled {
            let mut new_val = val.get() + delta;
            if let Some(lo) = min { new_val = new_val.max(lo); }
            if let Some(hi) = max { new_val = new_val.min(hi); }
            val.set(new_val);
            if let Some(handler) = &on_change { handler.run(new_val); }
        }
    };

    let classes = format!("flex items-center {}", class);
    let input_cls = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors text-center focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let btn_cls = "inline-flex items-center justify-center rounded-md border border-input bg-background h-9 w-9 text-sm hover:bg-accent disabled:opacity-50";

    view! {
        <div class=classes>
            <button class=btn_cls disabled=disabled on:click=move |_| update(-step) aria-label="Decrease">"-"</button>
            <input
                type="number"
                class=input_cls
                disabled=disabled
                placeholder=placeholder
                prop:value=move || val.get().to_string()
                on:input=move |ev| {
                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                        let mut clamped = v;
                        if let Some(lo) = min { clamped = clamped.max(lo); }
                        if let Some(hi) = max { clamped = clamped.min(hi); }
                        val.set(clamped);
                        if let Some(handler) = &on_change { handler.run(clamped); }
                    }
                }
                role="spinbutton"
                aria-valuemin=min.map(|v| v.to_string())
                aria-valuemax=max.map(|v| v.to_string())
                aria-valuenow=move || val.get()
            />
            <button class=btn_cls disabled=disabled on:click=move |_| update(step) aria-label="Increase">"+"</button>
        </div>
    }
}
