//! Tabs component for Leptos.

use leptos::prelude::*;

/// A tabbed interface container.
#[component]
pub fn Tabs(
    /// Currently active tab value.
    value: RwSignal<String>,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    provide_context(value);
    view! { <div class=class>{children()}</div> }
}

/// The tab button list.
#[component]
pub fn TabsList(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground {}",
        class
    );
    view! { <div class=classes role="tablist">{children()}</div> }
}

/// A single tab trigger button.
#[component]
pub fn TabsTrigger(
    /// Value that identifies this tab.
    value: String,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let active_tab = use_context::<RwSignal<String>>().expect("TabsTrigger must be inside Tabs");
    let val_class = value.clone();
    let val_aria = value.clone();
    let val_click = value.clone();

    let base = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

    view! {
        <button
            class=move || {
                let active = if active_tab.get() == val_class {
                    "bg-background text-foreground shadow"
                } else {
                    ""
                };
                format!("{} {} {}", base, active, class)
            }
            role="tab"
            aria-selected=move || if active_tab.get() == val_aria { "true" } else { "false" }
            on:click=move |_| active_tab.set(val_click.clone())
        >
            {children()}
        </button>
    }
}

/// Tab content panel.
#[component]
pub fn TabsContent(
    /// Value that matches the corresponding TabsTrigger.
    value: String,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let active_tab = use_context::<RwSignal<String>>().expect("TabsContent must be inside Tabs");
    let val = value.clone();

    let classes = format!(
        "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {}",
        class
    );

    let rendered = children();

    view! {
        <div
            class=classes
            role="tabpanel"
            style=move || if active_tab.get() == val { "" } else { "display:none" }
        >
            {rendered}
        </div>
    }
}
