//! Toggle primitive — pressable toggle button with on/off state.
//!
//! Renders with `role=button` and `aria-pressed` per WAI-ARIA.

use stratum_core::*;

/// Props for the Toggle primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToggleProps {
    /// Controlled pressed state. When `Some`, the component is controlled.
    pub pressed: Option<bool>,
    /// Default pressed state for uncontrolled mode.
    pub default_pressed: bool,
    /// Whether the toggle is disabled.
    pub disabled: bool,
    /// Callback invoked when the pressed state changes.
    pub on_pressed_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the Toggle primitive.
#[derive(Debug, Clone)]
pub struct ToggleState {
    /// Whether the toggle is currently pressed.
    pub pressed: bool,
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless toggle button primitive.
pub struct Toggle;

impl Toggle {
    fn effective_pressed(props: &ToggleProps, state: &ToggleState) -> bool {
        props.pressed.unwrap_or(state.pressed)
    }
}

impl Component for Toggle {
    type Props = ToggleProps;
    type State = ToggleState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| id::generators::TOGGLE.next());
        let pressed = props.pressed.unwrap_or(props.default_pressed);
        ToggleState { pressed, id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let is_pressed = Self::effective_pressed(props, state);
        let aria = AriaAttributes::new()
            .with_role(AriaRole::Button)
            .with_disabled(props.disabled);

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_attr("aria-pressed", AttrValue::String(is_pressed.to_string()))
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
                let next = !Self::effective_pressed(props, state);
                if let Some(ref cb) = props.on_pressed_change {
                    cb.call(next);
                }
                if props.pressed.is_none() {
                    state.pressed = next;
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown {
                key: Key::Enter | Key::Space,
                ..
            } => {
                let next = !Self::effective_pressed(props, state);
                if let Some(ref cb) = props.on_pressed_change {
                    cb.call(next);
                }
                if props.pressed.is_none() {
                    state.pressed = next;
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
        if let Some(controlled) = new_props.pressed {
            if state.pressed != controlled {
                state.pressed = controlled;
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

    fn default_props() -> ToggleProps {
        ToggleProps::default()
    }

    fn click_event() -> ComponentEvent {
        ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        }
    }

    fn key_event(key: Key) -> ComponentEvent {
        ComponentEvent::KeyDown {
            key,
            modifiers: ModifierKeys::default(),
        }
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Toggle::initial_state(&props);
        assert!(!state.pressed);
        assert!(state.id.starts_with("stratum-toggle-"));
    }

    #[test]
    fn initial_state_default_pressed() {
        let props = ToggleProps {
            default_pressed: true,
            ..default_props()
        };
        let state = Toggle::initial_state(&props);
        assert!(state.pressed);
    }

    #[test]
    fn initial_state_custom_id() {
        let props = ToggleProps {
            id: Some("my-toggle".to_string()),
            ..default_props()
        };
        let state = Toggle::initial_state(&props);
        assert_eq!(state.id, "my-toggle");
    }

    #[test]
    fn render_aria_attributes() {
        let props = default_props();
        let state = Toggle::initial_state(&props);
        let output = Toggle::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Button));
        assert_eq!(output.effective_tag(), "button");
        // Check aria-pressed attribute
        let pressed_attr = output
            .attrs
            .iter()
            .find(|(k, _)| k == "aria-pressed")
            .unwrap();
        assert_eq!(pressed_attr.1, AttrValue::String("false".to_string()));
    }

    #[test]
    fn render_pressed_state() {
        let props = ToggleProps {
            default_pressed: true,
            ..default_props()
        };
        let state = Toggle::initial_state(&props);
        let output = Toggle::render(&props, &state);
        let pressed_attr = output
            .attrs
            .iter()
            .find(|(k, _)| k == "aria-pressed")
            .unwrap();
        assert_eq!(pressed_attr.1, AttrValue::String("true".to_string()));
    }

    #[test]
    fn click_toggles_uncontrolled() {
        let props = default_props();
        let mut state = Toggle::initial_state(&props);
        assert!(!state.pressed);

        let result = Toggle::on_event(&props, &mut state, click_event());
        assert!(result.state_changed);
        assert!(state.pressed);

        let result = Toggle::on_event(&props, &mut state, click_event());
        assert!(result.state_changed);
        assert!(!state.pressed);
    }

    #[test]
    fn enter_toggles() {
        let props = default_props();
        let mut state = Toggle::initial_state(&props);
        let result = Toggle::on_event(&props, &mut state, key_event(Key::Enter));
        assert!(result.state_changed);
        assert!(result.prevent_default);
        assert!(state.pressed);
    }

    #[test]
    fn space_toggles() {
        let props = default_props();
        let mut state = Toggle::initial_state(&props);
        let result = Toggle::on_event(&props, &mut state, key_event(Key::Space));
        assert!(result.state_changed);
        assert!(state.pressed);
    }

    #[test]
    fn disabled_prevents_interaction() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();
        let props = ToggleProps {
            disabled: true,
            on_pressed_change: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Toggle::initial_state(&props);
        let result = Toggle::on_event(&props, &mut state, click_event());
        assert!(!result.state_changed);
        assert!(!state.pressed);
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn controlled_no_internal_state_change() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();
        let props = ToggleProps {
            pressed: Some(false),
            on_pressed_change: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = Toggle::initial_state(&props);
        let result = Toggle::on_event(&props, &mut state, click_event());
        assert!(!result.state_changed);
        assert!(!state.pressed);
        assert_eq!(*received.lock().unwrap(), Some(true));
    }

    #[test]
    fn on_pressed_change_receives_new_value() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();
        let props = ToggleProps {
            on_pressed_change: Some(Callback::new(move |val: bool| {
                *received_clone.lock().unwrap() = Some(val);
            })),
            ..default_props()
        };
        let mut state = Toggle::initial_state(&props);
        Toggle::on_event(&props, &mut state, click_event());
        assert_eq!(*received.lock().unwrap(), Some(true));
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old = ToggleProps {
            pressed: Some(false),
            ..default_props()
        };
        let new = ToggleProps {
            pressed: Some(true),
            ..default_props()
        };
        let mut state = Toggle::initial_state(&old);
        let changed = Toggle::props_changed(&old, &new, &mut state);
        assert!(changed);
        assert!(state.pressed);
    }

    #[test]
    fn unhandled_event_is_noop() {
        let props = default_props();
        let mut state = Toggle::initial_state(&props);
        let result = Toggle::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.state_changed);
    }
}
