//! Theme and toast context providers for Leptos.

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
/// In Leptos, this would be provided via `provide_context` in the
/// `ThemeProvider` component. Components access it via `use_context`.
#[derive(Debug, Clone)]
pub struct ThemeContext {
    /// The active theme.
    pub theme: Theme,
    /// Dark mode setting.
    pub dark_mode: DarkMode,
}

impl ThemeContext {
    /// Create a new theme context.
    pub fn new(theme: Theme, dark_mode: DarkMode) -> Self {
        Self { theme, dark_mode }
    }

    /// Get CSS variable declarations for the current theme.
    pub fn css_variables(&self) -> String {
        match self.dark_mode {
            DarkMode::Light => self.theme.to_css_variables(),
            DarkMode::Dark => self.theme.to_dark_css_variables(),
            DarkMode::System => {
                // Include both light and dark, using prefers-color-scheme
                format!(
                    "{}\n@media (prefers-color-scheme: dark) {{\n{}\n}}",
                    self.theme.to_css_variables(),
                    self.theme.to_dark_css_variables()
                )
            }
        }
    }

    /// Whether the current mode is dark.
    pub fn is_dark(&self) -> bool {
        matches!(self.dark_mode, DarkMode::Dark)
    }
}

/// Toast notification context for managing toast state.
#[derive(Debug, Clone, Default)]
pub struct ToasterContext {
    /// Maximum number of visible toasts.
    pub max_visible: usize,
    /// Default toast duration in milliseconds.
    pub default_duration_ms: u32,
    /// Position of the toast container.
    pub position: ToastPosition,
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

impl ToasterContext {
    pub fn new() -> Self {
        Self {
            max_visible: 5,
            default_duration_ms: 5000,
            position: ToastPosition::BottomRight,
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
    fn toast_position_classes() {
        assert!(ToastPosition::BottomRight.css_classes().contains("bottom"));
        assert!(ToastPosition::TopLeft.css_classes().contains("top"));
        assert!(ToastPosition::TopCenter.css_classes().contains("translate"));
    }

    #[test]
    fn toaster_context_defaults() {
        let ctx = ToasterContext::new();
        assert_eq!(ctx.max_visible, 5);
        assert_eq!(ctx.default_duration_ms, 5000);
        assert_eq!(ctx.position, ToastPosition::BottomRight);
    }
}
