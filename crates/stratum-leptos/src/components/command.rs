//! Command (searchable command palette) for Leptos.

use leptos::prelude::*;

/// A command palette / searchable list.
#[component]
pub fn Command(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground {}", class);
    view! { <div class=classes>{children()}</div> }
}

/// Search input for the command palette.
#[component]
pub fn CommandInput(
    #[prop(optional, default = String::from("Search..."))] placeholder: String,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let val = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let classes = format!("flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 {}", class);
    view! {
        <div class="flex items-center border-b px-3">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4 shrink-0 opacity-50"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></svg>
            <input class=classes placeholder=placeholder prop:value=move || val.get() on:input=move |ev| val.set(event_target_value(&ev)) />
        </div>
    }
}

/// Container for command items.
#[component]
pub fn CommandList(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("max-h-[300px] overflow-y-auto overflow-x-hidden {}", class);
    view! { <div class=classes role="listbox">{children()}</div> }
}

/// A group of command items with a heading.
#[component]
pub fn CommandGroup(
    #[prop(optional)] heading: Option<String>,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let classes = format!("overflow-hidden p-1 text-foreground {}", class);
    view! {
        <div class=classes role="group">
            {heading.map(|h| view! { <div class="px-2 py-1.5 text-xs font-medium text-muted-foreground">{h}</div> })}
            {children()}
        </div>
    }
}

/// A single selectable command item.
#[component]
pub fn CommandItem(
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = String::new())] class: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground {} {}",
        if disabled { "pointer-events-none opacity-50" } else { "cursor-pointer" }, class
    );
    view! {
        <div class=classes role="option" aria-disabled=disabled.then_some("true") on:click=move |_| {
            if !disabled { if let Some(handler) = &on_select { handler.run(()); } }
        }>{children()}</div>
    }
}

/// Visual separator in command list.
#[component]
pub fn CommandSeparator(#[prop(optional, default = String::new())] class: String) -> impl IntoView {
    let classes = format!("-mx-1 h-px bg-border {}", class);
    view! { <div class=classes></div> }
}

/// Empty state when no results found.
#[component]
pub fn CommandEmpty(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("py-6 text-center text-sm {}", class);
    view! { <div class=classes>{children()}</div> }
}
