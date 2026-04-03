//! Command palette for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Command(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground {class}");
    rsx! { div { class: "{classes}", {children} } }
}

#[component]
pub fn CommandInput(
    #[props(default = String::from("Search..."))] placeholder: String,
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = String::new())] class: String,
) -> Element {
    let mut val = value.unwrap_or_else(|| Signal::new(String::new()));
    let classes = format!("flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground {class}");
    rsx! {
        div { class: "flex items-center border-b px-3",
            svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", class: "mr-2 h-4 w-4 shrink-0 opacity-50",
                circle { cx: "11", cy: "11", r: "8" }
                path { d: "m21 21-4.3-4.3" }
            }
            input { class: "{classes}", placeholder: "{placeholder}", value: "{val()}", oninput: move |evt: Event<FormData>| val.set(evt.value()) }
        }
    }
}

#[component]
pub fn CommandList(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("max-h-[300px] overflow-y-auto overflow-x-hidden {class}");
    rsx! { div { class: "{classes}", role: "listbox", {children} } }
}

#[component]
pub fn CommandGroup(#[props(optional)] heading: Option<String>, #[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("overflow-hidden p-1 text-foreground {class}");
    rsx! {
        div { class: "{classes}", role: "group",
            if let Some(h) = &heading { div { class: "px-2 py-1.5 text-xs font-medium text-muted-foreground", "{h}" } }
            {children}
        }
    }
}

#[component]
pub fn CommandItem(
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_select: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let cls = if disabled { "pointer-events-none opacity-50" } else { "cursor-pointer" };
    let classes = format!("relative flex select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground {cls} {class}");
    rsx! {
        div { class: "{classes}", role: "option", onclick: move |_| {
            if !disabled { if let Some(handler) = &on_select { handler.call(()); } }
        }, {children} }
    }
}

#[component]
pub fn CommandSeparator(#[props(default = String::new())] class: String) -> Element {
    let classes = format!("-mx-1 h-px bg-border {class}");
    rsx! { div { class: "{classes}" } }
}

#[component]
pub fn CommandEmpty(#[props(default = String::new())] class: String, children: Element) -> Element {
    let classes = format!("py-6 text-center text-sm {class}");
    rsx! { div { class: "{classes}", {children} } }
}
