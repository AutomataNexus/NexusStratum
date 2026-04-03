//! InputOTP for Dioxus.

use dioxus::prelude::*;

#[component]
pub fn InputOTP(
    #[props(default = 6)] length: usize,
    #[props(optional)] value: Option<Signal<String>>,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
    #[props(optional)] on_complete: Option<EventHandler<String>>,
) -> Element {
    let mut val = value.unwrap_or_else(|| Signal::new(String::new()));
    let classes = format!("flex items-center gap-2 {class}");
    let current = val();
    let digits: Vec<String> = (0..length)
        .map(|i| {
            current
                .chars()
                .nth(i)
                .map(|c| c.to_string())
                .unwrap_or_default()
        })
        .collect();

    rsx! {
        div { class: "{classes}", role: "group",
            input {
                r#type: "text",
                class: "sr-only",
                maxlength: "{length}",
                disabled: disabled,
                value: "{val()}",
                autocomplete: "one-time-code",
                inputmode: "numeric",
                oninput: move |evt: Event<FormData>| {
                    let v: String = evt.value().chars().filter(|c| c.is_ascii_digit()).take(length).collect();
                    val.set(v.clone());
                    if v.len() == length {
                        if let Some(handler) = &on_complete { handler.call(v); }
                    }
                },
            }
            for (i, d) in digits.into_iter().enumerate() {
                div {
                    key: "{i}",
                    class: "relative flex h-10 w-10 items-center justify-center border-y border-r border-input text-sm transition-all first:rounded-l-md first:border-l last:rounded-r-md",
                    "{d}"
                }
            }
        }
    }
}
