//! Radio group component for Leptos.

use leptos::prelude::*;

/// A group of mutually exclusive radio options.
#[component]
pub fn RadioGroup(
    /// Selected value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Name attribute for the radio group.
    #[prop(optional, default = String::from("radio-group"))]
    name: String,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Change handler.
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let selected = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // Provide context so RadioItem can access group state
    provide_context(RadioGroupContext {
        name,
        selected,
        disabled,
        on_change,
    });

    let classes = format!("grid gap-2 {}", class);
    view! {
        <div class=classes role="radiogroup">
            {children()}
        </div>
    }
}

#[derive(Clone)]
struct RadioGroupContext {
    name: String,
    selected: RwSignal<String>,
    disabled: bool,
    on_change: Option<Callback<String>>,
}

/// A single radio option within a RadioGroup.
#[component]
pub fn RadioItem(
    /// Value for this option.
    value: String,
    /// Whether this specific item is disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Label content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<RadioGroupContext>().expect("RadioItem must be inside RadioGroup");
    let is_disabled = disabled || ctx.disabled;
    let val = value.clone();
    let val2 = value.clone();

    view! {
        <label class=format!("flex items-center gap-2 {}", if is_disabled { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" })>
            <input
                type="radio"
                name=ctx.name.clone()
                value=val.clone()
                disabled=is_disabled
                class="aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                prop:checked=move || ctx.selected.get() == val2
                on:change=move |_| {
                    ctx.selected.set(val.clone());
                    if let Some(handler) = &ctx.on_change {
                        handler.run(val.clone());
                    }
                }
            />
            <span class="text-sm font-medium leading-none">{children()}</span>
        </label>
    }
}
