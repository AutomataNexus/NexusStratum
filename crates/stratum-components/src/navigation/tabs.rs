//! Styled Tabs, TabList, Tab, and TabPanel components.

use crate::common::{Size, merge_classes};
use stratum_core::aria::{AriaAttributes, AriaRole, Orientation};
use stratum_core::render::{AttrValue, RenderOutput};

// --- TabList ---

#[derive(Debug, Clone, PartialEq)]
pub struct TabListProps {
    pub orientation: Orientation,
    pub class: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for TabListProps {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            class: None,
            aria_label: None,
        }
    }
}

pub struct TabList;

impl TabList {
    pub fn classes(props: &TabListProps) -> String {
        let base = match props.orientation {
            Orientation::Horizontal => {
                "inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground"
            }
            Orientation::Vertical => {
                "flex flex-col h-auto items-stretch rounded-lg bg-muted p-1 text-muted-foreground"
            }
        };
        merge_classes(base, &props.class)
    }

    pub fn render(props: &TabListProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::TabList)
            .with_orientation(props.orientation);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }

        RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
    }
}

// --- Tab ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TabProps {
    pub size: Size,
    pub selected: bool,
    pub disabled: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub controls: Option<String>,
    pub id: Option<String>,
}

pub struct Tab;

impl Tab {
    const BASE: &'static str = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow";

    pub fn classes(props: &TabProps) -> String {
        let size_cls = match props.size {
            Size::Xs => "text-xs py-0.5 px-2",
            Size::Sm => "text-xs py-1 px-2",
            Size::Md => "text-sm py-1 px-3",
            Size::Lg => "text-base py-1.5 px-4",
            Size::Xl => "text-lg py-2 px-5",
        };

        let computed = format!("{} {}", Self::BASE, size_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &TabProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Tab)
            .with_selected(props.selected);

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
            .with_data("state", if props.selected { "active" } else { "inactive" });

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

// --- TabPanel ---

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TabPanelProps {
    pub selected: bool,
    pub class: Option<String>,
    pub aria_labelledby: Option<String>,
    pub id: Option<String>,
}

pub struct TabPanel;

impl TabPanel {
    const BASE: &'static str = "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2";

    pub fn classes(props: &TabPanelProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &TabPanelProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::TabPanel);

        if let Some(ref labelledby) = props.aria_labelledby {
            aria = aria.with_labelledby(labelledby.clone());
        }
        if !props.selected {
            aria.hidden = Some(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.selected { "active" } else { "inactive" });

        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }
        if !props.selected {
            output = output.with_style("display", "none");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_list_horizontal() {
        let props = TabListProps::default();
        let classes = TabList::classes(&props);
        assert!(classes.contains("inline-flex"));
    }

    #[test]
    fn tab_list_vertical() {
        let props = TabListProps {
            orientation: Orientation::Vertical,
            ..Default::default()
        };
        let classes = TabList::classes(&props);
        assert!(classes.contains("flex-col"));
    }

    #[test]
    fn tab_selected() {
        let props = TabProps {
            selected: true,
            ..Default::default()
        };
        let output = Tab::render(&props);
        assert_eq!(output.aria.selected, Some(true));
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "state" && v == "active")
        );
    }

    #[test]
    fn tab_panel_hidden_when_not_selected() {
        let props = TabPanelProps::default();
        let output = TabPanel::render(&props);
        assert_eq!(output.aria.hidden, Some(true));
        assert!(
            output
                .styles
                .iter()
                .any(|(k, v)| k == "display" && v == "none")
        );
    }

    #[test]
    fn tab_panel_visible_when_selected() {
        let props = TabPanelProps {
            selected: true,
            ..Default::default()
        };
        let output = TabPanel::render(&props);
        assert!(output.aria.hidden.is_none());
    }
}
