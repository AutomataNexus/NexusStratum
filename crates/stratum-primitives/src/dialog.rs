//! Dialog primitive — Modal dialog component.
//!
//! Provides a headless dialog with focus trapping, backdrop support,
//! and proper ARIA attributes for modal interactions.

use stratum_core::callback::Callback;
use stratum_core::focus::{FocusManager, FocusStrategy};
use stratum_core::id::generators;
use stratum_core::render::AttrValue;
use stratum_core::{
    AriaAttributes, AriaRole, Component, ComponentEvent, EventResult, Key, RenderOutput,
};

/// Props for the Dialog primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct DialogProps {
    /// Controlled open state.
    pub open: Option<bool>,
    /// Default open state for uncontrolled usage.
    pub default_open: bool,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// If true, trap focus and show backdrop.
    pub modal: bool,
    /// Optional explicit ID.
    pub id: Option<String>,
}

impl Default for DialogProps {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            on_open_change: None,
            modal: true,
            id: None,
        }
    }
}

/// Internal state for the Dialog primitive.
#[derive(Debug, Clone)]
pub struct DialogState {
    /// Whether the dialog is open.
    pub open: bool,
    /// The dialog element ID.
    pub id: String,
    /// ID for the dialog title element.
    pub title_id: String,
    /// ID for the dialog description element.
    pub description_id: String,
    /// Focus manager for trapping/restoring focus.
    pub focus_manager: FocusManager,
}

/// Headless dialog primitive.
pub struct Dialog;

impl Component for Dialog {
    type Props = DialogProps;
    type State = DialogState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::DIALOG.group(&["root", "title", "desc"]);
        let id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let title_id = ids[1].clone();
        let description_id = ids[2].clone();

        let focus_manager = if props.modal {
            FocusManager::dialog().with_container(id.clone())
        } else {
            FocusManager::new(FocusStrategy::Auto)
        };

        DialogState {
            open: props.open.unwrap_or(props.default_open),
            id,
            title_id,
            description_id,
            focus_manager,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let is_open = props.open.unwrap_or(state.open);

        let mut aria = AriaAttributes::new()
            .with_role(AriaRole::Dialog)
            .with_labelledby(&state.title_id)
            .with_describedby(&state.description_id);

        if props.modal {
            aria = aria.with_modal(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()));

        output = output.with_data("state", if is_open { "open" } else { "closed" });

        if !is_open {
            output = output.with_attr("hidden", AttrValue::Bool(true));
        }

        output
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        let is_open = props.open.unwrap_or(state.open);
        match event {
            ComponentEvent::KeyDown {
                key: Key::Escape, ..
            } => {
                if is_open {
                    if let Some(ref cb) = props.on_open_change {
                        cb.call(false);
                    }
                    // Only update internal state in uncontrolled mode
                    if props.open.is_none() {
                        state.open = false;
                        return EventResult::prevent_and_changed();
                    }
                    return EventResult {
                        prevent_default: true,
                        stop_propagation: false,
                        state_changed: false,
                    };
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use stratum_core::event::ModifierKeys;

    fn default_props() -> DialogProps {
        DialogProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Dialog::initial_state(&props);
        assert!(!state.open);
        assert!(!state.title_id.is_empty());
        assert!(!state.description_id.is_empty());
    }

    #[test]
    fn initial_state_custom_id() {
        let props = DialogProps {
            id: Some("my-dialog".to_string()),
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        assert_eq!(state.id, "my-dialog");
    }

    #[test]
    fn initial_state_default_open() {
        let props = DialogProps {
            default_open: true,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        assert!(state.open);
    }

    #[test]
    fn initial_state_controlled_open() {
        let props = DialogProps {
            open: Some(true),
            default_open: false,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        assert!(state.open);
    }

    #[test]
    fn render_dialog_role() {
        let props = default_props();
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Dialog));
    }

    #[test]
    fn render_modal_has_aria_modal() {
        let props = DialogProps {
            modal: true,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert_eq!(output.aria.modal, Some(true));
    }

    #[test]
    fn render_non_modal_no_aria_modal() {
        let props = DialogProps {
            modal: false,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert!(output.aria.modal.is_none());
    }

    #[test]
    fn render_labelledby_and_describedby() {
        let props = default_props();
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert_eq!(output.aria.labelledby, Some(state.title_id.clone()));
        assert_eq!(output.aria.describedby, Some(state.description_id.clone()));
    }

    #[test]
    fn render_closed_has_hidden() {
        let props = default_props();
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert!(
            output
                .attrs
                .contains(&("hidden".to_string(), AttrValue::Bool(true)))
        );
    }

    #[test]
    fn render_open_no_hidden() {
        let props = DialogProps {
            default_open: true,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert!(
            !output
                .attrs
                .contains(&("hidden".to_string(), AttrValue::Bool(true)))
        );
    }

    #[test]
    fn escape_closes_dialog() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = DialogProps {
            default_open: true,
            on_open_change: Some(Callback::new(move |open: bool| {
                if !open {
                    called_clone.store(true, Ordering::SeqCst);
                }
            })),
            ..default_props()
        };
        let mut state = Dialog::initial_state(&props);
        assert!(state.open);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Dialog::on_event(&props, &mut state, event);
        assert!(!state.open);
        assert!(result.state_changed);
        assert!(result.prevent_default);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn escape_on_closed_dialog_is_noop() {
        let props = default_props();
        let mut state = Dialog::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Dialog::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
    }

    #[test]
    fn modal_dialog_focus_manager_traps() {
        let props = DialogProps {
            modal: true,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        assert!(state.focus_manager.is_trapping());
        assert!(state.focus_manager.should_restore());
    }

    #[test]
    fn non_modal_dialog_no_focus_trap() {
        let props = DialogProps {
            modal: false,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        assert!(!state.focus_manager.is_trapping());
    }

    #[test]
    fn data_state_attribute() {
        let props = DialogProps {
            default_open: true,
            ..default_props()
        };
        let state = Dialog::initial_state(&props);
        let output = Dialog::render(&props, &state);
        assert!(
            output
                .data_attrs
                .contains(&("state".to_string(), "open".to_string()))
        );
    }
}
