//! Loading spinner component.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::RenderOutput;

/// Properties for the Spinner component.
#[derive(Debug, Clone, PartialEq)]
pub struct SpinnerProps {
    pub size: Size,
    pub class: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for SpinnerProps {
    fn default() -> Self {
        Self {
            size: Size::default(),
            class: None,
            aria_label: Some("Loading".to_string()),
        }
    }
}

pub struct Spinner;

impl Spinner {
    const BASE: &'static str =
        "animate-spin rounded-full border-2 border-current border-t-transparent";

    pub fn classes(props: &SpinnerProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "h-3 w-3",
            Size::Sm => "h-4 w-4",
            Size::Md => "h-6 w-6",
            Size::Lg => "h-8 w-8",
            Size::Xl => "h-12 w-12",
        };

        let computed = format!("{} {}", Self::BASE, size_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &SpinnerProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Status);
        aria.busy = Some(true);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
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
    fn spinner_default_classes() {
        let props = SpinnerProps::default();
        let classes = Spinner::classes(&props);
        assert!(classes.contains("animate-spin"));
        assert!(classes.contains("h-6 w-6"));
    }

    #[test]
    fn spinner_small() {
        let props = SpinnerProps {
            size: Size::Sm,
            ..Default::default()
        };
        let classes = Spinner::classes(&props);
        assert!(classes.contains("h-4 w-4"));
    }

    #[test]
    fn spinner_aria() {
        let props = SpinnerProps::default();
        let output = Spinner::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::Status));
        assert_eq!(output.aria.busy, Some(true));
        assert_eq!(output.aria.label, Some("Loading".to_string()));
    }
}
