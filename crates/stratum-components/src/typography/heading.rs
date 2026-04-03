//! Styled Heading component (h1-h6) with size mapping.

use crate::common::merge_classes;
use stratum_core::aria::AriaAttributes;
use stratum_core::render::{AttrValue, RenderOutput};

/// Heading level (h1 through h6).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub fn tag(&self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
        }
    }

    pub fn aria_level(&self) -> u8 {
        match self {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }
}

/// Properties for the Heading component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HeadingProps {
    pub level: HeadingLevel,
    pub class: Option<String>,
    pub id: Option<String>,
}

pub struct Heading;

impl Heading {
    pub fn classes(props: &HeadingProps) -> String {
        let size_cls = match props.level {
            HeadingLevel::H1 => "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
            HeadingLevel::H2 => "scroll-m-20 text-3xl font-semibold tracking-tight",
            HeadingLevel::H3 => "scroll-m-20 text-2xl font-semibold tracking-tight",
            HeadingLevel::H4 => "scroll-m-20 text-xl font-semibold tracking-tight",
            HeadingLevel::H5 => "scroll-m-20 text-lg font-semibold tracking-tight",
            HeadingLevel::H6 => "scroll-m-20 text-base font-semibold tracking-tight",
        };
        merge_classes(size_cls, &props.class)
    }

    pub fn render(props: &HeadingProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new();
        aria.level = Some(props.level.aria_level());

        let mut output = RenderOutput::new()
            .with_tag(props.level.tag())
            .with_class(classes)
            .with_aria(aria);

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
    fn heading_h1_classes() {
        let props = HeadingProps {
            level: HeadingLevel::H1,
            ..Default::default()
        };
        let classes = Heading::classes(&props);
        assert!(classes.contains("text-4xl"));
        assert!(classes.contains("font-extrabold"));
    }

    #[test]
    fn heading_h3_tag() {
        let props = HeadingProps {
            level: HeadingLevel::H3,
            ..Default::default()
        };
        let output = Heading::render(&props);
        assert_eq!(output.effective_tag(), "h3");
        assert_eq!(output.aria.level, Some(3));
    }

    #[test]
    fn heading_default_is_h2() {
        let props = HeadingProps::default();
        let output = Heading::render(&props);
        assert_eq!(output.effective_tag(), "h2");
        assert_eq!(output.aria.level, Some(2));
    }

    #[test]
    fn heading_all_levels() {
        let levels = [
            HeadingLevel::H1,
            HeadingLevel::H2,
            HeadingLevel::H3,
            HeadingLevel::H4,
            HeadingLevel::H5,
            HeadingLevel::H6,
        ];
        for level in levels {
            let props = HeadingProps {
                level,
                ..Default::default()
            };
            let output = Heading::render(&props);
            assert_eq!(output.aria.level, Some(level.aria_level()));
        }
    }
}
