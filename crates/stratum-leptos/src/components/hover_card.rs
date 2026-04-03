//! HoverCard component for Leptos.

use leptos::prelude::*;

/// A card that appears on hover for rich previews.
#[component]
pub fn HoverCard(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (visible, set_visible) = signal(false);
    provide_context((visible, set_visible));
    view! { <div class=format!("relative inline-block {}", class)
        on:mouseenter=move |_| set_visible.set(true)
        on:mouseleave=move |_| set_visible.set(false)
    >{children()}</div> }
}

/// The trigger element.
#[component]
pub fn HoverCardTrigger(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    view! { <div class=class>{children()}</div> }
}

/// The hover content.
#[component]
pub fn HoverCardContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (visible, _) = use_context::<(ReadSignal<bool>, WriteSignal<bool>)>()
        .expect("HoverCardContent must be inside HoverCard");
    let classes = format!(
        "absolute z-50 w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none top-full mt-2 {}",
        class
    );
    let rendered = children();
    view! {
        <div class=classes style=move || if visible.get() { "" } else { "display:none" }>
            {rendered}
        </div>
    }
}
