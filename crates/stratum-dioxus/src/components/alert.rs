//! Alert component for Dioxus.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

#[component]
pub fn Alert(
    #[props(default = AlertVariant::Default)] variant: AlertVariant,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let base = "relative w-full rounded-lg border px-4 py-3 text-sm";
    let variant_cls = match variant {
        AlertVariant::Default => "bg-background text-foreground",
        AlertVariant::Destructive => "border-destructive/50 text-destructive",
    };
    let classes = format!("{base} {variant_cls} {class}");
    rsx! { div { class: "{classes}", role: "alert", {children} } }
}

#[component]
pub fn AlertTitle(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("mb-1 font-medium leading-none tracking-tight {class}");
    rsx! { h5 { class: "{classes}", {children} } }
}

#[component]
pub fn AlertDescription(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("text-sm [&_p]:leading-relaxed {class}");
    rsx! { div { class: "{classes}", {children} } }
}
