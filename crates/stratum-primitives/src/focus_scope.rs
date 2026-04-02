//! FocusScope primitive — focus trap container.
//!
//! When `trapped=true`, Tab/Shift+Tab events are intercepted to keep
//! focus cycling within the container. Framework adapters use this to
//! implement actual DOM focus management.

use stratum_core::*;

/// Props for the FocusScope primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FocusScopeProps {
    /// Whether focus is trapped within this container.
    pub trapped: bool,
    /// Whether Tab/Shift+Tab cycles (loops) within the container.
    pub loop_focus: bool,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the FocusScope primitive.
#[derive(Debug, Clone)]
pub struct FocusScopeState {
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless focus scope primitive.
pub struct FocusScope;

impl Component for FocusScope {
    type Props = FocusScopeProps;
    type State = FocusScopeState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| id::generators::FOCUS_SCOPE.next());
        FocusScopeState { id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let mut output = RenderOutput::new()
            .with_attr("id", AttrValue::String(state.id.clone()));

        if props.trapped {
            output = output.with_data("focus-trap", "true");
        }
        if props.loop_focus {
            output = output.with_data("focus-loop", "true");
        }

        output
    }

    fn on_event(
        props: &Self::Props,
        _state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        if !props.trapped {
            return EventResult::default();
        }

        // When trapped, intercept Tab/Shift+Tab to prevent focus from leaving
        match event {
            ComponentEvent::KeyDown {
                key: Key::Tab, ..
            } => {
                // Signal framework adapter to cycle focus within scope
                EventResult {
                    prevent_default: true,
                    stop_propagation: true,
                    state_changed: false,
                }
            }
            _ => EventResult::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_props() -> FocusScopeProps {
        FocusScopeProps::default()
    }

    fn tab_event() -> ComponentEvent {
        ComponentEvent::KeyDown {
            key: Key::Tab,
            modifiers: ModifierKeys::default(),
        }
    }

    fn shift_tab_event() -> ComponentEvent {
        ComponentEvent::KeyDown {
            key: Key::Tab,
            modifiers: ModifierKeys {
                shift: true,
                ..Default::default()
            },
        }
    }

    #[test]
    fn initial_state_generates_id() {
        let props = default_props();
        let state = FocusScope::initial_state(&props);
        assert!(state.id.starts_with("stratum-focus-scope-"));
    }

    #[test]
    fn initial_state_custom_id() {
        let props = FocusScopeProps {
            id: Some("dialog-scope".into()),
            ..default_props()
        };
        let state = FocusScope::initial_state(&props);
        assert_eq!(state.id, "dialog-scope");
    }

    #[test]
    fn render_not_trapped() {
        let props = default_props();
        let state = FocusScope::initial_state(&props);
        let output = FocusScope::render(&props, &state);
        assert!(!output
            .data_attrs
            .iter()
            .any(|(k, _)| k == "focus-trap"));
    }

    #[test]
    fn render_trapped() {
        let props = FocusScopeProps {
            trapped: true,
            ..default_props()
        };
        let state = FocusScope::initial_state(&props);
        let output = FocusScope::render(&props, &state);
        assert!(output
            .data_attrs
            .contains(&("focus-trap".to_string(), "true".to_string())));
    }

    #[test]
    fn render_loop_focus() {
        let props = FocusScopeProps {
            trapped: true,
            loop_focus: true,
            ..default_props()
        };
        let state = FocusScope::initial_state(&props);
        let output = FocusScope::render(&props, &state);
        assert!(output
            .data_attrs
            .contains(&("focus-loop".to_string(), "true".to_string())));
    }

    #[test]
    fn not_trapped_allows_tab() {
        let props = default_props();
        let mut state = FocusScope::initial_state(&props);
        let result = FocusScope::on_event(&props, &mut state, tab_event());
        assert!(!result.prevent_default);
        assert!(!result.stop_propagation);
    }

    #[test]
    fn trapped_intercepts_tab() {
        let props = FocusScopeProps {
            trapped: true,
            ..default_props()
        };
        let mut state = FocusScope::initial_state(&props);
        let result = FocusScope::on_event(&props, &mut state, tab_event());
        assert!(result.prevent_default);
        assert!(result.stop_propagation);
    }

    #[test]
    fn trapped_intercepts_shift_tab() {
        let props = FocusScopeProps {
            trapped: true,
            ..default_props()
        };
        let mut state = FocusScope::initial_state(&props);
        let result = FocusScope::on_event(&props, &mut state, shift_tab_event());
        assert!(result.prevent_default);
        assert!(result.stop_propagation);
    }

    #[test]
    fn trapped_ignores_other_keys() {
        let props = FocusScopeProps {
            trapped: true,
            ..default_props()
        };
        let mut state = FocusScope::initial_state(&props);
        let result = FocusScope::on_event(
            &props,
            &mut state,
            ComponentEvent::KeyDown {
                key: Key::Enter,
                modifiers: ModifierKeys::default(),
            },
        );
        assert!(!result.prevent_default);
    }

    #[test]
    fn non_keyboard_events_ignored() {
        let props = FocusScopeProps {
            trapped: true,
            ..default_props()
        };
        let mut state = FocusScope::initial_state(&props);
        let result = FocusScope::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.prevent_default);
    }
}
