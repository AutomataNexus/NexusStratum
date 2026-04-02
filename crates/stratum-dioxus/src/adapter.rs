//! Framework adapter bridging stratum-core types to Dioxus.
//!
//! Provides utilities to convert between stratum-core's framework-agnostic
//! types and Dioxus-specific rendering constructs. Mirrors the API of
//! `stratum_leptos::adapter::StratumAdapter`.

use stratum_core::event::{ComponentEvent, Key, ModifierKeys, MouseButton};
use stratum_core::render::RenderOutput;

/// The Dioxus framework adapter for NexusStratum.
pub struct DioxusAdapter;

impl DioxusAdapter {
    /// Convert a RenderOutput to HTML attribute pairs for Dioxus RSX rendering.
    pub fn render_attrs(output: &RenderOutput) -> Vec<(String, String)> {
        let mut attrs = Vec::new();

        for (key, value) in &output.attrs {
            if let Some(html_val) = value.to_html_value() {
                attrs.push((key.clone(), html_val));
            }
        }

        let class_str = output.class_string();
        if !class_str.is_empty() {
            attrs.push(("class".to_string(), class_str));
        }

        attrs.extend(output.aria.to_attr_pairs());

        for (key, value) in &output.data_attrs {
            attrs.push((format!("data-{}", key), value.clone()));
        }

        let style_str = output.style_string();
        if !style_str.is_empty() {
            attrs.push(("style".to_string(), style_str));
        }

        attrs
    }

    /// Convert a Dioxus keyboard event key string to stratum-core Key.
    pub fn key_from_str(key: &str) -> Key {
        match key {
            "Enter" => Key::Enter,
            " " | "Space" => Key::Space,
            "Escape" | "Esc" => Key::Escape,
            "Tab" => Key::Tab,
            "ArrowUp" => Key::ArrowUp,
            "ArrowDown" => Key::ArrowDown,
            "ArrowLeft" => Key::ArrowLeft,
            "ArrowRight" => Key::ArrowRight,
            "Home" => Key::Home,
            "End" => Key::End,
            "PageUp" => Key::PageUp,
            "PageDown" => Key::PageDown,
            "Backspace" => Key::Backspace,
            "Delete" => Key::Delete,
            s if s.chars().count() == 1 => Key::Char(s.chars().next().unwrap()),
            s if s.starts_with('F') => {
                if let Ok(n) = s[1..].parse::<u8>() {
                    Key::F(n)
                } else {
                    Key::Other(s.to_string())
                }
            }
            other => Key::Other(other.to_string()),
        }
    }

    /// Convert a DOM mouse button number to stratum-core MouseButton.
    ///
    /// Unknown values are mapped to `MouseButton::Left`.
    pub fn mouse_button_from_i16(button: i16) -> MouseButton {
        match button {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => MouseButton::Left,
        }
    }

    /// Create modifier keys from event state.
    pub fn modifiers(shift: bool, ctrl: bool, alt: bool, meta: bool) -> ModifierKeys {
        ModifierKeys { shift, ctrl, alt, meta }
    }

    /// Create a Click ComponentEvent.
    pub fn click_event(x: f32, y: f32, button: i16) -> ComponentEvent {
        ComponentEvent::Click {
            x,
            y,
            button: Self::mouse_button_from_i16(button),
        }
    }

    /// Create a KeyDown ComponentEvent.
    pub fn keydown_event(key: &str, shift: bool, ctrl: bool, alt: bool, meta: bool) -> ComponentEvent {
        ComponentEvent::KeyDown {
            key: Self::key_from_str(key),
            modifiers: Self::modifiers(shift, ctrl, alt, meta),
        }
    }

    /// Create a Change ComponentEvent.
    pub fn change_event(value: String) -> ComponentEvent {
        ComponentEvent::Change { value }
    }

    /// Create an Input ComponentEvent.
    pub fn input_event(value: String) -> ComponentEvent {
        ComponentEvent::Input { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stratum_core::aria::{AriaAttributes, AriaRole};

    #[test]
    fn key_from_str_named_keys() {
        assert_eq!(DioxusAdapter::key_from_str("Enter"), Key::Enter);
        assert_eq!(DioxusAdapter::key_from_str("Space"), Key::Space);
        assert_eq!(DioxusAdapter::key_from_str("Escape"), Key::Escape);
        assert_eq!(DioxusAdapter::key_from_str("Tab"), Key::Tab);
        assert_eq!(DioxusAdapter::key_from_str("ArrowUp"), Key::ArrowUp);
    }

    #[test]
    fn key_from_str_characters() {
        assert_eq!(DioxusAdapter::key_from_str("a"), Key::Char('a'));
        assert_eq!(DioxusAdapter::key_from_str("Z"), Key::Char('Z'));
    }

    #[test]
    fn key_from_str_function_keys() {
        assert_eq!(DioxusAdapter::key_from_str("F1"), Key::F(1));
        assert_eq!(DioxusAdapter::key_from_str("F12"), Key::F(12));
    }

    #[test]
    fn mouse_button_mapping() {
        assert_eq!(DioxusAdapter::mouse_button_from_i16(0), MouseButton::Left);
        assert_eq!(DioxusAdapter::mouse_button_from_i16(1), MouseButton::Middle);
        assert_eq!(DioxusAdapter::mouse_button_from_i16(2), MouseButton::Right);
        assert_eq!(DioxusAdapter::mouse_button_from_i16(99), MouseButton::Left);
    }

    #[test]
    fn click_event_creation() {
        let event = DioxusAdapter::click_event(10.0, 20.0, 0);
        match event {
            ComponentEvent::Click { x, y, button } => {
                assert_eq!(x, 10.0);
                assert_eq!(y, 20.0);
                assert_eq!(button, MouseButton::Left);
            }
            _ => panic!("Expected Click event"),
        }
    }

    #[test]
    fn keydown_event_creation() {
        let event = DioxusAdapter::keydown_event("Enter", true, false, false, false);
        match event {
            ComponentEvent::KeyDown { key, modifiers } => {
                assert_eq!(key, Key::Enter);
                assert!(modifiers.shift);
            }
            _ => panic!("Expected KeyDown event"),
        }
    }

    #[test]
    fn render_attrs_basic() {
        let output = RenderOutput::new()
            .with_tag("button")
            .with_class("btn")
            .with_aria(AriaAttributes::new().with_role(AriaRole::Button).with_disabled(false));

        let attrs = DioxusAdapter::render_attrs(&output);
        assert!(attrs.iter().any(|(k, v)| k == "class" && v.contains("btn")));
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "button"));
    }

    #[test]
    fn render_attrs_data() {
        let output = RenderOutput::new().with_data("testid", "save-btn");
        let attrs = DioxusAdapter::render_attrs(&output);
        assert!(attrs.iter().any(|(k, v)| k == "data-testid" && v == "save-btn"));
    }

    #[test]
    fn change_event_creation() {
        let event = DioxusAdapter::change_event("hello".to_string());
        match event {
            ComponentEvent::Change { value } => assert_eq!(value, "hello"),
            _ => panic!("Expected Change event"),
        }
    }
}
