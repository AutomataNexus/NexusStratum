//! Tabs component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Tabs(
    value: Signal<String>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    use_context_provider(|| value);
    rsx! { div { class: "{class}", {children} } }
}

#[component]
pub fn TabsList(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!(
        "inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground {class}"
    );
    rsx! { div { class: "{classes}", role: "tablist", {children} } }
}

#[component]
pub fn TabsTrigger(
    value: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let mut active_tab = use_context::<Signal<String>>();
    let val = value.clone();
    let val2 = value.clone();

    let base = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

    let active_cls = if active_tab() == val2 {
        "bg-background text-foreground shadow"
    } else {
        ""
    };
    let classes = format!("{base} {active_cls} {class}");

    rsx! {
        button {
            class: "{classes}",
            role: "tab",
            aria_selected: if active_tab() == val { "true" } else { "false" },
            onclick: move |_| active_tab.set(val.clone()),
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    value: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let active_tab = use_context::<Signal<String>>();
    if active_tab() != value {
        return rsx! {};
    }
    let classes = format!(
        "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {class}"
    );
    rsx! { div { class: "{classes}", role: "tabpanel", {children} } }
}
