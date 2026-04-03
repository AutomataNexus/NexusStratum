//! Styled Radio and RadioGroup components.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Properties for the styled Radio button.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RadioProps {
    pub size: Size,
    pub checked: bool,
    pub disabled: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
    pub value: Option<String>,
}

pub struct Radio;

impl Radio {
    const BASE: &'static str = "aspect-square rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

    pub fn classes(props: &RadioProps) -> String {
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

    pub fn render(props: &RadioProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Radio);
        aria.checked = Some(if props.checked {
            stratum_core::aria::TriState::True
        } else {
            stratum_core::aria::TriState::False
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
        if let Some(ref value) = props.value {
            output = output.with_attr("value", AttrValue::String(value.clone()));
        }

        output
    }
}

/// Properties for RadioGroup.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RadioGroupProps {
    pub orientation: Option<stratum_core::aria::Orientation>,
    pub class: Option<String>,
    pub aria_label: Option<String>,
}

pub struct RadioGroup;

impl RadioGroup {
    pub fn classes(props: &RadioGroupProps) -> String {
        let base = match props.orientation {
            Some(stratum_core::aria::Orientation::Horizontal) => "flex flex-row gap-2",
            _ => "flex flex-col gap-2",
        };
        merge_classes(base, &props.class)
    }

    pub fn render(props: &RadioGroupProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::RadioGroup);
        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if let Some(orientation) = props.orientation {
            aria = aria.with_orientation(orientation);
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
    fn default_radio_classes() {
        let props = RadioProps::default();
        let classes = Radio::classes(&props);
        assert!(classes.contains("rounded-full"));
        assert!(classes.contains("h-4 w-4"));
    }

    #[test]
    fn render_checked_radio() {
        let props = RadioProps {
            checked: true,
            ..Default::default()
        };
        let output = Radio::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "checked")
        );
    }

    #[test]
    fn radio_group_vertical() {
        let props = RadioGroupProps::default();
        let classes = RadioGroup::classes(&props);
        assert!(classes.contains("flex-col"));
    }

    #[test]
    fn radio_group_horizontal() {
        let props = RadioGroupProps {
            orientation: Some(stratum_core::aria::Orientation::Horizontal),
            ..Default::default()
        };
        let classes = RadioGroup::classes(&props);
        assert!(classes.contains("flex-row"));
    }
}
