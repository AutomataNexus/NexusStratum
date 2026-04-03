//! ToggleGroup for Leptos.

use leptos::prelude::*;

/// A group of toggle buttons where one or more can be active.
#[component]
pub fn ToggleGroup(
    /// Selected values.
    value: RwSignal<Vec<String>>,
    /// Whether only one can be active at a time.
    #[prop(optional, default = false)]
    single: bool,
    #[prop(optional, default = String::new())]
    class: String,
    children: Children,
) -> impl IntoView {
    provide_context((value, single));
    let classes = format!("flex items-center justify-center gap-1 {}", class);
    view! { <div class=classes role="group">{children()}</div> }
}

/// A single toggle within the group.
#[component]
pub fn ToggleGroupItem(
    value: String,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let (selected, single) = use_context::<(RwSignal<Vec<String>>, bool)>()
        .expect("ToggleGroupItem must be inside ToggleGroup");
    let val = value.clone();
    let val2 = value.clone();

    let base = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground h-9 px-3";

    view! {
        <button
            class=format!("{} {}", base, class)
            disabled=disabled
            aria-pressed=move || if selected.get().contains(&val2) { "true" } else { "false" }
            data-state=move || if selected.get().contains(&val) { "on" } else { "off" }
            on:click=move |_| {
                if !disabled {
                    selected.update(|v| {
                        if single {
                            if v.contains(&value) { v.clear(); } else { v.clear(); v.push(value.clone()); }
                        } else {
                            if let Some(pos) = v.iter().position(|x| x == &value) { v.remove(pos); } else { v.push(value.clone()); }
                        }
                    });
                }
            }
        >
            {children()}
        </button>
    }
}
