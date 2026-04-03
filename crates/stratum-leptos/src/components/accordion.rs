//! Accordion component for Leptos.

use leptos::prelude::*;

/// A collapsible accordion container.
#[component]
pub fn Accordion(
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("divide-y divide-border {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// A single accordion item.
#[component]
pub fn AccordionItem(
    /// Whether this item is open.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,
    /// Default open state.
    #[prop(optional, default = false)]
    default_open: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let is_open = open.unwrap_or_else(|| RwSignal::new(default_open));
    provide_context(is_open);

    let classes = format!("border-b {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// The clickable trigger that toggles an AccordionItem.
#[component]
pub fn AccordionTrigger(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let is_open =
        use_context::<RwSignal<bool>>().expect("AccordionTrigger must be inside AccordionItem");

    let classes = format!(
        "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180 {}",
        class
    );

    view! {
        <button
            class=classes
            aria-expanded=move || if is_open.get() { "true" } else { "false" }
            on:click=move |_| is_open.update(|v| *v = !*v)
        >
            {children()}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200"
                style=move || if is_open.get() { "transform: rotate(180deg)" } else { "" }
            >
                <path d="m6 9 6 6 6-6"></path>
            </svg>
        </button>
    }
}

/// The content revealed when an AccordionItem is open.
#[component]
pub fn AccordionContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let is_open =
        use_context::<RwSignal<bool>>().expect("AccordionContent must be inside AccordionItem");

    let classes = format!("overflow-hidden text-sm pb-4 pt-0 {}", class);

    let rendered = children();

    view! {
        <div
            class=classes
            role="region"
            style=move || if is_open.get() { "" } else { "display:none" }
        >
            {rendered}
        </div>
    }
}
