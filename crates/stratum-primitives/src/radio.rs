//! RadioGroup primitive — radio group with keyboard navigation.
//!
//! Implements ARIA APG radio group pattern with arrow key navigation
//! and automatic selection.

use stratum_a11y::keyboard::{KeyboardNav, NavStrategy};
use stratum_core::*;

/// Props for the RadioGroup primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Controlled selected value. When `Some`, the component is controlled.
    pub value: Option<String>,
    /// Default selected value for uncontrolled mode.
    pub default_value: Option<String>,
    /// Whether the radio group is disabled.
    pub disabled: bool,
    /// Orientation of the radio group (affects arrow key navigation).
    pub orientation: Orientation,
    /// Callback invoked when the selected value changes.
    pub on_change: Option<Callback<String>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Radio item values.
    pub items: Vec<String>,
}

impl Default for RadioGroupProps {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            disabled: false,
            orientation: Orientation::Vertical,
            on_change: None,
            id: None,
            items: Vec::new(),
        }
    }
}

/// Internal state for the RadioGroup primitive.
#[derive(Debug, Clone)]
pub struct RadioGroupState {
    /// Currently selected value.
    pub selected: Option<String>,
    /// Index of the currently focused radio item.
    pub focused_index: usize,
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless radio group primitive.
pub struct RadioGroup;

impl RadioGroup {
    /// Get the effective selected value, preferring controlled over internal.
    fn effective_selected<'a>(
        props: &'a RadioGroupProps,
        state: &'a RadioGroupState,
    ) -> &'a Option<String> {
        if props.value.is_some() {
            &props.value
        } else {
            &state.selected
        }
    }

    fn select(props: &RadioGroupProps, state: &mut RadioGroupState, value: String) -> EventResult {
        if props.value.is_none() {
            state.selected = Some(value.clone());
        }
        if let Some(ref cb) = props.on_change {
            cb.call(value);
        }
        EventResult::prevent_and_changed()
    }
}

