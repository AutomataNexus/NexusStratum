//! Styled Textarea component.

use crate::common::{merge_classes, Size};
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Resize behavior for the textarea.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResizeMode {
    /// No resizing allowed.
    None,
    /// Resize vertically only (default).
    #[default]
    Vertical,
    /// Resize horizontally only.
    Horizontal,
    /// Resize in both directions.
    Both,
}

/// Properties for the styled Textarea component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextareaProps {
    pub size: Size,
    pub resize: ResizeMode,
    pub disabled: bool,
    pub readonly: bool,
    pub required: bool,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

pub struct Textarea;

impl Textarea {
    const BASE: &'static str = "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

    pub fn classes(props: &TextareaProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "text-xs",
            Size::Sm => "text-xs",
            Size::Md => "text-sm",
            Size::Lg => "text-base",
            Size::Xl => "text-lg",
        };

        let resize_cls = match props.resize {
            ResizeMode::None => "resize-none",
            ResizeMode::Vertical => "resize-y",
            ResizeMode::Horizontal => "resize-x",
            ResizeMode::Both => "resize",
        };

        let computed = format!("{} {} {}", Self::BASE, size_cls, resize_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &TextareaProps) -> RenderOutput {
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

        let mut output = RenderOutput::new()
            .with_tag("textarea")
            .with_class(classes)
            .with_aria(aria);

        if let Some(ref p) = props.placeholder {
            output = output.with_attr("placeholder", AttrValue::String(p.clone()));
        }
        if let Some(rows) = props.rows {
            output = output.with_attr("rows", AttrValue::Number(rows as f64));
        }
        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
        if props.readonly {
            output = output.with_attr("readonly", AttrValue::Bool(true));
        }
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_textarea_classes() {
        let props = TextareaProps::default();
        let classes = Textarea::classes(&props);
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("resize-y"));
    }

    #[test]
    fn resize_none() {
        let props = TextareaProps {
            resize: ResizeMode::None,
            ..Default::default()
        };
        let classes = Textarea::classes(&props);
        assert!(classes.contains("resize-none"));
    }

    #[test]
    fn render_tag_is_textarea() {
        let props = TextareaProps::default();
        let output = Textarea::render(&props);
        assert_eq!(output.effective_tag(), "textarea");
    }

    #[test]
    fn render_with_rows() {
        let props = TextareaProps {
            rows: Some(5),
            ..Default::default()
        };
        let output = Textarea::render(&props);
        assert!(output.attrs.iter().any(|(k, _)| k == "rows"));
    }
}
