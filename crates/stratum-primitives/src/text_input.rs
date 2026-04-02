//! TextInput primitive — single-line text input.
//!
//! Renders with `role=textbox` and `tag=input`, supporting controlled/uncontrolled
//! modes, various input types, and proper ARIA attributes.

use stratum_core::callback::Callback;
use stratum_core::event::{ComponentEvent, EventResult};
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, RenderOutput};
use stratum_core::{AriaAttributes, AriaRole, Component};

/// The type of text input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextInputType {
    Text,
    Password,
    Email,
    Url,
    Tel,
    Search,
    Number,
}

impl TextInputType {
    /// HTML input type attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Url => "url",
            Self::Tel => "tel",
            Self::Search => "search",
            Self::Number => "number",
        }
    }
}

/// Props for the TextInput primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct TextInputProps {
    /// Controlled value. When `Some`, the component is controlled.
    pub value: Option<String>,
    /// Default value for uncontrolled mode.
    pub default_value: String,
    /// Placeholder text.
    pub placeholder: Option<String>,
    /// Whether the input is disabled.
    pub disabled: bool,
    /// Whether the input is read-only.
    pub readonly: bool,
    /// Whether the input is required.
    pub required: bool,
    /// The type of input.
    pub input_type: TextInputType,
    /// Callback invoked on change (blur or enter).
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

impl Default for TextInputProps {
    fn default() -> Self {
        Self {
            value: None,
            default_value: String::new(),
            placeholder: None,
            disabled: false,
            readonly: false,
            required: false,
            input_type: TextInputType::Text,
            on_change: None,
            on_input: None,
            id: None,
            name: None,
            aria_label: None,
        }
    }
}

/// Internal state for the TextInput primitive.
#[derive(Debug, Clone)]
pub struct TextInputState {
    /// Current value of the input.
    pub value: String,
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless text input primitive.
pub struct TextInput;

impl TextInput {
    fn effective_value<'a>(props: &'a TextInputProps, state: &'a TextInputState) -> &'a str {
        props.value.as_deref().unwrap_or(&state.value)
    }
}

impl Component for TextInput {
    type Props = TextInputProps;
    type State = TextInputState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| generators::INPUT.next());
        let value = props
            .value
            .clone()
            .unwrap_or_else(|| props.default_value.clone());
        TextInputState { value, id }
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

