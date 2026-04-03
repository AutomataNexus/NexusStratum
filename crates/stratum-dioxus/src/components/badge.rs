//! Badge component for Dioxus.

use dioxus::prelude::*;

/// Badge visual variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

/// A small status label.
#[component]
pub fn Badge(
    #[props(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let base = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors";

    let variant_cls = match variant {
        BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground shadow",
        BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground",
        BadgeVariant::Outline => "text-foreground",
        BadgeVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground shadow",
    };

    let classes = format!("{base} {variant_cls} {class}");

    rsx! { span { class: "{classes}", {children} } }
}
