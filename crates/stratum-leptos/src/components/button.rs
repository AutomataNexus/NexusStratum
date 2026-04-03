//! Button component for Leptos.

use leptos::prelude::*;

/// Button visual variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Button size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// A button component with multiple variants and sizes.
///
/// # Example
/// ```ignore
/// use stratum_leptos::components::button::*;
///
/// view! {
///     <Button>"Click me"</Button>
///     <Button variant=ButtonVariant::Destructive>"Delete"</Button>
///     <Button size=ButtonSize::Lg disabled=true>"Disabled"</Button>
/// }
/// ```
#[component]
pub fn Button(
    /// Visual style variant.
    #[prop(optional, default = ButtonVariant::Default)]
    variant: ButtonVariant,
    /// Size of the button.
    #[prop(optional, default = ButtonSize::Md)]
    size: ButtonSize,
    /// Whether the button is disabled.
    #[prop(optional, default = false)]
    disabled: bool,
    /// Whether the button is in a loading state.
    #[prop(optional, default = false)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, default = String::new())]
    class: String,
    /// Accessible label override.
    #[prop(optional)]
    aria_label: Option<String>,
    /// Click handler.
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Button content.
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center whitespace-nowrap rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

    let variant_cls = match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        }
        ButtonVariant::Outline => {
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        }
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };

    let size_cls = match size {
        ButtonSize::Xs => "h-7 px-2 text-xs",
        ButtonSize::Sm => "h-8 rounded-md px-3 text-xs",
        ButtonSize::Md => "h-9 px-4 py-2 text-sm",
        ButtonSize::Lg => "h-10 rounded-md px-8",
        ButtonSize::Xl => "h-12 rounded-md px-10 text-lg",
    };

    let classes = format!("{} {} {} {}", base, variant_cls, size_cls, class);
    let is_disabled = disabled || loading;

    view! {
        <button
            class=classes
            disabled=is_disabled
            aria-label=aria_label
            aria-busy=loading.then_some("true")
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler.run(());
                }
            }
        >
            {move || {
                if loading {
                    view! {
                        <svg
                            class="animate-spin -ml-1 mr-2 h-4 w-4"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <circle
                                class="opacity-25"
                                cx="12"
                                cy="12"
                                r="10"
                                stroke="currentColor"
                                stroke-width="4"
                            ></circle>
                            <path
                                class="opacity-75"
                                fill="currentColor"
                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                            ></path>
                        </svg>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
            {children()}
        </button>
    }
}
