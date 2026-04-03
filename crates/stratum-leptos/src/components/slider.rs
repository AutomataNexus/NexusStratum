//! Slider (range input) component for Leptos.

use leptos::prelude::*;

/// A range slider input.
#[component]
pub fn Slider(
    /// Current value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<f64>>,
    /// Minimum value.
    #[prop(optional, default = 0.0)]
    min: f64,
    /// Maximum value.
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Step increment.
    #[prop(optional, default = 1.0)]
    step: f64,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Change handler.
    #[prop(optional)]
    on_change: Option<Callback<f64>>,
) -> impl IntoView {
    let val = value.unwrap_or_else(|| RwSignal::new(min));

    let classes = format!(
        "relative flex w-full touch-none select-none items-center {}",
        class
    );

    view! {
        <div class=classes>
            <input
                type="range"
                class="w-full h-2 rounded-full appearance-none cursor-pointer bg-primary/20 accent-primary disabled:cursor-not-allowed disabled:opacity-50"
                min=min
                max=max
                step=step
                disabled=disabled
                prop:value=move || val.get().to_string()
                on:input=move |ev| {
                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                        val.set(v);
                        if let Some(handler) = &on_change {
                            handler.run(v);
                        }
                    }
                }
                aria-valuemin=min
                aria-valuemax=max
                aria-valuenow=move || val.get()
                role="slider"
            />
        </div>
    }
}
