//! Progress bar component for Leptos.

use leptos::prelude::*;

/// A progress bar indicating completion.
#[component]
pub fn Progress(
    /// Current value (0-100). None = indeterminate.
    #[prop(optional)]
    value: Option<f64>,
    /// Maximum value.
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
) -> impl IntoView {
    let pct = value.map(|v| ((v / max) * 100.0).clamp(0.0, 100.0));

    let classes = format!(
        "relative h-2 w-full overflow-hidden rounded-full bg-primary/20 {}",
        class
    );

    view! {
        <div
            class=classes
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax=max.to_string()
            aria-valuenow=pct.map(|v| v.to_string())
        >
            <div
                class="h-full w-full flex-1 bg-primary transition-all"
                style=move || match pct {
                    Some(p) => format!("transform: translateX(-{}%)", 100.0 - p),
                    None => "animation: indeterminate 1s ease infinite; transform-origin: 0% 50%".to_string(),
                }
            ></div>
        </div>
    }
}
