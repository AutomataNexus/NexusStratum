//! Tabs primitive — Tabbed interface component.
//!
//! Provides headless tab navigation with proper ARIA roles for
//! tablist, tab, and tabpanel elements with keyboard navigation.

use stratum_core::aria::Orientation;
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};
use stratum_core::{
    AriaAttributes, AriaRole, Component, ComponentEvent, EventResult, Key, RenderOutput,
};

/// Props for the Tabs primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct TabsProps {
    /// Controlled active tab value.
    pub value: Option<String>,
    /// Default active tab for uncontrolled usage.
    pub default_value: Option<String>,
    /// Orientation of the tab list.
    pub orientation: Orientation,
    /// Callback when the active tab changes.
    pub on_value_change: Option<Callback<String>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Tab identifiers.
    pub items: Vec<String>,
}

impl Default for TabsProps {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            orientation: Orientation::Horizontal,
            on_value_change: None,
            id: None,
            items: Vec::new(),
        }
    }
}

/// Internal state for the Tabs primitive.
#[derive(Debug, Clone)]
pub struct TabsState {
    /// Currently active tab identifier.
    pub active_tab: String,
    /// Currently focused tab index within the tab list.
    pub focused_index: usize,
    /// Root element ID.
    pub id: String,
    /// IDs for each tab element.
    pub tab_ids: Vec<String>,
    /// IDs for each panel element.
    pub panel_ids: Vec<String>,
}

/// Headless tabs primitive.
pub struct Tabs;

impl Component for Tabs {
    type Props = TabsProps;
    type State = TabsState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let base_id = props.id.clone().unwrap_or_else(|| generators::TABS.next());

        let active_tab = props
            .value
            .clone()
            .or_else(|| props.default_value.clone())
            .or_else(|| props.items.first().cloned())
            .unwrap_or_default();

        let focused_index = props
            .items
            .iter()
            .position(|item| *item == active_tab)
            .unwrap_or(0);

        let tab_ids: Vec<String> = props
            .items
            .iter()
            .map(|item| format!("{}-tab-{}", base_id, item))
            .collect();

        let panel_ids: Vec<String> = props
            .items
            .iter()
            .map(|item| format!("{}-panel-{}", base_id, item))
            .collect();

