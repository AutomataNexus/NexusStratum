//! ToggleGroup for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn ToggleGroup(
    value: Signal<Vec<String>>,
    #[props(default = false)] single: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    use_context_provider(|| (value, single));
    let classes = format!("flex items-center justify-center gap-1 {class}");
    rsx! { div { class: "{classes}", role: "group", {children} } }
}

#[component]
pub fn ToggleGroupItem(
    value: String,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let (mut selected, single) = use_context::<(Signal<Vec<String>>, bool)>();
    let is_active = selected().contains(&value);
    let active_cls = if is_active {
        "bg-accent text-accent-foreground"
    } else {
        ""
    };
    let base = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-muted hover:text-muted-foreground h-9 px-3";
    let dis_cls = if disabled {
        "pointer-events-none opacity-50"
    } else {
        ""
    };
    let classes = format!("{base} {active_cls} {dis_cls} {class}");

    let val = value.clone();
    rsx! {
        button {
            class: "{classes}",
            disabled: disabled,
            aria_pressed: if is_active { "true" } else { "false" },
            onclick: move |_| {
                if !disabled {
                    let mut v = selected();
                    if single {
                        if v.contains(&val) { v.clear(); } else { v.clear(); v.push(val.clone()); }
                    } else {
                        if let Some(pos) = v.iter().position(|x| x == &val) { v.remove(pos); } else { v.push(val.clone()); }
                    }
                    selected.set(v);
                }
            },
            {children}
        }
    }
}
