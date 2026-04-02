//! Form primitive — Form with validation support.
//!
//! Provides headless form and form field components with proper ARIA
//! attributes for label, error, and validation state relationships.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};

/// Props for the Form primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormProps {
    /// Callback when the form is submitted.
    pub on_submit: Option<Callback<()>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the Form primitive.
#[derive(Debug, Clone)]
pub struct FormState {
    /// Element ID.
    pub id: String,
}

/// Headless form primitive.
pub struct Form;

impl Component for Form {
    type Props = FormProps;
    type State = FormState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props.id.clone().unwrap_or_else(|| generators::FORM.next());
        FormState { id }
    }

    fn render(_props: &Self::Props, state: &Self::State) -> RenderOutput {
        let aria = AriaAttributes::new()
            .with_role(AriaRole::Form);

        RenderOutput::new()
            .with_tag("form")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
    }

    fn on_event(
        props: &Self::Props,
        _state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        match event {
            ComponentEvent::Custom { ref name, .. } if name == "submit" => {
                if let Some(ref cb) = props.on_submit {
                    cb.call(());
                }
                EventResult {
                    prevent_default: true,
                    stop_propagation: false,
                    state_changed: false,
                }
            }
            ComponentEvent::KeyDown { key: Key::Enter, .. } => {
                if let Some(ref cb) = props.on_submit {
                    cb.call(());
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
}

// --- FormField ---

/// Props for the FormField primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormFieldProps {
    /// Field name.
    pub name: String,
    /// Label text.
    pub label: Option<String>,
    /// Error message.
    pub error: Option<String>,
    /// Whether the field is required.
    pub required: bool,
    /// Whether the field is disabled.
    pub disabled: bool,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the FormField primitive.
#[derive(Debug, Clone)]
pub struct FormFieldState {
    /// Element ID.
    pub id: String,
    /// ID for the label element.
    pub label_id: String,
    /// ID for the error message element.
    pub error_id: String,
}

/// Headless form field primitive.
pub struct FormField;

impl Component for FormField {
    type Props = FormFieldProps;
    type State = FormFieldState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::FORM.group(&["field", "label", "error"]);
        let id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let label_id = ids[1].clone();
        let error_id = ids[2].clone();

        FormFieldState {
            id,
            label_id,
            error_id,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let mut aria = AriaAttributes::new();

        if props.required {
            aria.required = Some(true);
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }
        if props.error.is_some() {
            aria.invalid = Some(true);
            aria = aria.with_describedby(&state.error_id);
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_attr("name", AttrValue::String(props.name.clone()));

        // Label element
        let mut children = Vec::new();
        if let Some(ref label) = props.label {
            let label_elem = RenderOutput::new()
                .with_tag("label")
                .with_attr("id", AttrValue::String(state.label_id.clone()))
                .with_children(ChildrenSpec::Text(label.clone()));
            children.push(label_elem);
        }

        // Error element
        if let Some(ref error) = props.error {
            let error_elem = RenderOutput::new()
                .with_tag("div")
                .with_attr("id", AttrValue::String(state.error_id.clone()))
                .with_aria(AriaAttributes::new().with_role(AriaRole::Alert))
                .with_children(ChildrenSpec::Text(error.clone()));
            children.push(error_elem);
        }

        if !children.is_empty() {
            output = output.with_children(ChildrenSpec::Elements(children));
        }

        output
    }

    fn on_event(
        _props: &Self::Props,
        _state: &mut Self::State,
        _event: ComponentEvent,
    ) -> EventResult {
        EventResult::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        use stratum_core::event::ModifierKeys;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    // --- Form tests ---

    fn default_form_props() -> FormProps {
        FormProps::default()
    }

    #[test]
    fn form_initial_state() {
        let props = default_form_props();
        let state = Form::initial_state(&props);
        assert!(!state.id.is_empty());
    }

    #[test]
    fn form_custom_id() {
        let props = FormProps {
            id: Some("my-form".to_string()),
            ..default_form_props()
        };
        let state = Form::initial_state(&props);
        assert_eq!(state.id, "my-form");
    }

    #[test]
    fn form_render_role() {
        let props = default_form_props();
        let state = Form::initial_state(&props);
        let output = Form::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Form));
        assert_eq!(output.effective_tag(), "form");
    }

    #[test]
    fn form_submit_on_custom_event() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = FormProps {
            on_submit: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_form_props()
        };
        let mut state = Form::initial_state(&props);
        let event = ComponentEvent::Custom {
            name: "submit".to_string(),
            data: serde_json::Value::Null,
        };
        let result = Form::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
        assert!(result.prevent_default);
    }

    #[test]
    fn form_submit_on_enter() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = FormProps {
            on_submit: Some(Callback::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..default_form_props()
        };
        let mut state = Form::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Form::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
    }

    // --- FormField tests ---

    fn default_field_props() -> FormFieldProps {
        FormFieldProps {
            name: "email".to_string(),
            ..FormFieldProps::default()
        }
    }

    #[test]
    fn field_initial_state() {
        let props = default_field_props();
        let state = FormField::initial_state(&props);
        assert!(!state.id.is_empty());
        assert!(!state.label_id.is_empty());
        assert!(!state.error_id.is_empty());
    }

    #[test]
    fn field_custom_id() {
        let props = FormFieldProps {
            id: Some("my-field".to_string()),
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        assert_eq!(state.id, "my-field");
    }

    #[test]
    fn field_render_required() {
        let props = FormFieldProps {
            required: true,
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        assert_eq!(output.aria.required, Some(true));
    }

    #[test]
    fn field_render_disabled() {
        let props = FormFieldProps {
            disabled: true,
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        assert_eq!(output.aria.disabled, Some(true));
    }

    #[test]
    fn field_render_with_error() {
        let props = FormFieldProps {
            error: Some("Invalid email".to_string()),
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        assert_eq!(output.aria.invalid, Some(true));
        assert_eq!(output.aria.describedby, Some(state.error_id.clone()));
    }

    #[test]
    fn field_render_no_error() {
        let props = default_field_props();
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        assert!(output.aria.invalid.is_none());
        assert!(output.aria.describedby.is_none());
    }

    #[test]
    fn field_render_label_element() {
        let props = FormFieldProps {
            label: Some("Email Address".to_string()),
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].effective_tag(), "label");
            assert!(elems[0].attrs.contains(&("id".to_string(), AttrValue::String(state.label_id.clone()))));
        } else {
            panic!("Expected Elements with label");
        }
    }

    #[test]
    fn field_render_error_element() {
        let props = FormFieldProps {
            error: Some("Required".to_string()),
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            let error_elem = &elems[0]; // only error, no label
            assert_eq!(error_elem.aria.role, Some(AriaRole::Alert));
        }
    }

    #[test]
    fn field_render_label_and_error() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            error: Some("Invalid".to_string()),
            ..default_field_props()
        };
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems.len(), 2);
            assert_eq!(elems[0].effective_tag(), "label");
            assert_eq!(elems[1].aria.role, Some(AriaRole::Alert));
        }
    }

    #[test]
    fn field_render_name_attribute() {
        let props = default_field_props();
        let state = FormField::initial_state(&props);
        let output = FormField::render(&props, &state);
        assert!(output.attrs.contains(&("name".to_string(), AttrValue::String("email".to_string()))));
    }

    #[test]
    fn field_events_are_noop() {
        let props = default_field_props();
        let mut state = FormField::initial_state(&props);
        let result = FormField::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.state_changed);
    }
}
