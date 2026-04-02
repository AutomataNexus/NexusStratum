//! Accordion primitive — Collapsible sections component.
//!
//! Provides headless accordion with single or multiple open items,
//! keyboard navigation, and proper ARIA attributes.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};

/// Props for the Accordion primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct AccordionProps {
    /// Controlled open items.
    pub value: Option<Vec<String>>,
    /// Default open items for uncontrolled usage.
    pub default_value: Vec<String>,
    /// Allow multiple items to be open at once.
    pub multiple: bool,
    /// Allow all items to be closed.
    pub collapsible: bool,
    /// Whether the entire accordion is disabled.
    pub disabled: bool,
    /// Callback when open items change.
    pub on_value_change: Option<Callback<Vec<String>>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Item identifiers.
    pub items: Vec<String>,
}

impl Default for AccordionProps {
    fn default() -> Self {
        Self {
            value: None,
            default_value: Vec::new(),
            multiple: false,
            collapsible: true,
            disabled: false,
            on_value_change: None,
            id: None,
            items: Vec::new(),
        }
    }
}

/// Internal state for the Accordion primitive.
#[derive(Debug, Clone)]
pub struct AccordionState {
    /// Currently open item identifiers.
    pub open_items: Vec<String>,
    /// Index of the currently focused trigger.
    pub focused_index: usize,
    /// Root element ID.
    pub id: String,
    /// IDs for each item trigger.
    pub trigger_ids: Vec<String>,
    /// IDs for each item content region.
    pub content_ids: Vec<String>,
}

/// Headless accordion primitive.
pub struct Accordion;

impl Accordion {
    /// Toggle an item in the open_items list based on accordion rules.
    fn toggle_item(
        props: &AccordionProps,
        state: &mut AccordionState,
        item: &str,
    ) {
        if props.disabled {
            return;
        }

        let is_open = state.open_items.contains(&item.to_string());

        if is_open {
            if props.collapsible || state.open_items.len() > 1 {
                state.open_items.retain(|i| i != item);
            }
        } else if props.multiple {
            state.open_items.push(item.to_string());
        } else {
            state.open_items = vec![item.to_string()];
        }

        if let Some(ref cb) = props.on_value_change {
            cb.call(state.open_items.clone());
        }
    }
}

impl Component for Accordion {
    type Props = AccordionProps;
    type State = AccordionState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let base_id = props.id.clone().unwrap_or_else(|| generators::ACCORDION.next());

        let open_items = props.value.clone().unwrap_or_else(|| props.default_value.clone());

        let trigger_ids: Vec<String> = props.items.iter()
            .map(|item| format!("{}-trigger-{}", base_id, item))
            .collect();

        let content_ids: Vec<String> = props.items.iter()
            .map(|item| format!("{}-content-{}", base_id, item))
            .collect();

        AccordionState {
            open_items,
            focused_index: 0,
            id: base_id,
            trigger_ids,
            content_ids,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let mut items = Vec::new();

        for (i, item) in props.items.iter().enumerate() {
            let is_open = state.open_items.contains(item);

            // Trigger (button)
            let mut trigger_aria = AriaAttributes::new()
                .with_expanded(is_open);

            if i < state.content_ids.len() {
                trigger_aria = trigger_aria.with_controls(&state.content_ids[i]);
            }

            if props.disabled {
                trigger_aria = trigger_aria.with_disabled(true);
            }

            let trigger = RenderOutput::new()
                .with_tag("button")
                .with_aria(trigger_aria)
                .with_attr("id", AttrValue::String(state.trigger_ids[i].clone()));

            // Content (region)
            let mut content_aria = AriaAttributes::new()
                .with_role(AriaRole::Region);

            if i < state.trigger_ids.len() {
                content_aria = content_aria.with_labelledby(&state.trigger_ids[i]);
            }

            let mut content = RenderOutput::new()
                .with_tag("div")
                .with_aria(content_aria)
                .with_attr("id", AttrValue::String(state.content_ids[i].clone()));

            if !is_open {
                content = content.with_attr("hidden", AttrValue::Bool(true));
            }

            items.push(trigger);
            items.push(content);
        }

        RenderOutput::new()
            .with_tag("div")
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_data("disabled", props.disabled.to_string())
            .with_children(ChildrenSpec::Elements(items))
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        if props.disabled || props.items.is_empty() {
            return EventResult::default();
        }

        match event {
            ComponentEvent::KeyDown { key, .. } => {
                let len = props.items.len();
                match key {
                    Key::ArrowDown => {
                        state.focused_index = (state.focused_index + 1) % len;
                        EventResult::prevent_and_changed()
                    }
                    Key::ArrowUp => {
                        state.focused_index = if state.focused_index == 0 {
                            len - 1
                        } else {
                            state.focused_index - 1
                        };
                        EventResult::prevent_and_changed()
                    }
                    Key::Home => {
                        state.focused_index = 0;
                        EventResult::prevent_and_changed()
                    }
                    Key::End => {
                        state.focused_index = len - 1;
                        EventResult::prevent_and_changed()
                    }
                    Key::Enter | Key::Space => {
                        let item = props.items[state.focused_index].clone();
                        Accordion::toggle_item(props, state, &item);
                        EventResult::prevent_and_changed()
                    }
                    _ => EventResult::default(),
                }
            }
            _ => EventResult::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        use stratum_core::event::ModifierKeys;
    use std::sync::{Arc, Mutex};

    fn test_props() -> AccordionProps {
        AccordionProps {
            items: vec!["section1".to_string(), "section2".to_string(), "section3".to_string()],
            ..AccordionProps::default()
        }
    }

    #[test]
    fn initial_state_defaults() {
        let props = test_props();
        let state = Accordion::initial_state(&props);
        assert!(state.open_items.is_empty());
        assert_eq!(state.focused_index, 0);
        assert_eq!(state.trigger_ids.len(), 3);
        assert_eq!(state.content_ids.len(), 3);
    }

    #[test]
    fn initial_state_with_default_value() {
        let props = AccordionProps {
            default_value: vec!["section2".to_string()],
            ..test_props()
        };
        let state = Accordion::initial_state(&props);
        assert_eq!(state.open_items, vec!["section2"]);
    }

    #[test]
    fn initial_state_controlled_value() {
        let props = AccordionProps {
            value: Some(vec!["section1".to_string(), "section3".to_string()]),
            ..test_props()
        };
        let state = Accordion::initial_state(&props);
        assert_eq!(state.open_items.len(), 2);
    }

    #[test]
    fn render_trigger_expanded() {
        let props = AccordionProps {
            default_value: vec!["section1".to_string()],
            ..test_props()
        };
        let state = Accordion::initial_state(&props);
        let output = Accordion::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // trigger for section1 at index 0
            assert_eq!(elems[0].aria.expanded, Some(true));
            // trigger for section2 at index 2
            assert_eq!(elems[2].aria.expanded, Some(false));
        }
    }

    #[test]
    fn render_content_region_role() {
        let props = test_props();
        let state = Accordion::initial_state(&props);
        let output = Accordion::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // content at index 1
            assert_eq!(elems[1].aria.role, Some(AriaRole::Region));
        }
    }

