//! Alert component for Leptos.

use leptos::prelude::*;

/// Alert variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

/// A feedback message for important information.
#[component]
pub fn Alert(
    /// Visual variant.
    #[prop(optional, default = AlertVariant::Default)]
    variant: AlertVariant,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let base = "relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg~*]:pl-7";

    let variant_cls = match variant {
        AlertVariant::Default => "bg-background text-foreground",
        AlertVariant::Destructive => {
            "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
        }
    };

    let classes = format!("{} {} {}", base, variant_cls, class);

    view! {
        <div class=classes role="alert">
            {children()}
        </div>
    }
}

/// Alert title.
#[component]
pub fn AlertTitle(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "mb-1 font-medium leading-none tracking-tight {}",
        class
    );
    view! { <h5 class=classes>{children()}</h5> }
}

/// Alert description.
#[component]
pub fn AlertDescription(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("text-sm [&_p]:leading-relaxed {}", class);
    view! { <div class=classes>{children()}</div> }
}
