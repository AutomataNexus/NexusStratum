//! Disclosure primitive — show/hide content, base for Accordion.
//!
//! Provides a trigger that toggles visibility of associated content.
//! Implements ARIA disclosure pattern with `aria-expanded` and `aria-controls`.

use stratum_core::*;

/// Props for the Disclosure primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DisclosureProps {
    /// Controlled open state. When `Some`, the component is controlled.
    pub open: Option<bool>,
    /// Default open state for uncontrolled mode.
    pub default_open: bool,
    /// Whether the disclosure is disabled.
    pub disabled: bool,
    /// Callback invoked when the open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the Disclosure primitive.
#[derive(Debug, Clone)]
pub struct DisclosureState {
    /// Whether the content is open/visible.
    pub open: bool,
    /// ID of the trigger element.
    pub trigger_id: String,
    /// ID of the content element.
    pub content_id: String,
}

/// Headless disclosure primitive.
pub struct Disclosure;

impl Disclosure {
    fn effective_open(props: &DisclosureProps, state: &DisclosureState) -> bool {
        props.open.unwrap_or(state.open)
    }
}

impl Component for Disclosure {
    type Props = DisclosureProps;
    type State = DisclosureState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let open = props.open.unwrap_or(props.default_open);

        let (trigger_id, content_id) = if let Some(ref id) = props.id {
            (format!("{}-trigger", id), format!("{}-content", id))
        } else {
            let ids = id::generators::DISCLOSURE.group(&["trigger", "content"]);
            (ids[0].clone(), ids[1].clone())
        };

        DisclosureState {
            open,
            trigger_id,
            content_id,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let is_open = Self::effective_open(props, state);

        let trigger_aria = AriaAttributes::new()
            .with_role(AriaRole::Button)
            .with_expanded(is_open)
            .with_controls(state.content_id.clone())
            .with_disabled(props.disabled);

        let trigger = RenderOutput::new()
            .with_tag("button")
            .with_attr("id", AttrValue::String(state.trigger_id.clone()))
            .with_attr("tabindex", AttrValue::String("0".to_string()))
            .with_aria(trigger_aria);

        let mut content_aria = AriaAttributes::new();
        if !is_open {
            content_aria.hidden = Some(true);
        }

        let content = RenderOutput::new()
            .with_attr("id", AttrValue::String(state.content_id.clone()))
            .with_aria(content_aria);

        RenderOutput::new()
            .with_children(ChildrenSpec::Elements(vec![trigger, content]))
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
                let next = !Self::effective_open(props, state);
                if let Some(ref cb) = props.on_open_change {
                    cb.call(next);
                }
                if props.open.is_none() {
                    state.open = next;
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            ComponentEvent::KeyDown { key: Key::Enter | Key::Space, .. } => {
                    let next = !Self::effective_open(props, state);
                    if let Some(ref cb) = props.on_open_change {
                        cb.call(next);
                    }
                    if props.open.is_none() {
                        state.open = next;
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
        if let Some(controlled) = new_props.open {
            if state.open != controlled {
                state.open = controlled;
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

    fn default_props() -> DisclosureProps {
        DisclosureProps::default()
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
        let state = Disclosure::initial_state(&props);
        assert!(!state.open);
        assert!(state.trigger_id.contains("trigger"));
        assert!(state.content_id.contains("content"));
    }

    #[test]
    fn initial_state_custom_id() {
        let props = DisclosureProps {
            id: Some("faq".into()),
            ..default_props()
        };
        let state = Disclosure::initial_state(&props);
        assert_eq!(state.trigger_id, "faq-trigger");
        assert_eq!(state.content_id, "faq-content");
    }

    #[test]
    fn initial_state_default_open() {
        let props = DisclosureProps {
            default_open: true,
            ..default_props()
        };
        let state = Disclosure::initial_state(&props);
        assert!(state.open);
    }

    #[test]
    fn render_closed() {
        let props = default_props();
        let state = Disclosure::initial_state(&props);
        let output = Disclosure::render(&props, &state);

        if let ChildrenSpec::Elements(children) = &output.children {
            assert_eq!(children.len(), 2);
            // Trigger
            assert_eq!(children[0].aria.role, Some(AriaRole::Button));
            assert_eq!(children[0].aria.expanded, Some(false));
            assert_eq!(children[0].aria.controls, Some(state.content_id.clone()));
            // Content hidden
            assert_eq!(children[1].aria.hidden, Some(true));
        } else {
            panic!("Expected ChildrenSpec::Elements");
        }
    }

    #[test]
    fn render_open() {
        let props = DisclosureProps {
            default_open: true,
            ..default_props()
        };
        let state = Disclosure::initial_state(&props);
        let output = Disclosure::render(&props, &state);

        if let ChildrenSpec::Elements(children) = &output.children {
            assert_eq!(children[0].aria.expanded, Some(true));
            assert_eq!(children[1].aria.hidden, None);
        } else {
            panic!("Expected ChildrenSpec::Elements");
        }
    }

    #[test]
    fn click_toggles_uncontrolled() {
        let props = default_props();
        let mut state = Disclosure::initial_state(&props);
        assert!(!state.open);

        let result = Disclosure::on_event(&props, &mut state, click_event());
        assert!(result.state_changed);
        assert!(state.open);

        let result = Disclosure::on_event(&props, &mut state, click_event());
        assert!(result.state_changed);
        assert!(!state.open);
    }

    #[test]
    fn enter_toggles() {
        let props = default_props();
        let mut state = Disclosure::initial_state(&props);
        let result = Disclosure::on_event(&props, &mut state, key_event(Key::Enter));
        assert!(result.state_changed);
        assert!(result.prevent_default);
        assert!(state.open);
    }

    #[test]
    fn space_toggles() {
        let props = default_props();
        let mut state = Disclosure::initial_state(&props);
        let result = Disclosure::on_event(&props, &mut state, key_event(Key::Space));
        assert!(result.state_changed);
        assert!(state.open);
    }

    #[test]
    fn disabled_prevents_interaction() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();
        let props = DisclosureProps {
            disabled: true,
            on_open_change: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Disclosure::initial_state(&props);
        Disclosure::on_event(&props, &mut state, click_event());
        assert!(!called.load(Ordering::SeqCst));
        assert!(!state.open);
    }

    #[test]
    fn controlled_no_internal_state_change() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();
        let props = DisclosureProps {
            open: Some(false),
            on_open_change: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = Disclosure::initial_state(&props);
        let result = Disclosure::on_event(&props, &mut state, click_event());
        assert!(!result.state_changed);
        assert!(!state.open);
        assert_eq!(*received.lock().unwrap(), Some(true));
    }

    #[test]
    fn on_open_change_receives_new_value() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();
        let props = DisclosureProps {
            on_open_change: Some(Callback::new(move |val: bool| {
                *received_clone.lock().unwrap() = Some(val);
            })),
            ..default_props()
        };
        let mut state = Disclosure::initial_state(&props);
        Disclosure::on_event(&props, &mut state, click_event());
        assert_eq!(*received.lock().unwrap(), Some(true));
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old = DisclosureProps {
            open: Some(false),
            ..default_props()
        };
        let new = DisclosureProps {
            open: Some(true),
            ..default_props()
        };
        let mut state = Disclosure::initial_state(&old);
        let changed = Disclosure::props_changed(&old, &new, &mut state);
        assert!(changed);
        assert!(state.open);
    }
}
