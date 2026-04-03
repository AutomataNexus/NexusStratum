//! NavigationMenu for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn NavigationMenu(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes =
        format!("relative z-10 flex max-w-max flex-1 items-center justify-center {class}");
    rsx! { nav { class: "{classes}", {children} } }
}

#[component]
pub fn NavigationMenuList(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes =
        format!("group flex flex-1 list-none items-center justify-center space-x-1 {class}");
    rsx! { ul { class: "{classes}", {children} } }
}

#[component]
pub fn NavigationMenuItem(
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    rsx! { li { class: "{class}", {children} } }
}

#[component]
pub fn NavigationMenuLink(
    href: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let classes = format!(
        "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:outline-none {class}"
    );
    rsx! { a { href: "{href}", class: "{classes}", {children} } }
}
