//! Card component for Leptos.

use leptos::prelude::*;

/// A container with border, shadow, and rounded corners.
#[component]
pub fn Card(
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "rounded-xl border bg-card text-card-foreground shadow {}",
        class
    );
    view! { <div class=classes>{children()}</div> }
}

/// Card header section.
#[component]
pub fn CardHeader(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex flex-col space-y-1.5 p-6 {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// Card title.
#[component]
pub fn CardTitle(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "font-semibold leading-none tracking-tight {}",
        class
    );
    view! { <h3 class=classes>{children()}</h3> }
}

/// Card description.
#[component]
pub fn CardDescription(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("text-sm text-muted-foreground {}", class);
    view! { <p class=classes>{children()}</p> }
}

/// Card main content area.
#[component]
pub fn CardContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("p-6 pt-0 {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// Card footer section.
#[component]
pub fn CardFooter(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex items-center p-6 pt-0 {}", class);
    view! { <div class=classes>{children()}</div> }
}
