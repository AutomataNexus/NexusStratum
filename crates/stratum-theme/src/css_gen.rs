//! CSS custom property generation from a [`Theme`].

use crate::theme::Theme;
use crate::token::ColorToken;

impl Theme {
    /// Generate CSS custom properties for the light (`:root`) context.
    ///
    /// ```css
    /// :root {
    ///   --stratum-color-background: 0 0% 100%;
    ///   /* ... */
    /// }
    /// ```
    pub fn to_css_variables(&self) -> String {
        let mut css = String::from(":root {\n");
        self.write_color_vars(&mut css, false);
        self.write_typography_vars(&mut css);
        self.write_spacing_vars(&mut css);
        self.write_radii_vars(&mut css);
        self.write_shadow_vars(&mut css);
        self.write_transition_vars(&mut css);
        self.write_z_index_vars(&mut css);
        self.write_breakpoint_vars(&mut css);
        css.push_str("}\n");
        css
    }

    /// Generate CSS custom properties for the dark (`.dark`) context.
    ///
    /// ```css
    /// .dark {
    ///   --stratum-color-background: 222.2 84% 4.9%;
    ///   /* ... */
    /// }
    /// ```
    pub fn to_dark_css_variables(&self) -> String {
        let mut css = String::from(".dark {\n");
        self.write_color_vars(&mut css, true);
        css.push_str("}\n");
        css
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    fn write_color_var(css: &mut String, name: &str, token: &ColorToken, dark: bool) {
        let value = if dark {
            token.dark.to_css_value()
        } else {
            token.light.to_css_value()
        };
        css.push_str(&format!("  --stratum-color-{}: {};\n", name, value));
    }

    fn write_color_vars(&self, css: &mut String, dark: bool) {
        let c = &self.colors;
        Self::write_color_var(css, "background", &c.background, dark);
        Self::write_color_var(css, "foreground", &c.foreground, dark);
        Self::write_color_var(css, "card", &c.card, dark);
        Self::write_color_var(css, "card-fg", &c.card_fg, dark);
        Self::write_color_var(css, "popover", &c.popover, dark);
        Self::write_color_var(css, "popover-fg", &c.popover_fg, dark);
        Self::write_color_var(css, "primary", &c.primary, dark);
        Self::write_color_var(css, "primary-fg", &c.primary_fg, dark);
        Self::write_color_var(css, "secondary", &c.secondary, dark);
        Self::write_color_var(css, "secondary-fg", &c.secondary_fg, dark);
        Self::write_color_var(css, "muted", &c.muted, dark);
        Self::write_color_var(css, "muted-fg", &c.muted_fg, dark);
        Self::write_color_var(css, "accent", &c.accent, dark);
        Self::write_color_var(css, "accent-fg", &c.accent_fg, dark);
        Self::write_color_var(css, "destructive", &c.destructive, dark);
        Self::write_color_var(css, "destructive-fg", &c.destructive_fg, dark);
        Self::write_color_var(css, "border", &c.border, dark);
        Self::write_color_var(css, "input", &c.input, dark);
        Self::write_color_var(css, "ring", &c.ring, dark);

        // Palette steps — these are mode-independent by design.
        // Unlike semantic ColorTokens (which have light/dark), palettes
        // provide a fixed 9-step scale used for both modes. Consumers
        // should use semantic tokens (primary, accent, etc.) for mode-aware
        // colors and palettes for explicit color references.
        let palettes: &[(&str, &crate::token::ColorPalette)] = &[
            ("gray", &c.gray),
            ("red", &c.red),
            ("orange", &c.orange),
            ("yellow", &c.yellow),
            ("green", &c.green),
            ("teal", &c.teal),
            ("blue", &c.blue),
            ("indigo", &c.indigo),
            ("violet", &c.violet),
            ("pink", &c.pink),
        ];
        for (name, palette) in palettes {
            for (i, step) in palette.steps.iter().enumerate() {
                let level = (i + 1) * 100;
                css.push_str(&format!(
                    "  --stratum-color-{}-{}: {};\n",
                    name,
                    level,
                    step.to_css_value()
                ));
            }
        }
    }

    fn write_typography_vars(&self, css: &mut String) {
        let t = &self.typography;
        css.push_str(&format!(
            "  --stratum-font-sans: {};\n",
            t.font_sans
        ));
        css.push_str(&format!(
            "  --stratum-font-serif: {};\n",
            t.font_serif
        ));
        css.push_str(&format!(
            "  --stratum-font-mono: {};\n",
            t.font_mono
        ));

        let sizes = [
            ("xs", &t.size_xs),
            ("sm", &t.size_sm),
            ("md", &t.size_md),
            ("lg", &t.size_lg),
            ("xl", &t.size_xl),
            ("2xl", &t.size_2xl),
            ("3xl", &t.size_3xl),
            ("4xl", &t.size_4xl),
        ];
        for (name, rem) in &sizes {
            css.push_str(&format!(
                "  --stratum-font-size-{}: {};\n",
                name,
                rem.to_css()
            ));
        }

        let weights = [
            ("thin", t.weight_thin),
            ("light", t.weight_light),
            ("normal", t.weight_normal),
            ("medium", t.weight_medium),
            ("semibold", t.weight_semibold),
            ("bold", t.weight_bold),
            ("extrabold", t.weight_extrabold),
            ("black", t.weight_black),
        ];
        for (name, weight) in &weights {
            css.push_str(&format!(
                "  --stratum-font-weight-{}: {};\n",
                name, weight
            ));
        }

        css.push_str(&format!(
            "  --stratum-leading-tight: {};\n",
            t.leading_tight
        ));
        css.push_str(&format!(
            "  --stratum-leading-normal: {};\n",
            t.leading_normal
        ));
        css.push_str(&format!(
            "  --stratum-leading-relaxed: {};\n",
            t.leading_relaxed
        ));

        css.push_str(&format!(
            "  --stratum-tracking-tight: {};\n",
            t.tracking_tight.to_css()
        ));
        css.push_str(&format!(
            "  --stratum-tracking-normal: {};\n",
            t.tracking_normal.to_css()
        ));
        css.push_str(&format!(
            "  --stratum-tracking-wide: {};\n",
            t.tracking_wide.to_css()
        ));
    }

    fn write_spacing_vars(&self, css: &mut String) {
        let s = &self.spacing;
        let entries = [
            ("0", &s.s0),
            ("1", &s.s1),
            ("2", &s.s2),
            ("3", &s.s3),
            ("4", &s.s4),
            ("5", &s.s5),
            ("6", &s.s6),
            ("8", &s.s8),
            ("10", &s.s10),
            ("12", &s.s12),
            ("16", &s.s16),
            ("20", &s.s20),
            ("24", &s.s24),
        ];
        for (name, rem) in &entries {
            css.push_str(&format!(
                "  --stratum-spacing-{}: {};\n",
                name,
                rem.to_css()
            ));
        }
    }

    fn write_radii_vars(&self, css: &mut String) {
        let r = &self.radii;
        let entries = [
            ("none", &r.none),
            ("sm", &r.sm),
            ("md", &r.md),
            ("lg", &r.lg),
            ("xl", &r.xl),
            ("full", &r.full),
        ];
        for (name, rem) in &entries {
            css.push_str(&format!(
                "  --stratum-radius-{}: {};\n",
                name,
                rem.to_css()
            ));
        }
    }

    fn write_shadow_vars(&self, css: &mut String) {
        let s = &self.shadows;
        css.push_str(&format!("  --stratum-shadow-sm: {};\n", s.sm));
        css.push_str(&format!("  --stratum-shadow-md: {};\n", s.md));
        css.push_str(&format!("  --stratum-shadow-lg: {};\n", s.lg));
        css.push_str(&format!("  --stratum-shadow-xl: {};\n", s.xl));
    }

    fn write_transition_vars(&self, css: &mut String) {
        let t = &self.transitions;
        css.push_str(&format!(
            "  --stratum-transition-fast: {};\n",
            t.fast
        ));
        css.push_str(&format!(
            "  --stratum-transition-normal: {};\n",
            t.normal
        ));
        css.push_str(&format!(
            "  --stratum-transition-slow: {};\n",
            t.slow
        ));
    }

    fn write_z_index_vars(&self, css: &mut String) {
        let z = &self.z_indices;
        css.push_str(&format!(
            "  --stratum-z-dropdown: {};\n",
            z.dropdown
        ));
        css.push_str(&format!("  --stratum-z-sticky: {};\n", z.sticky));
        css.push_str(&format!("  --stratum-z-modal: {};\n", z.modal));
        css.push_str(&format!(
            "  --stratum-z-popover: {};\n",
            z.popover
        ));
        css.push_str(&format!(
            "  --stratum-z-tooltip: {};\n",
            z.tooltip
        ));
    }

    fn write_breakpoint_vars(&self, css: &mut String) {
        let b = &self.breakpoints;
        css.push_str(&format!(
            "  --stratum-breakpoint-sm: {};\n",
            b.sm.to_css()
        ));
        css.push_str(&format!(
            "  --stratum-breakpoint-md: {};\n",
            b.md.to_css()
        ));
        css.push_str(&format!(
            "  --stratum-breakpoint-lg: {};\n",
            b.lg.to_css()
        ));
        css.push_str(&format!(
            "  --stratum-breakpoint-xl: {};\n",
            b.xl.to_css()
        ));
        css.push_str(&format!(
            "  --stratum-breakpoint-xxl: {};\n",
            b.xxl.to_css()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_variables_start_with_root() {
        let theme = Theme::default();
        let css = theme.to_css_variables();
        assert!(css.starts_with(":root {"));
        assert!(css.ends_with("}\n"));
    }

    #[test]
    fn dark_css_variables_start_with_dark() {
        let theme = Theme::default();
        let css = theme.to_dark_css_variables();
        assert!(css.starts_with(".dark {"));
        assert!(css.ends_with("}\n"));
    }

    #[test]
    fn css_contains_color_variables() {
        let theme = Theme::default();
        let css = theme.to_css_variables();
        assert!(css.contains("--stratum-color-background:"));
        assert!(css.contains("--stratum-color-primary:"));
        assert!(css.contains("--stratum-color-destructive:"));
        assert!(css.contains("--stratum-color-ring:"));
    }

    #[test]
    fn css_contains_palette_variables() {
        let theme = Theme::default();
        let css = theme.to_css_variables();
        assert!(css.contains("--stratum-color-blue-500:"));
        assert!(css.contains("--stratum-color-red-100:"));
        assert!(css.contains("--stratum-color-gray-900:"));
    }

    #[test]
    fn css_contains_spacing_variables() {
        let theme = Theme::default();
        let css = theme.to_css_variables();
        assert!(css.contains("--stratum-spacing-4: 1rem;"));
    }

    #[test]
    fn css_contains_radius_variables() {
        let theme = Theme::default();
        let css = theme.to_css_variables();
        assert!(css.contains("--stratum-radius-md:"));
    }

    #[test]
    fn dark_uses_dark_color_values() {
        let theme = Theme::default();
        let light = theme.to_css_variables();
        let dark = theme.to_dark_css_variables();
        // The background values should differ between light and dark
        let extract = |css: &str| -> String {
            css.lines()
                .find(|l| l.contains("--stratum-color-background:"))
                .unwrap()
                .to_string()
        };
        assert_ne!(extract(&light), extract(&dark));
    }
}
