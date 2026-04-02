//! Styled Text component with size, weight, and color control.

use crate::common::{merge_classes, Size};
use stratum_core::render::RenderOutput;

/// Font weight options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FontWeight {
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
}

impl FontWeight {
    pub fn class(&self) -> &'static str {
        match self {
            FontWeight::Light => "font-light",
            FontWeight::Normal => "font-normal",
            FontWeight::Medium => "font-medium",
            FontWeight::Semibold => "font-semibold",
            FontWeight::Bold => "font-bold",
        }
    }
}

/// Text color semantic options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextColor {
    #[default]
    Default,
    Muted,
    Primary,
    Destructive,
}

impl TextColor {
    pub fn class(&self) -> &'static str {
        match self {
            TextColor::Default => "text-foreground",
            TextColor::Muted => "text-muted-foreground",
            TextColor::Primary => "text-primary",
            TextColor::Destructive => "text-destructive",
        }
    }
}

/// Properties for the styled Text component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextProps {
    pub size: Size,
    pub weight: FontWeight,
    pub color: TextColor,
    pub as_element: Option<String>,
    pub class: Option<String>,
}

pub struct Text;

impl Text {
    pub fn classes(props: &TextProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "text-xs",
            Size::Sm => "text-sm",
            Size::Md => "text-base",
            Size::Lg => "text-lg",
            Size::Xl => "text-xl",
        };

        let computed = format!(
            "{} {} {}",
            size_cls,
            props.weight.class(),
            props.color.class(),
        );
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &TextProps) -> RenderOutput {
        let tag = props.as_element.as_deref().unwrap_or("p");
        RenderOutput::new()
            .with_tag(tag)
            .with_class(Self::classes(props))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_default_classes() {
        let props = TextProps::default();
        let classes = Text::classes(&props);
        assert!(classes.contains("text-base"));
        assert!(classes.contains("font-normal"));
        assert!(classes.contains("text-foreground"));
    }

    #[test]
    fn text_custom_size_weight() {
        let props = TextProps {
            size: Size::Lg,
            weight: FontWeight::Bold,
            ..Default::default()
        };
        let classes = Text::classes(&props);
        assert!(classes.contains("text-lg"));
        assert!(classes.contains("font-bold"));
    }

    #[test]
    fn text_muted_color() {
        let props = TextProps {
            color: TextColor::Muted,
            ..Default::default()
        };
        let classes = Text::classes(&props);
        assert!(classes.contains("text-muted-foreground"));
    }

    #[test]
    fn text_default_tag_is_p() {
        let props = TextProps::default();
        let output = Text::render(&props);
        assert_eq!(output.effective_tag(), "p");
    }

    #[test]
    fn text_custom_tag() {
        let props = TextProps {
            as_element: Some("span".to_string()),
            ..Default::default()
        };
        let output = Text::render(&props);
        assert_eq!(output.effective_tag(), "span");
    }
}
