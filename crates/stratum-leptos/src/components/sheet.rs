//! Sheet (side panel overlay) for Leptos.

use leptos::prelude::*;

/// Side from which the sheet slides in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

/// A slide-in overlay panel.
#[component]
pub fn Sheet(
    open: ReadSignal<bool>,
    #[prop(optional, default = SheetSide::Right)] side: SheetSide,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let side_cls = match side {
        SheetSide::Top => "inset-x-0 top-0 border-b",
        SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
        SheetSide::Bottom => "inset-x-0 bottom-0 border-t",
        SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
    };

    let classes = format!(
        "fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out duration-300 {} {}",
        side_cls, class
    );
    let rendered = children();

    view! {
        <div style=move || if open.get() { "" } else { "display:none" }>
            <div
                class="fixed inset-0 z-50 bg-black/80"
                on:click=move |_| {
                    if let Some(handler) = &on_close {
                        handler.run(());
                    }
                }
            ></div>
            <div class=classes.clone() role="dialog">
                {rendered}
            </div>
        </div>
    }
}
