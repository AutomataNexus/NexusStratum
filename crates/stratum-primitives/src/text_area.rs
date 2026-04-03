//! TextArea primitive — multi-line text input.
//!
//! Similar to TextInput but renders a `textarea` tag instead of `input`.
//! No `input_type` field since textareas don't have types.

use stratum_core::callback::Callback;
use stratum_core::event::{ComponentEvent, EventResult};
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, RenderOutput};
use stratum_core::{AriaAttributes, AriaRole, Component};

/// Props for the TextArea primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextAreaProps {
    /// Controlled value. When `Some`, the component is controlled.
    pub value: Option<String>,
    /// Default value for uncontrolled mode.
    pub default_value: String,
    /// Placeholder text.
    pub placeholder: Option<String>,
    /// Whether the textarea is disabled.
    pub disabled: bool,
    /// Whether the textarea is read-only.
    pub readonly: bool,
    /// Whether the textarea is required.
    pub required: bool,
    /// Number of visible text rows.
    pub rows: Option<u32>,
    /// Number of visible text columns.
    pub cols: Option<u32>,
    /// Callback invoked on change.
    pub on_change: Option<Callback<String>>,
    /// Callback invoked on each keystroke.
    pub on_input: Option<Callback<String>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Form field name.
    pub name: Option<String>,
    /// Accessible label.
    pub aria_label: Option<String>,
}

/// Internal state for the TextArea primitive.
#[derive(Debug, Clone)]
pub struct TextAreaState {
    /// Current value of the textarea.
    pub value: String,
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless textarea primitive.
pub struct TextArea;

impl TextArea {
    fn effective_value<'a>(props: &'a TextAreaProps, state: &'a TextAreaState) -> &'a str {
        props.value.as_deref().unwrap_or(&state.value)
    }
}

impl Component for TextArea {
    type Props = TextAreaProps;
    type State = TextAreaState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props.id.clone().unwrap_or_else(|| generators::INPUT.next());
        let value = props
            .value
            .clone()
            .unwrap_or_else(|| props.default_value.clone());
        TextAreaState { value, id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let value = Self::effective_value(props, state);

        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::TextBox)
            .with_disabled(props.disabled);

        if props.required {
            aria.required = Some(true);
        }
        if props.readonly {
            aria.readonly = Some(true);
        }
        if let Some(ref label) = props.aria_label {
            aria.label = Some(label.clone());
        }
        if let Some(ref ph) = props.placeholder {
            aria.placeholder = Some(ph.clone());
        }
        // Multi-line textbox
        aria.multiselectable = None; // not applicable but ensure clean

        let mut output = RenderOutput::new()
            .with_tag("textarea")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_attr("value", AttrValue::String(value.to_string()));

