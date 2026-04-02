//! Select primitive — Native-style select component.
//!
//! Provides a headless select with listbox, keyboard navigation,
//! and proper ARIA combobox/listbox/option attributes.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};
use stratum_core::focus::FocusManager;
use stratum_core::aria::AriaHasPopup;

/// A single select option.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    /// The option value.
    pub value: String,
    /// The display label.
    pub label: String,
    /// Whether this option is disabled.
    pub disabled: bool,
}

/// Props for the Select primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectProps {
    /// Controlled selected value.
    pub value: Option<String>,
    /// Default selected value for uncontrolled usage.
    pub default_value: Option<String>,
    /// Placeholder text when no selection.
    pub placeholder: Option<String>,
    /// Whether the select is disabled.
    pub disabled: bool,
    /// Callback when selection changes.
    pub on_change: Option<Callback<String>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Available options.
    pub options: Vec<SelectOption>,
}

/// Internal state for the Select primitive.
#[derive(Debug, Clone)]
pub struct SelectState {
    /// Whether the listbox is open.
    pub open: bool,
    /// The currently selected value.
    pub selected: Option<String>,
    /// Index of the currently focused option.
    pub focused_index: usize,
    /// Root element ID.
    pub id: String,
    /// ID of the trigger element.
    pub trigger_id: String,
    /// ID of the listbox element.
    pub listbox_id: String,
    /// Focus manager.
    pub focus_manager: FocusManager,
}

/// Headless select primitive.
pub struct Select;

impl Select {
    fn next_enabled(options: &[SelectOption], from: usize) -> usize {
        let len = options.len();
        for offset in 1..=len {
            let idx = (from + offset) % len;
            if !options[idx].disabled {
                return idx;
            }
        }
        from
    }

    fn prev_enabled(options: &[SelectOption], from: usize) -> usize {
        let len = options.len();
        for offset in 1..=len {
            let idx = if from >= offset { from - offset } else { len - (offset - from) };
            if !options[idx].disabled {
                return idx;
            }
        }
        from
    }
}

impl Component for Select {
    type Props = SelectProps;
    type State = SelectState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::SELECT.group(&["root", "trigger", "listbox"]);
        let id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let trigger_id = ids[1].clone();
        let listbox_id = ids[2].clone();

        let selected = props.value.clone().or_else(|| props.default_value.clone());

        let focused_index = selected.as_ref()
            .and_then(|sel| props.options.iter().position(|o| o.value == *sel))
            .unwrap_or(0);

        let focus_manager = FocusManager::popover().with_container(listbox_id.clone());

