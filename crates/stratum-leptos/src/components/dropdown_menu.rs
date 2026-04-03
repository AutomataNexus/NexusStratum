//! DropdownMenu component for Leptos.

use leptos::prelude::*;

/// A dropdown menu triggered by a button.
#[component]
pub fn DropdownMenu(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);
    view! { <div class=format!("relative inline-block text-left {}", class)>{children()}</div> }
}

/// The trigger button for the dropdown.
#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("DropdownMenuTrigger must be inside DropdownMenu");
    view! {
        <div on:click=move |_| open.update(|v| *v = !*v) class=class aria-expanded=move || if open.get() { "true" } else { "false" } aria-haspopup="menu">
            {children()}
        </div>
    }
}

/// The dropdown content panel.
#[component]
pub fn DropdownMenuContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("DropdownMenuContent must be inside DropdownMenu");
    let classes = format!(
        "absolute right-0 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md {}",
        class
    );
    let rendered = children();
    view! {
        <div class=classes role="menu" style=move || if open.get() { "" } else { "display:none" }>
            {rendered}
        </div>
    }
}

/// A menu item.
#[component]
pub fn DropdownMenuItem(
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("DropdownMenuItem must be inside DropdownMenu");
    let classes = format!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground {} {}",
        if disabled { "pointer-events-none opacity-50" } else { "cursor-pointer" },
        class
    );
    view! {
        <div class=classes role="menuitem" aria-disabled=disabled.then_some("true") on:click=move |_| {
            if !disabled {
                if let Some(handler) = &on_select { handler.run(()); }
                open.set(false);
            }
        }>
            {children()}
        </div>
    }
}

/// A visual separator in the menu.
#[component]
pub fn DropdownMenuSeparator(#[prop(optional, default = String::new())] class: String) -> impl IntoView {
    let classes = format!("-mx-1 my-1 h-px bg-muted {}", class);
    view! { <div class=classes role="separator"></div> }
}

/// A label/heading within the menu.
#[component]
pub fn DropdownMenuLabel(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("px-2 py-1.5 text-sm font-semibold {}", class);
    view! { <div class=classes>{children()}</div> }
}
