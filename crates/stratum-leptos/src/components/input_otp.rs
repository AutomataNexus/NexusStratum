//! InputOTP (one-time password input) for Leptos.

use leptos::prelude::*;

/// A one-time password / verification code input.
#[component]
pub fn InputOTP(
    /// Number of digits.
    #[prop(optional, default = 6)]
    length: usize,
    /// Current value (controlled).
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Whether disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Complete handler (fires when all digits filled).
    #[prop(optional)]
    on_complete: Option<Callback<String>>,
) -> impl IntoView {
    let val = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let classes = format!("flex items-center gap-2 {}", class);

    view! {
        <div class=classes role="group">
            <input
                type="text"
                class="sr-only"
                maxlength=length
                disabled=disabled
                prop:value=move || val.get()
                on:input=move |ev| {
                    let v: String = event_target_value(&ev).chars().filter(|c| c.is_ascii_digit()).take(length).collect();
                    val.set(v.clone());
                    if v.len() == length {
                        if let Some(handler) = &on_complete { handler.run(v); }
                    }
                }
                autocomplete="one-time-code"
                inputmode="numeric"
            />
            {(0..length).map(|i| {
                view! {
                    <div class="relative flex h-10 w-10 items-center justify-center border-y border-r border-input text-sm transition-all first:rounded-l-md first:border-l last:rounded-r-md">
                        {move || val.get().chars().nth(i).map(|c| c.to_string()).unwrap_or_default()}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
