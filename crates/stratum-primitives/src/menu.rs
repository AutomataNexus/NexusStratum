//! Menu primitive — Dropdown menu component.
//!
//! Provides a headless dropdown menu with keyboard navigation,
//! type-ahead search, and proper ARIA attributes.

use stratum_core::{Component, ComponentEvent, EventResult, RenderOutput, AriaAttributes, AriaRole, Key};
use stratum_core::callback::Callback;
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, ChildrenSpec};
use stratum_core::focus::FocusManager;
use stratum_core::aria::AriaHasPopup;

/// Data for a single menu item.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItemData {
    /// Unique identifier for the item.
    pub id: String,
    /// Whether this item is disabled.
    pub disabled: bool,
}

/// Props for the Menu primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MenuProps {
    /// Controlled open state.
    pub open: Option<bool>,
    /// Default open state for uncontrolled usage.
    pub default_open: bool,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Menu items.
    pub items: Vec<MenuItemData>,
}

/// Internal state for the Menu primitive.
#[derive(Debug, Clone)]
pub struct MenuState {
    /// Whether the menu is open.
    pub open: bool,
    /// Index of the currently focused item, if any.
    pub focused_index: Option<usize>,
    /// Root element ID.
    pub id: String,
    /// ID of the trigger element.
    pub trigger_id: String,
    /// ID of the menu element.
    pub menu_id: String,
    /// Focus manager for the menu.
    pub focus_manager: FocusManager,
}

/// Headless menu primitive.
pub struct Menu;

impl Menu {
    /// Find the next enabled item index starting from a position, wrapping.
    fn next_enabled(items: &[MenuItemData], from: usize) -> Option<usize> {
        let len = items.len();
        for offset in 1..=len {
            let idx = (from + offset) % len;
            if !items[idx].disabled {
                return Some(idx);
            }
        }
        None
    }

    /// Find the previous enabled item index starting from a position, wrapping.
    fn prev_enabled(items: &[MenuItemData], from: usize) -> Option<usize> {
        let len = items.len();
        for offset in 1..=len {
            let idx = if from >= offset { from - offset } else { len - (offset - from) };
            if !items[idx].disabled {
                return Some(idx);
            }
        }
        None
    }

    /// Find first item whose ID starts with the given character (type-ahead).
    fn type_ahead(items: &[MenuItemData], ch: char, current: Option<usize>) -> Option<usize> {
        let ch_lower = ch.to_lowercase().next().unwrap_or(ch);
        let start = current.map(|i| i + 1).unwrap_or(0);
        let len = items.len();

        for offset in 0..len {
            let idx = (start + offset) % len;
            if !items[idx].disabled {
                if let Some(first_char) = items[idx].id.chars().next() {
                    if first_char.to_lowercase().next().unwrap_or(first_char) == ch_lower {
                        return Some(idx);
                    }
                }
            }
        }
        None
    }
}

impl Component for Menu {
    type Props = MenuProps;
    type State = MenuState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let ids = generators::MENU.group(&["root", "trigger", "list"]);
        let id = props.id.clone().unwrap_or_else(|| ids[0].clone());
        let trigger_id = ids[1].clone();
        let menu_id = ids[2].clone();

        let focus_manager = FocusManager::menu().with_container(menu_id.clone());

        MenuState {
            open: props.open.unwrap_or(props.default_open),
            focused_index: None,
            id,
            trigger_id,
            menu_id,
            focus_manager,
        }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        // Trigger
        let trigger_aria = AriaAttributes::new()
            .with_haspopup(AriaHasPopup::Menu)
            .with_expanded(state.open)
            .with_controls(&state.menu_id);

        let trigger = RenderOutput::new()
            .with_tag("button")
            .with_aria(trigger_aria)
            .with_attr("id", AttrValue::String(state.trigger_id.clone()));

        // Menu list
        let menu_aria = AriaAttributes::new()
            .with_role(AriaRole::Menu);

        let mut menu = RenderOutput::new()
            .with_tag("div")
            .with_aria(menu_aria)
            .with_attr("id", AttrValue::String(state.menu_id.clone()));

        if !state.open {
            menu = menu.with_attr("hidden", AttrValue::Bool(true));
        }

