//! Pressable primitive — base for Button, Link, MenuItem.
//!
//! Provides press interaction handling with support for mouse clicks,
//! keyboard activation (Enter/Space), and disabled state.

use stratum_core::*;

/// Props for the Pressable primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PressableProps {
    /// Whether the pressable element is disabled.
    pub disabled: bool,
    /// Callback invoked when a press completes (click or Enter/Space).
    pub on_press: Option<Callback<()>>,
    /// Callback invoked when a press starts (pointer down).
    pub on_press_start: Option<Callback<()>>,
    /// Callback invoked when a press ends (pointer up).
    pub on_press_end: Option<Callback<()>>,
    /// Optional explicit ID for the element.
    pub id: Option<String>,
}

/// Internal state for the Pressable primitive.
#[derive(Debug, Clone)]
pub struct PressableState {
    /// Whether the element is currently being pressed.
    pub pressed: bool,
    /// The generated or provided element ID.
    pub id: String,
}

/// Headless pressable primitive.
pub struct Pressable;

impl Component for Pressable {
    type Props = PressableProps;
    type State = PressableState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| id::generators::BUTTON.next());
        PressableState { pressed: false, id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let aria = AriaAttributes::new()
            .with_role(AriaRole::Button)
            .with_disabled(props.disabled);

        let mut output = RenderOutput::new()
            .with_tag("button")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()));

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }

        output = output.with_attr("tabindex", AttrValue::String("0".to_string()));
        output = output.with_data("pressed", state.pressed.to_string());

        output
    }

    fn on_event(
        props: &Self::Props,
        _state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        if props.disabled {
            return EventResult::default();
        }

        match event {
            ComponentEvent::Click { .. } => {
                if let Some(ref cb) = props.on_press {
                    cb.call(());
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown { key: Key::Enter | Key::Space, .. } => {
                    if let Some(ref cb) = props.on_press {
                        cb.call(());
                    }
                    EventResult {
                        prevent_default: true,
                        stop_propagation: false,
                        state_changed: false,
                    }
            }
            ComponentEvent::PointerEnter | ComponentEvent::PointerLeave => EventResult::default(),
            ComponentEvent::Focus | ComponentEvent::Blur => EventResult::default(),
            _ => EventResult::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn default_props() -> PressableProps {
        PressableProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Pressable::initial_state(&props);
        assert!(!state.pressed);
        assert!(state.id.starts_with("stratum-btn-"));
    }

    #[test]
    fn initial_state_custom_id() {
        let props = PressableProps {
            id: Some("my-btn".to_string()),
            ..default_props()
        };
        let state = Pressable::initial_state(&props);
        assert_eq!(state.id, "my-btn");
    }

    #[test]
    fn render_produces_correct_tag_and_role() {
        let props = default_props();
        let state = Pressable::initial_state(&props);
        let output = Pressable::render(&props, &state);
        assert_eq!(output.effective_tag(), "button");
        assert_eq!(output.aria.role, Some(AriaRole::Button));
    }

    #[test]
    fn render_disabled_state() {
        let props = PressableProps {
            disabled: true,
            ..default_props()
        };
        let state = Pressable::initial_state(&props);
        let output = Pressable::render(&props, &state);
        assert_eq!(output.aria.disabled, Some(true));
        assert!(output.attrs.contains(&("disabled".to_string(), AttrValue::Bool(true))));
    }

    #[test]
    fn render_has_tabindex() {
        let props = default_props();
        let state = Pressable::initial_state(&props);
        let output = Pressable::render(&props, &state);
        assert!(output
            .attrs
            .contains(&("tabindex".to_string(), AttrValue::String("0".to_string()))));
    }

    #[test]
    fn click_calls_on_press() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = PressableProps {
            on_press: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Pressable::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Pressable::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn enter_key_calls_on_press() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = PressableProps {
            on_press: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Pressable::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        let result = Pressable::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
        assert!(result.prevent_default);
    }

    #[test]
    fn space_key_calls_on_press() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = PressableProps {
            on_press: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Pressable::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Space,
            modifiers: ModifierKeys::default(),
        };
        let result = Pressable::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
        assert!(result.prevent_default);
    }

    #[test]
    fn disabled_prevents_click() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = PressableProps {
            disabled: true,
            on_press: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Pressable::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Pressable::on_event(&props, &mut state, event);
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn disabled_prevents_keyboard() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = PressableProps {
            disabled: true,
            on_press: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Pressable::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Pressable::on_event(&props, &mut state, event);
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn focus_blur_are_noop() {
        let props = default_props();
        let mut state = Pressable::initial_state(&props);
        let result = Pressable::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.state_changed);
        let result = Pressable::on_event(&props, &mut state, ComponentEvent::Blur);
        assert!(!result.state_changed);
    }
}
