use serde::{Deserialize, Serialize};

/// CSS easing functions for transitions and animations.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Easing {
    /// Linear interpolation.
    Linear,
    /// Ease in (slow start).
    EaseIn,
    /// Ease out (slow end).
    EaseOut,
    /// Ease in and out.
    #[default]
    EaseInOut,
    /// Spring-like physics easing.
    Spring { stiffness: f32, damping: f32 },
    /// Custom cubic-bezier curve.
    CubicBezier(f32, f32, f32, f32),
    /// Custom CSS easing string.
    Custom(String),
}

impl Easing {
    /// Convert to a CSS timing function string.
    pub fn to_css(&self) -> String {
        match self {
            Self::Linear => "linear".to_string(),
            Self::EaseIn => "ease-in".to_string(),
            Self::EaseOut => "ease-out".to_string(),
            Self::EaseInOut => "ease-in-out".to_string(),
            Self::Spring {
                stiffness,
                damping,
            } => {
                // Approximate spring physics with a cubic-bezier
                // These are rough approximations based on common spring configs
                let tension = stiffness / 100.0;
                let friction = damping / 10.0;
                let x1 = (0.5 - tension * 0.2).clamp(0.0, 1.0);
                let y1 = (1.0 + tension * 0.5).clamp(0.0, 2.0).min(1.0);
                let x2 = (0.3 + friction * 0.1).clamp(0.0, 1.0);
                let y2 = 1.0;
                format!("cubic-bezier({:.3}, {:.3}, {:.3}, {:.3})", x1, y1, x2, y2)
            }
            Self::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            Self::Custom(s) => s.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_to_css() {
        assert_eq!(Easing::Linear.to_css(), "linear");
        assert_eq!(Easing::EaseIn.to_css(), "ease-in");
        assert_eq!(Easing::EaseOut.to_css(), "ease-out");
        assert_eq!(Easing::EaseInOut.to_css(), "ease-in-out");
    }

    #[test]
    fn cubic_bezier_to_css() {
        let easing = Easing::CubicBezier(0.4, 0.0, 0.2, 1.0);
        assert_eq!(easing.to_css(), "cubic-bezier(0.4, 0, 0.2, 1)");
    }

    #[test]
    fn custom_easing() {
        let easing = Easing::Custom("steps(4, end)".to_string());
        assert_eq!(easing.to_css(), "steps(4, end)");
    }

    #[test]
    fn spring_produces_cubic_bezier() {
        let easing = Easing::Spring {
            stiffness: 100.0,
            damping: 10.0,
        };
        let css = easing.to_css();
        assert!(css.starts_with("cubic-bezier("));
    }

    #[test]
    fn default_easing() {
        assert_eq!(Easing::default(), Easing::EaseInOut);
    }
}