        // Menu items
        let mut items = Vec::new();
        for (i, item_data) in props.items.iter().enumerate() {
            let mut item_aria = AriaAttributes::new()
                .with_role(AriaRole::MenuItem);

            if item_data.disabled {
                item_aria = item_aria.with_disabled(true);
            }

            let item = RenderOutput::new()
                .with_tag("div")
                .with_aria(item_aria)
                .with_attr("id", AttrValue::String(item_data.id.clone()))
                .with_attr(
                    "tabindex",
                    AttrValue::String(
                        if state.focused_index == Some(i) { "0" } else { "-1" }.to_string(),
                    ),
                );
            items.push(item);
        }

        let mut all_children = vec![trigger, menu];
        all_children.extend(items);

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
        match event {
            ComponentEvent::Click { .. } => {
                state.open = !state.open;
                if state.open && !props.items.is_empty() {
                    // Focus first enabled item
                    state.focused_index = props.items.iter()
                        .position(|item| !item.disabled);
                } else {
                    state.focused_index = None;
                }
                if let Some(ref cb) = props.on_open_change {
                    cb.call(state.open);
                }
                EventResult::state_changed()
            }
            ComponentEvent::KeyDown { key, .. } if state.open => {
                match key {
                    Key::Escape => {
                        state.open = false;
                        state.focused_index = None;
                        if let Some(ref cb) = props.on_open_change {
                            cb.call(false);
                        }
                        EventResult::prevent_and_changed()
                    }
                    Key::ArrowDown => {
                        let current = state.focused_index.unwrap_or(props.items.len().wrapping_sub(1));
                        if let Some(next) = Menu::next_enabled(&props.items, current) {
                            state.focused_index = Some(next);
                        }
                        EventResult::prevent_and_changed()
                    }
                    Key::ArrowUp => {
                        let current = state.focused_index.unwrap_or(0);
                        if let Some(prev) = Menu::prev_enabled(&props.items, current) {
                            state.focused_index = Some(prev);
                        }
                        EventResult::prevent_and_changed()
                    }
                    Key::Home => {
                        state.focused_index = props.items.iter()
                            .position(|item| !item.disabled);
                        EventResult::prevent_and_changed()
                    }
                    Key::End => {
                        state.focused_index = props.items.iter()
                            .rposition(|item| !item.disabled);
                        EventResult::prevent_and_changed()
                    }
                    Key::Enter | Key::Space => {
                        // Activate current item and close
                        state.open = false;
                        if let Some(ref cb) = props.on_open_change {
                            cb.call(false);
                        }
                        EventResult::prevent_and_changed()
                    }
                    Key::Char(ch) => {
                        if let Some(idx) = Menu::type_ahead(&props.items, ch, state.focused_index) {
                            state.focused_index = Some(idx);
                            return EventResult::prevent_and_changed();
                        }
                        EventResult::default()
                    }
                    _ => EventResult::default(),
                }
            }
            ComponentEvent::KeyDown { key: Key::Enter | Key::Space | Key::ArrowDown, .. } if !state.open => {
                state.open = true;
                state.focused_index = props.items.iter()
                    .position(|item| !item.disabled);
                if let Some(ref cb) = props.on_open_change {
                    cb.call(true);
                }
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
    use stratum_core::focus::FocusStrategy;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn test_items() -> Vec<MenuItemData> {
        vec![
            MenuItemData { id: "cut".to_string(), disabled: false },
            MenuItemData { id: "copy".to_string(), disabled: false },
            MenuItemData { id: "paste".to_string(), disabled: true },
            MenuItemData { id: "delete".to_string(), disabled: false },
        ]
    }

    fn test_props() -> MenuProps {
        MenuProps {
            items: test_items(),
            ..MenuProps::default()
        }
    }

    #[test]
    fn initial_state_defaults() {
        let props = test_props();
        let state = Menu::initial_state(&props);
        assert!(!state.open);
        assert!(state.focused_index.is_none());
        assert!(!state.trigger_id.is_empty());
        assert!(!state.menu_id.is_empty());
    }

    #[test]
    fn render_trigger_aria() {
        let props = test_props();
        let state = Menu::initial_state(&props);
        let output = Menu::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[0].aria.haspopup, Some(AriaHasPopup::Menu));
            assert_eq!(elems[0].aria.expanded, Some(false));
            assert_eq!(elems[0].aria.controls, Some(state.menu_id.clone()));
        }
    }

