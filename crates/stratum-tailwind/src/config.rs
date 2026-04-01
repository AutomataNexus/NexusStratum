//! Tailwind config generation from NexusStratum themes.
//!
//! Generates `tailwind.config.js` theme extensions and content paths
//! so that Tailwind can pick up NexusStratum design tokens.

use stratum_theme::{ColorToken, Theme};

/// Tailwind configuration utilities.
pub struct TailwindConfig;

impl TailwindConfig {
    /// Generate a `tailwind.config.js` theme extension from a [`Theme`].
    ///
    /// The output is a JavaScript object literal (without surrounding braces of
    /// the `module.exports`) suitable for insertion into a Tailwind config's
    /// `theme.extend` section.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use stratum_theme::Theme;
    /// use stratum_tailwind::TailwindConfig;
    ///
    /// let js = TailwindConfig::generate_theme_extension(&Theme::default());
    /// // Returns a JS object with colors, borderRadius, fontFamily, etc.
    /// ```
    pub fn generate_theme_extension(theme: &Theme) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push("{".to_string());

        // -- Colors --------------------------------------------------------
        lines.push("  colors: {".to_string());
        push_color_token(&mut lines, "background", &theme.colors.background);
        push_color_token(&mut lines, "foreground", &theme.colors.foreground);
        push_color_token(&mut lines, "card", &theme.colors.card);
        push_color_token(&mut lines, "card-foreground", &theme.colors.card_fg);
        push_color_token(&mut lines, "popover", &theme.colors.popover);
        push_color_token(&mut lines, "popover-foreground", &theme.colors.popover_fg);
        push_color_token(&mut lines, "primary", &theme.colors.primary);
        push_color_token(&mut lines, "primary-foreground", &theme.colors.primary_fg);
        push_color_token(&mut lines, "secondary", &theme.colors.secondary);
        push_color_token(&mut lines, "secondary-foreground", &theme.colors.secondary_fg);
        push_color_token(&mut lines, "muted", &theme.colors.muted);
        push_color_token(&mut lines, "muted-foreground", &theme.colors.muted_fg);
        push_color_token(&mut lines, "accent", &theme.colors.accent);
        push_color_token(&mut lines, "accent-foreground", &theme.colors.accent_fg);
        push_color_token(&mut lines, "destructive", &theme.colors.destructive);
        push_color_token(&mut lines, "destructive-foreground", &theme.colors.destructive_fg);
        push_color_token(&mut lines, "border", &theme.colors.border);
        push_color_token(&mut lines, "input", &theme.colors.input);
        push_color_token(&mut lines, "ring", &theme.colors.ring);
        lines.push("  },".to_string());

        // -- Border radius -------------------------------------------------
        lines.push("  borderRadius: {".to_string());
        lines.push(format!("    none: \"{}\",", theme.radii.none.to_css()));
        lines.push(format!("    sm: \"{}\",", theme.radii.sm.to_css()));
        lines.push(format!("    md: \"{}\",", theme.radii.md.to_css()));
        lines.push(format!("    lg: \"{}\",", theme.radii.lg.to_css()));
        lines.push(format!("    xl: \"{}\",", theme.radii.xl.to_css()));
        lines.push(format!("    full: \"{}\",", theme.radii.full.to_css()));
        lines.push("  },".to_string());

        // -- Font family ---------------------------------------------------
        lines.push("  fontFamily: {".to_string());
        lines.push(format!(
            "    sans: [\"{}\"],",
            theme.typography.font_sans
        ));
        lines.push(format!(
            "    serif: [\"{}\"],",
            theme.typography.font_serif
        ));
        lines.push(format!(
            "    mono: [\"{}\"],",
            theme.typography.font_mono
        ));
        lines.push("  },".to_string());

        // -- Font size -----------------------------------------------------
        lines.push("  fontSize: {".to_string());
        lines.push(format!("    xs: \"{}\",", theme.typography.size_xs.to_css()));
        lines.push(format!("    sm: \"{}\",", theme.typography.size_sm.to_css()));
        lines.push(format!("    base: \"{}\",", theme.typography.size_md.to_css()));
        lines.push(format!("    lg: \"{}\",", theme.typography.size_lg.to_css()));
        lines.push(format!("    xl: \"{}\",", theme.typography.size_xl.to_css()));
        lines.push(format!(
            "    \"2xl\": \"{}\",",
            theme.typography.size_2xl.to_css()
        ));
        lines.push(format!(
            "    \"3xl\": \"{}\",",
            theme.typography.size_3xl.to_css()
        ));
        lines.push(format!(
            "    \"4xl\": \"{}\",",
            theme.typography.size_4xl.to_css()
        ));
        lines.push("  },".to_string());

