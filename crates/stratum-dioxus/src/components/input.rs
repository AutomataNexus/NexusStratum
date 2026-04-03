//! Input component for Dioxus.

use dioxus::prelude::*;

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

/// A single-line text input.
#[component]
pub fn Input(
    #[props(default = InputType::Text)] input_type: InputType,
    #[props(default = String::new())] placeholder: String,
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] readonly: bool,
    #[props(default = false)] required: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] aria_label: Option<String>,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
) -> Element {
    let base = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{base} {class}");

    rsx! {
        input {
            r#type: "{input_type.as_str()}",
            class: "{classes}",
            placeholder: "{placeholder}",
            disabled: disabled,
            readonly: readonly,
            required: required,
            aria_label: aria_label,
            value: value.map(|v| v.read().clone()),
            oninput: move |evt| {
                if let Some(handler) = &on_input {
                    handler.call(evt);
                }
            },
        }
    }
}