        if props.disabled {
            output = output.with_attr("disabled", AttrValue::Bool(true));
        }
        if props.readonly {
            output = output.with_attr("readonly", AttrValue::Bool(true));
        }
        if props.required {
            output = output.with_attr("required", AttrValue::Bool(true));
        }
        if let Some(ref placeholder) = props.placeholder {
            output = output.with_attr("placeholder", AttrValue::String(placeholder.clone()));
        }
        if let Some(ref name) = props.name {
            output = output.with_attr("name", AttrValue::String(name.clone()));
        }
        if let Some(rows) = props.rows {
            output = output.with_attr("rows", AttrValue::Number(rows as f64));
        }
        if let Some(cols) = props.cols {
            output = output.with_attr("cols", AttrValue::Number(cols as f64));
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
            ComponentEvent::Change { value } => {
                if let Some(ref cb) = props.on_change {
                    cb.call(value.clone());
                }
                if props.value.is_none() {
                    state.value = value;
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            ComponentEvent::Input { value } => {
                if let Some(ref cb) = props.on_input {
                    cb.call(value.clone());
                }
                if props.value.is_none() {
                    state.value = value;
                    return EventResult::state_changed();
                }
                EventResult::default()
            }
            _ => EventResult::default(),
        }
    }

    fn props_changed(
        _old_props: &Self::Props,
        new_props: &Self::Props,
        state: &mut Self::State,
    ) -> bool {
        if let Some(ref value) = new_props.value {
            if &state.value != value {
                state.value = value.clone();
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

    fn default_props() -> TextAreaProps {
        TextAreaProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = TextArea::initial_state(&props);
        assert_eq!(state.value, "");
        assert!(state.id.starts_with("stratum-input-"));
    }

    #[test]
    fn initial_state_with_default_value() {
        let props = TextAreaProps {
            default_value: "hello\nworld".to_string(),
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        assert_eq!(state.value, "hello\nworld");
    }

    #[test]
    fn initial_state_controlled() {
        let props = TextAreaProps {
            value: Some("controlled".to_string()),
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        assert_eq!(state.value, "controlled");
    }

    #[test]
    fn render_produces_textarea_tag() {
        let props = default_props();
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert_eq!(output.effective_tag(), "textarea");
        assert_eq!(output.aria.role, Some(AriaRole::TextBox));
    }

    #[test]
    fn render_disabled() {
        let props = TextAreaProps {
            disabled: true,
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert_eq!(output.aria.disabled, Some(true));
        assert!(
            output
                .attrs
                .contains(&("disabled".to_string(), AttrValue::Bool(true)))
        );
    }

    #[test]
    fn render_readonly() {
        let props = TextAreaProps {
            readonly: true,
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert_eq!(output.aria.readonly, Some(true));
    }

    #[test]
    fn render_required() {
        let props = TextAreaProps {
            required: true,
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert_eq!(output.aria.required, Some(true));
    }

    #[test]
    fn render_rows_and_cols() {
        let props = TextAreaProps {
            rows: Some(5),
            cols: Some(40),
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert!(
            output
                .attrs
                .contains(&("rows".to_string(), AttrValue::Number(5.0)))
        );
        assert!(
            output
                .attrs
                .contains(&("cols".to_string(), AttrValue::Number(40.0)))
        );
    }

    #[test]
    fn render_placeholder() {
        let props = TextAreaProps {
            placeholder: Some("Enter text here".to_string()),
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert!(output.attrs.contains(&(
            "placeholder".to_string(),
            AttrValue::String("Enter text here".to_string())
        )));
    }

    #[test]
    fn change_event_uncontrolled() {
        let props = default_props();
        let mut state = TextArea::initial_state(&props);

        let event = ComponentEvent::Change {
            value: "new content".to_string(),
        };
        let result = TextArea::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert_eq!(state.value, "new content");
    }

    #[test]
    fn input_event_uncontrolled() {
        let props = default_props();
        let mut state = TextArea::initial_state(&props);

        let event = ComponentEvent::Input {
            value: "typing...".to_string(),
        };
        let result = TextArea::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert_eq!(state.value, "typing...");
    }

    #[test]
    fn controlled_mode_no_state_change() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = Arc::clone(&received);
        let props = TextAreaProps {
            value: Some("fixed".to_string()),
            on_change: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = TextArea::initial_state(&props);

        let event = ComponentEvent::Change {
            value: "changed".to_string(),
        };
        let result = TextArea::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
        assert_eq!(state.value, "fixed");
        assert_eq!(*received.lock().unwrap(), Some("changed".to_string()));
    }

    #[test]
    fn disabled_prevents_events() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = TextAreaProps {
            disabled: true,
            on_change: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = TextArea::initial_state(&props);

        let event = ComponentEvent::Change {
            value: "nope".to_string(),
        };
        TextArea::on_event(&props, &mut state, event);
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old = TextAreaProps {
            value: Some("old".to_string()),
            ..default_props()
        };
        let new = TextAreaProps {
            value: Some("new".to_string()),
            ..default_props()
        };
        let mut state = TextArea::initial_state(&old);
        let changed = TextArea::props_changed(&old, &new, &mut state);
        assert!(changed);
        assert_eq!(state.value, "new");
    }

    #[test]
    fn aria_label() {
        let props = TextAreaProps {
            aria_label: Some("Description".to_string()),
            ..default_props()
        };
        let state = TextArea::initial_state(&props);
        let output = TextArea::render(&props, &state);
        assert_eq!(output.aria.label, Some("Description".to_string()));
    }
}
