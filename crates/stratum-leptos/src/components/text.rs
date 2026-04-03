//! Text component for Leptos.

use leptos::prelude::*;

/// A paragraph or inline text element.
#[component]
pub fn Text(
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("leading-7 {}", class);
    view! { <p class=classes>{children()}</p> }
}

/// Muted/secondary text.
#[component]
pub fn Muted(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("text-sm text-muted-foreground {}", class);
    view! { <p class=classes>{children()}</p> }
}

/// Inline code display.
#[component]
pub fn InlineCode(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold {}",
        class
    );
    view! { <code class=classes>{children()}</code> }
}

/// A styled link.
#[component]
pub fn Link(
    /// URL destination.
    href: String,
    /// Whether external (adds target="_blank").
    #[prop(optional, default = false)]
    external: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "font-medium text-primary underline underline-offset-4 hover:text-primary/80 {}",
        class
    );
    view! {
        <a
            href=href
            class=classes
            target=external.then_some("_blank")
            rel=external.then_some("noopener noreferrer")
        >
            {children()}
        </a>
    }
}
