//! Core design token types for colors and measurements.

use serde::{Deserialize, Serialize};

/// An HSL color value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Hsl {
    /// Hue in degrees (0-360).
    pub h: f64,
    /// Saturation as a percentage (0-100).
    pub s: f64,
    /// Lightness as a percentage (0-100).
    pub l: f64,
}

impl Hsl {
    /// Create a new HSL color.
    ///
    /// Values are clamped to valid ranges: h to 0..360, s and l to 0..100.
    /// NaN values are replaced with 0.
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self {
            h: clamp_or_zero(h, 0.0, 360.0),
            s: clamp_or_zero(s, 0.0, 100.0),
            l: clamp_or_zero(l, 0.0, 100.0),
        }
    }

    /// Render as a full CSS `hsl(...)` function string.
    ///
    /// Example: `"hsl(222.2 84% 4.9%)"`
    pub fn to_css(&self) -> String {
        format!(
            "hsl({} {}% {}%)",
            format_f64(self.h),
            format_f64(self.s),
            format_f64(self.l)
        )
    }

    /// Render as a bare CSS value string suitable for `var()` usage.
    ///
    /// Example: `"222.2 84% 4.9%"`
    pub fn to_css_value(&self) -> String {
        format!(
            "{} {}% {}%",
            format_f64(self.h),
            format_f64(self.s),
            format_f64(self.l)
        )
    }
}

/// Clamp a value to [min, max], replacing NaN/Infinity with 0.
fn clamp_or_zero(v: f64, min: f64, max: f64) -> f64 {
    if v.is_nan() || v.is_infinite() {
        0.0
    } else {
        v.clamp(min, max)
    }
}

/// Format an f64 without trailing zeros. NaN/Infinity produce "0".
fn format_f64(v: f64) -> String {
    if v.is_nan() || v.is_infinite() {
        return "0".to_string();
    }
    if v == v.floor() && v.abs() < i64::MAX as f64 {
        format!("{}", v as i64)
    } else {
        let s = format!("{}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// A pair of light/dark mode color values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorToken {
    /// Color used in light mode.
    pub light: Hsl,
    /// Color used in dark mode.
    pub dark: Hsl,
}

/// A nine-step color palette (100 through 900).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Steps 100, 200, 300, 400, 500, 600, 700, 800, 900.
    pub steps: [Hsl; 9],
}

/// A measurement in `rem` units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rem(pub f64);

/// A measurement in `em` units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Em(pub f64);

/// A measurement in `px` units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Px(pub f64);

impl Rem {
    /// Render as CSS, e.g. `"0.375rem"`.
    pub fn to_css(&self) -> String {
        format!("{}rem", format_f64(self.0))
    }
}

impl Em {
    /// Render as CSS, e.g. `"0.025em"`.
    pub fn to_css(&self) -> String {
        format!("{}em", format_f64(self.0))
    }
}

impl Px {
    /// Render as CSS, e.g. `"640px"`.
    pub fn to_css(&self) -> String {
        format!("{}px", format_f64(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsl_to_css() {
        let c = Hsl::new(222.2, 84.0, 4.9);
        assert_eq!(c.to_css(), "hsl(222.2 84% 4.9%)");
    }

    #[test]
    fn hsl_to_css_value() {
        let c = Hsl::new(222.2, 84.0, 4.9);
        assert_eq!(c.to_css_value(), "222.2 84% 4.9%");
    }

    #[test]
    fn hsl_integer_values() {
        let c = Hsl::new(0.0, 0.0, 100.0);
        assert_eq!(c.to_css(), "hsl(0 0% 100%)");
    }

    #[test]
    fn hsl_clamps_nan_and_infinity() {
        let c = Hsl::new(f64::NAN, f64::INFINITY, f64::NEG_INFINITY);
        assert_eq!(c.h, 0.0);
        assert_eq!(c.s, 0.0);
        assert_eq!(c.l, 0.0);
        assert_eq!(c.to_css(), "hsl(0 0% 0%)");
    }

    #[test]
    fn hsl_clamps_out_of_range() {
        let c = Hsl::new(400.0, 150.0, -10.0);
        assert_eq!(c.h, 360.0);
        assert_eq!(c.s, 100.0);
        assert_eq!(c.l, 0.0);
    }

    #[test]
    fn rem_to_css() {
        assert_eq!(Rem(0.375).to_css(), "0.375rem");
        assert_eq!(Rem(1.0).to_css(), "1rem");
    }

    #[test]
    fn em_to_css() {
        assert_eq!(Em(0.025).to_css(), "0.025em");
    }

    #[test]
    fn px_to_css() {
        assert_eq!(Px(640.0).to_css(), "640px");
    }
}
