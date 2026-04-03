//! Styled Popover component.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Side the popover appears on.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverSide {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

/// Alignment of the popover relative to the trigger.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverAlign {
    #[default]
    Center,
    Start,
    End,
}

/// Properties for the styled Popover content.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PopoverProps {
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub open: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

pub struct Popover;

impl Popover {
    const BASE: &'static str = "z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";

    pub fn classes(props: &PopoverProps) -> String {
        let side_cls = match props.side {
            PopoverSide::Top => "data-[side=top]:slide-in-from-bottom-2",
            PopoverSide::Bottom => "data-[side=bottom]:slide-in-from-top-2",
            PopoverSide::Left => "data-[side=left]:slide-in-from-right-2",
            PopoverSide::Right => "data-[side=right]:slide-in-from-left-2",
        };

        let computed = format!("{} {}", Self::BASE, side_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &PopoverProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Dialog);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.open { "open" } else { "closed" })
            .with_data(
                "side",
                match props.side {
                    PopoverSide::Top => "top",
                    PopoverSide::Bottom => "bottom",
                    PopoverSide::Left => "left",
                    PopoverSide::Right => "right",
                },
            )
            .with_data(
                "align",
                match props.align {
                    PopoverAlign::Center => "center",
                    PopoverAlign::Start => "start",
                    PopoverAlign::End => "end",
                },
            );

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
    fn popover_default_classes() {
        let props = PopoverProps::default();
        let classes = Popover::classes(&props);
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("bg-popover"));
    }

    #[test]
    fn popover_render_role() {
        let props = PopoverProps::default();
        let output = Popover::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::Dialog));
    }

    #[test]
    fn popover_data_attrs() {
        let props = PopoverProps {
            open: true,
            side: PopoverSide::Top,
            align: PopoverAlign::End,
            ..Default::default()
        };
        let output = Popover::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "open")
        );
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "side" && v == "top")
        );
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "align" && v == "end")
        );
    }
}
