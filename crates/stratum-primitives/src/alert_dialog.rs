//! AlertDialog primitive — Confirmation dialog component.
//!
//! Similar to Dialog but always modal with role=alertdialog.
//! Used for important confirmations that require user acknowledgment.

use stratum_core::callback::Callback;
use stratum_core::focus::FocusManager;
use stratum_core::id::generators;
use stratum_core::render::AttrValue;
use stratum_core::{
    AriaAttributes, AriaRole, Component, ComponentEvent, EventResult, Key, RenderOutput,
};

/// Props for the AlertDialog primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AlertDialogProps {
    /// Controlled open state.
    pub open: Option<bool>,
    /// Default open state for uncontrolled usage.
    pub default_open: bool,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
}

/// Internal state for the AlertDialog primitive.
#[derive(Debug, Clone)]
pub struct AlertDialogState {
    /// Whether the dialog is open.
    pub open: bool,
    /// The dialog element ID.
    pub id: String,
    /// ID for the dialog title element.
    pub title_id: String,
    /// ID for the dialog description element.
    pub description_id: String,
    /// Focus manager — always traps and restores.
    pub focus_manager: FocusManager,
}

/// Headless alert dialog primitive.
pub struct AlertDialog;

impl Component for AlertDialog {
    type Props = AlertDialogProps;
    type State = AlertDialogState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::DIALOG.group(&["root", "title", "desc"]);
        let id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let title_id = ids[1].clone();
        let description_id = ids[2].clone();

        let focus_manager = FocusManager::dialog().with_container(id.clone());

        AlertDialogState {
            open: props.open.unwrap_or(props.default_open),
            id,
            title_id,
            description_id,
            focus_manager,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let aria = AriaAttributes::new()
            .with_role(AriaRole::AlertDialog)
            .with_modal(true)
            .with_labelledby(&state.title_id)
            .with_describedby(&state.description_id);

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()));

        let effective_open = props.open.unwrap_or(state.open);
        output = output.with_data("state", if effective_open { "open" } else { "closed" });

        if !effective_open {
            output = output.with_attr("hidden", AttrValue::Bool(true));
        }

        output
    }

    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult {
        let current = props.open.unwrap_or(state.open);

        match event {
            ComponentEvent::KeyDown {
                key: Key::Escape, ..
            } => {
                if current {
                    if let Some(ref cb) = props.on_open_change {
                        cb.call(false);
                    }
                    if props.open.is_none() {
                        state.open = false;
                    }
                    return EventResult::prevent_and_changed();
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

    fn default_props() -> AlertDialogProps {
        AlertDialogProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = AlertDialog::initial_state(&props);
        assert!(!state.open);
        assert!(!state.title_id.is_empty());
        assert!(!state.description_id.is_empty());
    }

    #[test]
    fn initial_state_custom_id() {
        let props = AlertDialogProps {
            id: Some("my-alert".to_string()),
            ..default_props()
        };
        let state = AlertDialog::initial_state(&props);
        assert_eq!(state.id, "my-alert");
    }

    #[test]
    fn initial_state_default_open() {
        let props = AlertDialogProps {
            default_open: true,
            ..default_props()
        };
        let state = AlertDialog::initial_state(&props);
        assert!(state.open);
    }

    #[test]
    fn render_alertdialog_role() {
        let props = default_props();
        let state = AlertDialog::initial_state(&props);
        let output = AlertDialog::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::AlertDialog));
    }

    #[test]
    fn render_always_modal() {
        let props = default_props();
        let state = AlertDialog::initial_state(&props);
        let output = AlertDialog::render(&props, &state);
        assert_eq!(output.aria.modal, Some(true));
    }

    #[test]
    fn render_labelledby_and_describedby() {
        let props = default_props();
        let state = AlertDialog::initial_state(&props);
        let output = AlertDialog::render(&props, &state);
        assert_eq!(output.aria.labelledby, Some(state.title_id.clone()));
        assert_eq!(output.aria.describedby, Some(state.description_id.clone()));
    }

    #[test]
    fn render_closed_has_hidden() {
        let props = default_props();
        let state = AlertDialog::initial_state(&props);
        let output = AlertDialog::render(&props, &state);
        assert!(
            output
                .attrs
                .contains(&("hidden".to_string(), AttrValue::Bool(true)))
        );
    }

    #[test]
    fn escape_closes_alert_dialog() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = AlertDialogProps {
            default_open: true,
            on_open_change: Some(Callback::new(move |open: bool| {
                if !open {
                    called_clone.store(true, Ordering::SeqCst);
                }
            })),
            ..default_props()
        };
        let mut state = AlertDialog::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = AlertDialog::on_event(&props, &mut state, event);
        assert!(!state.open);
        assert!(result.state_changed);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn focus_manager_always_traps() {
        let props = default_props();
        let state = AlertDialog::initial_state(&props);
        assert!(state.focus_manager.is_trapping());
        assert!(state.focus_manager.should_restore());
    }

    #[test]
    fn data_state_attribute() {
        let props = AlertDialogProps {
            default_open: true,
            ..default_props()
        };
        let state = AlertDialog::initial_state(&props);
        let output = AlertDialog::render(&props, &state);
        assert!(
            output
                .data_attrs
                .contains(&("state".to_string(), "open".to_string()))
        );
    }
}
