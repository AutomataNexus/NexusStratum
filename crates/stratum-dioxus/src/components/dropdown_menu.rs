//! DropdownMenu component for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn DropdownMenu(#[props(default = String::new())] class: String, children: Element) -> Element {
    let open = use_signal(|| false);
    use_context_provider(|| open);
    let classes = format!("relative inline-block text-left {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn DropdownMenuTrigger(#[props(default = String::new())] class: String, children: Element) -> Element {
    let mut open = use_context::<Signal<bool>>();
    rsx! {
        div { class: "{class}", onclick: move |_| open.set(!open()), aria_expanded: if open() { "true" } else { "false" }, aria_haspopup: "menu",
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuContent(#[props(default = String::new())] class: String, children: Element) -> Element {
    let open = use_context::<Signal<bool>>();
    if !open() { return rsx! {}; }
    let classes = format!("absolute right-0 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md {class}");
    rsx! { div { class: "{classes}", role: "menu", {children} } }
}

#[component]
pub fn DropdownMenuItem(
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_select: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let mut open = use_context::<Signal<bool>>();
    let cls = if disabled { "pointer-events-none opacity-50" } else { "cursor-pointer" };
    let classes = format!("relative flex select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground {cls} {class}");
    rsx! {
        div { class: "{classes}", role: "menuitem", onclick: move |_| {
            if !disabled { if let Some(handler) = &on_select { handler.call(()); } open.set(false); }
        }, {children} }
    }
}

#[component]
pub fn DropdownMenuSeparator(#[props(default = String::new())] class: String) -> Element {
    let classes = format!("-mx-1 my-1 h-px bg-muted {class}");
    rsx! { div { class: "{classes}", role: "separator" } }
}

#[component]
pub fn DropdownMenuLabel(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("px-2 py-1.5 text-sm font-semibold {class}");
    rsx! { div { class: "{classes}", {children} } }
}
