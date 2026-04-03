//! Form layout components: Form, FormField, FormLabel, FormError, FormHelperText.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, ChildrenSpec, RenderOutput};

// --- Form ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormProps {
    pub class: Option<String>,
    pub id: Option<String>,
}

pub struct Form;

impl Form {
    pub fn classes(props: &FormProps) -> String {
        merge_classes("space-y-6", &props.class)
    }

    pub fn render(props: &FormProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut output = RenderOutput::new()
            .with_tag("form")
            .with_class(classes)
            .with_aria(AriaAttributes::new().with_role(AriaRole::Form));
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }
        output
    }
}

// --- FormField ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormFieldProps {
    pub class: Option<String>,
}

pub struct FormField;

impl FormField {
    pub fn classes(props: &FormFieldProps) -> String {
        merge_classes("space-y-2", &props.class)
    }

    pub fn render(props: &FormFieldProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

// --- FormLabel ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormLabelProps {
    pub html_for: Option<String>,
    pub required: bool,
    pub class: Option<String>,
}

pub struct FormLabel;

impl FormLabel {
    const BASE: &'static str = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

    pub fn classes(props: &FormLabelProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &FormLabelProps) -> RenderOutput {
        let mut output = RenderOutput::new()
            .with_tag("label")
            .with_class(Self::classes(props));
        if let Some(ref f) = props.html_for {
            output = output.with_attr("for", AttrValue::String(f.clone()));
        }
        output
    }
}

// --- FormError ---

#[derive(Debug, Clone, PartialEq)]
pub struct FormErrorProps {
    pub message: String,
    pub class: Option<String>,
    pub id: Option<String>,
}

pub struct FormError;

impl FormError {
    const BASE: &'static str = "text-[0.8rem] font-medium text-destructive";

    pub fn classes(props: &FormErrorProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &FormErrorProps) -> RenderOutput {
        let mut output = RenderOutput::new()
            .with_tag("p")
            .with_class(Self::classes(props))
            .with_aria(AriaAttributes::new().with_role(AriaRole::Alert))
            .with_children(ChildrenSpec::Text(props.message.clone()));
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }
        output
    }
}

// --- FormHelperText ---

#[derive(Debug, Clone, PartialEq)]
pub struct FormHelperTextProps {
    pub text: String,
    pub class: Option<String>,
    pub id: Option<String>,
}

pub struct FormHelperText;

impl FormHelperText {
    const BASE: &'static str = "text-[0.8rem] text-muted-foreground";

    pub fn classes(props: &FormHelperTextProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &FormHelperTextProps) -> RenderOutput {
        let mut output = RenderOutput::new()
            .with_tag("p")
            .with_class(Self::classes(props))
            .with_children(ChildrenSpec::Text(props.text.clone()));
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
    fn form_render_tag() {
        let props = FormProps::default();
        let output = Form::render(&props);
        assert_eq!(output.effective_tag(), "form");
        assert_eq!(output.aria.role, Some(AriaRole::Form));
    }

    #[test]
    fn form_field_classes() {
        let props = FormFieldProps::default();
        let classes = FormField::classes(&props);
        assert!(classes.contains("space-y-2"));
    }

    #[test]
    fn form_label_render() {
        let props = FormLabelProps {
            html_for: Some("email".to_string()),
            ..Default::default()
        };
        let output = FormLabel::render(&props);
        assert_eq!(output.effective_tag(), "label");
        assert!(output.attrs.iter().any(|(k, _)| k == "for"));
    }

    #[test]
    fn form_error_render() {
        let props = FormErrorProps {
            message: "Required field".to_string(),
            class: None,
            id: Some("email-error".to_string()),
        };
        let output = FormError::render(&props);
        assert_eq!(output.effective_tag(), "p");
        assert_eq!(output.aria.role, Some(AriaRole::Alert));
        assert_eq!(
            output.children,
            ChildrenSpec::Text("Required field".to_string())
        );
    }

    #[test]
    fn form_helper_text_render() {
        let props = FormHelperTextProps {
            text: "Enter your email".to_string(),
            class: None,
            id: None,
        };
        let output = FormHelperText::render(&props);
        assert_eq!(output.effective_tag(), "p");
        assert!(output.class_string().contains("text-muted-foreground"));
    }
}
