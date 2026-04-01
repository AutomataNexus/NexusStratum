//! Typography design tokens.

use serde::{Deserialize, Serialize};

use crate::token::{Em, Rem};

/// Typography tokens covering font families, sizes, weights, line-heights, and letter-spacing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    // Font families
    pub font_sans: String,
    pub font_serif: String,
    pub font_mono: String,

    // Font sizes
    pub size_xs: Rem,
    pub size_sm: Rem,
    pub size_md: Rem,
    pub size_lg: Rem,
    pub size_xl: Rem,
    pub size_2xl: Rem,
    pub size_3xl: Rem,
    pub size_4xl: Rem,

    // Font weights
    pub weight_thin: u16,
    pub weight_light: u16,
    pub weight_normal: u16,
    pub weight_medium: u16,
    pub weight_semibold: u16,
    pub weight_bold: u16,
    pub weight_extrabold: u16,
    pub weight_black: u16,

    // Line heights
    pub leading_tight: f32,
    pub leading_normal: f32,
    pub leading_relaxed: f32,

    // Letter spacing
    pub tracking_tight: Em,
    pub tracking_normal: Em,
    pub tracking_wide: Em,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_sans: "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"".into(),
            font_serif: "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif".into(),
            font_mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace".into(),

            size_xs: Rem(0.75),
            size_sm: Rem(0.875),
            size_md: Rem(1.0),
            size_lg: Rem(1.125),
            size_xl: Rem(1.25),
            size_2xl: Rem(1.5),
            size_3xl: Rem(1.875),
            size_4xl: Rem(2.25),

            weight_thin: 100,
            weight_light: 300,
            weight_normal: 400,
            weight_medium: 500,
            weight_semibold: 600,
            weight_bold: 700,
            weight_extrabold: 800,
            weight_black: 900,

            leading_tight: 1.25,
            leading_normal: 1.5,
            leading_relaxed: 1.75,

            tracking_tight: Em(-0.025),
            tracking_normal: Em(0.0),
            tracking_wide: Em(0.025),
        }
    }
}
