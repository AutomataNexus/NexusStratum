//! Sidebar layout component for Leptos.

use leptos::prelude::*;

/// A collapsible sidebar layout.
#[component]
pub fn Sidebar(
    /// Whether the sidebar is open.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,
    /// Default open state.
    #[prop(optional, default = true)]
    default_open: bool,
    /// Side of the screen.
    #[prop(optional, default = String::from("left"))]
    side: String,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    let is_open = open.unwrap_or_else(|| RwSignal::new(default_open));
    provide_context(is_open);

    let side_cls = if side == "right" { "order-last" } else { "" };
    let width = move || {
        if is_open.get() {
            "width: 16rem"
        } else {
            "width: 3rem"
        }
    };

    let classes = format!(
        "flex h-full flex-col border-r bg-sidebar text-sidebar-foreground transition-all duration-300 {} {}",
        side_cls, class
    );

    view! {
        <aside class=classes style=width aria-label="Sidebar">
            {children()}
        </aside>
    }
}

/// Sidebar header section.
#[component]
pub fn SidebarHeader(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex flex-col gap-2 p-2 {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// Sidebar content (scrollable).
#[component]
pub fn SidebarContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "flex min-h-0 flex-1 flex-col gap-2 overflow-auto p-2 {}",
        class
    );
    view! { <div class=classes>{children()}</div> }
}

/// Sidebar footer section.
#[component]
pub fn SidebarFooter(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex flex-col gap-2 p-2 {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// A group of sidebar items with optional heading.
#[component]
pub fn SidebarGroup(
    #[prop(optional)] label: Option<String>,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("relative flex w-full min-w-0 flex-col p-2 {}", class);
    view! {
        <div class=classes>
            {label.map(|l| view! { <div class="flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium text-sidebar-foreground/70">{l}</div> })}
            {children()}
        </div>
    }
}

/// A sidebar menu item button.
#[component]
pub fn SidebarMenuItem(
    #[prop(optional, default = false)] active: bool,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let active_cls = if active {
        "bg-sidebar-accent text-sidebar-accent-foreground"
    } else {
        ""
    };
    let classes = format!(
        "flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-none ring-sidebar-ring transition-all hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 {} {}",
        active_cls, class
    );
    view! {
        <button class=classes on:click=move |_| { if let Some(h) = &on_click { h.run(()); } }>
            {children()}
        </button>
    }
}

/// Toggle button to collapse/expand the sidebar.
#[component]
pub fn SidebarTrigger(#[prop(optional, default = String::new())] class: String) -> impl IntoView {
    let is_open = use_context::<RwSignal<bool>>().expect("SidebarTrigger must be inside Sidebar");
    let classes = format!(
        "inline-flex items-center justify-center rounded-md h-7 w-7 text-sm hover:bg-sidebar-accent hover:text-sidebar-accent-foreground {}",
        class
    );
    view! {
        <button class=classes aria-label="Toggle sidebar" on:click=move |_| is_open.update(|v| *v = !*v)>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4"><rect width="18" height="18" x="3" y="3" rx="2"></rect><path d="M9 3v18"></path></svg>
        </button>
    }
}
