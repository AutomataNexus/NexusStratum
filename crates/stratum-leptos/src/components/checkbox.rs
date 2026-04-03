//! Checkbox component for Leptos.

use leptos::prelude::*;

/// A checkbox input with label support.
#[component]
pub fn Checkbox(
    /// Whether checked (controlled).
    #[prop(optional)]
    checked: Option<RwSignal<bool>>,
    /// Default checked state (uncontrolled).
    #[prop(optional, default = false)]
    default_checked: bool,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether required.
    #[prop(optional, default = false)]
    required: bool,
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
    let is_checked = checked.unwrap_or_else(|| RwSignal::new(default_checked));

    let base = "peer h-4 w-4 shrink-0 rounded-sm border border-primary shadow focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground";
    let classes = format!("{} {}", base, class);

    view! {
        <label class="flex items-center gap-2 cursor-pointer">
            <input
                type="checkbox"
                class=classes
                disabled=disabled
                required=required
                aria-required=required.then_some("true")
                prop:checked=move || is_checked.get()
                on:change=move |ev| {
                    let val = event_target_checked(&ev);
                    is_checked.set(val);
                    if let Some(handler) = &on_change {
                        handler.run(val);
                    }
                }
            />
            {children.map(|c| c())}
        </label>
    }
}
