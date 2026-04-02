//! Theme and toast context providers for Dioxus.

use stratum_theme::Theme;

/// Dark mode configuration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DarkMode {
    /// Follow operating system preference.
    #[default]
    System,
    /// Always light mode.
    Light,
    /// Always dark mode.
    Dark,
}

/// Theme context available to all NexusStratum components.
///
/// In Dioxus, this would be provided via `use_context_provider` and
/// accessed via `use_context`.
#[derive(Debug, Clone)]
pub struct ThemeContext {
    pub theme: Theme,
    pub dark_mode: DarkMode,
}

impl ThemeContext {
    pub fn new(theme: Theme, dark_mode: DarkMode) -> Self {
        Self { theme, dark_mode }
    }

    /// Get CSS variable declarations for the current theme.
    pub fn css_variables(&self) -> String {
        match self.dark_mode {
            DarkMode::Light => self.theme.to_css_variables(),
            DarkMode::Dark => self.theme.to_dark_css_variables(),
            DarkMode::System => {
                format!(
                    "{}\n@media (prefers-color-scheme: dark) {{\n{}\n}}",
                    self.theme.to_css_variables(),
                    self.theme.to_dark_css_variables()
                )
            }
        }
    }

    /// Whether the current mode is dark.
    ///
    /// Returns false for System mode since OS preference is determined
    /// at runtime in the browser via CSS media queries.
    pub fn is_dark(&self) -> bool {
        matches!(self.dark_mode, DarkMode::Dark)
    }
}

/// Toast notification context.
#[derive(Debug, Clone)]
pub struct ToasterContext {
    pub max_visible: usize,
    pub default_duration_ms: u32,
    pub position: ToastPosition,
}

impl Default for ToasterContext {
    fn default() -> Self {
        Self {
            max_visible: 5,
            default_duration_ms: 5000,
            position: ToastPosition::BottomRight,
        }
    }
}

impl ToasterContext {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Position of the toast container on screen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl ToastPosition {
    pub fn css_classes(&self) -> &'static str {
        match self {
            Self::TopLeft => "fixed top-4 left-4",
            Self::TopCenter => "fixed top-4 left-1/2 -translate-x-1/2",
            Self::TopRight => "fixed top-4 right-4",
            Self::BottomLeft => "fixed bottom-4 left-4",
            Self::BottomCenter => "fixed bottom-4 left-1/2 -translate-x-1/2",
            Self::BottomRight => "fixed bottom-4 right-4",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_context_css_variables() {
        let ctx = ThemeContext::new(Theme::default(), DarkMode::Light);
        let css = ctx.css_variables();
        assert!(css.contains("--stratum"));
    }

    #[test]
    fn theme_context_dark_mode() {
        let ctx = ThemeContext::new(Theme::default(), DarkMode::Dark);
        assert!(ctx.is_dark());
        let ctx_light = ThemeContext::new(Theme::default(), DarkMode::Light);
        assert!(!ctx_light.is_dark());
    }

    #[test]
    fn theme_context_system_includes_both() {
        let ctx = ThemeContext::new(Theme::default(), DarkMode::System);
        let css = ctx.css_variables();
        assert!(css.contains("prefers-color-scheme"));
    }

    #[test]
    fn toaster_context_defaults() {
        let ctx = ToasterContext::new();
        assert_eq!(ctx.max_visible, 5);
        assert_eq!(ctx.default_duration_ms, 5000);
        assert_eq!(ctx.position, ToastPosition::BottomRight);
    }

    #[test]
    fn toast_position_classes() {
        assert!(ToastPosition::BottomRight.css_classes().contains("bottom"));
        assert!(ToastPosition::TopLeft.css_classes().contains("top"));
    }
}
