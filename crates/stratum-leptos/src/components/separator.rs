//! Separator component for Leptos.

use leptos::prelude::*;

/// Orientation of the separator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A visual divider between content sections.
#[component]
pub fn Separator(
    /// Direction.
    #[prop(optional, default = Orientation::Horizontal)]
    orientation: Orientation,
    /// Whether purely decorative (hidden from screen readers).
    #[prop(optional, default = true)]
    decorative: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
) -> impl IntoView {
    let base = "shrink-0 bg-border";
    let dir_cls = match orientation {
        Orientation::Horizontal => "h-[1px] w-full",
        Orientation::Vertical => "h-full w-[1px]",
    };
    let classes = format!("{} {} {}", base, dir_cls, class);

    view! {
        <div
            class=classes
            role=if decorative { "none" } else { "separator" }
            aria-orientation=if !decorative {
                Some(match orientation {
                    Orientation::Horizontal => "horizontal",
                    Orientation::Vertical => "vertical",
                })
            } else {
                None
            }
        ></div>
    }
}