        // -- Spacing -------------------------------------------------------
        lines.push("  spacing: {".to_string());
        lines.push(format!("    0: \"{}\",", theme.spacing.s0.to_css()));
        lines.push(format!("    1: \"{}\",", theme.spacing.s1.to_css()));
        lines.push(format!("    2: \"{}\",", theme.spacing.s2.to_css()));
        lines.push(format!("    3: \"{}\",", theme.spacing.s3.to_css()));
        lines.push(format!("    4: \"{}\",", theme.spacing.s4.to_css()));
        lines.push(format!("    5: \"{}\",", theme.spacing.s5.to_css()));
        lines.push(format!("    6: \"{}\",", theme.spacing.s6.to_css()));
        lines.push(format!("    8: \"{}\",", theme.spacing.s8.to_css()));
        lines.push(format!("    10: \"{}\",", theme.spacing.s10.to_css()));
        lines.push(format!("    12: \"{}\",", theme.spacing.s12.to_css()));
        lines.push(format!("    16: \"{}\",", theme.spacing.s16.to_css()));
        lines.push(format!("    20: \"{}\",", theme.spacing.s20.to_css()));
        lines.push(format!("    24: \"{}\",", theme.spacing.s24.to_css()));
        lines.push("  },".to_string());

        // -- Z-index -------------------------------------------------------
        lines.push("  zIndex: {".to_string());
        lines.push(format!(
            "    dropdown: \"{}\",",
            theme.z_indices.dropdown
        ));
        lines.push(format!("    sticky: \"{}\",", theme.z_indices.sticky));
        lines.push(format!("    modal: \"{}\",", theme.z_indices.modal));
        lines.push(format!(
            "    popover: \"{}\",",
            theme.z_indices.popover
        ));
        lines.push(format!(
            "    tooltip: \"{}\",",
            theme.z_indices.tooltip
        ));
        lines.push("  },".to_string());

        // -- Screens (breakpoints) -----------------------------------------
        lines.push("  screens: {".to_string());
        lines.push(format!("    sm: \"{}\",", theme.breakpoints.sm.to_css()));
        lines.push(format!("    md: \"{}\",", theme.breakpoints.md.to_css()));
        lines.push(format!("    lg: \"{}\",", theme.breakpoints.lg.to_css()));
        lines.push(format!("    xl: \"{}\",", theme.breakpoints.xl.to_css()));
        lines.push(format!(
            "    \"2xl\": \"{}\",",
            theme.breakpoints.xxl.to_css()
        ));
        lines.push("  },".to_string());

        lines.push("}".to_string());

        lines.join("\n")
    }

    /// Generate the content paths for NexusStratum components.
    ///
    /// These paths should be included in `tailwind.config.js` `content` array
    /// so that Tailwind can scan component source for class names.
    pub fn content_paths() -> Vec<String> {
        vec![
            "./src/**/*.{rs,html}".to_string(),
            "./crates/stratum-components/src/**/*.rs".to_string(),
            "./crates/stratum-primitives/src/**/*.rs".to_string(),
            "./crates/stratum-dioxus/src/**/*.rs".to_string(),
            "./crates/stratum-leptos/src/**/*.rs".to_string(),
        ]
    }
}

/// Helper: push a semantic color token entry (light value as `hsl(...)`).
fn push_color_token(
    lines: &mut Vec<String>,
    name: &str,
    token: &ColorToken,
) {
    lines.push(format!(
        "    \"{}\": \"{}\",",
        name,
        token.light.to_css()
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_theme_extension_contains_colors() {
        let theme = Theme::default();
        let js = TailwindConfig::generate_theme_extension(&theme);
        assert!(js.contains("colors:"));
        assert!(js.contains("\"primary\":"));
        assert!(js.contains("\"background\":"));
        assert!(js.contains("hsl("));
    }

    #[test]
    fn generate_theme_extension_contains_border_radius() {
        let theme = Theme::default();
        let js = TailwindConfig::generate_theme_extension(&theme);
        assert!(js.contains("borderRadius:"));
        assert!(js.contains("rem"));
    }

    #[test]
    fn generate_theme_extension_contains_font_family() {
        let theme = Theme::default();
        let js = TailwindConfig::generate_theme_extension(&theme);
        assert!(js.contains("fontFamily:"));
        assert!(js.contains("sans:"));
    }

    #[test]
    fn generate_theme_extension_contains_spacing() {
        let theme = Theme::default();
        let js = TailwindConfig::generate_theme_extension(&theme);
        assert!(js.contains("spacing:"));
    }

    #[test]
    fn generate_theme_extension_contains_screens() {
        let theme = Theme::default();
        let js = TailwindConfig::generate_theme_extension(&theme);
        assert!(js.contains("screens:"));
        assert!(js.contains("640px"));
    }

    #[test]
    fn content_paths_not_empty() {
        let paths = TailwindConfig::content_paths();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.contains("stratum-components")));
    }
}
