//! Toggle button component for Leptos.

use leptos::prelude::*;

/// Toggle variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// A pressable toggle button with aria-pressed.
#[component]
pub fn Toggle(
    /// Whether pressed (controlled).
    #[prop(optional)]
    pressed: Option<RwSignal<bool>>,
    /// Default state.
    #[prop(optional, default = false)]
    default_pressed: bool,
    /// Variant style.
    #[prop(optional, default = ToggleVariant::Default)]
    variant: ToggleVariant,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Change handler.
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let is_pressed = pressed.unwrap_or_else(|| RwSignal::new(default_pressed));

    let base = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground h-9 px-3";

    let variant_cls = match variant {
        ToggleVariant::Default => "",
        ToggleVariant::Outline => "border border-input bg-transparent shadow-sm hover:bg-accent hover:text-accent-foreground",
    };

    view! {
        <button
            class=format!("{} {} {}", base, variant_cls, class)
            aria-pressed=move || if is_pressed.get() { "true" } else { "false" }
            data-state=move || if is_pressed.get() { "on" } else { "off" }
            disabled=disabled
            on:click=move |_| {
                let new_val = !is_pressed.get();
                is_pressed.set(new_val);
                if let Some(handler) = &on_change {
                    handler.run(new_val);
                }
            }
        >
            {children()}
        </button>
    }
}