        SelectState {
            open: false,
            selected,
            focused_index,
            id,
            trigger_id,
            listbox_id,
            focus_manager,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        // Trigger (combobox)
        let mut trigger_aria = AriaAttributes::new()
            .with_role(AriaRole::Combobox)
            .with_expanded(state.open)
            .with_controls(&state.listbox_id)
            .with_haspopup(AriaHasPopup::ListBox);

        if props.disabled {
            trigger_aria = trigger_aria.with_disabled(true);
        }

        let effective_selected = props.value.as_ref().or(state.selected.as_ref());

        let display_text = effective_selected
            .and_then(|sel| props.options.iter().find(|o| o.value == *sel))
            .map(|o| o.label.clone())
            .or_else(|| props.placeholder.clone())
            .unwrap_or_default();

        let trigger = RenderOutput::new()
            .with_tag("button")
            .with_aria(trigger_aria)
            .with_attr("id", AttrValue::String(state.trigger_id.clone()))
            .with_data("value", display_text);

        // Listbox
        let listbox_aria = AriaAttributes::new()
            .with_role(AriaRole::ListBox);

        let mut listbox = RenderOutput::new()
            .with_tag("div")
            .with_aria(listbox_aria)
            .with_attr("id", AttrValue::String(state.listbox_id.clone()));

        if !state.open {
            listbox = listbox.with_attr("hidden", AttrValue::Bool(true));
        }

        // Options
        let mut option_elems = Vec::new();
        for (i, opt) in props.options.iter().enumerate() {
            let is_selected = effective_selected == Some(&opt.value);
            let mut opt_aria = AriaAttributes::new()
                .with_role(AriaRole::Option)
                .with_selected(is_selected);

            if opt.disabled {
                opt_aria = opt_aria.with_disabled(true);
            }

            let option_elem = RenderOutput::new()
                .with_tag("div")
                .with_aria(opt_aria)
                .with_attr(
                    "tabindex",
                    AttrValue::String(
                        if state.focused_index == i { "0" } else { "-1" }.to_string(),
                    ),
                )
                .with_data("value", opt.value.clone())
                .with_children(ChildrenSpec::Text(opt.label.clone()));
            option_elems.push(option_elem);
        }

        let mut all_children = vec![trigger, listbox];
        all_children.extend(option_elems);

        RenderOutput::new()
            .with_tag("div")
            .with_attr("id", AttrValue::String(state.id.clone()))
            .with_data("state", if state.open { "open" } else { "closed" })
            .with_children(ChildrenSpec::Elements(all_children))
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
                state.open = !state.open;
                EventResult::state_changed()
            }
            ComponentEvent::KeyDown { key, .. } if state.open => {
                match key {
                    Key::Escape => {
                        state.open = false;
                        EventResult::prevent_and_changed()
                    }
                    Key::ArrowDown => {
                        if !props.options.is_empty() {
                            state.focused_index = Select::next_enabled(&props.options, state.focused_index);
                        }
                        EventResult::prevent_and_changed()
                    }
                    Key::ArrowUp => {
                        if !props.options.is_empty() {
                            state.focused_index = Select::prev_enabled(&props.options, state.focused_index);
                        }
                        EventResult::prevent_and_changed()
                    }
                    Key::Home => {
                        state.focused_index = props.options.iter()
                            .position(|o| !o.disabled)
                            .unwrap_or(0);
                        EventResult::prevent_and_changed()
                    }
                    Key::End => {
                        state.focused_index = props.options.iter()
                            .rposition(|o| !o.disabled)
                            .unwrap_or(0);
                        EventResult::prevent_and_changed()
                    }
                    Key::Enter => {
                        if state.focused_index < props.options.len() {
                            let opt = &props.options[state.focused_index];
                            if !opt.disabled {
                                if props.value.is_none() {
                                    state.selected = Some(opt.value.clone());
                                }
                                if let Some(ref cb) = props.on_change {
                                    cb.call(opt.value.clone());
                                }
                            }
                        }
                        state.open = false;
                        EventResult::prevent_and_changed()
                    }
                    _ => EventResult::default(),
                }
            }
            ComponentEvent::KeyDown { key: Key::Space, .. } if !state.open => {
                state.open = true;
                EventResult::prevent_and_changed()
            }
            ComponentEvent::KeyDown { key: Key::ArrowDown, .. } if !state.open => {
                state.open = true;
                EventResult::prevent_and_changed()
            }
            ComponentEvent::KeyDown { key: Key::ArrowUp, .. } if !state.open => {
                state.open = true;
                EventResult::prevent_and_changed()
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
    use std::sync::{Arc, Mutex};

    fn test_options() -> Vec<SelectOption> {
        vec![
            SelectOption { value: "a".to_string(), label: "Apple".to_string(), disabled: false },
            SelectOption { value: "b".to_string(), label: "Banana".to_string(), disabled: false },
            SelectOption { value: "c".to_string(), label: "Cherry".to_string(), disabled: true },
            SelectOption { value: "d".to_string(), label: "Date".to_string(), disabled: false },
        ]
    }

    fn test_props() -> SelectProps {
        SelectProps {
            options: test_options(),
            ..SelectProps::default()
        }
    }

    #[test]
    fn initial_state_defaults() {
        let props = test_props();
        let state = Select::initial_state(&props);
        assert!(!state.open);
        assert!(state.selected.is_none());
        assert_eq!(state.focused_index, 0);
    }

    #[test]
    fn initial_state_with_default_value() {
        let props = SelectProps {
            default_value: Some("b".to_string()),
            ..test_props()
        };
        let state = Select::initial_state(&props);
        assert_eq!(state.selected, Some("b".to_string()));
        assert_eq!(state.focused_index, 1);
    }

    #[test]
    fn render_trigger_combobox_role() {
        let props = test_props();
        let state = Select::initial_state(&props);
        let output = Select::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.role, Some(AriaRole::Combobox));
            assert_eq!(elems[0].aria.haspopup, Some(AriaHasPopup::ListBox));
            assert_eq!(elems[0].aria.expanded, Some(false));
        }
    }

    #[test]
    fn render_listbox_role() {
        let props = test_props();
        let state = Select::initial_state(&props);
        let output = Select::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[1].aria.role, Some(AriaRole::ListBox));
        }
    }

    #[test]
    fn render_option_roles() {
        let props = test_props();
        let state = Select::initial_state(&props);
        let output = Select::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // options start at index 2
            assert_eq!(elems[2].aria.role, Some(AriaRole::Option));
            assert_eq!(elems[2].aria.selected, Some(false));
            assert_eq!(elems[4].aria.disabled, Some(true)); // Cherry disabled
        }
    }

    #[test]
    fn render_selected_option() {
        let props = SelectProps {
            default_value: Some("a".to_string()),
            ..test_props()
        };
        let state = Select::initial_state(&props);
        let output = Select::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[2].aria.selected, Some(true));
        }
    }

    #[test]
    fn render_placeholder() {
        let props = SelectProps {
            placeholder: Some("Pick one".to_string()),
            ..test_props()
        };
        let state = Select::initial_state(&props);
        let output = Select::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(elems[0].data_attrs.contains(&("value".to_string(), "Pick one".to_string())));
        }
    }

    #[test]
    fn click_opens_select() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        Select::on_event(&props, &mut state, event);
        assert!(state.open);
    }

    #[test]
    fn space_opens_when_closed() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Space,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert!(state.open);
    }

    #[test]
    fn escape_closes_select() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        state.open = true;
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert!(!state.open);
    }

    #[test]
    fn arrow_down_navigates_skipping_disabled() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        state.open = true;
        state.focused_index = 1; // Banana

        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 3); // Date (skips Cherry)
    }

    #[test]
    fn arrow_up_navigates_skipping_disabled() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        state.open = true;
        state.focused_index = 3; // Date

        let event = ComponentEvent::KeyDown {
            key: Key::ArrowUp,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 1); // Banana (skips Cherry)
    }

    #[test]
    fn enter_selects_and_closes() {
        let received = Arc::new(Mutex::new(String::new()));
        let received_clone = Arc::clone(&received);
        let props = SelectProps {
            on_change: Some(Callback::new(move |val: String| {
                *received_clone.lock().unwrap() = val;
            })),
            ..test_props()
        };
        let mut state = Select::initial_state(&props);
        state.open = true;
        state.focused_index = 1;
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert!(!state.open);
        assert_eq!(state.selected, Some("b".to_string()));
        assert_eq!(*received.lock().unwrap(), "b");
    }

    #[test]
    fn disabled_prevents_interaction() {
        let props = SelectProps {
            disabled: true,
            ..test_props()
        };
        let mut state = Select::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        let result = Select::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
        assert!(!state.open);
    }

    #[test]
    fn render_disabled_trigger() {
        let props = SelectProps {
            disabled: true,
            ..test_props()
        };
        let state = Select::initial_state(&props);
        let output = Select::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.disabled, Some(true));
        }
    }

    #[test]
    fn home_end_navigation() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        state.open = true;
        state.focused_index = 1;

        let event = ComponentEvent::KeyDown {
            key: Key::End,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 3); // last enabled

        let event = ComponentEvent::KeyDown {
            key: Key::Home,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, 0); // first enabled
    }

    #[test]
    fn focus_manager_restores() {
        let props = test_props();
        let state = Select::initial_state(&props);
        assert!(state.focus_manager.should_restore());
    }

    #[test]
    fn enter_does_not_select_disabled() {
        let props = test_props();
        let mut state = Select::initial_state(&props);
        state.open = true;
        state.focused_index = 2; // Cherry (disabled)
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Select::on_event(&props, &mut state, event);
        assert!(state.selected.is_none());
    }
}
