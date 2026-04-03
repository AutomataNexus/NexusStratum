//! Styled Dialog component with overlay backdrop.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Properties for the Dialog overlay/backdrop.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DialogOverlayProps {
    pub class: Option<String>,
}

pub struct DialogOverlay;

impl DialogOverlay {
    const BASE: &'static str = "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0";

    pub fn classes(props: &DialogOverlayProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &DialogOverlayProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
            .with_aria({
                let mut a = AriaAttributes::new();
                a.hidden = Some(true);
                a
            })
    }
}

/// Properties for the Dialog content panel.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DialogProps {
    pub open: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub aria_describedby: Option<String>,
    pub id: Option<String>,
}

pub struct Dialog;

impl Dialog {
    const BASE: &'static str = "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg";

    pub fn classes(props: &DialogProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &DialogProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Dialog)
            .with_modal(true);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if let Some(ref desc) = props.aria_describedby {
            aria = aria.with_describedby(desc.clone());
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.open { "open" } else { "closed" });

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
    fn dialog_overlay_classes() {
        let props = DialogOverlayProps::default();
        let classes = DialogOverlay::classes(&props);
        assert!(classes.contains("fixed inset-0"));
        assert!(classes.contains("bg-black/80"));
    }

    #[test]
    fn dialog_render_open() {
        let props = DialogProps {
            open: true,
            aria_label: Some("Confirm".to_string()),
            ..Default::default()
        };
        let output = Dialog::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::Dialog));
        assert_eq!(output.aria.modal, Some(true));
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "open")
        );
    }

    #[test]
    fn dialog_render_closed() {
        let props = DialogProps::default();
        let output = Dialog::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "closed")
        );
    }
}
