//! Stack, HStack, and VStack layout components.

use crate::common::merge_classes;
use stratum_core::render::RenderOutput;

/// Spacing between stack items, mapped to Tailwind gap classes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StackSpacing {
    None,
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl StackSpacing {
    pub fn gap_class(&self) -> &'static str {
        match self {
            StackSpacing::None => "gap-0",
            StackSpacing::Xs => "gap-1",
            StackSpacing::Sm => "gap-2",
            StackSpacing::Md => "gap-4",
            StackSpacing::Lg => "gap-6",
            StackSpacing::Xl => "gap-8",
        }
    }
}

/// Alignment along the cross axis.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StackAlign {
    #[default]
    Stretch,
    Start,
    Center,
    End,
    Baseline,
}

impl StackAlign {
    pub fn class(&self) -> &'static str {
        match self {
            StackAlign::Stretch => "items-stretch",
            StackAlign::Start => "items-start",
            StackAlign::Center => "items-center",
            StackAlign::End => "items-end",
            StackAlign::Baseline => "items-baseline",
        }
    }
}

/// Justification along the main axis.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StackJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

impl StackJustify {
    pub fn class(&self) -> &'static str {
        match self {
            StackJustify::Start => "justify-start",
            StackJustify::Center => "justify-center",
            StackJustify::End => "justify-end",
            StackJustify::Between => "justify-between",
            StackJustify::Around => "justify-around",
            StackJustify::Evenly => "justify-evenly",
        }
    }
}

/// Direction of the stack.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StackDirection {
    Horizontal,
    #[default]
    Vertical,
}

/// Properties for the Stack component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StackProps {
    pub direction: StackDirection,
    pub spacing: StackSpacing,
    pub align: StackAlign,
    pub justify: StackJustify,
    pub wrap: bool,
    pub class: Option<String>,
}

pub struct Stack;

impl Stack {
    pub fn classes(props: &StackProps) -> String {
        let dir = match props.direction {
            StackDirection::Horizontal => "flex-row",
            StackDirection::Vertical => "flex-col",
        };
        let wrap = if props.wrap { "flex-wrap" } else { "" };
        let computed = format!(
            "flex {} {} {} {} {}",
            dir,
            props.spacing.gap_class(),
            props.align.class(),
            props.justify.class(),
            wrap,
        );
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &StackProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

/// Horizontal stack convenience component.
pub struct HStack;

impl HStack {
    pub fn classes(spacing: StackSpacing, align: StackAlign, class: &Option<String>) -> String {
        let props = StackProps {
            direction: StackDirection::Horizontal,
            spacing,
            align,
            ..Default::default()
        };
        let mut cls = Stack::classes(&props);
        if let Some(extra) = class {
            cls = format!("{} {}", cls, extra);
        }
        cls
    }

    pub fn render(
        spacing: StackSpacing,
        align: StackAlign,
        class: &Option<String>,
    ) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(spacing, align, class))
    }
}

/// Vertical stack convenience component.
pub struct VStack;

impl VStack {
    pub fn classes(spacing: StackSpacing, align: StackAlign, class: &Option<String>) -> String {
        let props = StackProps {
            direction: StackDirection::Vertical,
            spacing,
            align,
            ..Default::default()
        };
        let mut cls = Stack::classes(&props);
        if let Some(extra) = class {
            cls = format!("{} {}", cls, extra);
        }
        cls
    }

    pub fn render(
        spacing: StackSpacing,
        align: StackAlign,
        class: &Option<String>,
    ) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(spacing, align, class))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_default_is_vertical() {
        let props = StackProps::default();
        let classes = Stack::classes(&props);
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("gap-4"));
    }

    #[test]
    fn hstack_is_horizontal() {
        let classes = HStack::classes(StackSpacing::Md, StackAlign::Center, &None);
        assert!(classes.contains("flex-row"));
        assert!(classes.contains("items-center"));
    }

    #[test]
    fn vstack_is_vertical() {
        let classes = VStack::classes(StackSpacing::Sm, StackAlign::Start, &None);
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("gap-2"));
    }

    #[test]
    fn stack_with_wrap() {
        let props = StackProps {
            wrap: true,
            ..Default::default()
        };
        let classes = Stack::classes(&props);
        assert!(classes.contains("flex-wrap"));
    }

    #[test]
    fn stack_render_tag() {
        let props = StackProps::default();
        let output = Stack::render(&props);
        assert_eq!(output.effective_tag(), "div");
    }
}
