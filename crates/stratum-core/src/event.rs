use serde::{Deserialize, Serialize};

/// Events that components can receive and handle.
///
/// Framework adapters translate framework-specific events (e.g., Leptos `on:click`,
/// Dioxus `onclick`) into `ComponentEvent` before passing to `Component::on_event`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentEvent {
    /// Mouse click event.
    Click { x: f32, y: f32, button: MouseButton },

    /// Key pressed down.
    KeyDown { key: Key, modifiers: ModifierKeys },

    /// Key released.
    KeyUp { key: Key, modifiers: ModifierKeys },

    /// Component received focus.
    Focus,

    /// Component lost focus.
    Blur,

    /// Value changed (inputs, selects, etc.).
    Change { value: String },

    /// Input event (fires on every keystroke, before Change).
    Input { value: String },

    /// Mouse entered the component.
    PointerEnter,

    /// Mouse left the component.
    PointerLeave,

    /// Touch start (mobile).
    TouchStart { x: f32, y: f32 },

    /// Touch end (mobile).
    TouchEnd,

    /// Custom event for extensibility.
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

/// Result of handling an event.
///
/// Tells the framework adapter how to handle the event after the component
/// has processed it, and whether a re-render is needed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EventResult {
    /// Whether to call `preventDefault()` on the DOM event.
    pub prevent_default: bool,
    /// Whether to call `stopPropagation()` on the DOM event.
    pub stop_propagation: bool,
    /// Whether component state changed (triggers re-render).
    pub state_changed: bool,
}

impl EventResult {
    /// Create an EventResult indicating state changed.
    pub fn state_changed() -> Self {
        Self {
            prevent_default: false,
            stop_propagation: false,
            state_changed: true,
        }
    }

    /// Create an EventResult that prevents default and indicates state changed.
    pub fn prevent_and_changed() -> Self {
        Self {
            prevent_default: true,
            stop_propagation: false,
            state_changed: true,
        }
    }
}

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// Keyboard key identifier.
///
/// Covers all keys needed for ARIA keyboard navigation patterns
/// as specified in the ARIA Authoring Practices Guide (APG).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    Enter,
    Space,
    Escape,
    Tab,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Backspace,
    Delete,
    /// A printable character (for type-ahead in menus, listboxes, etc.).
    Char(char),
    /// Function keys.
    F(u8),
    /// Any other key not explicitly listed.
    Other(String),
}

/// Modifier keys held during a keyboard or mouse event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_result_default_is_no_op() {
        let result = EventResult::default();
        assert!(!result.prevent_default);
        assert!(!result.stop_propagation);
        assert!(!result.state_changed);
    }

    #[test]
    fn event_result_state_changed() {
        let result = EventResult::state_changed();
        assert!(!result.prevent_default);
        assert!(result.state_changed);
    }

    #[test]
    fn event_result_prevent_and_changed() {
        let result = EventResult::prevent_and_changed();
        assert!(result.prevent_default);
        assert!(result.state_changed);
    }

    #[test]
    fn component_event_serialization() {
        let event = ComponentEvent::Click {
            x: 10.0,
            y: 20.0,
            button: MouseButton::Left,
        };
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: ComponentEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn modifier_keys_default() {
        let mods = ModifierKeys::default();
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.meta);
    }

    #[test]
    fn key_char_variant() {
        let key = Key::Char('a');
        assert_eq!(key, Key::Char('a'));
        assert_ne!(key, Key::Char('b'));
    }
}
