//! Menubar (horizontal menu) for Leptos.

use leptos::prelude::*;

#[component]
pub fn Menubar(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "flex h-9 items-center space-x-1 rounded-md border bg-background p-1 shadow-sm {}",
        class
    );
    view! { <div class=classes role="menubar">{children()}</div> }
}

#[component]
pub fn MenubarMenu(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);
    view! { <div class=format!("relative {}", class)>{children()}</div> }
}

#[component]
pub fn MenubarTrigger(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("MenubarTrigger must be inside MenubarMenu");
    let classes = format!(
        "flex cursor-default select-none items-center rounded-sm px-3 py-1 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground {}",
        class
    );
    view! {
        <button class=classes data-state=move || if open.get() { "open" } else { "closed" } on:click=move |_| open.update(|v| *v = !*v)>
            {children()}
        </button>
    }
}

#[component]
pub fn MenubarContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("MenubarContent must be inside MenubarMenu");
    let classes = format!(
        "absolute left-0 top-full z-50 mt-1 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md {}",
        class
    );
    let rendered = children();
    view! { <div class=classes role="menu" style=move || if open.get() { "" } else { "display:none" }>{rendered}</div> }
}

#[component]
pub fn MenubarItem(
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("MenubarItem must be inside MenubarMenu");
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
            if !disabled { if let Some(h) = &on_select { h.run(()); } open.set(false); }
        }>{children()}</div>
    }
}

#[component]
pub fn MenubarSeparator(#[prop(optional, default = String::new())] class: String) -> impl IntoView {
    let classes = format!("-mx-1 my-1 h-px bg-muted {}", class);
    view! { <div class=classes role="separator"></div> }
}
