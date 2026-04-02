//! The main [`Theme`] struct that aggregates all design tokens.

use serde::{Deserialize, Serialize};

use crate::color::ColorScale;
use crate::scale::{
    BreakpointScale, RadiiScale, ShadowScale, SpacingScale, TransitionScale, ZIndexScale,
};
use crate::token::{ColorToken, Hsl};
use crate::typography::Typography;

/// A complete theme definition for NexusStratum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScale,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub radii: RadiiScale,
    pub shadows: ShadowScale,
    pub transitions: TransitionScale,
    pub z_indices: ZIndexScale,
    pub breakpoints: BreakpointScale,
}

impl Default for Theme {
    /// The default neutral theme, similar to shadcn/ui defaults.
    fn default() -> Self {
        Self {
            name: "default".into(),
            colors: ColorScale::default(),
            typography: Typography::default(),
            spacing: SpacingScale::default(),
            radii: RadiiScale::default(),
            shadows: ShadowScale::default(),
            transitions: TransitionScale::default(),
            z_indices: ZIndexScale::default(),
            breakpoints: BreakpointScale::default(),
        }
    }
}

#[allow(clippy::field_reassign_with_default)]
impl Theme {
    // ------------------------------------------------------------------
    // Built-in theme constructors
    // ------------------------------------------------------------------

    /// Cool slate grays theme.
    pub fn slate() -> Self {
        let mut theme = Self::default();
        theme.name = "slate".into();
        // Slate-tinted semantic colors
        theme.colors.background.light = Hsl::new(0.0, 0.0, 100.0);
        theme.colors.background.dark = Hsl::new(222.2, 84.0, 4.9);
        theme.colors.primary = ColorToken {
            light: Hsl::new(215.4, 16.3, 46.9),
            dark: Hsl::new(210.0, 40.0, 98.0),
        };
        theme.colors.primary_fg = ColorToken {
            light: Hsl::new(210.0, 40.0, 98.0),
            dark: Hsl::new(215.4, 16.3, 46.9),
        };
        theme.colors.accent = ColorToken {
            light: Hsl::new(210.0, 40.0, 96.1),
            dark: Hsl::new(215.0, 25.0, 20.0),
        };
        theme
    }

    /// Warm zinc grays theme.
    pub fn zinc() -> Self {
        let mut theme = Self::default();
        theme.name = "zinc".into();
        theme.colors.primary = ColorToken {
            light: Hsl::new(240.0, 5.9, 10.0),
            dark: Hsl::new(0.0, 0.0, 98.0),
        };
        theme.colors.primary_fg = ColorToken {
            light: Hsl::new(0.0, 0.0, 98.0),
            dark: Hsl::new(240.0, 5.9, 10.0),
        };
        theme.colors.muted = ColorToken {
            light: Hsl::new(240.0, 4.8, 95.9),
            dark: Hsl::new(240.0, 3.7, 15.9),
        };
        theme.colors.muted_fg = ColorToken {
            light: Hsl::new(240.0, 3.8, 46.1),
            dark: Hsl::new(240.0, 5.0, 64.9),
        };
        theme
    }

    /// Rose accent theme.
    pub fn rose() -> Self {
        let mut theme = Self::default();
        theme.name = "rose".into();
        theme.colors.primary = ColorToken {
            light: Hsl::new(346.8, 77.2, 49.8),
            dark: Hsl::new(346.8, 77.2, 49.8),
        };
        theme.colors.primary_fg = ColorToken {
            light: Hsl::new(355.7, 100.0, 97.3),
            dark: Hsl::new(355.7, 100.0, 97.3),
        };
        theme.colors.accent = ColorToken {
            light: Hsl::new(346.0, 50.0, 95.0),
            dark: Hsl::new(346.0, 40.0, 20.0),
        };
        theme.colors.ring = ColorToken {
            light: Hsl::new(346.8, 77.2, 49.8),
            dark: Hsl::new(346.8, 77.2, 49.8),
        };
        theme
    }

