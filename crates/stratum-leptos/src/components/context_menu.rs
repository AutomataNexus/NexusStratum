//! ContextMenu (right-click menu) for Leptos.

use leptos::prelude::*;

#[component]
pub fn ContextMenu(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let pos = RwSignal::new((0i32, 0i32));
    provide_context((open, pos));

    view! {
        <div class=class on:contextmenu=move |ev| {
            ev.prevent_default();
            pos.set((ev.client_x(), ev.client_y()));
            open.set(true);
        } on:click=move |_| open.set(false)>
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (open, pos) = use_context::<(RwSignal<bool>, RwSignal<(i32, i32)>)>()
        .expect("ContextMenuContent must be inside ContextMenu");

    let classes = format!(
        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md {}",
        class
    );
    let rendered = children();

    view! {
        <div
            class=classes
            role="menu"
            style=move || {
                if open.get() {
                    let (x, y) = pos.get();
                    format!("position:fixed;left:{}px;top:{}px", x, y)
                } else {
                    "display:none".to_string()
                }
            }
        >
            {rendered}
        </div>
    }
}

#[component]
pub fn ContextMenuItem(
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let (open, _) = use_context::<(RwSignal<bool>, RwSignal<(i32, i32)>)>()
        .expect("ContextMenuItem must be inside ContextMenu");
    let cls = if disabled {
        "pointer-events-none opacity-50"
    } else {
        "cursor-pointer"
    };
    let classes = format!(
        "relative flex select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground {} {}",
        cls, class
    );
    view! {
        <div class=classes role="menuitem" on:click=move |_| {
            if !disabled { if let Some(handler) = &on_select { handler.run(()); } open.set(false); }
        }>{children()}</div>
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let classes = format!("-mx-1 my-1 h-px bg-border {}", class);
    view! { <div class=classes role="separator"></div> }
}
