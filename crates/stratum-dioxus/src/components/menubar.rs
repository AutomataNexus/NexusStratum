//! Menubar for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Menubar(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!(
        "flex h-9 items-center space-x-1 rounded-md border bg-background p-1 shadow-sm {class}"
    );
    rsx! { div { class: "{classes}", role: "menubar", {children} } }
}

#[component]
pub fn MenubarMenu(#[props(default = String::new())] class: String, children: Element) -> Element {
    let open = use_signal(|| false);
    use_context_provider(|| open);
    rsx! { div { class: "relative {class}", {children} } }
}

#[component]
pub fn MenubarTrigger(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let mut open = use_context::<Signal<bool>>();
    let classes = format!(
        "flex cursor-default select-none items-center rounded-sm px-3 py-1 text-sm font-medium outline-none focus:bg-accent {class}"
    );
    rsx! { button { class: "{classes}", onclick: move |_| open.set(!open()), {children} } }
}

#[component]
pub fn MenubarContent(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let open = use_context::<Signal<bool>>();
    if !open() {
        return rsx! {};
    }
    let classes = format!(
        "absolute left-0 top-full z-50 mt-1 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md {class}"
    );
    rsx! { div { class: "{classes}", role: "menu", {children} } }
}

#[component]
pub fn MenubarItem(
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_select: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let mut open = use_context::<Signal<bool>>();
    let cls = if disabled {
        "pointer-events-none opacity-50"
    } else {
        "cursor-pointer"
    };
    let classes = format!(
        "relative flex select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground {cls} {class}"
    );
    rsx! { div { class: "{classes}", role: "menuitem", onclick: move |_| {
        if !disabled { if let Some(h) = &on_select { h.call(()); } open.set(false); }
    }, {children} } }
}

#[component]
pub fn MenubarSeparator(#[props(default = String::new())] class: String) -> Element {
    let classes = format!("-mx-1 my-1 h-px bg-muted {class}");
    rsx! { div { class: "{classes}", role: "separator" } }
}
