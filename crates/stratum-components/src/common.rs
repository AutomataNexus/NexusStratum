//! Shared types used across all stratum-components.

/// Standard size enum used by most components.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Size {
    /// Extra small
    Xs,
    /// Small
    Sm,
    /// Medium (default)
    #[default]
    Md,
    /// Large
    Lg,
    /// Extra large
    Xl,
}

impl Size {
    /// Get the string representation of this size.
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::Xs => "xs",
            Size::Sm => "sm",
            Size::Md => "md",
            Size::Lg => "lg",
            Size::Xl => "xl",
        }
    }
}

/// Helper to merge a user-provided class override into a computed class string.
pub fn merge_classes(computed: &str, user_class: &Option<String>) -> String {
    match user_class {
        Some(extra) => format!("{} {}", computed, extra),
        None => computed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_default_is_md() {
        assert_eq!(Size::default(), Size::Md);
    }

    #[test]
    fn size_as_str() {
        assert_eq!(Size::Xs.as_str(), "xs");
        assert_eq!(Size::Sm.as_str(), "sm");
        assert_eq!(Size::Md.as_str(), "md");
        assert_eq!(Size::Lg.as_str(), "lg");
        assert_eq!(Size::Xl.as_str(), "xl");
    }

    #[test]
    fn merge_classes_without_override() {
        let result = merge_classes("btn btn-primary", &None);
        assert_eq!(result, "btn btn-primary");
    }

    #[test]
    fn merge_classes_with_override() {
        let result = merge_classes("btn btn-primary", &Some("my-custom".to_string()));
        assert_eq!(result, "btn btn-primary my-custom");
    }
}