        TabsState {
            active_tab,
            focused_index,
            id: base_id,
            tab_ids,
            panel_ids,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let active_tab = props.value.as_deref().unwrap_or(&state.active_tab);

        // TabList
        let tablist_aria = AriaAttributes::new()
            .with_role(AriaRole::TabList)
            .with_orientation(props.orientation);

        let tablist = RenderOutput::new().with_tag("div").with_aria(tablist_aria);

        // Individual tabs
        let mut tabs = Vec::new();
        for (i, item) in props.items.iter().enumerate() {
            let is_active = *item == *active_tab;
            let mut tab_aria = AriaAttributes::new()
                .with_role(AriaRole::Tab)
                .with_selected(is_active);

            if i < state.panel_ids.len() {
                tab_aria = tab_aria.with_controls(&state.panel_ids[i]);
            }

            let tab = RenderOutput::new()
                .with_tag("button")
                .with_aria(tab_aria)
                .with_attr("id", AttrValue::String(state.tab_ids[i].clone()))
                .with_attr(
                    "tabindex",
                    AttrValue::String(if is_active { "0" } else { "-1" }.to_string()),
                );
            tabs.push(tab);
        }

        // Panels
        let mut panels = Vec::new();
        for (i, item) in props.items.iter().enumerate() {
            let is_active = *item == *active_tab;
            let mut panel_aria = AriaAttributes::new().with_role(AriaRole::TabPanel);

            if i < state.tab_ids.len() {
                panel_aria = panel_aria.with_labelledby(&state.tab_ids[i]);
            }

            let mut panel = RenderOutput::new()
                .with_tag("div")
                .with_aria(panel_aria)
                .with_attr("id", AttrValue::String(state.panel_ids[i].clone()))
                .with_attr("tabindex", AttrValue::String("0".to_string()));

            if !is_active {
                panel = panel.with_attr("hidden", AttrValue::Bool(true));
            }

            panels.push(panel);
        }

        let mut all_children = vec![tablist];
        all_children.extend(tabs);
        all_children.extend(panels);

        RenderOutput::new()
            .with_tag("div")
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_data("orientation", props.orientation.as_str())
            .with_children(ChildrenSpec::Elements(all_children))
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        if props.items.is_empty() {
            return EventResult::default();
        }

        match event {
            ComponentEvent::KeyDown { key, .. } => {
                let len = props.items.len();
                let new_index = match key {
                    Key::ArrowRight if props.orientation == Orientation::Horizontal => {
                        Some((state.focused_index + 1) % len)
                    }
                    Key::ArrowLeft if props.orientation == Orientation::Horizontal => {
                        Some(if state.focused_index == 0 {
                            len - 1
                        } else {
                            state.focused_index - 1
                        })
                    }
                    Key::ArrowDown if props.orientation == Orientation::Vertical => {
                        Some((state.focused_index + 1) % len)
                    }
                    Key::ArrowUp if props.orientation == Orientation::Vertical => {
                        Some(if state.focused_index == 0 {
                            len - 1
                        } else {
                            state.focused_index - 1
                        })
                    }
                    Key::Home => Some(0),
                    Key::End => Some(len - 1),
                    _ => None,
                };

                if let Some(idx) = new_index {
                    let new_value = props.items[idx].clone();
                    if let Some(ref cb) = props.on_value_change {
                        cb.call(new_value.clone());
                    }
                    // Only update internal state in uncontrolled mode
                    if props.value.is_none() {
                        state.focused_index = idx;
                        state.active_tab = new_value;
                        return EventResult::prevent_and_changed();
                    }
                    return EventResult {
                        prevent_default: true,
                        stop_propagation: false,
                        state_changed: false,
                    };
                }

                EventResult::default()
            }
            _ => EventResult::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use stratum_core::event::ModifierKeys;

    fn test_props() -> TabsProps {
        TabsProps {
            items: vec!["tab1".to_string(), "tab2".to_string(), "tab3".to_string()],
            ..TabsProps::default()
        }
    }

    #[test]
    fn initial_state_defaults_to_first_tab() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        assert_eq!(state.active_tab, "tab1");
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn initial_state_with_default_value() {
        let props = TabsProps {
            default_value: Some("tab2".to_string()),
            ..test_props()
        };
        let state = Tabs::initial_state(&props);
        assert_eq!(state.active_tab, "tab2");
        assert_eq!(state.focused_index, 1);
    }

    #[test]
    fn initial_state_controlled_value() {
        let props = TabsProps {
            value: Some("tab3".to_string()),
            ..test_props()
        };
        let state = Tabs::initial_state(&props);
        assert_eq!(state.active_tab, "tab3");
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn initial_state_generates_ids() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        assert_eq!(state.tab_ids.len(), 3);
        assert_eq!(state.panel_ids.len(), 3);
        for (tab_id, panel_id) in state.tab_ids.iter().zip(state.panel_ids.iter()) {
            assert!(tab_id.contains("-tab-"));
            assert!(panel_id.contains("-panel-"));
        }
    }

    #[test]
    fn render_tablist_role() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        let output = Tabs::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.role, Some(AriaRole::TabList));
            assert_eq!(elems[0].aria.orientation, Some(Orientation::Horizontal));
        } else {
            panic!("Expected Elements");
        }
    }

    #[test]
    fn render_tab_roles_and_selected() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        let output = Tabs::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // tabs start at index 1 (after tablist)
            assert_eq!(elems[1].aria.role, Some(AriaRole::Tab));
            assert_eq!(elems[1].aria.selected, Some(true)); // first tab active
            assert_eq!(elems[2].aria.selected, Some(false));
            assert_eq!(elems[3].aria.selected, Some(false));
        }
    }

    #[test]
    fn render_panel_roles() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        let output = Tabs::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // panels start at index 4 (after tablist + 3 tabs)
            assert_eq!(elems[4].aria.role, Some(AriaRole::TabPanel));
            // first panel visible, rest hidden
            assert!(
                !elems[4]
                    .attrs
                    .contains(&("hidden".to_string(), AttrValue::Bool(true)))
            );
            assert!(
                elems[5]
                    .attrs
                    .contains(&("hidden".to_string(), AttrValue::Bool(true)))
            );
            assert!(
                elems[6]
                    .attrs
                    .contains(&("hidden".to_string(), AttrValue::Bool(true)))
            );
        }
    }

    #[test]
    fn render_tab_controls_panel() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        let output = Tabs::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[1].aria.controls, Some(state.panel_ids[0].clone()));
        }
    }

    #[test]
    fn render_panel_labelledby_tab() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        let output = Tabs::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[4].aria.labelledby, Some(state.tab_ids[0].clone()));
        }
    }

    #[test]
    fn arrow_right_moves_to_next_tab() {
        let props = test_props();
        let mut state = Tabs::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowRight,
            modifiers: ModifierKeys::default(),
        };
        let result = Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab2");
        assert_eq!(state.focused_index, 1);
        assert!(result.state_changed);
        assert!(result.prevent_default);
    }

    #[test]
    fn arrow_right_wraps_around() {
        let props = test_props();
        let mut state = Tabs::initial_state(&props);
        state.focused_index = 2;
        state.active_tab = "tab3".to_string();
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowRight,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab1");
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn arrow_left_moves_to_prev_tab() {
        let props = test_props();
        let mut state = Tabs::initial_state(&props);
        state.focused_index = 1;
        state.active_tab = "tab2".to_string();
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowLeft,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab1");
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn arrow_left_wraps_around() {
        let props = test_props();
        let mut state = Tabs::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowLeft,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab3");
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn vertical_uses_up_down_arrows() {
        let props = TabsProps {
            orientation: Orientation::Vertical,
            ..test_props()
        };
        let mut state = Tabs::initial_state(&props);

        // ArrowRight should NOT work in vertical mode
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowRight,
            modifiers: ModifierKeys::default(),
        };
        let result = Tabs::on_event(&props, &mut state, event);
        assert!(!result.state_changed);

        // ArrowDown should work
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab2");

        // ArrowUp should work
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowUp,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab1");
    }

    #[test]
    fn home_jumps_to_first() {
        let props = test_props();
        let mut state = Tabs::initial_state(&props);
        state.focused_index = 2;
        state.active_tab = "tab3".to_string();
        let event = ComponentEvent::KeyDown {
            key: Key::Home,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab1");
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn end_jumps_to_last() {
        let props = test_props();
        let mut state = Tabs::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::End,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(state.active_tab, "tab3");
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn callback_fires_on_tab_change() {
        let received = Arc::new(Mutex::new(String::new()));
        let received_clone = Arc::clone(&received);
        let props = TabsProps {
            on_value_change: Some(Callback::new(move |val: String| {
                *received_clone.lock().unwrap() = val;
            })),
            ..test_props()
        };
        let mut state = Tabs::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowRight,
            modifiers: ModifierKeys::default(),
        };
        Tabs::on_event(&props, &mut state, event);
        assert_eq!(*received.lock().unwrap(), "tab2");
    }

    #[test]
    fn empty_items_is_noop() {
        let props = TabsProps::default();
        let mut state = Tabs::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowRight,
            modifiers: ModifierKeys::default(),
        };
        let result = Tabs::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
    }

    #[test]
    fn active_tab_has_tabindex_0() {
        let props = test_props();
        let state = Tabs::initial_state(&props);
        let output = Tabs::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // First tab (active)
            assert!(
                elems[1]
                    .attrs
                    .contains(&("tabindex".to_string(), AttrValue::String("0".to_string())))
            );
            // Second tab (inactive)
            assert!(
                elems[2]
                    .attrs
                    .contains(&("tabindex".to_string(), AttrValue::String("-1".to_string())))
            );
        }
    }
}
