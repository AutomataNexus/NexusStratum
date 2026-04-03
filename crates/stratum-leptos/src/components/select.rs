//! Select dropdown component for Leptos.

use leptos::prelude::*;

/// A native select dropdown.
#[component]
pub fn Select(
    /// Placeholder text.
    #[prop(optional, default = String::new())]
    placeholder: String,
    /// Selected value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Change handler.
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Select options.
    children: Children,
) -> impl IntoView {
    let base = "flex h-9 w-full items-center justify-between whitespace-nowrap rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{} {}", base, class);

    let selected = value.unwrap_or_else(|| RwSignal::new(String::new()));

    view! {
        <select
            class=classes
            disabled=disabled
            prop:value=move || selected.get()
            on:change=move |ev| {
                let val = event_target_value(&ev);
                selected.set(val.clone());
                if let Some(handler) = &on_change {
                    handler.run(val);
                }
            }
        >
            {(!placeholder.is_empty()).then(|| view! {
                <option value="" disabled=true selected=true>{placeholder.clone()}</option>
            })}
            {children()}
        </select>
    }
}

/// A select option.
#[component]
pub fn SelectOption(
    /// Option value.
    value: String,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Display text.
    children: Children,
) -> impl IntoView {
    view! {
        <option value=value disabled=disabled>
            {children()}
        </option>
    }
}
