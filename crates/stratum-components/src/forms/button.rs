//! Styled Button component wrapping the button primitive.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::AttrValue;
use stratum_core::render::RenderOutput;

/// Visual variant of the button.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Primary action button.
    #[default]
    Default,
    /// Destructive / danger action.
    Destructive,
    /// Outline / bordered button.
    Outline,
    /// Secondary / muted button.
    Secondary,
    /// Ghost / transparent button.
    Ghost,
    /// Link-styled button.
    Link,
}

/// Properties for the styled Button component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: Size,
    pub disabled: bool,
    pub loading: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

/// Styled Button component.
pub struct Button;

impl Button {
    const BASE: &'static str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

    /// Get the CSS classes for this button configuration.
    pub fn classes(props: &ButtonProps) -> String {
        let variant = match props.variant {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        };

        let size = match props.size {
            Size::Xs => "h-7 px-2 text-xs",
            Size::Sm => "h-8 rounded-md px-3 text-xs",
            Size::Md => "h-9 px-4 py-2",
            Size::Lg => "h-10 rounded-md px-8",
            Size::Xl => "h-12 rounded-md px-10 text-lg",
        };

        let computed = format!("{} {} {}", Self::BASE, variant, size);
        merge_classes(&computed, &props.class)
    }

    /// Get the RenderOutput for this button.
    pub fn render(props: &ButtonProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Button);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled || props.loading {
            aria = aria.with_disabled(true);
        }
        if props.loading {
            aria.busy = Some(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_class(classes)
            .with_aria(aria)
            .with_attr("type", AttrValue::String("button".to_string()));

        if props.disabled || props.loading {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }
        if props.loading {
            output = output.with_data("loading", "true");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_button_classes() {
        let props = ButtonProps::default();
        let classes = Button::classes(&props);
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("bg-primary"));
        assert!(classes.contains("h-9"));
    }

    #[test]
    fn destructive_variant_classes() {
        let props = ButtonProps {
            variant: ButtonVariant::Destructive,
            ..Default::default()
        };
        let classes = Button::classes(&props);
        assert!(classes.contains("bg-destructive"));
    }

    #[test]
    fn small_size_classes() {
        let props = ButtonProps {
            size: Size::Sm,
            ..Default::default()
        };
        let classes = Button::classes(&props);
        assert!(classes.contains("h-8"));
        assert!(classes.contains("px-3"));
    }

    #[test]
    fn user_class_override() {
        let props = ButtonProps {
            class: Some("my-custom-class".to_string()),
            ..Default::default()
        };
        let classes = Button::classes(&props);
        assert!(classes.contains("my-custom-class"));
    }

    #[test]
    fn render_output_tag_is_button() {
        let props = ButtonProps::default();
        let output = Button::render(&props);
        assert_eq!(output.effective_tag(), "button");
    }

    #[test]
    fn render_disabled_button() {
        let props = ButtonProps {
            disabled: true,
            ..Default::default()
        };
        let output = Button::render(&props);
        assert_eq!(output.aria.disabled, Some(true));
    }

    #[test]
    fn render_loading_button() {
        let props = ButtonProps {
            loading: true,
            ..Default::default()
        };
        let output = Button::render(&props);
        assert_eq!(output.aria.busy, Some(true));
        assert_eq!(output.aria.disabled, Some(true));
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "loading" && v == "true")
        );
    }

    #[test]
    fn render_with_aria_label() {
        let props = ButtonProps {
            aria_label: Some("Save document".to_string()),
            ..Default::default()
        };
        let output = Button::render(&props);
        assert_eq!(output.aria.label, Some("Save document".to_string()));
    }

    #[test]
    fn all_variants_produce_classes() {
        let variants = [
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];
        for variant in variants {
            let props = ButtonProps {
                variant,
                ..Default::default()
            };
            let classes = Button::classes(&props);
            assert!(classes.contains("inline-flex"));
        }
    }

    #[test]
    fn all_sizes_produce_classes() {
        let sizes = [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl];
        for size in sizes {
            let props = ButtonProps {
                size,
                ..Default::default()
            };
            let classes = Button::classes(&props);
            assert!(!classes.is_empty());
        }
    }
}
