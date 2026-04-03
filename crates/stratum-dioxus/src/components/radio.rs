//! Radio group component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn RadioGroup(
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = String::from("radio-group"))] name: String,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("grid gap-2 {class}");
    rsx! { div { class: "{classes}", role: "radiogroup", {children} } }
}

#[component]
pub fn RadioItem(
    value: String,
    name: String,
    #[props(optional)] selected: Option<Signal<String>>,
    #[props(default = false)] disabled: bool,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let opacity = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };
    let val = value.clone();
    let val2 = value.clone();
    let is_selected = selected.map(|s| s() == val2).unwrap_or(false);

    rsx! {
        label { class: "flex items-center gap-2 {opacity}",
            input {
                r#type: "radio",
                name: "{name}",
                value: "{val}",
                disabled: disabled,
                checked: is_selected,
                class: "aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                onchange: move |_| {
                    if let Some(handler) = &on_change {
                        handler.call(val.clone());
                    }
                },
            }
            span { class: "text-sm font-medium leading-none", {children} }
        }
    }
}
