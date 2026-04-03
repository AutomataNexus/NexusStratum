//! Checkbox component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Checkbox(
    #[props(optional)] checked: Option<Signal<bool>>,
    #[props(default = false)] default_checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] required: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<bool>>,
    #[props(optional)] children: Option<Element>,
) -> Element {
    let mut is_checked = checked.unwrap_or_else(|| Signal::new(default_checked));

    let base = "peer h-4 w-4 shrink-0 rounded-sm border border-primary shadow focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let classes = format!("{base} {class}");

    rsx! {
        label {
            class: "flex items-center gap-2 cursor-pointer",
            input {
                r#type: "checkbox",
                class: "{classes}",
                disabled: disabled,
                required: required,
                checked: is_checked(),
                onchange: move |evt: Event<FormData>| {
                    let val = evt.checked();
                    is_checked.set(val);
                    if let Some(handler) = &on_change {
                        handler.call(val);
                    }
                },
            }
            {children}
        }
    }
}
