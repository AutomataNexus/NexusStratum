//! Switch primitive — binary toggle with controlled/uncontrolled modes.
//!
//! Renders with `role=switch` and `aria-checked` per WAI-ARIA.

use stratum_core::*;

/// Props for the Switch primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SwitchProps {
    /// Controlled checked state. When `Some`, the component is controlled.
    pub checked: Option<bool>,
    /// Default checked state for uncontrolled mode.
    pub default_checked: bool,
    /// Whether the switch is disabled.
    pub disabled: bool,
    /// Callback invoked when the checked state changes.
    pub on_checked_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the Switch primitive.
#[derive(Debug, Clone)]
pub struct SwitchState {
    /// Current checked state.
    pub checked: bool,
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless switch primitive.
pub struct Switch;

impl Switch {
    fn effective_checked(props: &SwitchProps, state: &SwitchState) -> bool {
        props.checked.unwrap_or(state.checked)
    }
}

impl Component for Switch {
    type Props = SwitchProps;
    type State = SwitchState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| id::generators::SWITCH.next());
        let checked = props.checked.unwrap_or(props.default_checked);
        SwitchState { checked, id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let checked = Self::effective_checked(props, state);
        let aria = AriaAttributes::new()
            .with_role(AriaRole::Switch)
            .with_checked(TriState::from(checked))
            .with_disabled(props.disabled);

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_attr("tabindex", AttrValue::String("0".to_string()));

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }

        output
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        if props.disabled {
            return EventResult::default();
        }

        match event {
            ComponentEvent::Click { .. } => {
                let next = !Self::effective_checked(props, state);
                if let Some(ref cb) = props.on_checked_change {
                    cb.call(next);
                }
                if props.checked.is_none() {
                    state.checked = next;
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown {
                key: Key::Enter | Key::Space,
                ..
            } => {
                let next = !Self::effective_checked(props, state);
                if let Some(ref cb) = props.on_checked_change {
                    cb.call(next);
                }
                if props.checked.is_none() {
                    state.checked = next;
                    return EventResult::prevent_and_changed();
                }
                EventResult {
                    prevent_default: true,
                    stop_propagation: false,
                    state_changed: false,
                }
            }
            _ => EventResult::default(),
        }
    }

    fn props_changed(
        _old_props: &Self::Props,
        new_props: &Self::Props,
        state: &mut Self::State,
    ) -> bool {
        if let Some(checked) = new_props.checked {
            if state.checked != checked {
                state.checked = checked;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    fn default_props() -> SwitchProps {
        SwitchProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Switch::initial_state(&props);
        assert!(!state.checked);
        assert!(state.id.starts_with("stratum-switch-"));
    }

    #[test]
    fn initial_state_default_checked() {
        let props = SwitchProps {
            default_checked: true,
            ..default_props()
        };
        let state = Switch::initial_state(&props);
        assert!(state.checked);
    }

    #[test]
    fn render_aria_attributes() {
        let props = default_props();
        let state = Switch::initial_state(&props);
        let output = Switch::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Switch));
        assert_eq!(output.aria.checked, Some(TriState::False));
    }

    #[test]
    fn render_checked_aria() {
        let props = SwitchProps {
            default_checked: true,
            ..default_props()
        };
        let state = Switch::initial_state(&props);
        let output = Switch::render(&props, &state);
        assert_eq!(output.aria.checked, Some(TriState::True));
    }

    #[test]
    fn click_toggles_uncontrolled() {
        let props = default_props();
        let mut state = Switch::initial_state(&props);
        assert!(!state.checked);

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        let result = Switch::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert!(state.checked);
    }

    #[test]
    fn enter_toggles() {
        let props = default_props();
        let mut state = Switch::initial_state(&props);

        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        let result = Switch::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert!(result.prevent_default);
        assert!(state.checked);
    }

    #[test]
    fn space_toggles() {
        let props = default_props();
        let mut state = Switch::initial_state(&props);

        let event = ComponentEvent::KeyDown {
            key: Key::Space,
            modifiers: ModifierKeys::default(),
        };
        let result = Switch::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert!(state.checked);
    }

    #[test]
    fn controlled_mode_no_internal_state_change() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = Arc::clone(&received);
        let props = SwitchProps {
            checked: Some(false),
            on_checked_change: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = Switch::initial_state(&props);

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        let result = Switch::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
        assert_eq!(*received.lock().unwrap(), Some(true));
    }

    #[test]
    fn disabled_prevents_interaction() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = SwitchProps {
            disabled: true,
            on_checked_change: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Switch::initial_state(&props);

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Switch::on_event(&props, &mut state, event);
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old = SwitchProps {
            checked: Some(false),
            ..default_props()
        };
        let new = SwitchProps {
            checked: Some(true),
            ..default_props()
        };
        let mut state = Switch::initial_state(&old);
        let changed = Switch::props_changed(&old, &new, &mut state);
        assert!(changed);
        assert!(state.checked);
    }
}
