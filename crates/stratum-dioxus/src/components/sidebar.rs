//! Sidebar for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Sidebar(
    #[props(optional)] open: Option<Signal<bool>>,
    #[props(default = true)] default_open: bool,
    #[props(default = String::from("left"))] side: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let is_open = open.unwrap_or_else(|| Signal::new(default_open));
    use_context_provider(|| is_open);
    let side_cls = if side == "right" { "order-last" } else { "" };
    let width = if is_open() {
        "width: 16rem"
    } else {
        "width: 3rem"
    };
    let classes = format!(
        "flex h-full flex-col border-r bg-sidebar text-sidebar-foreground transition-all duration-300 {side_cls} {class}"
    );
    rsx! { aside { class: "{classes}", style: "{width}", aria_label: "Sidebar", {children} } }
}

#[component]
pub fn SidebarHeader(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("flex flex-col gap-2 p-2 {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn SidebarContent(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("flex min-h-0 flex-1 flex-col gap-2 overflow-auto p-2 {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn SidebarFooter(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("flex flex-col gap-2 p-2 {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn SidebarGroup(
    #[props(optional)] label: Option<String>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!("relative flex w-full min-w-0 flex-col p-2 {class}");
    rsx! {
        div { class: "{classes}",
            if let Some(l) = &label { div { class: "flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium text-sidebar-foreground/70", "{l}" } }
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuItem(
    #[props(default = false)] active: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_click: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let active_cls = if active {
        "bg-sidebar-accent text-sidebar-accent-foreground"
    } else {
        ""
    };
    let classes = format!(
        "flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-none transition-all hover:bg-sidebar-accent hover:text-sidebar-accent-foreground {active_cls} {class}"
    );
    rsx! { button { class: "{classes}", onclick: move |_| { if let Some(h) = &on_click { h.call(()); } }, {children} } }
}

#[component]
pub fn SidebarTrigger(#[props(default = String::new())] class: String) -> Element {
    let mut is_open = use_context::<Signal<bool>>();
    let classes = format!(
        "inline-flex items-center justify-center rounded-md h-7 w-7 text-sm hover:bg-sidebar-accent hover:text-sidebar-accent-foreground {class}"
    );
    rsx! {
        button { class: "{classes}", aria_label: "Toggle sidebar", onclick: move |_| is_open.set(!is_open()),
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "h-4 w-4",
                rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
                path { d: "M9 3v18" }
            }
        }
    }
}
