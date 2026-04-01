//! Media query detection helpers for accessibility preferences.
//!
//! These return descriptors that framework adapters evaluate at runtime
//! against the actual browser/platform state.

/// A media query preference descriptor.
///
/// Framework adapters use this to check the actual media query state at runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaQueryPreference {
    /// The CSS media query string to evaluate.
    pub query: String,
    /// A human-readable name for the preference.
    pub name: &'static str,
}

/// Returns a media query descriptor for detecting reduced motion preference.
pub fn prefers_reduced_motion() -> MediaQueryPreference {
    MediaQueryPreference {
        query: "(prefers-reduced-motion: reduce)".to_string(),
        name: "prefers-reduced-motion",
    }
}

/// Returns a media query descriptor for detecting high contrast preference.
pub fn prefers_high_contrast() -> MediaQueryPreference {
    MediaQueryPreference {
        query: "(prefers-contrast: more)".to_string(),
        name: "prefers-high-contrast",
    }
}

/// Returns a media query descriptor for detecting dark color scheme preference.
pub fn prefers_color_scheme_dark() -> MediaQueryPreference {
    MediaQueryPreference {
        query: "(prefers-color-scheme: dark)".to_string(),
        name: "prefers-color-scheme-dark",
    }
}

/// Returns a description of the strategy used to detect keyboard-only users.
///
/// Framework adapters implement this detection by tracking input modality
/// (keyboard vs pointer events).
pub fn is_keyboard_user() -> &'static str {
    "Track the most recent input modality (keyboard vs pointer) to determine \
     if the user is navigating with a keyboard. Show focus indicators only when \
     the last input was a keyboard event."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduced_motion_query() {
        let pref = prefers_reduced_motion();
        assert_eq!(pref.query, "(prefers-reduced-motion: reduce)");
        assert_eq!(pref.name, "prefers-reduced-motion");
    }

    #[test]
    fn high_contrast_query() {
        let pref = prefers_high_contrast();
        assert_eq!(pref.query, "(prefers-contrast: more)");
        assert_eq!(pref.name, "prefers-high-contrast");
    }

    #[test]
    fn dark_scheme_query() {
        let pref = prefers_color_scheme_dark();
        assert_eq!(pref.query, "(prefers-color-scheme: dark)");
        assert_eq!(pref.name, "prefers-color-scheme-dark");
    }

    #[test]
    fn keyboard_user_strategy_is_non_empty() {
        let strategy = is_keyboard_user();
        assert!(!strategy.is_empty());
    }
}
