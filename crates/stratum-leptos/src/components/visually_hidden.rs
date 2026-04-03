//! VisuallyHidden component for Leptos.

use leptos::prelude::*;

/// Content hidden visually but accessible to screen readers.
#[component]
pub fn VisuallyHidden(children: Children) -> impl IntoView {
    view! {
        <span style="position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0">
            {children()}
        </span>
    }
}
