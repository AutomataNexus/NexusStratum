//! Styled Select dropdown component.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaHasPopup, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Properties for the styled Select trigger.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectProps {
    pub size: Size,
    pub disabled: bool,
    pub required: bool,
    pub placeholder: Option<String>,
    pub open: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

pub struct Select;

impl Select {
    const BASE: &'static str = "flex w-full items-center justify-between whitespace-nowrap rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";

    pub fn classes(props: &SelectProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "h-7 text-xs",
            Size::Sm => "h-8 text-xs",
            Size::Md => "h-9 text-sm",
            Size::Lg => "h-10 text-base",
            Size::Xl => "h-12 text-lg",
        };

        let computed = format!("{} {}", Self::BASE, size_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &SelectProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Combobox)
            .with_expanded(props.open)
            .with_haspopup(AriaHasPopup::ListBox);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }
        if props.required {
            aria.required = Some(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_class(classes)
            .with_aria(aria)
            .with_attr("type", AttrValue::String("button".to_string()));

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
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
    fn default_select_classes() {
        let props = SelectProps::default();
        let classes = Select::classes(&props);
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("h-9"));
    }

    #[test]
    fn render_open_select() {
        let props = SelectProps {
            open: true,
            ..Default::default()
        };
        let output = Select::render(&props);
        assert_eq!(output.aria.expanded, Some(true));
    }

    #[test]
    fn render_closed_select() {
        let props = SelectProps::default();
        let output = Select::render(&props);
        assert_eq!(output.aria.expanded, Some(false));
    }

    #[test]
    fn render_has_combobox_role() {
        let props = SelectProps::default();
        let output = Select::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::Combobox));
    }
}
