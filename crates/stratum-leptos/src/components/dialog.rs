//! Dialog component for Leptos.

use leptos::prelude::*;

/// A modal dialog overlay.
#[component]
pub fn Dialog(
    /// Whether the dialog is open.
    open: ReadSignal<bool>,
    /// Close handler.
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Additional CSS classes for the content panel.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let content_classes = format!(
        "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 sm:rounded-lg {}",
        class,
    );

    let rendered = children();

    view! {
        {move || {
            if open.get() {
                let on_close_backdrop = on_close;
                let on_close_escape = on_close;
                Some(
                    view! {
                        <div
                            class="fixed inset-0 z-50 bg-black/80"
                            on:click=move |_| {
                                if let Some(handler) = &on_close_backdrop {
                                    handler.run(());
                                }
                            }
                        ></div>
                        <div
                            class=content_classes.clone()
                            role="dialog"
                            aria-modal="true"
                            on:keydown=move |ev| {
                                if ev.key() == "Escape" {
                                    if let Some(handler) = &on_close_escape {
                                        handler.run(());
                                    }
                                }
                            }
                        ></div>
                    },
                )
            } else {
                None
            }
        }}
        <div style=move || {
            if open.get() { "" } else { "display:none" }
        }>
            {rendered}
        </div>
    }
}

/// Dialog header section.
#[component]
pub fn DialogHeader(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "flex flex-col space-y-1.5 text-center sm:text-left {}",
        class
    );
    view! { <div class=classes>{children()}</div> }
}

/// Dialog title.
#[component]
pub fn DialogTitle(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "text-lg font-semibold leading-none tracking-tight {}",
        class
    );
    view! { <h2 class=classes>{children()}</h2> }
}

/// Dialog description.
#[component]
pub fn DialogDescription(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("text-sm text-muted-foreground {}", class);
    view! { <p class=classes>{children()}</p> }
}

/// Dialog footer (for action buttons).
#[component]
pub fn DialogFooter(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}",
        class
    );
    view! { <div class=classes>{children()}</div> }
}
