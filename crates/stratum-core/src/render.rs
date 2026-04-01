use crate::aria::AriaAttributes;
use serde::{Deserialize, Serialize};

/// Framework-agnostic render description produced by components.
///
/// A `RenderOutput` describes what a component looks like without
/// committing to any specific framework's rendering model. Framework
/// adapters translate this into Leptos `view!` or Dioxus `rsx!` output.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RenderOutput {
    /// HTML attributes to set on the root element.
    pub attrs: Vec<(String, AttrValue)>,

    /// CSS class names to add to the root element.
    pub classes: Vec<String>,

    /// ARIA attributes for accessibility.
    pub aria: AriaAttributes,

    /// Child content specification.
    pub children: ChildrenSpec,

    /// Data attributes (data-*) for testing and JS interop.
    pub data_attrs: Vec<(String, String)>,

    /// The HTML tag name to render (default: "div").
    pub tag: Option<String>,

    /// Inline styles (escape hatch — prefer classes).
    pub styles: Vec<(String, String)>,
}

impl RenderOutput {
    /// Create a new empty RenderOutput.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the HTML tag name.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Add an HTML attribute.
    pub fn with_attr(mut self, name: impl Into<String>, value: AttrValue) -> Self {
        self.attrs.push((name.into(), value));
        self
    }

    /// Add a CSS class.
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add multiple CSS classes.
    pub fn with_classes(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.classes.extend(classes.into_iter().map(|c| c.into()));
        self
    }

    /// Set the ARIA attributes.
    pub fn with_aria(mut self, aria: AriaAttributes) -> Self {
        self.aria = aria;
        self
    }

    /// Set the children specification.
    pub fn with_children(mut self, children: ChildrenSpec) -> Self {
        self.children = children;
        self
    }

    /// Add a data attribute.
    pub fn with_data(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.data_attrs.push((name.into(), value.into()));
        self
    }

    /// Add an inline style.
    pub fn with_style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.styles.push((property.into(), value.into()));
        self
    }

    /// Get the effective tag name (defaults to "div").
    pub fn effective_tag(&self) -> &str {
        self.tag.as_deref().unwrap_or("div")
    }

    /// Get all classes as a single space-separated string.
    pub fn class_string(&self) -> String {
        self.classes.join(" ")
    }

    /// Get all inline styles as a CSS string.
    pub fn style_string(&self) -> String {
        self.styles
            .iter()
            .map(|(prop, val)| format!("{}: {};", prop, val))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Merge another RenderOutput into this one.
    ///
    /// Attributes, classes, data attributes, and styles are appended.
    /// ARIA attributes from `other` overwrite `self` where set.
    /// Tag and children from `other` take precedence if set.
    pub fn merge(mut self, other: RenderOutput) -> Self {
        self.attrs.extend(other.attrs);
        self.classes.extend(other.classes);
        self.data_attrs.extend(other.data_attrs);
        self.styles.extend(other.styles);

        if other.tag.is_some() {
            self.tag = other.tag;
        }
        if other.children != ChildrenSpec::default() {
            self.children = other.children;
        }

        // Merge ARIA: other's Some values overwrite self's
        macro_rules! merge_aria_field {
            ($field:ident) => {
                if other.aria.$field.is_some() {
                    self.aria.$field = other.aria.$field;
                }
            };
        }
        merge_aria_field!(role);
        merge_aria_field!(label);
        merge_aria_field!(labelledby);
        merge_aria_field!(describedby);
        merge_aria_field!(expanded);
        merge_aria_field!(selected);
        merge_aria_field!(checked);
        merge_aria_field!(disabled);
        merge_aria_field!(required);
        merge_aria_field!(invalid);
        merge_aria_field!(live);
        merge_aria_field!(atomic);
        merge_aria_field!(controls);
        merge_aria_field!(owns);
        merge_aria_field!(haspopup);
        merge_aria_field!(level);
        merge_aria_field!(orientation);
        merge_aria_field!(readonly);
        merge_aria_field!(multiselectable);
        merge_aria_field!(valuemin);
        merge_aria_field!(valuemax);
        merge_aria_field!(valuenow);
        merge_aria_field!(valuetext);
        merge_aria_field!(hidden);
        merge_aria_field!(activedescendant);
        merge_aria_field!(busy);
        merge_aria_field!(modal);

        self
    }
}

