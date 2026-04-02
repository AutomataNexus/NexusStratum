//! Styled Card compound component: Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter.

use crate::common::merge_classes;
use stratum_core::render::RenderOutput;

// --- Card ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardProps {
    pub class: Option<String>,
}

pub struct Card;

impl Card {
    const BASE: &'static str = "rounded-xl border bg-card text-card-foreground shadow";

    pub fn classes(props: &CardProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &CardProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

// --- CardHeader ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardHeaderProps {
    pub class: Option<String>,
}

pub struct CardHeader;

impl CardHeader {
    const BASE: &'static str = "flex flex-col space-y-1.5 p-6";

    pub fn classes(props: &CardHeaderProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &CardHeaderProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

// --- CardTitle ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardTitleProps {
    pub class: Option<String>,
}

pub struct CardTitle;

impl CardTitle {
    const BASE: &'static str = "font-semibold leading-none tracking-tight";

    pub fn classes(props: &CardTitleProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &CardTitleProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("h3")
            .with_class(Self::classes(props))
    }
}

// --- CardDescription ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardDescriptionProps {
    pub class: Option<String>,
}

pub struct CardDescription;

impl CardDescription {
    const BASE: &'static str = "text-sm text-muted-foreground";

    pub fn classes(props: &CardDescriptionProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &CardDescriptionProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("p")
            .with_class(Self::classes(props))
    }
}

// --- CardContent ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardContentProps {
    pub class: Option<String>,
}

pub struct CardContent;

impl CardContent {
    const BASE: &'static str = "p-6 pt-0";

    pub fn classes(props: &CardContentProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &CardContentProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

// --- CardFooter ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardFooterProps {
    pub class: Option<String>,
}

pub struct CardFooter;

impl CardFooter {
    const BASE: &'static str = "flex items-center p-6 pt-0";

    pub fn classes(props: &CardFooterProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &CardFooterProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_classes() {
        let props = CardProps::default();
        let classes = Card::classes(&props);
        assert!(classes.contains("rounded-xl"));
        assert!(classes.contains("bg-card"));
    }

    #[test]
    fn card_header_tag() {
        let props = CardHeaderProps::default();
        let output = CardHeader::render(&props);
        assert_eq!(output.effective_tag(), "div");
        assert!(output.class_string().contains("p-6"));
    }

    #[test]
    fn card_title_tag() {
        let props = CardTitleProps::default();
        let output = CardTitle::render(&props);
        assert_eq!(output.effective_tag(), "h3");
    }

    #[test]
    fn card_description_tag() {
        let props = CardDescriptionProps::default();
        let output = CardDescription::render(&props);
        assert_eq!(output.effective_tag(), "p");
    }

    #[test]
    fn card_content_classes() {
        let props = CardContentProps::default();
        let classes = CardContent::classes(&props);
        assert!(classes.contains("p-6"));
    }

    #[test]
    fn card_footer_classes() {
        let props = CardFooterProps::default();
        let classes = CardFooter::classes(&props);
        assert!(classes.contains("flex"));
    }
}
