//! Collapsible primitive — Simple show/hide component.
//!
//! A simpler alternative to Disclosure that wraps content with
//! open/close state without explicit trigger/content slots.

use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};
use stratum_core::{
    AriaAttributes, AriaRole, Component, ComponentEvent, EventResult, Key, RenderOutput,
};

/// Props for the Collapsible primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollapsibleProps {
    /// Controlled open state.
    pub open: Option<bool>,
    /// Default open state for uncontrolled usage.
    pub default_open: bool,
    /// Whether the collapsible is disabled.
    pub disabled: bool,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the Collapsible primitive.
#[derive(Debug, Clone)]
pub struct CollapsibleState {
    /// Whether the content is visible.
    pub open: bool,
    /// Root element ID.
    pub id: String,
    /// ID for the content region.
    pub content_id: String,
}

/// Headless collapsible primitive.
pub struct Collapsible;

impl Component for Collapsible {
    type Props = CollapsibleProps;
    type State = CollapsibleState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::DISCLOSURE.group(&["root", "content"]);
        let id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let content_id = ids[1].clone();

        CollapsibleState {
            open: props.open.unwrap_or(props.default_open),
            id,
            content_id,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let effective_open = props.open.unwrap_or(state.open);

        let mut aria = AriaAttributes::new().with_expanded(effective_open);

        if props.disabled {
            aria = aria.with_disabled(true);
        }

        let mut content = RenderOutput::new()
            .with_tag("div")
            .with_attr("id", AttrValue::String(state.content_id.clone()))
            .with_aria(AriaAttributes::new().with_role(AriaRole::Region));

        if !effective_open {
            content = content.with_attr("hidden", AttrValue::Bool(true));
        }

        RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_data("state", if effective_open { "open" } else { "closed" })
            .with_data("disabled", props.disabled.to_string())
            .with_children(ChildrenSpec::Elements(vec![content]))
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
                let current = props.open.unwrap_or(state.open);
                let next = !current;
                if let Some(ref cb) = props.on_open_change {
                    cb.call(next);
                }
                if props.open.is_none() {
                    state.open = next;
                }
                EventResult::state_changed()
            }
            ComponentEvent::KeyDown {
                key: Key::Enter | Key::Space,
                ..
            } => {
                let current = props.open.unwrap_or(state.open);
                let next = !current;
                if let Some(ref cb) = props.on_open_change {
                    cb.call(next);
                }
                if props.open.is_none() {
                    state.open = next;
                }
                EventResult::prevent_and_changed()
            }
            _ => EventResult::default(),
        }
    }

    fn props_changed(
        _old_props: &Self::Props,
        new_props: &Self::Props,
        state: &mut Self::State,
    ) -> bool {
        if let Some(open) = new_props.open {
            if state.open != open {
                state.open = open;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use stratum_core::event::ModifierKeys;
    use stratum_core::event::MouseButton;

    fn default_props() -> CollapsibleProps {
        CollapsibleProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Collapsible::initial_state(&props);
        assert!(!state.open);
        assert!(!state.id.is_empty());
        assert!(!state.content_id.is_empty());
    }

    #[test]
    fn initial_state_custom_id() {
        let props = CollapsibleProps {
            id: Some("my-collapse".to_string()),
            ..default_props()
        };
        let state = Collapsible::initial_state(&props);
        assert_eq!(state.id, "my-collapse");
    }

    #[test]
    fn initial_state_default_open() {
        let props = CollapsibleProps {
            default_open: true,
            ..default_props()
        };
        let state = Collapsible::initial_state(&props);
        assert!(state.open);
    }

    #[test]
    fn initial_state_controlled() {
        let props = CollapsibleProps {
            open: Some(true),
            default_open: false,
            ..default_props()
        };
        let state = Collapsible::initial_state(&props);
        assert!(state.open);
    }

    #[test]
    fn render_expanded_attribute() {
        let props = CollapsibleProps {
            default_open: true,
            ..default_props()
        };
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        assert_eq!(output.aria.expanded, Some(true));
    }

    #[test]
    fn render_collapsed_attribute() {
        let props = default_props();
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        assert_eq!(output.aria.expanded, Some(false));
    }

    #[test]
    fn render_content_hidden_when_closed() {
        let props = default_props();
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(
                elems[0]
                    .attrs
                    .contains(&("hidden".to_string(), AttrValue::Bool(true)))
            );
        }
    }

    #[test]
    fn render_content_visible_when_open() {
        let props = CollapsibleProps {
            default_open: true,
            ..default_props()
        };
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(
                !elems[0]
                    .attrs
                    .contains(&("hidden".to_string(), AttrValue::Bool(true)))
            );
        }
    }

    #[test]
    fn render_content_region_role() {
        let props = default_props();
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.role, Some(AriaRole::Region));
        }
    }

    #[test]
    fn click_toggles_open() {
        let props = default_props();
        let mut state = Collapsible::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Collapsible::on_event(&props, &mut state, event.clone());
        assert!(state.open);
        Collapsible::on_event(&props, &mut state, event);
        assert!(!state.open);
    }

    #[test]
    fn enter_toggles_open() {
        let props = default_props();
        let mut state = Collapsible::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        let result = Collapsible::on_event(&props, &mut state, event);
        assert!(state.open);
        assert!(result.prevent_default);
    }

    #[test]
    fn space_toggles_open() {
        let props = default_props();
        let mut state = Collapsible::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Space,
            modifiers: ModifierKeys::default(),
        };
        Collapsible::on_event(&props, &mut state, event);
        assert!(state.open);
    }

    #[test]
    fn disabled_prevents_click() {
        let props = CollapsibleProps {
            disabled: true,
            ..default_props()
        };
        let mut state = Collapsible::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        let result = Collapsible::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
        assert!(!state.open);
    }

    #[test]
    fn disabled_prevents_keyboard() {
        let props = CollapsibleProps {
            disabled: true,
            ..default_props()
        };
        let mut state = Collapsible::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        let result = Collapsible::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
    }

    #[test]
    fn render_disabled() {
        let props = CollapsibleProps {
            disabled: true,
            ..default_props()
        };
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        assert_eq!(output.aria.disabled, Some(true));
    }

    #[test]
    fn callback_fires_on_toggle() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = CollapsibleProps {
            on_open_change: Some(Callback::new(move |open: bool| {
                if open {
                    called_clone.store(true, Ordering::SeqCst);
                }
            })),
            ..default_props()
        };
        let mut state = Collapsible::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        Collapsible::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn data_state_attribute() {
        let props = default_props();
        let state = Collapsible::initial_state(&props);
        let output = Collapsible::render(&props, &state);
        assert!(
            output
                .data_attrs
                .contains(&("state".to_string(), "closed".to_string()))
        );
    }
}
