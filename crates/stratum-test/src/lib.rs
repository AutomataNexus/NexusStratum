//! # stratum-test
//!
//! Test utilities for NexusStratum components.
//!
//! Provides assertion helpers and test harness utilities for verifying
//! component render output, accessibility attributes, and state transitions.
//!
//! ## Usage
//!
//! ```ignore
//! use stratum_test::*;
//! use stratum_core::aria::AriaRole;
//!
//! let output = MyComponent::render(&props);
//! assert!(assert_aria_role(&output, AriaRole::Button));
//! assert!(assert_has_class(&output, "btn-primary"));
//! ```

use stratum_core::aria::AriaRole;
use stratum_core::render::RenderOutput;

/// Assert that a `RenderOutput` has the expected ARIA role.
///
/// Returns `true` if the role matches, `false` otherwise.
pub fn assert_aria_role(output: &RenderOutput, expected: AriaRole) -> bool {
    output.aria.role == Some(expected)
}

/// Assert that a `RenderOutput` contains the specified CSS class.
///
/// Returns `true` if the class is present.
pub fn assert_has_class(output: &RenderOutput, class: &str) -> bool {
    output.classes.iter().any(|c| c == class)
}

/// Assert that a `RenderOutput` has an HTML attribute with the given name.
///
/// Returns `true` if an attribute with that name exists (regardless of value).
pub fn assert_has_attr(output: &RenderOutput, name: &str) -> bool {
    output.attrs.iter().any(|(k, _)| k == name)
}

/// Assert that a `RenderOutput` has an HTML attribute with the expected string value.
///
/// Returns `true` if the attribute exists and its value matches.
pub fn assert_attr_value(output: &RenderOutput, name: &str, expected: &str) -> bool {
    output.attrs.iter().any(|(k, v)| {
        k == name && v.to_html_value().as_deref() == Some(expected)
    })
}

/// Assert that a `RenderOutput` has a data attribute with the given name.
///
/// Returns `true` if a `data-{name}` attribute is present.
pub fn assert_has_data_attr(output: &RenderOutput, name: &str) -> bool {
    output.data_attrs.iter().any(|(k, _)| k == name)
}

/// Assert that a `RenderOutput` does **not** have the `aria-hidden` attribute
/// set to `"true"`, ensuring the element is visible to assistive technology.
pub fn assert_not_aria_hidden(output: &RenderOutput) -> bool {
    output.aria.hidden != Some(true)
}

/// Assert that the render output uses the expected HTML tag.
///
/// Returns `true` if the effective tag matches.
pub fn assert_tag(output: &RenderOutput, expected: &str) -> bool {
    output.effective_tag() == expected
}

#[cfg(test)]
mod tests {
    use super::*;
    use stratum_core::aria::AriaAttributes;

    #[test]
    fn test_assert_aria_role_match() {
        let output = RenderOutput::new()
            .with_aria(AriaAttributes::new().with_role(AriaRole::Button));
        assert!(assert_aria_role(&output, AriaRole::Button));
    }

    #[test]
    fn test_assert_aria_role_mismatch() {
        let output = RenderOutput::new()
            .with_aria(AriaAttributes::new().with_role(AriaRole::Link));
        assert!(!assert_aria_role(&output, AriaRole::Button));
    }

    #[test]
    fn test_assert_has_class() {
        let output = RenderOutput::new()
            .with_class("btn")
            .with_class("btn-primary");
        assert!(assert_has_class(&output, "btn"));
        assert!(assert_has_class(&output, "btn-primary"));
        assert!(!assert_has_class(&output, "btn-secondary"));
    }

    #[test]
    fn test_assert_has_data_attr() {
        let output = RenderOutput::new()
            .with_data("testid", "save-btn");
        assert!(assert_has_data_attr(&output, "testid"));
        assert!(!assert_has_data_attr(&output, "other"));
    }

    #[test]
    fn test_assert_tag() {
        let output = RenderOutput::new().with_tag("button");
        assert!(assert_tag(&output, "button"));
        assert!(!assert_tag(&output, "div"));
    }

    #[test]
    fn test_assert_tag_default() {
        let output = RenderOutput::new();
        assert!(assert_tag(&output, "div"));
    }
}
