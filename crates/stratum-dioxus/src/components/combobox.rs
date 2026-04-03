//! Combobox for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn Combobox(
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = String::from("Select..."))] placeholder: String,
    #[props(default = String::from("Search..."))] search_placeholder: String,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let mut open = use_signal(|| false);
    let mut search = use_signal(String::new);
    let selected = value.unwrap_or_else(|| Signal::new(String::new()));

    use_context_provider(|| (selected, open));

    let display = if selected().is_empty() {
        placeholder.clone()
    } else {
        selected()
    };
    let text_cls = if selected().is_empty() {
        "text-muted-foreground"
    } else {
        ""
    };
    let trigger_cls = "flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

    rsx! {
        div { class: "relative {class}",
            button { class: "{trigger_cls}", disabled: disabled,
                onclick: move |_| open.set(!open()),
                aria_expanded: if open() { "true" } else { "false" },
                aria_haspopup: "listbox",
                span { class: "{text_cls}", "{display}" }
                svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "ml-2 h-4 w-4 shrink-0 opacity-50", path { d: "m6 9 6 6 6-6" } }
            }
            if open() {
                div { class: "absolute left-0 top-full z-50 mt-1 w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md",
                    div { class: "flex items-center border-b px-3",
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "mr-2 h-4 w-4 shrink-0 opacity-50", circle { cx: "11", cy: "11", r: "8" } path { d: "m21 21-4.3-4.3" } }
                        input { class: "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground", placeholder: "{search_placeholder}", value: "{search()}", oninput: move |evt: Event<FormData>| search.set(evt.value()) }
                    }
                    div { class: "max-h-[200px] overflow-y-auto p-1", role: "listbox", {children} }
                }
            }
        }
    }
}

#[component]
pub fn ComboboxItem(
    value: String,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let (mut selected, mut open) = use_context::<(Signal<String>, Signal<bool>)>();
    let is_selected = selected() == value;
    let val = value.clone();
    let classes = format!(
        "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none hover:bg-accent hover:text-accent-foreground {class}"
    );

    rsx! {
        div { class: "{classes}", role: "option", aria_selected: if is_selected { "true" } else { "false" },
            onclick: move |_| { selected.set(val.clone()); open.set(false); },
            {children}
            if is_selected {
                svg { xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: "absolute right-2 h-4 w-4", path { d: "M20 6 9 17l-5-5" } }
            }
        }
    }
}