/// An HTML attribute value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttrValue {
    /// A string value.
    String(String),
    /// A boolean attribute (present or absent).
    Bool(bool),
    /// A numeric value.
    Number(f64),
    /// Attribute is not set.
    None,
}

impl AttrValue {
    /// Convert to a string representation for HTML rendering.
    pub fn to_html_value(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            Self::Bool(true) => Some(String::new()),
            Self::Bool(false) => None,
            Self::Number(n) => Some(n.to_string()),
            Self::None => None,
        }
    }
}

impl From<String> for AttrValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for AttrValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<bool> for AttrValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<f64> for AttrValue {
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}

/// Specification for component children.
///
/// Components can render different types of children — slots, text,
/// or delegate to the consumer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum ChildrenSpec {
    /// No children.
    Empty,
    /// Static text content.
    Text(String),
    /// Named slot — the consumer provides content for this slot.
    Slot(String),
    /// Multiple named slots.
    Slots(Vec<String>),
    /// Consumer-provided children (the default for most components).
    #[default]
    Children,
    /// Multiple child render outputs (compound components).
    Elements(Vec<RenderOutput>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aria::AriaRole;

    #[test]
    fn render_output_builder() {
        let output = RenderOutput::new()
            .with_tag("button")
            .with_class("btn")
            .with_class("btn-primary")
            .with_attr("type", AttrValue::String("button".to_string()))
            .with_data("testid", "save-btn");

        assert_eq!(output.effective_tag(), "button");
        assert_eq!(output.class_string(), "btn btn-primary");
        assert_eq!(output.data_attrs.len(), 1);
        assert_eq!(output.attrs.len(), 1);
    }

    #[test]
    fn render_output_default_tag() {
        let output = RenderOutput::new();
        assert_eq!(output.effective_tag(), "div");
    }

    #[test]
    fn render_output_style_string() {
        let output = RenderOutput::new()
            .with_style("display", "flex")
            .with_style("gap", "8px");
        assert_eq!(output.style_string(), "display: flex; gap: 8px;");
    }

    #[test]
    fn render_output_merge() {
        let base = RenderOutput::new()
            .with_class("base")
            .with_aria(AriaAttributes::new().with_role(AriaRole::Button));

        let overlay = RenderOutput::new()
            .with_class("overlay")
            .with_aria(AriaAttributes::new().with_label("Save"));

        let merged = base.merge(overlay);
        assert_eq!(merged.classes, vec!["base", "overlay"]);
        assert_eq!(merged.aria.role, Some(AriaRole::Button));
        assert_eq!(merged.aria.label, Some("Save".to_string()));
    }

    #[test]
    fn attr_value_to_html() {
        assert_eq!(
            AttrValue::String("hello".to_string()).to_html_value(),
            Some("hello".to_string())
        );
        assert_eq!(AttrValue::Bool(true).to_html_value(), Some(String::new()));
        assert_eq!(AttrValue::Bool(false).to_html_value(), None);
        assert_eq!(
            AttrValue::Number(42.0).to_html_value(),
            Some("42".to_string())
        );
        assert_eq!(AttrValue::None.to_html_value(), None);
    }

    #[test]
    fn children_spec_default() {
        assert_eq!(ChildrenSpec::default(), ChildrenSpec::Children);
    }

    #[test]
    fn attr_value_from_impls() {
        let _: AttrValue = "hello".into();
        let _: AttrValue = String::from("hello").into();
        let _: AttrValue = true.into();
        let _: AttrValue = 3.14.into();
    }
}
