//! Framework adapter bridging stratum-core types to Leptos.
//!
//! This module provides the `StratumAdapter` that converts between
//! NexusStratum's framework-agnostic types and Leptos-specific constructs.

use stratum_core::event::{ComponentEvent, Key, ModifierKeys, MouseButton};
use stratum_core::render::RenderOutput;

/// The Leptos framework adapter for NexusStratum.
///
/// Provides utilities to convert between stratum-core types and
/// Leptos-specific rendering constructs.
pub struct StratumAdapter;

impl StratumAdapter {
    /// Convert a RenderOutput to HTML attribute pairs for Leptos rendering.
    ///
    /// This generates the attribute key-value pairs that Leptos components
    /// use in `view!` macro output.
    pub fn render_attrs(output: &RenderOutput) -> Vec<(String, String)> {
        let mut attrs = Vec::new();

        // HTML attributes
        for (key, value) in &output.attrs {
            if let Some(html_val) = value.to_html_value() {
                attrs.push((key.clone(), html_val));
            }
        }

        // Class
        let class_str = output.class_string();
        if !class_str.is_empty() {
            attrs.push(("class".to_string(), class_str));
        }

        // ARIA attributes
        attrs.extend(output.aria.to_attr_pairs());

        // Data attributes
        for (key, value) in &output.data_attrs {
            attrs.push((format!("data-{}", key), value.clone()));
        }

        // Style
        let style_str = output.style_string();
        if !style_str.is_empty() {
            attrs.push(("style".to_string(), style_str));
        }

        attrs
    }

    /// Convert a DOM keyboard event key string to stratum-core Key.
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
            s if s.len() == 1 => Key::Char(s.chars().next().unwrap()),
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

    /// Convert DOM mouse button number to stratum-core MouseButton.
    pub fn mouse_button_from_u16(button: u16) -> MouseButton {
        match button {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => MouseButton::Left,
        }
    }

    /// Create modifier keys from DOM event modifier state.
    pub fn modifiers(shift: bool, ctrl: bool, alt: bool, meta: bool) -> ModifierKeys {
        ModifierKeys {
            shift,
            ctrl,
            alt,
            meta,
        }
    }

    /// Create a Click ComponentEvent from DOM coordinates and button.
    pub fn click_event(x: f32, y: f32, button: u16) -> ComponentEvent {
        ComponentEvent::Click {
            x,
            y,
            button: Self::mouse_button_from_u16(button),
        }
    }

    /// Create a KeyDown ComponentEvent from DOM key string and modifiers.
    pub fn keydown_event(
        key: &str,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    ) -> ComponentEvent {
        ComponentEvent::KeyDown {
            key: Self::key_from_str(key),
            modifiers: Self::modifiers(shift, ctrl, alt, meta),
        }
    }

    /// Create a Change ComponentEvent from an input value.
    pub fn change_event(value: String) -> ComponentEvent {
        ComponentEvent::Change { value }
    }

    /// Create an Input ComponentEvent from an input value.
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
        assert_eq!(StratumAdapter::key_from_str("Enter"), Key::Enter);
        assert_eq!(StratumAdapter::key_from_str("Space"), Key::Space);
        assert_eq!(StratumAdapter::key_from_str(" "), Key::Space);
        assert_eq!(StratumAdapter::key_from_str("Escape"), Key::Escape);
        assert_eq!(StratumAdapter::key_from_str("Tab"), Key::Tab);
        assert_eq!(StratumAdapter::key_from_str("ArrowUp"), Key::ArrowUp);
        assert_eq!(StratumAdapter::key_from_str("Home"), Key::Home);
        assert_eq!(StratumAdapter::key_from_str("End"), Key::End);
    }

    #[test]
    fn key_from_str_characters() {
        assert_eq!(StratumAdapter::key_from_str("a"), Key::Char('a'));
        assert_eq!(StratumAdapter::key_from_str("Z"), Key::Char('Z'));
        assert_eq!(StratumAdapter::key_from_str("1"), Key::Char('1'));
    }

    #[test]
    fn key_from_str_function_keys() {
        assert_eq!(StratumAdapter::key_from_str("F1"), Key::F(1));
        assert_eq!(StratumAdapter::key_from_str("F12"), Key::F(12));
    }

    #[test]
    fn mouse_button_mapping() {
        assert_eq!(
            StratumAdapter::mouse_button_from_u16(0),
            MouseButton::Left
        );
        assert_eq!(
            StratumAdapter::mouse_button_from_u16(1),
            MouseButton::Middle
        );
        assert_eq!(
            StratumAdapter::mouse_button_from_u16(2),
            MouseButton::Right
        );
    }

    #[test]
    fn click_event_creation() {
        let event = StratumAdapter::click_event(10.0, 20.0, 0);
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
        let event = StratumAdapter::keydown_event("Enter", true, false, false, false);
        match event {
            ComponentEvent::KeyDown { key, modifiers } => {
                assert_eq!(key, Key::Enter);
                assert!(modifiers.shift);
                assert!(!modifiers.ctrl);
            }
            _ => panic!("Expected KeyDown event"),
        }
    }

    #[test]
    fn render_attrs_basic() {
        let output = RenderOutput::new()
            .with_tag("button")
            .with_class("btn")
            .with_class("btn-primary")
            .with_aria(AriaAttributes::new().with_role(AriaRole::Button).with_disabled(false));

        let attrs = StratumAdapter::render_attrs(&output);
        assert!(attrs.iter().any(|(k, v)| k == "class" && v.contains("btn")));
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "button"));
    }

    #[test]
    fn render_attrs_data_attributes() {
        let output = RenderOutput::new().with_data("testid", "save-btn");
        let attrs = StratumAdapter::render_attrs(&output);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-testid" && v == "save-btn"));
    }

    #[test]
    fn render_attrs_style() {
        let output = RenderOutput::new()
            .with_style("display", "flex")
            .with_style("gap", "8px");
        let attrs = StratumAdapter::render_attrs(&output);
        assert!(attrs.iter().any(|(k, v)| k == "style" && v.contains("flex")));
    }

    #[test]
    fn change_event_creation() {
        let event = StratumAdapter::change_event("hello".to_string());
        match event {
            ComponentEvent::Change { value } => assert_eq!(value, "hello"),
            _ => panic!("Expected Change event"),
        }
    }
}
