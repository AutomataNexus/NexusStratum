//! Tooltip primitive — Hover/focus tooltip component.
//!
//! Provides a headless tooltip that appears on hover or focus,
//! with configurable delay and proper ARIA attributes.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};

/// Props for the Tooltip primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct TooltipProps {
    /// Controlled open state.
    pub open: Option<bool>,
    /// Default open state for uncontrolled usage.
    pub default_open: bool,
    /// Delay in milliseconds before showing the tooltip.
    pub delay_ms: u32,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            delay_ms: 700,
            on_open_change: None,
            id: None,
        }
    }
}

/// Internal state for the Tooltip primitive.
#[derive(Debug, Clone)]
pub struct TooltipState {
    /// Whether the tooltip is visible.
    pub open: bool,
    /// ID of the trigger element.
    pub trigger_id: String,
    /// ID of the tooltip content element.
    pub content_id: String,
}

/// Headless tooltip primitive.
pub struct Tooltip;

impl Component for Tooltip {
    type Props = TooltipProps;
    type State = TooltipState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::TOOLTIP.group(&["trigger", "content"]);
        let trigger_id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let content_id = ids[1].clone();

        TooltipState {
            open: props.open.unwrap_or(props.default_open),
            trigger_id,
            content_id,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        // Trigger element output with aria-describedby pointing to content
        let aria = AriaAttributes::new()
            .with_describedby(&state.content_id);

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.trigger_id.clone()));

        output = output.with_data("state", if state.open { "open" } else { "closed" });
        output = output.with_data("delay", props.delay_ms.to_string());

        // Tooltip content as a child element
        let content_aria = AriaAttributes::new()
            .with_role(AriaRole::ToolTip);

        let mut content = RenderOutput::new()
            .with_tag("div")
            .with_aria(content_aria)
            .with_attr("id", AttrValue::String(state.content_id.clone()));

        if !state.open {
            content = content.with_attr("hidden", AttrValue::Bool(true));
        }

        output = output.with_children(ChildrenSpec::Elements(vec![content]));

        output
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        match event {
            ComponentEvent::PointerEnter | ComponentEvent::Focus => {
                if !state.open {
                    state.open = true;
                    if let Some(ref cb) = props.on_open_change {
                        cb.call(true);
                    }
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            ComponentEvent::PointerLeave | ComponentEvent::Blur => {
                if state.open {
                    state.open = false;
                    if let Some(ref cb) = props.on_open_change {
                        cb.call(false);
                    }
                    return EventResult::state_changed();
                }
                EventResult::default()
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
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn default_props() -> TooltipProps {
        TooltipProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Tooltip::initial_state(&props);
        assert!(!state.open);
        assert!(!state.trigger_id.is_empty());
        assert!(!state.content_id.is_empty());
    }

    #[test]
    fn initial_state_custom_id() {
        let props = TooltipProps {
            id: Some("my-tip".to_string()),
            ..default_props()
        };
        let state = Tooltip::initial_state(&props);
        assert_eq!(state.trigger_id, "my-tip");
    }

    #[test]
    fn default_delay_is_700() {
        let props = default_props();
        assert_eq!(props.delay_ms, 700);
    }

    #[test]
    fn render_tooltip_role_on_content() {
        let props = default_props();
        let state = Tooltip::initial_state(&props);
        let output = Tooltip::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.role, Some(AriaRole::ToolTip));
        } else {
            panic!("Expected Elements children spec");
        }
    }

    #[test]
    fn render_describedby_on_trigger() {
        let props = default_props();
        let state = Tooltip::initial_state(&props);
        let output = Tooltip::render(&props, &state);
        assert_eq!(output.aria.describedby, Some(state.content_id.clone()));
    }

    #[test]
    fn render_closed_content_hidden() {
        let props = default_props();
        let state = Tooltip::initial_state(&props);
        let output = Tooltip::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(elems[0].attrs.contains(&("hidden".to_string(), AttrValue::Bool(true))));
        }
    }

    #[test]
    fn pointer_enter_opens_tooltip() {
        let props = default_props();
        let mut state = Tooltip::initial_state(&props);
        let result = Tooltip::on_event(&props, &mut state, ComponentEvent::PointerEnter);
        assert!(state.open);
        assert!(result.state_changed);
    }

    #[test]
    fn focus_opens_tooltip() {
        let props = default_props();
        let mut state = Tooltip::initial_state(&props);
        let result = Tooltip::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(state.open);
        assert!(result.state_changed);
    }

    #[test]
    fn pointer_leave_closes_tooltip() {
        let props = TooltipProps {
            default_open: true,
            ..default_props()
        };
        let mut state = Tooltip::initial_state(&props);
        let result = Tooltip::on_event(&props, &mut state, ComponentEvent::PointerLeave);
        assert!(!state.open);
        assert!(result.state_changed);
    }

    #[test]
    fn blur_closes_tooltip() {
        let props = TooltipProps {
            default_open: true,
            ..default_props()
        };
        let mut state = Tooltip::initial_state(&props);
        let result = Tooltip::on_event(&props, &mut state, ComponentEvent::Blur);
        assert!(!state.open);
        assert!(result.state_changed);
    }

    #[test]
    fn escape_closes_tooltip() {
        let props = TooltipProps {
            default_open: true,
            ..default_props()
        };
        let mut state = Tooltip::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Tooltip::on_event(&props, &mut state, event);
        assert!(!state.open);
        assert!(result.prevent_default);
    }

    #[test]
    fn open_callback_fires() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = TooltipProps {
            on_open_change: Some(Callback::new(move |open: bool| {
                if open {
                    called_clone.store(true, Ordering::SeqCst);
                }
            })),
            ..default_props()
        };
        let mut state = Tooltip::initial_state(&props);
        Tooltip::on_event(&props, &mut state, ComponentEvent::PointerEnter);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn already_open_pointer_enter_is_noop() {
        let props = TooltipProps {
            default_open: true,
            ..default_props()
        };
        let mut state = Tooltip::initial_state(&props);
        let result = Tooltip::on_event(&props, &mut state, ComponentEvent::PointerEnter);
        assert!(!result.state_changed);
    }

    #[test]
    fn data_delay_attribute() {
        let props = TooltipProps {
            delay_ms: 500,
            ..default_props()
        };
        let state = Tooltip::initial_state(&props);
        let output = Tooltip::render(&props, &state);
        assert!(output.data_attrs.contains(&("delay".to_string(), "500".to_string())));
    }
}
