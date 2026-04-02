//! Popover primitive — Anchored overlay component.
//!
//! Provides a headless popover that appears anchored to a trigger element,
//! with focus management and proper ARIA attributes.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};
use stratum_core::focus::FocusManager;
use stratum_core::aria::AriaHasPopup;

/// Props for the Popover primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PopoverProps {
    /// Controlled open state.
    pub open: Option<bool>,
    /// Default open state for uncontrolled usage.
    pub default_open: bool,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// If true, trap focus within the popover.
    pub modal: bool,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the Popover primitive.
#[derive(Debug, Clone)]
pub struct PopoverState {
    /// Whether the popover is open.
    pub open: bool,
    /// ID of the trigger element.
    pub trigger_id: String,
    /// ID of the popover content element.
    pub content_id: String,
    /// Focus manager for restore-on-close behavior.
    pub focus_manager: FocusManager,
}

/// Headless popover primitive.
pub struct Popover;

impl Component for Popover {
    type Props = PopoverProps;
    type State = PopoverState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::POPOVER.group(&["trigger", "content"]);
        let trigger_id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let content_id = ids[1].clone();

        let focus_manager = FocusManager::popover().with_container(content_id.clone());

        PopoverState {
            open: props.open.unwrap_or(props.default_open),
            trigger_id,
            content_id,
            focus_manager,
        }
    }

    fn render(_props: &Self::Props, state: &Self::State) -> RenderOutput {
        // Trigger element
        let trigger_aria = AriaAttributes::new()
            .with_expanded(state.open)
            .with_controls(&state.content_id)
            .with_haspopup(AriaHasPopup::Dialog);

        let trigger = RenderOutput::new()
            .with_tag("button")
            .with_aria(trigger_aria)
            .with_attr("id", AttrValue::String(state.trigger_id.clone()));

        // Content element
        let content_aria = AriaAttributes::new()
            .with_role(AriaRole::Dialog);

        let mut content = RenderOutput::new()
            .with_tag("div")
            .with_aria(content_aria)
            .with_attr("id", AttrValue::String(state.content_id.clone()));

        if !state.open {
            content = content.with_attr("hidden", AttrValue::Bool(true));
        }

        RenderOutput::new()
            .with_tag("div")
            .with_data("state", if state.open { "open" } else { "closed" })
            .with_children(ChildrenSpec::Elements(vec![trigger, content]))
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        match event {
            ComponentEvent::Click { .. } => {
                state.open = !state.open;
                if let Some(ref cb) = props.on_open_change {
                    cb.call(state.open);
                }
                EventResult::state_changed()
            }
            ComponentEvent::KeyDown { key: Key::Escape, .. } => {
                if state.open {
                    state.open = false;
                    if let Some(ref cb) = props.on_open_change {
                        cb.call(false);
                    }
                    return EventResult::prevent_and_changed();
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
        use stratum_core::event::ModifierKeys;
    use stratum_core::event::MouseButton;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn default_props() -> PopoverProps {
        PopoverProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Popover::initial_state(&props);
        assert!(!state.open);
        assert!(!state.trigger_id.is_empty());
        assert!(!state.content_id.is_empty());
    }

    #[test]
    fn initial_state_custom_id() {
        let props = PopoverProps {
            id: Some("my-pop".to_string()),
            ..default_props()
        };
        let state = Popover::initial_state(&props);
        assert_eq!(state.trigger_id, "my-pop");
    }

    #[test]
    fn render_trigger_aria_expanded() {
        let props = default_props();
        let state = Popover::initial_state(&props);
        let output = Popover::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.expanded, Some(false));
            assert_eq!(elems[0].aria.controls, Some(state.content_id.clone()));
            assert_eq!(elems[0].aria.haspopup, Some(AriaHasPopup::Dialog));
        } else {
            panic!("Expected Elements");
        }
    }

    #[test]
    fn render_content_dialog_role() {
        let props = default_props();
        let state = Popover::initial_state(&props);
        let output = Popover::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[1].aria.role, Some(AriaRole::Dialog));
        }
    }

    #[test]
    fn render_closed_content_hidden() {
        let props = default_props();
        let state = Popover::initial_state(&props);
        let output = Popover::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(elems[1].attrs.contains(&("hidden".to_string(), AttrValue::Bool(true))));
        }
    }

    #[test]
    fn click_toggles_open() {
        let props = default_props();
        let mut state = Popover::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        Popover::on_event(&props, &mut state, event.clone());
        assert!(state.open);
        Popover::on_event(&props, &mut state, event);
        assert!(!state.open);
    }

    #[test]
    fn escape_closes_popover() {
        let props = PopoverProps {
            default_open: true,
            ..default_props()
        };
        let mut state = Popover::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Popover::on_event(&props, &mut state, event);
        assert!(!state.open);
        assert!(result.prevent_default);
    }

    #[test]
    fn escape_when_closed_is_noop() {
        let props = default_props();
        let mut state = Popover::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Popover::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
    }

    #[test]
    fn focus_manager_restores() {
        let props = default_props();
        let state = Popover::initial_state(&props);
        assert!(state.focus_manager.should_restore());
    }

    #[test]
    fn callback_fires_on_toggle() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = PopoverProps {
            on_open_change: Some(Callback::new(move |_: bool| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Popover::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        Popover::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
    }
}
