//! Textarea component for Leptos.

use leptos::prelude::*;

/// A multi-line text input.
#[component]
pub fn Textarea(
    /// Placeholder text.
    #[prop(optional, default = String::new())]
    placeholder: String,
    /// Current value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Number of visible rows.
    #[prop(optional, default = 3)]
    rows: u32,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether read-only.
    #[prop(optional, default = false)]
    readonly: bool,
    /// Whether required.
    #[prop(optional, default = false)]
    required: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Input handler.
    #[prop(optional)]
    on_input: Option<Callback<String>>,
) -> impl IntoView {
    let base = "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{} {}", base, class);

    let val = value.unwrap_or_else(|| RwSignal::new(String::new()));

    view! {
        <textarea
            class=classes
            placeholder=placeholder
            rows=rows
            disabled=disabled
            readonly=readonly
            required=required
            aria-required=required.then_some("true")
            prop:value=move || val.get()
            on:input=move |ev| {
                let v = event_target_value(&ev);
                val.set(v.clone());
                if let Some(handler) = &on_input {
                    handler.run(v);
                }
            }
        ></textarea>
    }
}