    /// Blue accent theme.
    pub fn blue() -> Self {
        let mut theme = Self::default();
        theme.name = "blue".into();
        theme.colors.primary = ColorToken {
            light: Hsl::new(221.2, 83.2, 53.3),
            dark: Hsl::new(217.2, 91.2, 59.8),
        };
        theme.colors.primary_fg = ColorToken {
            light: Hsl::new(210.0, 40.0, 98.0),
            dark: Hsl::new(210.0, 40.0, 98.0),
        };
        theme.colors.accent = ColorToken {
            light: Hsl::new(214.0, 50.0, 95.0),
            dark: Hsl::new(214.0, 40.0, 20.0),
        };
        theme.colors.ring = ColorToken {
            light: Hsl::new(221.2, 83.2, 53.3),
            dark: Hsl::new(217.2, 91.2, 59.8),
        };
        theme
    }

    /// Green accent theme.
    pub fn green() -> Self {
        let mut theme = Self::default();
        theme.name = "green".into();
        theme.colors.primary = ColorToken {
            light: Hsl::new(142.1, 76.2, 36.3),
            dark: Hsl::new(142.1, 70.6, 45.3),
        };
        theme.colors.primary_fg = ColorToken {
            light: Hsl::new(355.7, 100.0, 97.3),
            dark: Hsl::new(144.9, 80.4, 10.0),
        };
        theme.colors.accent = ColorToken {
            light: Hsl::new(142.0, 50.0, 95.0),
            dark: Hsl::new(142.0, 40.0, 18.0),
        };
        theme.colors.ring = ColorToken {
            light: Hsl::new(142.1, 76.2, 36.3),
            dark: Hsl::new(142.1, 70.6, 45.3),
        };
        theme
    }

    /// Orange accent theme.
    pub fn orange() -> Self {
        let mut theme = Self::default();
        theme.name = "orange".into();
        theme.colors.primary = ColorToken {
            light: Hsl::new(24.6, 95.0, 53.1),
            dark: Hsl::new(20.5, 90.2, 48.2),
        };
        theme.colors.primary_fg = ColorToken {
            light: Hsl::new(60.0, 9.1, 97.8),
            dark: Hsl::new(60.0, 9.1, 97.8),
        };
        theme.colors.accent = ColorToken {
            light: Hsl::new(30.0, 50.0, 95.0),
            dark: Hsl::new(30.0, 40.0, 18.0),
        };
        theme.colors.ring = ColorToken {
            light: Hsl::new(24.6, 95.0, 53.1),
            dark: Hsl::new(20.5, 90.2, 48.2),
        };
        theme
    }

    // ------------------------------------------------------------------
    // Builder methods
    // ------------------------------------------------------------------

    /// Override the primary color with separate light and dark values.
    pub fn with_primary(mut self, light: Hsl, dark: Hsl) -> Self {
        self.colors.primary = ColorToken { light, dark };
        self
    }

    /// Override the secondary color.
    pub fn with_secondary(mut self, light: Hsl, dark: Hsl) -> Self {
        self.colors.secondary = ColorToken { light, dark };
        self
    }

    /// Override the accent color.
    pub fn with_accent(mut self, light: Hsl, dark: Hsl) -> Self {
        self.colors.accent = ColorToken { light, dark };
        self
    }

    /// Override the destructive color.
    pub fn with_destructive(mut self, light: Hsl, dark: Hsl) -> Self {
        self.colors.destructive = ColorToken { light, dark };
        self
    }

    /// Override the border-radius scale.
    pub fn with_radius(mut self, radii: RadiiScale) -> Self {
        self.radii = radii;
        self
    }

    /// Override the spacing scale.
    pub fn with_spacing(mut self, spacing: SpacingScale) -> Self {
        self.spacing = spacing;
        self
    }

    /// Override the shadow scale.
    pub fn with_shadows(mut self, shadows: ShadowScale) -> Self {
        self.shadows = shadows;
        self
    }

    /// Override the sans-serif font family.
    pub fn with_font_sans(mut self, font: impl Into<String>) -> Self {
        self.typography.font_sans = font.into();
        self
    }

    /// Override the serif font family.
    pub fn with_font_serif(mut self, font: impl Into<String>) -> Self {
        self.typography.font_serif = font.into();
        self
    }

    /// Override the monospace font family.
    pub fn with_font_mono(mut self, font: impl Into<String>) -> Self {
        self.typography.font_mono = font.into();
        self
    }
}
