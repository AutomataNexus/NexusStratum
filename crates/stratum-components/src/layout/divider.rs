//! Styled Divider/Separator component.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole, Orientation};
use stratum_core::render::RenderOutput;

/// Properties for the Divider component.
#[derive(Debug, Clone, PartialEq)]
pub struct DividerProps {
    pub orientation: Orientation,
    pub decorative: bool,
    pub class: Option<String>,
}

impl Default for DividerProps {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            decorative: true,
            class: None,
        }
    }
}

pub struct Divider;

impl Divider {
    pub fn classes(props: &DividerProps) -> String {
        let base = match props.orientation {
            Orientation::Horizontal => "shrink-0 bg-border h-[1px] w-full",
            Orientation::Vertical => "shrink-0 bg-border h-full w-[1px]",
        };
        merge_classes(base, &props.class)
    }

    pub fn render(props: &DividerProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Separator)
            .with_orientation(props.orientation);

        if props.decorative {
            aria.hidden = Some(true);
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
    fn divider_horizontal() {
        let props = DividerProps::default();
        let classes = Divider::classes(&props);
        assert!(classes.contains("w-full"));
        assert!(classes.contains("h-[1px]"));
    }

    #[test]
    fn divider_vertical() {
        let props = DividerProps {
            orientation: Orientation::Vertical,
            ..Default::default()
        };
        let classes = Divider::classes(&props);
        assert!(classes.contains("w-[1px]"));
        assert!(classes.contains("h-full"));
    }

    #[test]
    fn divider_decorative_hidden() {
        let props = DividerProps::default();
        let output = Divider::render(&props);
        assert_eq!(output.aria.hidden, Some(true));
    }

    #[test]
    fn divider_non_decorative_visible() {
        let props = DividerProps {
            decorative: false,
            ..Default::default()
        };
        let output = Divider::render(&props);
        assert!(output.aria.hidden.is_none());
    }
}
