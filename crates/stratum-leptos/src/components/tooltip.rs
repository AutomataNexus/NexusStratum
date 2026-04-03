//! Tooltip component for Leptos.

use leptos::prelude::*;

/// A tooltip that appears on hover.
#[component]
pub fn Tooltip(
    /// Tooltip text content.
    content: String,
    /// Side to show on.
    #[prop(optional, default = String::from("top"))]
    side: String,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Trigger element.
    children: Children,
) -> impl IntoView {
    let (visible, set_visible) = signal(false);

    let position_cls = match side.as_str() {
        "bottom" => "top-full left-1/2 -translate-x-1/2 mt-2",
        "left" => "right-full top-1/2 -translate-y-1/2 mr-2",
        "right" => "left-full top-1/2 -translate-y-1/2 ml-2",
        _ => "bottom-full left-1/2 -translate-x-1/2 mb-2", // top
    };

    view! {
        <div
            class="relative inline-flex"
            on:mouseenter=move |_| set_visible.set(true)
            on:mouseleave=move |_| set_visible.set(false)
            on:focusin=move |_| set_visible.set(true)
            on:focusout=move |_| set_visible.set(false)
        >
            {children()}
            <Show when=move || visible.get()>
                <div
                    role="tooltip"
                    class=format!(
                        "absolute z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground animate-in fade-in-0 zoom-in-95 {} {}",
                        position_cls,
                        class,
                    )
                >
                    {content.clone()}
                </div>
            </Show>
        </div>
    }
}
