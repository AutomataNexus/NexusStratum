//! Programmatic Tailwind class builder.
//!
//! [`ClassBuilder`] provides a fluent API for constructing class strings
//! with conditional logic and automatic conflict resolution via [`merge_classes`].

use crate::merge::merge_classes;

/// Builder for constructing Tailwind class strings.
///
/// # Examples
///
/// ```
/// use stratum_tailwind::ClassBuilder;
///
/// let classes = ClassBuilder::new()
///     .add("flex items-center h-8")
///     .add_if(true, "bg-primary text-white")
///     .add_if(false, "bg-secondary")
///     .build();
///
/// assert_eq!(classes, "flex items-center h-8 bg-primary text-white");
/// ```
pub struct ClassBuilder {
    classes: Vec<String>,
}

impl ClassBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    /// Add static classes.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, classes: &str) -> Self {
        if !classes.is_empty() {
            self.classes.push(classes.to_string());
        }
        self
    }

    /// Add classes conditionally.
    pub fn add_if(mut self, condition: bool, classes: &str) -> Self {
        if condition && !classes.is_empty() {
            self.classes.push(classes.to_string());
        }
        self
    }

    /// Add one of two class sets based on condition.
    pub fn add_either(mut self, condition: bool, if_true: &str, if_false: &str) -> Self {
        let chosen = if condition { if_true } else { if_false };
        if !chosen.is_empty() {
            self.classes.push(chosen.to_string());
        }
        self
    }

    /// Build the final class string with conflict resolution.
    pub fn build(self) -> String {
        let refs: Vec<&str> = self.classes.iter().map(|s| s.as_str()).collect();
        merge_classes(&refs)
    }

    /// Build and merge with user-provided override classes.
    ///
    /// Override classes are appended last, so they take precedence
    /// over any conflicting builder classes.
    pub fn build_with_override(self, user_class: Option<&str>) -> String {
        let mut refs: Vec<&str> = self.classes.iter().map(|s| s.as_str()).collect();
        if let Some(overrides) = user_class {
            refs.push(overrides);
        }
        merge_classes(&refs)
    }
}

impl Default for ClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let result = ClassBuilder::new()
            .add("flex items-center")
            .add("h-8 px-3")
            .build();
        assert_eq!(result, "flex items-center h-8 px-3");
    }

    #[test]
    fn conditional_classes_true() {
        let result = ClassBuilder::new()
            .add("flex")
            .add_if(true, "bg-primary")
            .build();
        assert_eq!(result, "flex bg-primary");
    }

    #[test]
    fn conditional_classes_false() {
        let result = ClassBuilder::new()
            .add("flex")
            .add_if(false, "bg-primary")
            .build();
        assert_eq!(result, "flex");
    }

    #[test]
    fn either_classes() {
        let active = ClassBuilder::new()
            .add("btn")
            .add_either(true, "bg-primary text-white", "bg-secondary text-black")
            .build();
        assert_eq!(active, "btn bg-primary text-white");

        let inactive = ClassBuilder::new()
            .add("btn")
            .add_either(false, "bg-primary text-white", "bg-secondary text-black")
            .build();
        assert_eq!(inactive, "btn bg-secondary text-black");
    }

    #[test]
    fn conflict_resolution_in_builder() {
        let result = ClassBuilder::new().add("h-8 px-3").add("h-12").build();
        assert_eq!(result, "px-3 h-12");
    }

    #[test]
    fn with_override_none() {
        let result = ClassBuilder::new()
            .add("h-8 px-3")
            .build_with_override(None);
        assert_eq!(result, "h-8 px-3");
    }

    #[test]
    fn with_override_some() {
        let result = ClassBuilder::new()
            .add("h-8 px-3 bg-primary")
            .build_with_override(Some("h-12 bg-secondary"));
        assert_eq!(result, "px-3 h-12 bg-secondary");
    }

    #[test]
    fn empty_builder() {
        let result = ClassBuilder::new().build();
        assert_eq!(result, "");
    }

    #[test]
    fn default_impl() {
        let result = ClassBuilder::default().add("flex").build();
        assert_eq!(result, "flex");
    }
}
