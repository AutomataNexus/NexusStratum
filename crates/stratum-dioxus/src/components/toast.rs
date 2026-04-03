//! Toast notification component for Dioxus.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
}

#[component]
pub fn Toast(
    #[props(default = ToastVariant::Default)] variant: ToastVariant,
    #[props(default = true)] open: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let base = "group pointer-events-auto relative flex w-full items-center justify-between space-x-2 overflow-hidden rounded-md border p-4 pr-6 shadow-lg transition-all";
    let variant_cls = match variant {
        ToastVariant::Default => "bg-background text-foreground border",
        ToastVariant::Success => "bg-background text-foreground border-l-4 border-l-green-500",
        ToastVariant::Error => "bg-destructive text-destructive-foreground border-destructive",
        ToastVariant::Warning => "bg-background text-foreground border-l-4 border-l-yellow-500",
    };
    let role = if matches!(variant, ToastVariant::Error) {
        "alert"
    } else {
        "status"
    };
    let live = if matches!(variant, ToastVariant::Error) {
        "assertive"
    } else {
        "polite"
    };
    let classes = format!("{base} {variant_cls} {class}");

    if !open {
        return rsx! {};
    }

    rsx! { div { class: "{classes}", role: "{role}", aria_live: "{live}", {children} } }
}

#[component]
pub fn Toaster(
    #[props(default = String::from("bottom-right"))] position: String,
    children: Element,
) -> Element {
    let pos_cls = match position.as_str() {
        "top-left" => "top-0 left-0",
        "top-center" => "top-0 left-1/2 -translate-x-1/2",
        "top-right" => "top-0 right-0",
        "bottom-left" => "bottom-0 left-0",
        "bottom-center" => "bottom-0 left-1/2 -translate-x-1/2",
        _ => "bottom-0 right-0",
    };
    let classes = format!(
        "fixed z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:flex-col md:max-w-[420px] {pos_cls}"
    );
    rsx! { div { class: "{classes}", {children} } }
}
