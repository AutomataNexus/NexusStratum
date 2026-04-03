//! Carousel for Leptos.

use leptos::prelude::*;

#[component]
pub fn Carousel(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("relative {}", class);
    view! { <div class=classes role="region" aria-roledescription="carousel">{children()}</div> }
}

#[component]
pub fn CarouselContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex overflow-hidden {}", class);
    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CarouselItem(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("min-w-0 shrink-0 grow-0 basis-full {}", class);
    view! { <div class=classes role="group" aria-roledescription="slide">{children()}</div> }
}

#[component]
pub fn CarouselPrevious(
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let classes = format!(
        "absolute left-2 top-1/2 -translate-y-1/2 inline-flex items-center justify-center rounded-full border bg-background h-8 w-8 shadow-sm hover:bg-accent {}",
        class
    );
    view! {
        <button class=classes aria-label="Previous slide" on:click=move |_| { if let Some(h) = &on_click { h.run(()); } }>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4"><path d="m15 18-6-6 6-6"></path></svg>
        </button>
    }
}

#[component]
pub fn CarouselNext(
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let classes = format!(
        "absolute right-2 top-1/2 -translate-y-1/2 inline-flex items-center justify-center rounded-full border bg-background h-8 w-8 shadow-sm hover:bg-accent {}",
        class
    );
    view! {
        <button class=classes aria-label="Next slide" on:click=move |_| { if let Some(h) = &on_click { h.run(()); } }>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4"><path d="m9 18 6-6-6-6"></path></svg>
        </button>
    }
}
