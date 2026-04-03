//! Switch toggle component for Leptos.

use leptos::prelude::*;

/// A binary toggle switch.
#[component]
pub fn Switch(
    /// Whether on (controlled).
    #[prop(optional)]
    checked: Option<RwSignal<bool>>,
    /// Default state (uncontrolled).
    #[prop(optional, default = false)]
    default_checked: bool,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Change handler.
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
    /// Label content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let is_on = checked.unwrap_or_else(|| RwSignal::new(default_checked));

    let toggle = move |_| {
        if !disabled {
            let new_val = !is_on.get();
            is_on.set(new_val);
            if let Some(handler) = &on_change {
                handler.run(new_val);
            }
        }
    };

    view! {
        <label class=format!("flex items-center gap-2 {}", if disabled { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" })>
            <button
                type="button"
                role="switch"
                class=format!(
                    "peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 {} {}",
                    "data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
                    class
                )
                disabled=disabled
                aria-checked=move || if is_on.get() { "true" } else { "false" }
                data-state=move || if is_on.get() { "checked" } else { "unchecked" }
                on:click=toggle
            >
                <span
                    class="pointer-events-none block h-4 w-4 rounded-full bg-background shadow-lg ring-0 transition-transform"
                    style=move || if is_on.get() { "transform: translateX(16px)" } else { "transform: translateX(0px)" }
                ></span>
            </button>
            {children.map(|c| c())}
        </label>
    }
}
