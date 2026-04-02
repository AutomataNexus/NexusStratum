//! Styled Accordion component.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

// --- Accordion root ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccordionProps {
    pub class: Option<String>,
    pub allow_multiple: bool,
}

pub struct Accordion;

impl Accordion {
    pub fn classes(props: &AccordionProps) -> String {
        merge_classes("divide-y divide-border", &props.class)
    }

    pub fn render(props: &AccordionProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
            .with_data("type", if props.allow_multiple { "multiple" } else { "single" })
    }
}

// --- AccordionItem ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccordionItemProps {
    pub expanded: bool,
    pub disabled: bool,
    pub class: Option<String>,
    pub id: Option<String>,
}

pub struct AccordionItem;

impl AccordionItem {
    const BASE: &'static str = "border-b";

    pub fn classes(props: &AccordionItemProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &AccordionItemProps) -> RenderOutput {
        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
            .with_data("state", if props.expanded { "open" } else { "closed" });

        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

// --- AccordionTrigger ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccordionTriggerProps {
    pub expanded: bool,
    pub disabled: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub controls: Option<String>,
}

pub struct AccordionTrigger;

impl AccordionTrigger {
    const BASE: &'static str = "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180";

    pub fn classes(props: &AccordionTriggerProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &AccordionTriggerProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Button)
            .with_expanded(props.expanded);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }
        if let Some(ref controls) = props.controls {
            aria = aria.with_controls(controls.clone());
        }

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_class(classes)
            .with_aria(aria)
            .with_attr("type", AttrValue::String("button".to_string()))
            .with_data("state", if props.expanded { "open" } else { "closed" });

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }

        output
    }
}

// --- AccordionContent ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccordionContentProps {
    pub expanded: bool,
    pub class: Option<String>,
    pub id: Option<String>,
}

pub struct AccordionContent;

impl AccordionContent {
    const BASE: &'static str = "overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down";

    pub fn classes(props: &AccordionContentProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &AccordionContentProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Region);
        if !props.expanded {
            aria.hidden = Some(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.expanded { "open" } else { "closed" });

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
    fn accordion_single_mode() {
        let props = AccordionProps::default();
        let output = Accordion::render(&props);
        assert!(output.data_attrs.iter().any(|(k, v)| k == "type" && v == "single"));
    }

    #[test]
    fn accordion_multiple_mode() {
        let props = AccordionProps {
            allow_multiple: true,
            ..Default::default()
        };
        let output = Accordion::render(&props);
        assert!(output.data_attrs.iter().any(|(k, v)| k == "type" && v == "multiple"));
    }

    #[test]
    fn accordion_trigger_expanded() {
        let props = AccordionTriggerProps {
            expanded: true,
            ..Default::default()
        };
        let output = AccordionTrigger::render(&props);
        assert_eq!(output.aria.expanded, Some(true));
    }

    #[test]
    fn accordion_content_hidden_when_collapsed() {
        let props = AccordionContentProps::default();
        let output = AccordionContent::render(&props);
        assert_eq!(output.aria.hidden, Some(true));
    }

    #[test]
    fn accordion_content_visible_when_expanded() {
        let props = AccordionContentProps {
            expanded: true,
            ..Default::default()
        };
        let output = AccordionContent::render(&props);
        assert!(output.aria.hidden.is_none());
    }
}
