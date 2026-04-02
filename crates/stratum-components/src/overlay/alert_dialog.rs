//! Styled AlertDialog component.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::RenderOutput;

/// Properties for the AlertDialog content.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AlertDialogProps {
    pub open: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub aria_describedby: Option<String>,
}

pub struct AlertDialog;

impl AlertDialog {
    const BASE: &'static str = "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 sm:rounded-lg";

    pub fn classes(props: &AlertDialogProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &AlertDialogProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::AlertDialog)
            .with_modal(true);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if let Some(ref desc) = props.aria_describedby {
            aria = aria.with_describedby(desc.clone());
        }

        RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.open { "open" } else { "closed" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_dialog_role() {
        let props = AlertDialogProps::default();
        let output = AlertDialog::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::AlertDialog));
        assert_eq!(output.aria.modal, Some(true));
    }

    #[test]
    fn alert_dialog_open_state() {
        let props = AlertDialogProps {
            open: true,
            ..Default::default()
        };
        let output = AlertDialog::render(&props);
        assert!(output.data_attrs.iter().any(|(k, v)| k == "state" && v == "open"));
    }
}
