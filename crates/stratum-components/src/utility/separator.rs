//! Styled Separator utility component.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole, Orientation};
use stratum_core::render::RenderOutput;

/// Properties for the Separator.
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatorProps {
    pub orientation: Orientation,
    pub decorative: bool,
    pub class: Option<String>,
}

impl Default for SeparatorProps {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            decorative: true,
            class: None,
        }
    }
}

pub struct Separator;

impl Separator {
    pub fn classes(props: &SeparatorProps) -> String {
        let base = match props.orientation {
            Orientation::Horizontal => "shrink-0 bg-border h-[1px] w-full",
            Orientation::Vertical => "shrink-0 bg-border h-full w-[1px]",
        };
        merge_classes(base, &props.class)
    }

    pub fn render(props: &SeparatorProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(if props.decorative { AriaRole::None } else { AriaRole::Separator })
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
    fn separator_horizontal() {
        let props = SeparatorProps::default();
        let classes = Separator::classes(&props);
        assert!(classes.contains("w-full"));
    }

    #[test]
    fn separator_decorative_is_hidden() {
        let props = SeparatorProps::default();
        let output = Separator::render(&props);
        assert_eq!(output.aria.hidden, Some(true));
        assert_eq!(output.aria.role, Some(AriaRole::None));
    }

    #[test]
    fn separator_non_decorative() {
        let props = SeparatorProps {
            decorative: false,
            ..Default::default()
        };
        let output = Separator::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::Separator));
        assert!(output.aria.hidden.is_none());
    }
}
