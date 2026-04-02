//! Checkbox primitive — tri-state checkbox with controlled/uncontrolled modes.
//!
//! Supports checked, unchecked, and indeterminate (mixed) states per WAI-ARIA.

use stratum_core::*;

/// Props for the Checkbox primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CheckboxProps {
    /// Controlled checked state. When `Some`, the component is controlled.
    pub checked: Option<TriState>,
    /// Default checked state for uncontrolled mode.
    pub default_checked: bool,
    /// Whether the checkbox is disabled.
    pub disabled: bool,
    /// Whether the checkbox is required.
    pub required: bool,
    /// Callback invoked when the checked state changes.
    pub on_checked_change: Option<Callback<TriState>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Form field name.
    pub name: Option<String>,
}

/// Internal state for the Checkbox primitive.
#[derive(Debug, Clone)]
pub struct CheckboxState {
    /// Current checked state.
    pub checked: TriState,
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless checkbox primitive.
pub struct Checkbox;

impl Checkbox {
    /// Get the effective checked state, preferring controlled over internal.
    fn effective_checked(props: &CheckboxProps, state: &CheckboxState) -> TriState {
        props.checked.unwrap_or(state.checked)
    }
}

impl Component for Checkbox {
    type Props = CheckboxProps;
    type State = CheckboxState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| id::generators::CHECKBOX.next());
        let checked = props
            .checked
            .unwrap_or_else(|| TriState::from(props.default_checked));
        CheckboxState { checked, id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let checked = Self::effective_checked(props, state);

        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Checkbox)
            .with_checked(checked)
            .with_disabled(props.disabled);

        if props.required {
            aria.required = Some(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_attr("tabindex", AttrValue::String("0".to_string()));

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }

        if let Some(ref name) = props.name {
            output = output.with_attr("name", AttrValue::String(name.clone()));
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
                let current = Self::effective_checked(props, state);
                let next = current.toggle();
                if let Some(ref cb) = props.on_checked_change {
                    cb.call(next);
                }
                // Only update internal state in uncontrolled mode
                if props.checked.is_none() {
                    state.checked = next;
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown { key: Key::Space, .. } => {
                    let current = Self::effective_checked(props, state);
                    let next = current.toggle();
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
        // Sync controlled state
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
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicBool, Ordering};

    fn default_props() -> CheckboxProps {
        CheckboxProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Checkbox::initial_state(&props);
        assert_eq!(state.checked, TriState::False);
        assert!(state.id.starts_with("stratum-chk-"));
    }

    #[test]
    fn initial_state_default_checked() {
        let props = CheckboxProps {
            default_checked: true,
            ..default_props()
        };
        let state = Checkbox::initial_state(&props);
        assert_eq!(state.checked, TriState::True);
    }

    #[test]
    fn initial_state_controlled() {
        let props = CheckboxProps {
            checked: Some(TriState::Mixed),
            ..default_props()
        };
        let state = Checkbox::initial_state(&props);
        assert_eq!(state.checked, TriState::Mixed);
    }

    #[test]
    fn render_aria_attributes() {
        let props = default_props();
        let state = Checkbox::initial_state(&props);
        let output = Checkbox::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Checkbox));
        assert_eq!(output.aria.checked, Some(TriState::False));
        assert_eq!(output.aria.disabled, Some(false));
    }

    #[test]
    fn render_required() {
        let props = CheckboxProps {
            required: true,
            ..default_props()
        };
        let state = Checkbox::initial_state(&props);
        let output = Checkbox::render(&props, &state);
        assert_eq!(output.aria.required, Some(true));
    }

    #[test]
    fn click_toggles_uncontrolled() {
        let props = default_props();
        let mut state = Checkbox::initial_state(&props);
        assert_eq!(state.checked, TriState::False);

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        let result = Checkbox::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert_eq!(state.checked, TriState::True);
    }

    #[test]
    fn space_toggles_uncontrolled() {
        let props = default_props();
        let mut state = Checkbox::initial_state(&props);

        let event = ComponentEvent::KeyDown {
            key: Key::Space,
            modifiers: ModifierKeys::default(),
        };
        let result = Checkbox::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert!(result.prevent_default);
        assert_eq!(state.checked, TriState::True);
    }

    #[test]
    fn click_controlled_calls_callback_no_state_change() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = Arc::clone(&received);
        let props = CheckboxProps {
            checked: Some(TriState::False),
            on_checked_change: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = Checkbox::initial_state(&props);

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        let result = Checkbox::on_event(&props, &mut state, event);
        // Controlled mode: no internal state change
        assert!(!result.state_changed);
        assert_eq!(*received.lock().unwrap(), Some(TriState::True));
    }

    #[test]
    fn disabled_prevents_interaction() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = CheckboxProps {
            disabled: true,
            on_checked_change: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Checkbox::initial_state(&props);

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Checkbox::on_event(&props, &mut state, event);
        assert!(!called.load(Ordering::SeqCst));
        assert_eq!(state.checked, TriState::False);
    }

    #[test]
    fn mixed_toggles_to_true() {
        let props = CheckboxProps {
            checked: None,
            ..default_props()
        };
        let mut state = Checkbox::initial_state(&props);
        state.checked = TriState::Mixed;

        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Checkbox::on_event(&props, &mut state, event);
        assert_eq!(state.checked, TriState::True);
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old_props = CheckboxProps {
            checked: Some(TriState::False),
            ..default_props()
        };
        let new_props = CheckboxProps {
            checked: Some(TriState::True),
            ..default_props()
        };
        let mut state = Checkbox::initial_state(&old_props);
        let changed = Checkbox::props_changed(&old_props, &new_props, &mut state);
        assert!(changed);
        assert_eq!(state.checked, TriState::True);
    }
}
