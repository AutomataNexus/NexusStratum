//! AspectRatio component for Leptos.

use leptos::prelude::*;

/// Maintains a fixed aspect ratio for its children.
#[component]
pub fn AspectRatio(
    /// Aspect ratio (e.g., 16.0/9.0 = 1.777).
    #[prop(optional, default = 1.0)]
    ratio: f64,
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let padding = format!("{}%", (1.0 / ratio) * 100.0);
    view! {
        <div class=class style=format!("position:relative;width:100%;padding-bottom:{}", padding)>
            <div style="position:absolute;inset:0">
                {children()}
            </div>
        </div>
    }
}