impl Component for RadioGroup {
    type Props = RadioGroupProps;
    type State = RadioGroupState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| id::generators::RADIO.next());
        let selected = if props.value.is_some() {
            props.value.clone()
        } else {
            props.default_value.clone()
        };
        let focused_index = selected
            .as_ref()
            .and_then(|v| props.items.iter().position(|item| item == v))
            .unwrap_or(0);

        RadioGroupState {
            selected,
            focused_index,
            id,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let selected = Self::effective_selected(props, state);

        let aria = AriaAttributes::new()
            .with_role(AriaRole::RadioGroup)
            .with_orientation(props.orientation)
            .with_disabled(props.disabled);

        let mut children = Vec::new();
        for (i, item) in props.items.iter().enumerate() {
            let is_selected = selected.as_ref() == Some(item);
            let is_focused = i == state.focused_index;
            let item_aria = AriaAttributes::new()
                .with_role(AriaRole::Radio)
                .with_checked(if is_selected {
                    TriState::True
                } else {
                    TriState::False
                })
                .with_disabled(props.disabled);

            let child = RenderOutput::new()
                .with_attr("id", AttrValue::String(format!("{}-item-{}", state.id, i)))
                .with_attr(
                    "tabindex",
                    AttrValue::String(if is_focused { "0" } else { "-1" }.to_string()),
                )
                .with_data("value", item.clone())
                .with_aria(item_aria);
            children.push(child);
        }

        RenderOutput::new()
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_aria(aria)
            .with_children(ChildrenSpec::Elements(children))
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
            ComponentEvent::Click { .. } => {
                // Select the focused item on click
                if let Some(value) = props.items.get(state.focused_index) {
                    return Self::select(props, state, value.clone());
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown { key: Key::Space, .. } => {
                if let Some(value) = props.items.get(state.focused_index) {
                    return Self::select(props, state, value.clone());
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown { ref key, .. } => {
                let nav = KeyboardNav::new(props.orientation, NavStrategy::Arrow)
                    .with_loop(true);
                if let Some(new_index) = nav.handle(key, props.items.len(), state.focused_index) {
                    if new_index != state.focused_index {
                        state.focused_index = new_index;
                        // Per ARIA APG, arrow keys also select in radio groups
                        if let Some(value) = props.items.get(new_index) {
                            return Self::select(props, state, value.clone());
                        }
                    }
                }
                EventResult::default()
            }
            _ => EventResult::default(),
        }
    }

    fn props_changed(
        _old_props: &Self::Props,
        new_props: &Self::Props,
        state: &mut Self::State,
    ) -> bool {
        if let Some(ref controlled) = new_props.value {
            if state.selected.as_ref() != Some(controlled) {
                state.selected = Some(controlled.clone());
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn items() -> Vec<String> {
        vec!["a".into(), "b".into(), "c".into()]
    }

    fn default_props() -> RadioGroupProps {
        RadioGroupProps {
            items: items(),
            ..Default::default()
        }
    }

    fn key_event(key: Key) -> ComponentEvent {
        ComponentEvent::KeyDown {
            key,
            modifiers: ModifierKeys::default(),
        }
    }

    fn click_event() -> ComponentEvent {
        ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        }
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = RadioGroup::initial_state(&props);
        assert!(state.id.starts_with("stratum-radio-"));
        assert_eq!(state.selected, None);
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn initial_state_with_default_value() {
        let props = RadioGroupProps {
            default_value: Some("b".into()),
            ..default_props()
        };
        let state = RadioGroup::initial_state(&props);
        assert_eq!(state.selected, Some("b".into()));
        assert_eq!(state.focused_index, 1);
    }

    #[test]
    fn initial_state_controlled() {
        let props = RadioGroupProps {
            value: Some("c".into()),
            ..default_props()
        };
        let state = RadioGroup::initial_state(&props);
        assert_eq!(state.selected, Some("c".into()));
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn render_aria_attributes() {
        let props = default_props();
        let state = RadioGroup::initial_state(&props);
        let output = RadioGroup::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::RadioGroup));
        assert_eq!(output.aria.orientation, Some(Orientation::Vertical));
        assert_eq!(output.aria.disabled, Some(false));
    }

    #[test]
    fn render_children_have_radio_role() {
        let props = RadioGroupProps {
            default_value: Some("b".into()),
            ..default_props()
        };
        let state = RadioGroup::initial_state(&props);
        let output = RadioGroup::render(&props, &state);
        if let ChildrenSpec::Elements(children) = &output.children {
            assert_eq!(children.len(), 3);
            assert_eq!(children[0].aria.role, Some(AriaRole::Radio));
            assert_eq!(children[0].aria.checked, Some(TriState::False));
            assert_eq!(children[1].aria.role, Some(AriaRole::Radio));
            assert_eq!(children[1].aria.checked, Some(TriState::True));
            assert_eq!(children[2].aria.checked, Some(TriState::False));
        } else {
            panic!("Expected ChildrenSpec::Elements");
        }
    }

    #[test]
    fn space_selects_focused() {
        let received = Arc::new(Mutex::new(String::new()));
        let received_clone = received.clone();
        let props = RadioGroupProps {
            on_change: Some(Callback::new(move |val: String| {
                *received_clone.lock().unwrap() = val;
            })),
            ..default_props()
        };
        let mut state = RadioGroup::initial_state(&props);
        state.focused_index = 1;

        let result = RadioGroup::on_event(&props, &mut state, key_event(Key::Space));
        assert!(result.state_changed);
        assert_eq!(state.selected, Some("b".into()));
        assert_eq!(*received.lock().unwrap(), "b");
    }

    #[test]
    fn click_selects_focused() {
        let props = default_props();
        let mut state = RadioGroup::initial_state(&props);
        state.focused_index = 2;

        let result = RadioGroup::on_event(&props, &mut state, click_event());
        assert!(result.state_changed);
        assert_eq!(state.selected, Some("c".into()));
    }

    #[test]
    fn arrow_down_navigates_vertical() {
        let props = RadioGroupProps {
            orientation: Orientation::Vertical,
            ..default_props()
        };
        let mut state = RadioGroup::initial_state(&props);
        assert_eq!(state.focused_index, 0);

        RadioGroup::on_event(&props, &mut state, key_event(Key::ArrowDown));
        assert_eq!(state.focused_index, 1);
        assert_eq!(state.selected, Some("b".into()));
    }

    #[test]
    fn arrow_right_navigates_horizontal() {
        let props = RadioGroupProps {
            orientation: Orientation::Horizontal,
            ..default_props()
        };
        let mut state = RadioGroup::initial_state(&props);

        RadioGroup::on_event(&props, &mut state, key_event(Key::ArrowRight));
        assert_eq!(state.focused_index, 1);
        assert_eq!(state.selected, Some("b".into()));
    }

    #[test]
    fn arrow_wraps_around() {
        let props = default_props();
        let mut state = RadioGroup::initial_state(&props);
        state.focused_index = 2;

        RadioGroup::on_event(&props, &mut state, key_event(Key::ArrowDown));
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn disabled_prevents_interaction() {
        let props = RadioGroupProps {
            disabled: true,
            ..default_props()
        };
        let mut state = RadioGroup::initial_state(&props);
        let result = RadioGroup::on_event(&props, &mut state, key_event(Key::Space));
        assert!(!result.state_changed);
    }

    #[test]
    fn controlled_does_not_update_internal_state() {
        let received = Arc::new(Mutex::new(String::new()));
        let received_clone = received.clone();
        let props = RadioGroupProps {
            value: Some("a".into()),
            on_change: Some(Callback::new(move |val: String| {
                *received_clone.lock().unwrap() = val;
            })),
            ..default_props()
        };
        let mut state = RadioGroup::initial_state(&props);
        state.focused_index = 1;

        RadioGroup::on_event(&props, &mut state, key_event(Key::Space));
        // In controlled mode, internal selected should not change
        assert_eq!(state.selected, Some("a".into()));
        assert_eq!(*received.lock().unwrap(), "b");
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old = RadioGroupProps {
            value: Some("a".into()),
            ..default_props()
        };
        let new = RadioGroupProps {
            value: Some("c".into()),
            ..default_props()
        };
        let mut state = RadioGroup::initial_state(&old);
        let changed = RadioGroup::props_changed(&old, &new, &mut state);
        assert!(changed);
        assert_eq!(state.selected, Some("c".into()));
    }

    #[test]
    fn empty_items_no_interaction() {
        let props = RadioGroupProps {
            items: vec![],
            ..Default::default()
        };
        let mut state = RadioGroup::initial_state(&props);
        let result = RadioGroup::on_event(&props, &mut state, key_event(Key::Space));
        assert!(!result.state_changed);
    }
}
