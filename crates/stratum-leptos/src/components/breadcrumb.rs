//! Breadcrumb navigation for Leptos.

use leptos::prelude::*;

#[component]
pub fn Breadcrumb(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 {}",
        class
    );
    view! { <nav aria-label="breadcrumb"><ol class=classes>{children()}</ol></nav> }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("inline-flex items-center gap-1.5 {}", class);
    view! { <li class=classes>{children()}</li> }
}

#[component]
pub fn BreadcrumbLink(
    href: String,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("transition-colors hover:text-foreground {}", class);
    view! { <a href=href class=classes>{children()}</a> }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let classes = format!("[&>svg]:size-3.5 {}", class);
    view! {
        <li role="presentation" aria-hidden="true" class=classes>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-3.5 w-3.5"><path d="m9 18 6-6-6-6"></path></svg>
        </li>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("font-normal text-foreground {}", class);
    view! { <span role="link" aria-disabled="true" aria-current="page" class=classes>{children()}</span> }
}
