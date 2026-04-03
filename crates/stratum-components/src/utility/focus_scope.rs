//! FocusScope utility — a pass-through container that traps focus within its children.

use crate::common::merge_classes;
use stratum_core::render::RenderOutput;

/// Properties for FocusScope.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FocusScopeProps {
    /// Whether focus should be trapped within this scope.
    pub trapped: bool,
    /// Whether to auto-focus the first focusable element on mount.
    pub auto_focus: bool,
    pub class: Option<String>,
}

pub struct FocusScope;

impl FocusScope {
    pub fn classes(props: &FocusScopeProps) -> String {
        merge_classes("", &props.class)
    }

    pub fn render(props: &FocusScopeProps) -> RenderOutput {
        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props));

        if props.trapped {
            output = output.with_data("focus-trap", "true");
        }
        if props.auto_focus {
            output = output.with_data("auto-focus", "true");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_scope_default() {
        let props = FocusScopeProps::default();
        let output = FocusScope::render(&props);
        assert_eq!(output.effective_tag(), "div");
        assert!(output.data_attrs.is_empty());
    }

    #[test]
    fn focus_scope_trapped() {
        let props = FocusScopeProps {
            trapped: true,
            ..Default::default()
        };
        let output = FocusScope::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "focus-trap" && v == "true")
        );
    }

    #[test]
    fn focus_scope_auto_focus() {
        let props = FocusScopeProps {
            auto_focus: true,
            ..Default::default()
        };
        let output = FocusScope::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "auto-focus" && v == "true")
        );
    }
}
