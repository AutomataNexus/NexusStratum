//! Styled Input component.

use crate::common::{merge_classes, Size};
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Visual variant of the input.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputVariant {
    #[default]
    Default,
    /// Shows a destructive/error border.
    Error,
    /// Shows a success border.
    Success,
}

/// Properties for the styled Input component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InputProps {
    pub variant: InputVariant,
    pub size: Size,
    pub disabled: bool,
    pub readonly: bool,
    pub required: bool,
    pub placeholder: Option<String>,
    pub input_type: Option<String>,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

/// Styled Input component.
pub struct Input;

impl Input {
    const BASE: &'static str = "flex w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

    pub fn classes(props: &InputProps) -> String {
        let variant = match props.variant {
            InputVariant::Default => "",
            InputVariant::Error => "border-destructive focus-visible:ring-destructive",
            InputVariant::Success => "border-green-500 focus-visible:ring-green-500",
        };

        let size = match props.size {
            Size::Xs => "h-7 text-xs",
            Size::Sm => "h-8 text-xs",
            Size::Md => "h-9 text-sm",
            Size::Lg => "h-10 text-base",
            Size::Xl => "h-12 text-lg",
        };

        let computed = format!("{} {} {}", Self::BASE, variant, size);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &InputProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::TextBox);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }
        if props.required {
            aria.required = Some(true);
        }
        if props.readonly {
            aria.readonly = Some(true);
        }
        if props.variant == InputVariant::Error {
            aria.invalid = Some(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("input")
            .with_class(classes)
            .with_aria(aria);

        if let Some(ref t) = props.input_type {
            output = output.with_attr("type", AttrValue::String(t.clone()));
        }
        if let Some(ref p) = props.placeholder {
            output = output.with_attr("placeholder", AttrValue::String(p.clone()));
        }
        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
        if props.readonly {
            output = output.with_attr("readonly", AttrValue::Bool(true));
        }
        if props.required {
            output = output.with_attr("required", AttrValue::Bool(true));
        }
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

/// InputGroup wraps an input with optional leading/trailing addons.
pub struct InputGroup;

impl InputGroup {
    pub fn classes() -> String {
        "flex items-center".to_string()
    }

    pub fn render() -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes())
            .with_aria(AriaAttributes::new().with_role(AriaRole::Group))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_input_classes() {
        let props = InputProps::default();
        let classes = Input::classes(&props);
        assert!(classes.contains("flex"));
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("h-9"));
    }

    #[test]
    fn error_variant() {
        let props = InputProps {
            variant: InputVariant::Error,
            ..Default::default()
        };
        let classes = Input::classes(&props);
        assert!(classes.contains("border-destructive"));
    }

    #[test]
    fn render_tag_is_input() {
        let props = InputProps::default();
        let output = Input::render(&props);
        assert_eq!(output.effective_tag(), "input");
    }

    #[test]
    fn render_required_input() {
        let props = InputProps {
            required: true,
            ..Default::default()
        };
        let output = Input::render(&props);
        assert_eq!(output.aria.required, Some(true));
    }

    #[test]
    fn render_error_sets_invalid() {
        let props = InputProps {
            variant: InputVariant::Error,
            ..Default::default()
        };
        let output = Input::render(&props);
        assert_eq!(output.aria.invalid, Some(true));
    }

    #[test]
    fn input_group_render() {
        let output = InputGroup::render();
        assert_eq!(output.effective_tag(), "div");
    }
}
