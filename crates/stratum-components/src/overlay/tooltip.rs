//! Styled Tooltip component.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::RenderOutput;

/// Side the tooltip appears on.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

/// Properties for the styled Tooltip.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TooltipProps {
    pub side: TooltipSide,
    pub open: bool,
    pub class: Option<String>,
    pub content: String,
    pub id: Option<String>,
}

pub struct Tooltip;

impl Tooltip {
    const BASE: &'static str = "z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground animate-in fade-in-0 zoom-in-95";

    pub fn classes(props: &TooltipProps) -> String {
        let side_cls = match props.side {
            TooltipSide::Top => "slide-in-from-bottom-2",
            TooltipSide::Right => "slide-in-from-left-2",
            TooltipSide::Bottom => "slide-in-from-top-2",
            TooltipSide::Left => "slide-in-from-right-2",
        };

        let computed = format!("{} {}", Self::BASE, side_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &TooltipProps) -> RenderOutput {
        let classes = Self::classes(props);
        let aria = AriaAttributes::new().with_role(AriaRole::ToolTip);

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.open { "open" } else { "closed" })
            .with_data("side", match props.side {
                TooltipSide::Top => "top",
                TooltipSide::Right => "right",
                TooltipSide::Bottom => "bottom",
                TooltipSide::Left => "left",
            });

        if let Some(ref id) = props.id {
            output = output.with_data("tooltip-id", id.clone());
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_default_classes() {
        let props = TooltipProps::default();
        let classes = Tooltip::classes(&props);
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("slide-in-from-bottom-2"));
    }

    #[test]
    fn tooltip_side_classes() {
        let props = TooltipProps {
            side: TooltipSide::Left,
            ..Default::default()
        };
        let classes = Tooltip::classes(&props);
        assert!(classes.contains("slide-in-from-right-2"));
    }

    #[test]
    fn tooltip_render_role() {
        let props = TooltipProps::default();
        let output = Tooltip::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::ToolTip));
    }
}
