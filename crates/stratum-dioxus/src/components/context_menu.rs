//! ContextMenu for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn ContextMenu(#[props(default = String::new())] class: String, children: Element) -> Element {
    let mut open = use_signal(|| false);
    let mut pos = use_signal(|| (0i32, 0i32));
    use_context_provider(|| (open, pos));

    rsx! {
        div { class: "{class}",
            oncontextmenu: move |evt| { evt.prevent_default(); pos.set((evt.client_coordinates().x as i32, evt.client_coordinates().y as i32)); open.set(true); },
            onclick: move |_| open.set(false),
            {children}
        }
    }
}

#[component]
pub fn ContextMenuContent(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let (open, pos) = use_context::<(Signal<bool>, Signal<(i32, i32)>)>();
    if !open() {
        return rsx! {};
    }
    let (x, y) = pos();
    let classes = format!(
        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md {class}"
    );
    rsx! { div { class: "{classes}", role: "menu", style: "position:fixed;left:{x}px;top:{y}px", {children} } }
}

#[component]
pub fn ContextMenuItem(
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_select: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let mut open = use_context::<(Signal<bool>, Signal<(i32, i32)>)>().0;
    let cls = if disabled {
        "pointer-events-none opacity-50"
    } else {
        "cursor-pointer"
    };
    let classes = format!(
        "relative flex select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground {cls} {class}"
    );
    rsx! {
        div { class: "{classes}", role: "menuitem", onclick: move |_| {
            if !disabled { if let Some(handler) = &on_select { handler.call(()); } open.set(false); }
        }, {children} }
    }
}

#[component]
pub fn ContextMenuSeparator(#[props(default = String::new())] class: String) -> Element {
    let classes = format!("-mx-1 my-1 h-px bg-border {class}");
    rsx! { div { class: "{classes}", role: "separator" } }
}
