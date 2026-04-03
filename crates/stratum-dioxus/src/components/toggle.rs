//! Toggle button component for Dioxus.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[component]
pub fn Toggle(
    #[props(optional)] pressed: Option<Signal<bool>>,
    #[props(default = false)] default_pressed: bool,
    #[props(default = ToggleVariant::Default)] variant: ToggleVariant,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let mut is_pressed = pressed.unwrap_or_else(|| Signal::new(default_pressed));
    let base = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 h-9 px-3";
    let variant_cls = match variant {
        ToggleVariant::Default => "",
        ToggleVariant::Outline => "border border-input bg-transparent shadow-sm",
    };
    let state_cls = if is_pressed() {
        "bg-accent text-accent-foreground"
    } else {
        ""
    };
    let classes = format!("{base} {variant_cls} {state_cls} {class}");

    rsx! {
        button {
            class: "{classes}",
            aria_pressed: if is_pressed() { "true" } else { "false" },
            disabled: disabled,
            onclick: move |_| {
                let new_val = !is_pressed();
                is_pressed.set(new_val);
                if let Some(handler) = &on_change { handler.call(new_val); }
            },
            {children}
        }
    }
}
