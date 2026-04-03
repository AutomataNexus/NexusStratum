//! Switch component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Switch(
    #[props(optional)] checked: Option<Signal<bool>>,
    #[props(default = false)] default_checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<bool>>,
    #[props(optional)] children: Option<Element>,
) -> Element {
    let mut is_on = checked.unwrap_or_else(|| Signal::new(default_checked));

    let track_cls = if is_on() { "bg-primary" } else { "bg-input" };
    let thumb_translate = if is_on() { "translateX(16px)" } else { "translateX(0px)" };
    let opacity = if disabled { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" };

    rsx! {
        label { class: "flex items-center gap-2 {opacity}",
            button {
                r#type: "button",
                role: "switch",
                class: "peer inline-flex h-5 w-9 shrink-0 items-center rounded-full border-2 border-transparent shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {track_cls} {class}",
                disabled: disabled,
                aria_checked: if is_on() { "true" } else { "false" },
                onclick: move |_| {
                    if !disabled {
                        let new_val = !is_on();
                        is_on.set(new_val);
                        if let Some(handler) = &on_change {
                            handler.call(new_val);
                        }
                    }
                },
                span {
                    class: "pointer-events-none block h-4 w-4 rounded-full bg-background shadow-lg ring-0 transition-transform",
                    style: "transform: {thumb_translate}",
                }
            }
            {children}
        }
    }
}
