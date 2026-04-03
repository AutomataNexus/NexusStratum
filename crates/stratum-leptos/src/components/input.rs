//! Input component for Leptos.

use leptos::prelude::*;

/// Input type variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Search,
    Tel,
    Url,
    Number,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Number => "number",
        }
    }
}

/// A single-line text input field.
///
/// # Example
/// ```ignore
/// view! {
///     <Input placeholder="Enter email" />
///     <Input input_type=InputType::Password placeholder="Password" />
/// }
/// ```
#[component]
pub fn Input(
    /// Input type.
    #[prop(optional, default = InputType::Text)]
    input_type: InputType,
    /// Placeholder text.
    #[prop(optional, default = String::new())]
    placeholder: String,
    /// Current value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Whether the input is disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the input is read-only.
    #[prop(optional, default = false)]
    readonly: bool,
    /// Whether the input is required.
    #[prop(optional, default = false)]
    required: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Accessible label.
    #[prop(optional)]
    aria_label: Option<String>,
    /// Input handler (fires on every keystroke).
    #[prop(optional)]
    on_input: Option<Callback<String>>,
    /// Change handler (fires on blur/enter).
    #[prop(optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let base = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{} {}", base, class);

    let input_val = value.unwrap_or_else(|| RwSignal::new(String::new()));

    view! {
        <input
            type=input_type.as_str()
            class=classes
            placeholder=placeholder
            disabled=disabled
            readonly=readonly
            required=required
            aria-label=aria_label
            aria-required=required.then_some("true")
            aria-disabled=disabled.then_some("true")
            aria-readonly=readonly.then_some("true")
            prop:value=move || input_val.get()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                input_val.set(val.clone());
                if let Some(handler) = &on_input {
                    handler.run(val);
                }
            }
            on:change=move |ev| {
                let val = event_target_value(&ev);
                if let Some(handler) = &on_change {
                    handler.run(val);
                }
            }
        />
    }
}
