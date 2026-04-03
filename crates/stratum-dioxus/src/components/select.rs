//! Select component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Select(
    #[props(default = String::new())] placeholder: String,
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let base = "flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{base} {class}");

    let mut selected = value.unwrap_or_else(|| Signal::new(String::new()));

    rsx! {
        select {
            class: "{classes}",
            disabled: disabled,
            value: selected(),
            onchange: move |evt: Event<FormData>| {
                let val = evt.value();
                selected.set(val.clone());
                if let Some(handler) = &on_change {
                    handler.call(val);
                }
            },
            if !placeholder.is_empty() {
                option { value: "", disabled: true, selected: true, "{placeholder}" }
            }
            {children}
        }
    }
}

#[component]
pub fn SelectOption(
    value: String,
    #[props(default = false)] disabled: bool,
    children: Element,
) -> Element {
    rsx! { option { value: "{value}", disabled: disabled, {children} } }
}