    #[test]
    fn render_content_labelledby_trigger() {
        let props = test_props();
        let state = Accordion::initial_state(&props);
        let output = Accordion::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[1].aria.labelledby, Some(state.trigger_ids[0].clone()));
        }
    }

    #[test]
    fn render_trigger_controls_content() {
        let props = test_props();
        let state = Accordion::initial_state(&props);
        let output = Accordion::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.controls, Some(state.content_ids[0].clone()));
        }
    }

    #[test]
    fn render_closed_content_hidden() {
        let props = test_props();
        let state = Accordion::initial_state(&props);
        let output = Accordion::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(elems[1].attrs.contains(&("hidden".to_string(), AttrValue::Bool(true))));
        }
    }

    #[test]
    fn enter_toggles_item() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert!(state.open_items.contains(&"section1".to_string()));
    }

    #[test]
    fn space_toggles_item() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Space,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert!(state.open_items.contains(&"section1".to_string()));
    }

    #[test]
    fn single_mode_closes_others() {
        let props = AccordionProps {
            multiple: false,
            default_value: vec!["section1".to_string()],
            ..test_props()
        };
        let mut state = Accordion::initial_state(&props);
        state.focused_index = 1;
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.open_items, vec!["section2"]);
    }

    #[test]
    fn multiple_mode_keeps_others() {
        let props = AccordionProps {
            multiple: true,
            default_value: vec!["section1".to_string()],
            ..test_props()
        };
        let mut state = Accordion::initial_state(&props);
        state.focused_index = 1;
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.open_items.len(), 2);
        assert!(state.open_items.contains(&"section1".to_string()));
        assert!(state.open_items.contains(&"section2".to_string()));
    }

    #[test]
    fn collapsible_allows_all_closed() {
        let props = AccordionProps {
            collapsible: true,
            default_value: vec!["section1".to_string()],
            ..test_props()
        };
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert!(state.open_items.is_empty());
    }

    #[test]
    fn non_collapsible_prevents_all_closed() {
        let props = AccordionProps {
            collapsible: false,
            default_value: vec!["section1".to_string()],
            ..test_props()
        };
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.open_items, vec!["section1"]); // can't close last one
    }

    #[test]
    fn arrow_down_moves_focus() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 1);
    }

    #[test]
    fn arrow_up_moves_focus() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        state.focused_index = 1;
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowUp,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn arrow_down_wraps() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        state.focused_index = 2;
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn arrow_up_wraps() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowUp,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn home_jumps_to_first() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        state.focused_index = 2;
        let event = ComponentEvent::KeyDown {
            key: Key::Home,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn end_jumps_to_last() {
        let props = test_props();
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::End,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn disabled_prevents_interaction() {
        let props = AccordionProps {
            disabled: true,
            ..test_props()
        };
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        let result = Accordion::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
    }

    #[test]
    fn callback_fires_on_change() {
        let received = Arc::new(Mutex::new(Vec::<String>::new()));
        let received_clone = Arc::clone(&received);
        let props = AccordionProps {
            on_value_change: Some(Callback::new(move |val: Vec<String>| {
                *received_clone.lock().unwrap() = val;
            })),
            ..test_props()
        };
        let mut state = Accordion::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Accordion::on_event(&props, &mut state, event);
        assert_eq!(*received.lock().unwrap(), vec!["section1"]);
    }

    #[test]
    fn render_disabled_triggers() {
        let props = AccordionProps {
            disabled: true,
            ..test_props()
        };
        let state = Accordion::initial_state(&props);
        let output = Accordion::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.disabled, Some(true));
        }
    }
}
