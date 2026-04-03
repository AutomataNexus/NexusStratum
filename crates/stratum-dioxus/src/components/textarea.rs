//! Textarea component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Textarea(
    #[props(default = String::new())] placeholder: String,
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = 3)] rows: u32,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] readonly: bool,
    #[props(default = false)] required: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
) -> Element {
    let base = "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{base} {class}");

    rsx! {
        textarea {
            class: "{classes}",
            placeholder: "{placeholder}",
            rows: "{rows}",
            disabled: disabled,
            readonly: readonly,
            required: required,
            value: value.map(|v| v()),
            oninput: move |evt| {
                if let Some(handler) = &on_input {
                    handler.call(evt);
                }
            },
        }
    }
}
