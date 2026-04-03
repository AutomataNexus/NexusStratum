//! Styled Switch toggle component.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaRole, TriState};
use stratum_core::render::{AttrValue, RenderOutput};

/// Properties for the styled Switch.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SwitchProps {
    pub size: Size,
    pub checked: bool,
    pub disabled: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

pub struct Switch;

impl Switch {
    const BASE: &'static str = "peer inline-flex shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input";

    pub fn classes(props: &SwitchProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "h-4 w-7",
            Size::Sm => "h-4 w-8",
            Size::Md => "h-5 w-9",
            Size::Lg => "h-6 w-11",
            Size::Xl => "h-7 w-14",
        };

        let computed = format!("{} {}", Self::BASE, size_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &SwitchProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Switch)
            .with_checked(if props.checked {
                TriState::True
            } else {
                TriState::False
            });

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_class(classes)
            .with_aria(aria)
            .with_attr("type", AttrValue::String("button".to_string()))
            .with_data(
                "state",
                if props.checked {
                    "checked"
                } else {
                    "unchecked"
                },
            );

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
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
    fn default_switch_classes() {
        let props = SwitchProps::default();
        let classes = Switch::classes(&props);
        assert!(classes.contains("rounded-full"));
        assert!(classes.contains("h-5 w-9"));
    }

    #[test]
    fn render_checked_switch() {
        let props = SwitchProps {
            checked: true,
            ..Default::default()
        };
        let output = Switch::render(&props);
        assert_eq!(output.aria.checked, Some(TriState::True));
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "checked")
        );
    }

    #[test]
    fn render_unchecked_switch() {
        let props = SwitchProps::default();
        let output = Switch::render(&props);
        assert_eq!(output.aria.checked, Some(TriState::False));
    }

    #[test]
    fn render_tag_is_button() {
        let props = SwitchProps::default();
        let output = Switch::render(&props);
        assert_eq!(output.effective_tag(), "button");
    }
}
