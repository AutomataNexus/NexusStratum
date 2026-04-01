use serde::{Deserialize, Serialize};

/// Properties for rendering an icon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconProps {
    /// Icon size in pixels (applied to both width and height).
    pub size: u32,
    /// Icon color (CSS color value). Default: "currentColor".
    pub color: String,
    /// SVG stroke width. Default: 2.0.
    pub stroke_width: f32,
    /// Additional CSS classes.
    pub class: String,
    /// Accessible label. If None, icon is decorative (aria-hidden="true").
    pub aria_label: Option<String>,
    /// Whether the icon is hidden from assistive technology. Default: true.
    pub aria_hidden: bool,
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            size: 24,
            color: "currentColor".to_string(),
            stroke_width: 2.0,
            class: String::new(),
            aria_label: None,
            aria_hidden: true,
        }
    }
}

impl IconProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self.aria_hidden = false;
        self
    }
}

/// An icon definition containing SVG path data.
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    /// The icon name (e.g., "chevron-down").
    pub name: &'static str,
    /// The SVG path data (inner content of the `<svg>` tag).
    pub svg_content: &'static str,
    /// Default viewBox dimensions (typically "0 0 24 24").
    pub view_box: &'static str,
}

impl Icon {
    /// Render this icon as an SVG string with the given props.
    pub fn render(&self, props: &IconProps) -> String {
        let mut attrs = format!(
            "xmlns=\"http://www.w3.org/2000/svg\" \
             width=\"{}\" height=\"{}\" \
             viewBox=\"{}\" \
             fill=\"none\" \
             stroke=\"{}\" \
             stroke-width=\"{}\" \
             stroke-linecap=\"round\" \
             stroke-linejoin=\"round\"",
            props.size, props.size, self.view_box, props.color, props.stroke_width
        );

        if !props.class.is_empty() {
            attrs.push_str(&format!(" class=\"{}\"", props.class));
        }

        if props.aria_hidden {
            attrs.push_str(" aria-hidden=\"true\"");
        }
        if let Some(ref label) = props.aria_label {
            attrs.push_str(&format!(
                " role=\"img\" aria-label=\"{}\"",
                stratum_core::security::escape_attr(label)
            ));
        }

        format!("<svg {}>{}</svg>", attrs, self.svg_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_props_default() {
        let props = IconProps::default();
        assert_eq!(props.size, 24);
        assert_eq!(props.color, "currentColor");
        assert_eq!(props.stroke_width, 2.0);
        assert!(props.aria_hidden);
        assert!(props.aria_label.is_none());
    }

    #[test]
    fn icon_props_builder() {
        let props = IconProps::new()
            .with_size(16)
            .with_color("red")
            .with_stroke_width(1.5)
            .with_class("my-icon");

        assert_eq!(props.size, 16);
        assert_eq!(props.color, "red");
        assert_eq!(props.stroke_width, 1.5);
        assert_eq!(props.class, "my-icon");
    }

    #[test]
    fn icon_props_with_label() {
        let props = IconProps::new().with_label("Close dialog");
        assert_eq!(props.aria_label, Some("Close dialog".to_string()));
        assert!(!props.aria_hidden);
    }

    #[test]
    fn icon_render_decorative() {
        let icon = Icon {
            name: "x",
            svg_content: "<line x1=\"18\" y1=\"6\" x2=\"6\" y2=\"18\"/><line x1=\"6\" y1=\"6\" x2=\"18\" y2=\"18\"/>",
            view_box: "0 0 24 24",
        };
        let html = icon.render(&IconProps::default());
        assert!(html.contains("aria-hidden=\"true\""));
        assert!(html.contains("width=\"24\""));
        assert!(html.contains("stroke=\"currentColor\""));
        assert!(html.contains("<line"));
    }

    #[test]
    fn icon_render_accessible() {
        let icon = Icon {
            name: "x",
            svg_content: "<line x1=\"18\" y1=\"6\" x2=\"6\" y2=\"18\"/>",
            view_box: "0 0 24 24",
        };
        let props = IconProps::new().with_label("Close").with_size(16);
        let html = icon.render(&props);
        assert!(html.contains("role=\"img\""));
        assert!(html.contains("aria-label=\"Close\""));
        assert!(html.contains("width=\"16\""));
        assert!(!html.contains("aria-hidden"));
    }

    #[test]
    fn icon_render_xss_prevention() {
        let icon = Icon {
            name: "test",
            svg_content: "<circle/>",
            view_box: "0 0 24 24",
        };
        let props = IconProps::new().with_label("test\" onmouseover=\"alert(1)");
        let html = icon.render(&props);
        // The injected attribute quotes should be escaped so the attribute can't break out
        assert!(html.contains("&quot;"));
        // The onmouseover should be inside the escaped aria-label value, not a standalone attribute
        assert!(!html.contains("\" onmouseover=\""));
    }
}
