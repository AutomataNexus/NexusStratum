//! Pagination component for Leptos.

use leptos::prelude::*;

#[component]
pub fn Pagination(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("mx-auto flex w-full justify-center {}", class);
    view! { <nav role="navigation" aria-label="pagination" class=classes>{children()}</nav> }
}

#[component]
pub fn PaginationContent(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("flex flex-row items-center gap-1 {}", class);
    view! { <ul class=classes>{children()}</ul> }
}

#[component]
pub fn PaginationItem(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    view! { <li class=class>{children()}</li> }
}

#[component]
pub fn PaginationLink(
    href: String,
    #[prop(optional, default = false)] is_active: bool,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9";
    let active_cls = if is_active { "border border-input bg-background shadow-sm" } else { "" };
    let classes = format!("{} {} {}", base, active_cls, class);
    view! { <a href=href class=classes aria-current=is_active.then_some("page")>{children()}</a> }
}

#[component]
pub fn PaginationPrevious(
    href: String,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let classes = format!("gap-1 pl-2.5 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 {}", class);
    view! {
        <a href=href class=classes aria-label="Go to previous page">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4"><path d="m15 18-6-6 6-6"></path></svg>
            "Previous"
        </a>
    }
}

#[component]
pub fn PaginationNext(
    href: String,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let classes = format!("gap-1 pr-2.5 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 {}", class);
    view! {
        <a href=href class=classes aria-label="Go to next page">
            "Next"
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4"><path d="m9 18 6-6-6-6"></path></svg>
        </a>
    }
}