    #[test]
    fn render_menu_role() {
        let props = test_props();
        let state = Menu::initial_state(&props);
        let output = Menu::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert_eq!(elems[1].aria.role, Some(AriaRole::Menu));
        }
    }

    #[test]
    fn render_menuitem_roles() {
        let props = test_props();
        let state = Menu::initial_state(&props);
        let output = Menu::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            // items start at index 2
            assert_eq!(elems[2].aria.role, Some(AriaRole::MenuItem));
            assert_eq!(elems[4].aria.disabled, Some(true)); // paste is disabled
        }
    }

    #[test]
    fn render_closed_menu_hidden() {
        let props = test_props();
        let state = Menu::initial_state(&props);
        let output = Menu::render(&props, &state);
        if let ChildrenSpec::Elements(ref elems) = output.children {
            assert!(elems[1].attrs.contains(&("hidden".to_string(), AttrValue::Bool(true))));
        }
    }

    #[test]
    fn click_opens_menu() {
        let props = test_props();
        let mut state = Menu::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        Menu::on_event(&props, &mut state, event);
        assert!(state.open);
        assert_eq!(state.focused_index, Some(0));
    }

    #[test]
    fn click_toggles_menu() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(0);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        Menu::on_event(&props, &mut state, event);
        assert!(!state.open);
    }

    #[test]
    fn escape_closes_menu() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::Escape,
            modifiers: ModifierKeys::default(),
        };
        let result = Menu::on_event(&props, &mut state, event);
        assert!(!state.open);
        assert!(result.prevent_default);
    }

    #[test]
    fn arrow_down_navigates() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(0);

        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, Some(1)); // copy

        // Arrow down again skips disabled paste
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, Some(3)); // delete (skips paste)
    }

    #[test]
    fn arrow_up_navigates() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(3);

        let event = ComponentEvent::KeyDown {
            key: Key::ArrowUp,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, Some(1)); // copy (skips disabled paste)
    }

    #[test]
    fn home_focuses_first_enabled() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(3);
        let event = ComponentEvent::KeyDown {
            key: Key::Home,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, Some(0));
    }

    #[test]
    fn end_focuses_last_enabled() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::End,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, Some(3));
    }

    #[test]
    fn enter_activates_and_closes() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(0);
        let event = ComponentEvent::KeyDown {
            key: Key::Enter,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert!(!state.open);
    }

    #[test]
    fn type_ahead_jumps_to_item() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(0);
        let event = ComponentEvent::KeyDown {
            key: Key::Char('d'),
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert_eq!(state.focused_index, Some(3)); // "delete"
    }

    #[test]
    fn type_ahead_skips_disabled() {
        let props = MenuProps {
            default_open: true,
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        state.focused_index = Some(0);
        let event = ComponentEvent::KeyDown {
            key: Key::Char('p'),
            modifiers: ModifierKeys::default(),
        };
        let result = Menu::on_event(&props, &mut state, event);
        // paste is disabled, no match
        assert!(!result.state_changed);
    }

    #[test]
    fn keyboard_opens_closed_menu() {
        let props = test_props();
        let mut state = Menu::initial_state(&props);
        let event = ComponentEvent::KeyDown {
            key: Key::ArrowDown,
            modifiers: ModifierKeys::default(),
        };
        Menu::on_event(&props, &mut state, event);
        assert!(state.open);
        assert_eq!(state.focused_index, Some(0));
    }

    #[test]
    fn callback_fires_on_open() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let props = MenuProps {
            on_open_change: Some(Callback::new(move |_: bool| {
                called_clone.store(true, Ordering::SeqCst);
            })),
            ..test_props()
        };
        let mut state = Menu::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0, y: 0.0,
            button: MouseButton::Left,
        };
        Menu::on_event(&props, &mut state, event);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn focus_manager_is_initial_strategy() {
        let props = test_props();
        let state = Menu::initial_state(&props);
        assert_eq!(state.focus_manager.strategy, FocusStrategy::Initial);
    }
}
