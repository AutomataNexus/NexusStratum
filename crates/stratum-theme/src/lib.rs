//! # stratum-theme
//!
//! Design token and theme system for NexusStratum.
//!
//! Provides a complete set of design tokens — colors, typography, spacing,
//! radii, shadows, transitions, z-indices, and breakpoints — organised into
//! composable [`Theme`] objects with built-in CSS custom property generation.

pub mod color;
pub mod css_gen;
pub mod scale;
pub mod theme;
pub mod token;
pub mod typography;

// Re-export all public types for convenient use.
pub use color::ColorScale;
pub use scale::{
    BreakpointScale, RadiiScale, ShadowScale, SpacingScale, TransitionScale, ZIndexScale,
};
pub use theme::Theme;
pub use token::{ColorPalette, ColorToken, Em, Hsl, Px, Rem};
pub use typography::Typography;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_themes_have_distinct_primary_colors() {
        let themes = vec![
            Theme::default(),
            Theme::slate(),
            Theme::zinc(),
            Theme::rose(),
            Theme::blue(),
            Theme::green(),
            Theme::orange(),
        ];

        // Collect primary light hues
        let hues: Vec<f64> = themes.iter().map(|t| t.colors.primary.light.h).collect();

        // At least the accent themes (rose, blue, green, orange) should all differ
        let accent_hues = &hues[3..]; // rose, blue, green, orange
        for i in 0..accent_hues.len() {
            for j in (i + 1)..accent_hues.len() {
                assert_ne!(
                    accent_hues[i], accent_hues[j],
                    "accent themes {} and {} should have distinct primary hues",
                    i + 3,
                    j + 3
                );
            }
        }
    }

    #[test]
    fn builder_with_primary() {
        let light = Hsl::new(280.0, 80.0, 50.0);
        let dark = Hsl::new(280.0, 70.0, 60.0);
        let theme = Theme::default().with_primary(light, dark);
        assert_eq!(theme.colors.primary.light, light);
        assert_eq!(theme.colors.primary.dark, dark);
    }

    #[test]
    fn builder_with_radius() {
        let radii = RadiiScale {
            none: Rem(0.0),
            sm: Rem(0.25),
            md: Rem(0.5),
            lg: Rem(1.0),
            xl: Rem(1.5),
            full: Rem(9999.0),
        };
        let theme = Theme::default().with_radius(radii.clone());
        assert_eq!(theme.radii, radii);
    }

    #[test]
    fn builder_with_font_sans() {
        let theme = Theme::default().with_font_sans("Inter, sans-serif");
        assert_eq!(theme.typography.font_sans, "Inter, sans-serif");
    }

    #[test]
    fn default_scales_are_sensible() {
        let theme = Theme::default();
        // Spacing should be monotonically increasing
        assert!(theme.spacing.s1.0 < theme.spacing.s2.0);
        assert!(theme.spacing.s2.0 < theme.spacing.s4.0);
        assert!(theme.spacing.s4.0 < theme.spacing.s8.0);

        // Radii should be monotonically increasing
        assert!(theme.radii.none.0 < theme.radii.sm.0);
        assert!(theme.radii.sm.0 < theme.radii.md.0);
        assert!(theme.radii.md.0 < theme.radii.lg.0);

        // Breakpoints should be increasing
        assert!(theme.breakpoints.sm.0 < theme.breakpoints.md.0);
        assert!(theme.breakpoints.md.0 < theme.breakpoints.lg.0);
    }

    #[test]
    fn theme_serializes_to_json() {
        let theme = Theme::default();
        let json = serde_json::to_string(&theme);
        assert!(json.is_ok());
    }
}
