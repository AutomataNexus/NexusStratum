//! Combobox (searchable select) for Leptos.

use leptos::prelude::*;

/// A searchable dropdown select.
#[component]
pub fn Combobox(
    /// Selected value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Placeholder when nothing selected.
    #[prop(optional, default = String::from("Select..."))]
    placeholder: String,
    /// Search placeholder.
    #[prop(optional, default = String::from("Search..."))]
    search_placeholder: String,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Change handler.
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Options.
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let search = RwSignal::new(String::new());
    let selected = value.unwrap_or_else(|| RwSignal::new(String::new()));

    provide_context((selected, open, search, on_change));

    let trigger_cls = "flex h-9 w-full items-center justify-between whitespace-nowrap rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let display_text = move || {
        let v = selected.get();
        if v.is_empty() { placeholder.clone() } else { v }
    };

    view! {
        <div class=format!("relative {}", class)>
            <button
                class=trigger_cls
                disabled=disabled
                on:click=move |_| open.update(|v| *v = !*v)
                aria-expanded=move || if open.get() { "true" } else { "false" }
                aria-haspopup="listbox"
            >
                <span class=move || if selected.get().is_empty() { "text-muted-foreground" } else { "" }>{display_text}</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-2 h-4 w-4 shrink-0 opacity-50"><path d="m6 9 6 6 6-6"></path></svg>
            </button>
            <div
                class="absolute left-0 top-full z-50 mt-1 w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md"
                style=move || if open.get() { "" } else { "display:none" }
            >
                <div class="flex items-center border-b px-3">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4 shrink-0 opacity-50"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></svg>
                    <input
                        class="flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground"
                        placeholder=search_placeholder
                        prop:value=move || search.get()
                        on:input=move |ev| search.set(event_target_value(&ev))
                    />
                </div>
                <div class="max-h-[200px] overflow-y-auto p-1" role="listbox">
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// A combobox option.
#[component]
pub fn ComboboxItem(
    value: String,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (selected, open, _search, on_change) = use_context::<(
        RwSignal<String>,
        RwSignal<bool>,
        RwSignal<String>,
        Option<Callback<String>>,
    )>()
    .expect("ComboboxItem must be inside Combobox");

    let val = value.clone();
    let val_aria = value.clone();
    let val_check = value.clone();

    let classes = format!(
        "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none hover:bg-accent hover:text-accent-foreground {}",
        class
    );

    view! {
        <div class=classes role="option" aria-selected=move || if selected.get() == val_aria { "true" } else { "false" } on:click=move |_| {
            selected.set(val.clone());
            open.set(false);
            if let Some(handler) = &on_change { handler.run(val.clone()); }
        }>
            {children()}
            {move || if selected.get() == val_check {
                view! { <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="absolute right-2 h-4 w-4"><path d="M20 6 9 17l-5-5"></path></svg> }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
