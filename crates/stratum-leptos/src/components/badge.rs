//! Badge component for Leptos.

use leptos::prelude::*;

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
    /// Visual variant.
    #[prop(optional, default = BadgeVariant::Default)]
    variant: BadgeVariant,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Badge content.
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

    let variant_cls = match variant {
        BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80",
        BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
        BadgeVariant::Outline => "text-foreground",
        BadgeVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80",
    };

    let classes = format!("{} {} {}", base, variant_cls, class);

    view! { <span class=classes>{children()}</span> }
}
