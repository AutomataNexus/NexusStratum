//! NavigationMenu for Leptos.

use leptos::prelude::*;

#[component]
pub fn NavigationMenu(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "relative z-10 flex max-w-max flex-1 items-center justify-center {}",
        class
    );
    view! { <nav class=classes>{children()}</nav> }
}

#[component]
pub fn NavigationMenuList(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "group flex flex-1 list-none items-center justify-center space-x-1 {}",
        class
    );
    view! { <ul class=classes>{children()}</ul> }
}

#[component]
pub fn NavigationMenuItem(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    view! { <li class=class>{children()}</li> }
}

#[component]
pub fn NavigationMenuLink(
    href: String,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 {}",
        class
    );
    view! { <a href=href class=classes>{children()}</a> }
}
