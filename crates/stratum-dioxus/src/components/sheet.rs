//! Sheet (side panel overlay) for Dioxus.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

#[component]
pub fn Sheet(
    open: Signal<bool>,
    #[props(default = SheetSide::Right)] side: SheetSide,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    if !open() {
        return rsx! {};
    }

    let side_cls = match side {
        SheetSide::Top => "inset-x-0 top-0 border-b",
        SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
        SheetSide::Bottom => "inset-x-0 bottom-0 border-t",
        SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
    };
    let classes = format!(
        "fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out duration-300 {side_cls} {class}"
    );

    rsx! {
        div { class: "fixed inset-0 z-50 bg-black/80",
            onclick: move |_| { if let Some(handler) = &on_close { handler.call(()); } },
        }
        div { class: "{classes}", role: "dialog", {children} }
    }
}
