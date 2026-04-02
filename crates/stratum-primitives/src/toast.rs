//! Toast primitive — Notification system component.
//!
//! Provides headless toast notifications with proper ARIA live region
//! attributes for screen reader announcements.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::AttrValue;
use stratum_core::AriaLive;

/// Toast variant determining urgency and ARIA role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
}

impl ToastVariant {
    /// Get the string representation of the variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

/// Props for the Toast primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct ToastProps {
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Duration in milliseconds. 0 = persistent.
    pub duration_ms: u32,
    /// Toast variant.
    pub variant: ToastVariant,
    /// Callback when the toast is closed.
    pub on_close: Option<Callback<()>>,
}

impl Default for ToastProps {
    fn default() -> Self {
        Self {
            id: None,
            duration_ms: 5000,
            variant: ToastVariant::Default,
            on_close: None,
        }
    }
}

/// Internal state for the Toast primitive.
#[derive(Debug, Clone)]
pub struct ToastState {
    /// Whether the toast is visible.
    pub visible: bool,
    /// Element ID.
    pub id: String,
}

/// Headless toast primitive.
pub struct Toast;

impl Component for Toast {
    type Props = ToastProps;
    type State = ToastState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props.id.clone().unwrap_or_else(|| generators::TOAST.next());
        ToastState { visible: true, id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let (role, live) = match props.variant {
            ToastVariant::Error => (AriaRole::Alert, AriaLive::Assertive),
            _ => (AriaRole::Status, AriaLive::Polite),
        };

        let mut aria = AriaAttributes::new()
            .with_role(role);
        aria.live = Some(live);

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_data("variant", props.variant.as_str())
            .with_data("state", if state.visible { "visible" } else { "hidden" });

        if props.duration_ms > 0 {
            output = output.with_data("duration", props.duration_ms.to_string());
        }

        if !state.visible {
            output = output.with_attr("hidden", AttrValue::Bool(true));
        }

        output
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        match event {
            ComponentEvent::KeyDown { key: Key::Escape, .. } => {
                if state.visible {
                    state.visible = false;
                    if let Some(ref cb) = props.on_close {
                        cb.call(());
                    }
                    return EventResult::prevent_and_changed();
                }
                EventResult::default()
            }
            // Custom event for timer-based dismissal
            ComponentEvent::Custom { ref name, .. } if name == "dismiss" => {
                if state.visible {
                    state.visible = false;
                    if let Some(ref cb) = props.on_close {
                        cb.call(());
                    }
                    return EventResult::state_changed();
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

    fn default_props() -> ToastProps {
        ToastProps::default()
    }

    #[test]
    fn initial_state_visible() {
        let props = default_props();
        let state = Toast::initial_state(&props);
        assert!(state.visible);
        assert!(!state.id.is_empty());
    }

    #[test]
    fn initial_state_custom_id() {
        let props = ToastProps {
            id: Some("my-toast".to_string()),
            ..default_props()
        };
        let state = Toast::initial_state(&props);
        assert_eq!(state.id, "my-toast");
    }

    #[test]
    fn default_duration_5000() {
        let props = default_props();
        assert_eq!(props.duration_ms, 5000);
    }

    #[test]
    fn render_status_role_for_default() {
        let props = default_props();
        let state = Toast::initial_state(&props);
        let output = Toast::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Status));
        assert_eq!(output.aria.live, Some(AriaLive::Polite));
    }

    #[test]
    fn render_alert_role_for_error() {
        let props = ToastProps {
            variant: ToastVariant::Error,
            ..default_props()
        };
        let state = Toast::initial_state(&props);
        let output = Toast::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Alert));
        assert_eq!(output.aria.live, Some(AriaLive::Assertive));
    }

    #[test]
    fn render_status_role_for_success() {
        let props = ToastProps {
            variant: ToastVariant::Success,
            ..default_props()
        };
        let state = Toast::initial_state(&props);
        let output = Toast::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Status));
        assert_eq!(output.aria.live, Some(AriaLive::Polite));
    }

    #[test]
    fn render_variant_data() {
        let props = ToastProps {
            variant: ToastVariant::Warning,
            ..default_props()
        };
        let state = Toast::initial_state(&props);
        let output = Toast::render(&props, &state);
        assert!(output.data_attrs.contains(&("variant".to_string(), "warning".to_string())));
    }

    #[test]
    fn render_duration_data() {
        let props = default_props();
        let state = Toast::initial_state(&props);
        let output = Toast::render(&props, &state);
        assert!(output.data_attrs.contains(&("duration".to_string(), "5000".to_string())));
    }

    #[test]
    fn render_persistent_no_duration() {
        let props = ToastProps {
            duration_ms: 0,
            ..default_props()
        };
        let state = Toast::initial_state(&props);
        let output = Toast::render(&props, &state);
        assert!(!output.data_attrs.iter().any(|(k, _)| k == "duration"));
    }

    #[test]
    fn escape_closes_toast() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = ToastProps {
            on_close: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = Toast::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Toast::on_event(&props, &mut state, event);
        assert!(!state.visible);
        assert!(result.state_changed);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn dismiss_event_closes_toast() {
        let props = default_props();
        let mut state = Toast::initial_state(&props);
        let event = ComponentEvent::Custom {
            name: "dismiss".to_string(),
            data: serde_json::Value::Null,
        };
        let result = Toast::on_event(&props, &mut state, event);
        assert!(!state.visible);
        assert!(result.state_changed);
    }

    #[test]
    fn render_hidden_when_not_visible() {
        let props = default_props();
        let mut state = Toast::initial_state(&props);
        state.visible = false;
        let output = Toast::render(&props, &state);
        assert!(output.attrs.contains(&("hidden".to_string(), AttrValue::Bool(true))));
        assert!(output.data_attrs.contains(&("state".to_string(), "hidden".to_string())));
    }

    #[test]
    fn toast_variant_as_str() {
        assert_eq!(ToastVariant::Default.as_str(), "default");
        assert_eq!(ToastVariant::Success.as_str(), "success");
        assert_eq!(ToastVariant::Error.as_str(), "error");
        assert_eq!(ToastVariant::Warning.as_str(), "warning");
        assert_eq!(ToastVariant::Info.as_str(), "info");
    }

    #[test]
    fn escape_on_hidden_is_noop() {
        let props = default_props();
        let mut state = Toast::initial_state(&props);
        state.visible = false;
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Toast::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
    }
}
