//! Styled Checkbox and CheckboxGroup components.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaRole, TriState};
use stratum_core::render::{AttrValue, RenderOutput};

/// Properties for the styled Checkbox.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckboxProps {
    pub size: Size,
    pub checked: TriState,
    pub disabled: bool,
    pub required: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            size: Size::default(),
            checked: TriState::False,
            disabled: false,
            required: false,
            class: None,
            aria_label: None,
            id: None,
        }
    }
}

pub struct Checkbox;

impl Checkbox {
    const BASE: &'static str = "peer shrink-0 rounded-sm border border-primary shadow focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground";

    pub fn classes(props: &CheckboxProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "h-3 w-3",
            Size::Sm => "h-3.5 w-3.5",
            Size::Md => "h-4 w-4",
            Size::Lg => "h-5 w-5",
            Size::Xl => "h-6 w-6",
        };

        let computed = format!("{} {}", Self::BASE, size_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &CheckboxProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Checkbox)
            .with_checked(props.checked);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }
        if props.required {
            aria.required = Some(true);
        }

        let state_str = match props.checked {
            TriState::True => "checked",
            TriState::False => "unchecked",
            TriState::Mixed => "indeterminate",
        };

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_class(classes)
            .with_aria(aria)
            .with_attr("type", AttrValue::String("button".to_string()))
            .with_data("state", state_str);

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

/// Properties for CheckboxGroup.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CheckboxGroupProps {
    pub class: Option<String>,
    pub aria_label: Option<String>,
}

pub struct CheckboxGroup;

impl CheckboxGroup {
    pub fn classes(props: &CheckboxGroupProps) -> String {
        let computed = "flex flex-col gap-2";
        merge_classes(computed, &props.class)
    }

    pub fn render(props: &CheckboxGroupProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Group);
        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }

        RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_checkbox_classes() {
        let props = CheckboxProps::default();
        let classes = Checkbox::classes(&props);
        assert!(classes.contains("rounded-sm"));
        assert!(classes.contains("h-4 w-4"));
    }

    #[test]
    fn render_checked_state() {
        let props = CheckboxProps {
            checked: TriState::True,
            ..Default::default()
        };
        let output = Checkbox::render(&props);
        assert_eq!(output.aria.checked, Some(TriState::True));
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "checked")
        );
    }

    #[test]
    fn render_indeterminate() {
        let props = CheckboxProps {
            checked: TriState::Mixed,
            ..Default::default()
        };
        let output = Checkbox::render(&props);
        assert_eq!(output.aria.checked, Some(TriState::Mixed));
    }

    #[test]
    fn checkbox_group_render() {
        let props = CheckboxGroupProps::default();
        let output = CheckboxGroup::render(&props);
        assert_eq!(output.effective_tag(), "div");
        assert_eq!(output.aria.role, Some(AriaRole::Group));
    }
}