        let mut output = RenderOutput::new()
            .with_tag("input")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_attr("type", AttrValue::String(props.input_type.as_str().to_string()))
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
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicBool, Ordering};

    fn default_props() -> TextInputProps {
        TextInputProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = TextInput::initial_state(&props);
        assert_eq!(state.value, "");
        assert!(state.id.starts_with("stratum-input-"));
    }

    #[test]
    fn initial_state_with_default_value() {
        let props = TextInputProps {
            default_value: "hello".to_string(),
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        assert_eq!(state.value, "hello");
    }

    #[test]
    fn initial_state_controlled() {
        let props = TextInputProps {
            value: Some("controlled".to_string()),
            default_value: "ignored".to_string(),
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        assert_eq!(state.value, "controlled");
    }

    #[test]
    fn render_produces_input_tag() {
        let props = default_props();
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert_eq!(output.effective_tag(), "input");
        assert_eq!(output.aria.role, Some(AriaRole::TextBox));
    }

    #[test]
    fn render_input_type() {
        let props = TextInputProps {
            input_type: TextInputType::Password,
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert!(output
            .attrs
            .contains(&("type".to_string(), AttrValue::String("password".to_string()))));
    }

    #[test]
    fn render_disabled() {
        let props = TextInputProps {
            disabled: true,
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert_eq!(output.aria.disabled, Some(true));
        assert!(output.attrs.contains(&("disabled".to_string(), AttrValue::Bool(true))));
    }

    #[test]
    fn render_readonly() {
        let props = TextInputProps {
            readonly: true,
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert_eq!(output.aria.readonly, Some(true));
        assert!(output.attrs.contains(&("readonly".to_string(), AttrValue::Bool(true))));
    }

    #[test]
    fn render_required() {
        let props = TextInputProps {
            required: true,
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert_eq!(output.aria.required, Some(true));
        assert!(output.attrs.contains(&("required".to_string(), AttrValue::Bool(true))));
    }

    #[test]
    fn render_placeholder() {
        let props = TextInputProps {
            placeholder: Some("Enter text".to_string()),
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert!(output.attrs.contains(&(
            "placeholder".to_string(),
            AttrValue::String("Enter text".to_string())
        )));
    }

    #[test]
    fn render_aria_label() {
        let props = TextInputProps {
            aria_label: Some("Username".to_string()),
            ..default_props()
        };
        let state = TextInput::initial_state(&props);
        let output = TextInput::render(&props, &state);
        assert_eq!(output.aria.label, Some("Username".to_string()));
    }

    #[test]
    fn change_event_uncontrolled() {
        let props = default_props();
        let mut state = TextInput::initial_state(&props);

        let event = ComponentEvent::Change {
            value: "new value".to_string(),
        };
        let result = TextInput::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert_eq!(state.value, "new value");
    }

    #[test]
    fn input_event_uncontrolled() {
        let props = default_props();
        let mut state = TextInput::initial_state(&props);

        let event = ComponentEvent::Input {
            value: "typing".to_string(),
        };
        let result = TextInput::on_event(&props, &mut state, event);
        assert!(result.state_changed);
        assert_eq!(state.value, "typing");
    }

    #[test]
    fn change_event_controlled() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = Arc::clone(&received);
        let props = TextInputProps {
            value: Some("fixed".to_string()),
            on_change: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = TextInput::initial_state(&props);

        let event = ComponentEvent::Change {
            value: "new".to_string(),
        };
        let result = TextInput::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
        assert_eq!(*received.lock().unwrap(), Some("new".to_string()));
        // Internal state unchanged in controlled mode
        assert_eq!(state.value, "fixed");
    }

    #[test]
    fn input_event_calls_on_input() {
        let received = Arc::new(Mutex::new(None));
        let received_clone = Arc::clone(&received);
        let props = TextInputProps {
            on_input: Some(Callback::new(move |v| {
                *received_clone.lock().unwrap() = Some(v);
            })),
            ..default_props()
        };
        let mut state = TextInput::initial_state(&props);

        let event = ComponentEvent::Input {
            value: "k".to_string(),
        };
        TextInput::on_event(&props, &mut state, event);
        assert_eq!(*received.lock().unwrap(), Some("k".to_string()));
    }

    #[test]
    fn disabled_prevents_change() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = TextInputProps {
            disabled: true,
            on_change: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_props()
        };
        let mut state = TextInput::initial_state(&props);

        let event = ComponentEvent::Change {
            value: "nope".to_string(),
        };
        TextInput::on_event(&props, &mut state, event);
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn props_changed_syncs_controlled() {
        let old = TextInputProps {
            value: Some("old".to_string()),
            ..default_props()
        };
        let new = TextInputProps {
            value: Some("new".to_string()),
            ..default_props()
        };
        let mut state = TextInput::initial_state(&old);
        let changed = TextInput::props_changed(&old, &new, &mut state);
        assert!(changed);
        assert_eq!(state.value, "new");
    }

    #[test]
    fn all_input_types() {
        let types = [
            (TextInputType::Text, "text"),
            (TextInputType::Password, "password"),
            (TextInputType::Email, "email"),
            (TextInputType::Url, "url"),
            (TextInputType::Tel, "tel"),
            (TextInputType::Search, "search"),
            (TextInputType::Number, "number"),
        ];
        for (t, expected) in types {
            assert_eq!(t.as_str(), expected);
        }
    }
}
