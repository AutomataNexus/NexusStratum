//! Toast notification component for Leptos.

use leptos::prelude::*;

/// Toast visual variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
}

/// A toast notification message.
#[component]
pub fn Toast(
    #[prop(optional, default = ToastVariant::Default)]
    variant: ToastVariant,
    /// Whether visible.
    #[prop(optional, default = true)]
    open: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let base = "group pointer-events-auto relative flex w-full items-center justify-between space-x-2 overflow-hidden rounded-md border p-4 pr-6 shadow-lg transition-all";

    let variant_cls = match variant {
        ToastVariant::Default => "bg-background text-foreground border",
        ToastVariant::Success => "bg-background text-foreground border-l-4 border-l-green-500",
        ToastVariant::Error => "bg-destructive text-destructive-foreground border-destructive",
        ToastVariant::Warning => "bg-background text-foreground border-l-4 border-l-yellow-500",
    };

    let role = match variant {
        ToastVariant::Error => "alert",
        _ => "status",
    };

    let classes = format!("{} {} {}", base, variant_cls, class);

    view! {
        <div
            class=classes
            role=role
            aria-live=if matches!(variant, ToastVariant::Error) { "assertive" } else { "polite" }
            style=if open { "" } else { "display:none" }
        >
            {children()}
        </div>
    }
}

/// Container for positioning toasts.
#[component]
pub fn Toaster(
    #[prop(optional, default = String::from("bottom-right"))]
    position: String,
    children: Children,
) -> impl IntoView {
    let pos_cls = match position.as_str() {
        "top-left" => "top-0 left-0",
        "top-center" => "top-0 left-1/2 -translate-x-1/2",
        "top-right" => "top-0 right-0",
        "bottom-left" => "bottom-0 left-0",
        "bottom-center" => "bottom-0 left-1/2 -translate-x-1/2",
        _ => "bottom-0 right-0",
    };

    let classes = format!(
        "fixed z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:flex-col md:max-w-[420px] {}",
        pos_cls
    );

    view! { <div class=classes>{children()}</div> }
}
