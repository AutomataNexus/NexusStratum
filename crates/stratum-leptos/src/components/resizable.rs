//! Resizable panels for Leptos.

use leptos::prelude::*;

/// A container for resizable panels.
#[component]
pub fn ResizablePanelGroup(
    /// Direction: "horizontal" or "vertical".
    #[prop(optional, default = String::from("horizontal"))]
    direction: String,
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let dir_cls = if direction == "vertical" { "flex-col" } else { "flex-row" };
    let classes = format!("flex h-full w-full data-[panel-group-direction=vertical]:flex-col {} {}", dir_cls, class);
    view! { <div class=classes data-panel-group-direction=direction>{children()}</div> }
}

/// A single resizable panel.
#[component]
pub fn ResizablePanel(
    /// Default size as percentage.
    #[prop(optional, default = 50)]
    default_size: u32,
    /// Minimum size as percentage.
    #[prop(optional, default = 10)]
    min_size: u32,
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex-1 overflow-auto {}", class);
    view! {
        <div class=classes style=format!("flex-basis:{}%;min-width:{}%", default_size, min_size)>
            {children()}
        </div>
    }
}

/// A drag handle between resizable panels.
#[component]
pub fn ResizableHandle(#[prop(optional, default = String::new())] class: String) -> impl IntoView {
    let classes = format!(
        "relative flex w-px items-center justify-center bg-border after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1 data-[panel-group-direction=vertical]:h-px data-[panel-group-direction=vertical]:w-full cursor-col-resize {}",
        class
    );
    view! {
        <div class=classes role="separator" tabindex="0" aria-orientation="vertical">
            <div class="z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-border">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-2.5 w-2.5"><circle cx="12" cy="9" r="1"></circle><circle cx="12" cy="15" r="1"></circle></svg>
            </div>
        </div>
    }
}
