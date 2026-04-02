//! Component metadata extraction for tooling and documentation.
//!
//! Provides structured metadata about NexusStratum components that powers
//! IDE plugins, documentation generators, and the component explorer.

use serde::{Deserialize, Serialize};

/// Metadata for a single component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMeta {
    /// Component name (e.g., "Button").
    pub name: String,
    /// Crate the component belongs to.
    pub crate_name: String,
    /// Category (e.g., "Forms", "Overlay").
    pub category: String,
    /// Short description.
    pub description: String,
    /// Props metadata.
    pub props: Vec<PropMeta>,
    /// ARIA role if applicable.
    pub aria_role: Option<String>,
    /// Keyboard interactions.
    pub keyboard: Vec<KeyboardMeta>,
}

/// Metadata for a single prop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropMeta {
    /// Prop name (e.g., "variant").
    pub name: String,
    /// Rust type as a string (e.g., "ButtonVariant").
    pub prop_type: String,
    /// Default value as a string (e.g., "Default").
    pub default: Option<String>,
    /// Whether the prop is required.
    pub required: bool,
    /// Description.
    pub description: String,
    /// Possible values for enum types.
    pub values: Vec<String>,
}

/// Metadata for a keyboard interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardMeta {
    /// Key or key combination (e.g., "Enter", "Arrow keys").
    pub key: String,
    /// What the key does.
    pub action: String,
}

/// The full component registry metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMeta {
    /// Schema version.
    pub version: String,
    /// All components.
    pub components: Vec<ComponentMeta>,
}

impl RegistryMeta {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            version: "0.1.0".to_string(),
            components: Vec::new(),
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserialize from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get the built-in NexusStratum component registry.
    pub fn builtin() -> Self {
        let mut reg = Self::new();

        reg.components.push(ComponentMeta {
            name: "Button".to_string(),
            crate_name: "stratum-components".to_string(),
            category: "Forms".to_string(),
            description: "A button with multiple variants and sizes.".to_string(),
            props: vec![
                PropMeta {
                    name: "variant".to_string(),
                    prop_type: "ButtonVariant".to_string(),
                    default: Some("Default".to_string()),
                    required: false,
                    description: "Visual style variant.".to_string(),
                    values: vec!["Default".into(), "Destructive".into(), "Outline".into(), "Secondary".into(), "Ghost".into(), "Link".into()],
                },
                PropMeta {
                    name: "size".to_string(),
                    prop_type: "Size".to_string(),
                    default: Some("Md".to_string()),
                    required: false,
                    description: "Button size.".to_string(),
                    values: vec!["Xs".into(), "Sm".into(), "Md".into(), "Lg".into(), "Xl".into()],
                },
                PropMeta {
                    name: "disabled".to_string(),
                    prop_type: "bool".to_string(),
                    default: Some("false".to_string()),
                    required: false,
                    description: "Prevents interaction.".to_string(),
                    values: vec![],
                },
                PropMeta {
                    name: "loading".to_string(),
                    prop_type: "bool".to_string(),
                    default: Some("false".to_string()),
                    required: false,
                    description: "Shows loading spinner.".to_string(),
                    values: vec![],
                },
            ],
            aria_role: Some("button".to_string()),
            keyboard: vec![
                KeyboardMeta { key: "Enter".to_string(), action: "Activates the button.".to_string() },
                KeyboardMeta { key: "Space".to_string(), action: "Activates the button.".to_string() },
            ],
        });

        reg.components.push(ComponentMeta {
            name: "Dialog".to_string(),
            crate_name: "stratum-components".to_string(),
            category: "Overlay".to_string(),
            description: "A modal dialog with focus trap and backdrop.".to_string(),
            props: vec![
                PropMeta { name: "open".to_string(), prop_type: "Option<bool>".to_string(), default: None, required: false, description: "Controlled open state.".to_string(), values: vec![] },
                PropMeta { name: "modal".to_string(), prop_type: "bool".to_string(), default: Some("true".to_string()), required: false, description: "Whether to trap focus.".to_string(), values: vec![] },
            ],
            aria_role: Some("dialog".to_string()),
            keyboard: vec![
                KeyboardMeta { key: "Escape".to_string(), action: "Closes the dialog.".to_string() },
                KeyboardMeta { key: "Tab".to_string(), action: "Cycles focus within the dialog.".to_string() },
            ],
        });

        reg.components.push(ComponentMeta {
            name: "Tabs".to_string(),
            crate_name: "stratum-components".to_string(),
            category: "Navigation".to_string(),
            description: "Tabbed interface with arrow key navigation.".to_string(),
            props: vec![
                PropMeta { name: "value".to_string(), prop_type: "Option<String>".to_string(), default: None, required: false, description: "Controlled active tab.".to_string(), values: vec![] },
                PropMeta { name: "orientation".to_string(), prop_type: "Orientation".to_string(), default: Some("Horizontal".to_string()), required: false, description: "Tab direction.".to_string(), values: vec!["Horizontal".into(), "Vertical".into()] },
            ],
            aria_role: Some("tablist".to_string()),
            keyboard: vec![
                KeyboardMeta { key: "Arrow keys".to_string(), action: "Navigate between tabs.".to_string() },
                KeyboardMeta { key: "Home/End".to_string(), action: "Jump to first/last tab.".to_string() },
            ],
        });

        reg
    }

    /// Find a component by name (case-insensitive).
    pub fn find(&self, name: &str) -> Option<&ComponentMeta> {
        let lower = name.to_lowercase();
        self.components.iter().find(|c| c.name.to_lowercase() == lower)
    }
}

impl Default for RegistryMeta {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_registry() {
        let reg = RegistryMeta::builtin();
        assert!(reg.components.len() >= 3);
        assert!(reg.find("Button").is_some());
        assert!(reg.find("Dialog").is_some());
        assert!(reg.find("Tabs").is_some());
    }

    #[test]
    fn find_case_insensitive() {
        let reg = RegistryMeta::builtin();
        assert!(reg.find("button").is_some());
        assert!(reg.find("DIALOG").is_some());
        assert!(reg.find("nonexistent").is_none());
    }

    #[test]
    fn json_roundtrip() {
        let reg = RegistryMeta::builtin();
        let json = reg.to_json();
        assert!(json.contains("Button"));
        let parsed = RegistryMeta::from_json(&json).unwrap();
        assert_eq!(parsed.components.len(), reg.components.len());
    }

    #[test]
    fn button_props() {
        let reg = RegistryMeta::builtin();
        let button = reg.find("Button").unwrap();
        assert_eq!(button.props.len(), 4);
        assert_eq!(button.aria_role, Some("button".to_string()));
        let variant = button.props.iter().find(|p| p.name == "variant").unwrap();
        assert_eq!(variant.values.len(), 6);
    }

    #[test]
    fn dialog_keyboard() {
        let reg = RegistryMeta::builtin();
        let dialog = reg.find("Dialog").unwrap();
        assert!(dialog.keyboard.iter().any(|k| k.key == "Escape"));
    }
}
