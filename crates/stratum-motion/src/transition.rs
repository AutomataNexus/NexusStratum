use crate::easing::Easing;
use serde::{Deserialize, Serialize};

/// Predefined transition presets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transition {
    FadeIn,
    FadeOut,
    SlideInFromTop,
    SlideInFromBottom,
    SlideInFromLeft,
    SlideInFromRight,
    SlideOutToTop,
    SlideOutToBottom,
    SlideOutToLeft,
    SlideOutToRight,
    ScaleIn,
    ScaleOut,
    /// Custom transition configuration.
    Custom(Box<TransitionConfig>),
}

impl Transition {
    /// Convert to a CSS transition configuration.
    pub fn to_config(&self) -> TransitionConfig {
        match self {
            Self::FadeIn => TransitionConfig {
                duration_ms: 150,
                easing: Easing::EaseOut,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(0.0),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(1.0),
                    ..Default::default()
                },
            },
            Self::FadeOut => TransitionConfig {
                duration_ms: 150,
                easing: Easing::EaseIn,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(1.0),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(0.0),
                    ..Default::default()
                },
            },
            Self::SlideInFromTop => TransitionConfig {
                duration_ms: 200,
                easing: Easing::EaseOut,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(0.0),
                    transform: Some("translateY(-10px)".to_string()),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(1.0),
                    transform: Some("translateY(0)".to_string()),
                    ..Default::default()
                },
            },
            Self::SlideInFromBottom => TransitionConfig {
                duration_ms: 200,
                easing: Easing::EaseOut,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(0.0),
                    transform: Some("translateY(10px)".to_string()),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(1.0),
                    transform: Some("translateY(0)".to_string()),
                    ..Default::default()
                },
            },
            Self::SlideInFromLeft => TransitionConfig {
                duration_ms: 200,
                easing: Easing::EaseOut,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(0.0),
                    transform: Some("translateX(-10px)".to_string()),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(1.0),
                    transform: Some("translateX(0)".to_string()),
                    ..Default::default()
                },
            },
            Self::SlideInFromRight => TransitionConfig {
                duration_ms: 200,
                easing: Easing::EaseOut,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(0.0),
                    transform: Some("translateX(10px)".to_string()),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(1.0),
                    transform: Some("translateX(0)".to_string()),
                    ..Default::default()
                },
            },
            Self::SlideOutToTop => Self::SlideInFromTop.to_config().reversed(),
            Self::SlideOutToBottom => Self::SlideInFromBottom.to_config().reversed(),
            Self::SlideOutToLeft => Self::SlideInFromLeft.to_config().reversed(),
            Self::SlideOutToRight => Self::SlideInFromRight.to_config().reversed(),
            Self::ScaleIn => TransitionConfig {
                duration_ms: 200,
                easing: Easing::EaseOut,
                delay_ms: 0,
                from: CssState {
                    opacity: Some(0.0),
                    transform: Some("scale(0.95)".to_string()),
                    ..Default::default()
                },
                to: CssState {
                    opacity: Some(1.0),
                    transform: Some("scale(1)".to_string()),
                    ..Default::default()
                },
            },
            Self::ScaleOut => Self::ScaleIn.to_config().reversed(),
            Self::Custom(config) => *config.clone(),
        }
    }
}

/// Custom transition configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionConfig {
    /// Duration in milliseconds.
    pub duration_ms: u32,
    /// Easing function.
    pub easing: Easing,
    /// Delay before transition starts (milliseconds).
    pub delay_ms: u32,
    /// CSS state at the start of the transition.
    pub from: CssState,
    /// CSS state at the end of the transition.
    pub to: CssState,
}

impl TransitionConfig {
    /// Reverse the transition (swap from/to).
    pub fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            ..self
        }
    }

    /// Override the duration.
    pub fn with_duration(mut self, ms: u32) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Override the easing.
    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Override the delay.
    pub fn with_delay(mut self, ms: u32) -> Self {
        self.delay_ms = ms;
        self
    }
}

/// CSS property state for transitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CssState {
    pub opacity: Option<f64>,
    pub transform: Option<String>,
    pub height: Option<String>,
    pub width: Option<String>,
    pub max_height: Option<String>,
}

impl CssState {
    /// Convert to inline CSS string.
    pub fn to_inline_css(&self) -> String {
        let mut parts = Vec::new();
        if let Some(opacity) = self.opacity {
            parts.push(format!("opacity: {}", opacity));
        }
        if let Some(ref transform) = self.transform {
            parts.push(format!("transform: {}", transform));
        }
        if let Some(ref height) = self.height {
            parts.push(format!("height: {}", height));
        }
        if let Some(ref width) = self.width {
            parts.push(format!("width: {}", width));
        }
        if let Some(ref max_height) = self.max_height {
            parts.push(format!("max-height: {}", max_height));
        }
        parts.join("; ")
    }
}

/// Generated CSS for applying a transition with reduced-motion support.
///
/// Framework adapters use this to apply transition styles to elements.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationStyle {
    /// CSS for the entering/active state.
    pub enter_css: String,
    /// CSS for the exiting/inactive state.
    pub exit_css: String,
    /// CSS transition property string.
    pub transition_css: String,
    /// CSS for reduced-motion users (instant, no animation).
    pub reduced_motion_css: String,
}

impl AnimationStyle {
    /// Generate animation CSS from a transition preset.
    pub fn from_transition(enter: &Transition, exit: &Transition) -> Self {
        let enter_config = enter.to_config();
        let exit_config = exit.to_config();

        let transition_props = "opacity, transform, height, width, max-height";

        Self {
            enter_css: enter_config.to.to_inline_css(),
            exit_css: exit_config.to.to_inline_css(),
            transition_css: format!(
                "transition: {} {}ms {} {}ms",
                transition_props,
                enter_config.duration_ms,
                enter_config.easing.to_css(),
                enter_config.delay_ms,
            ),
            reduced_motion_css: format!(
                "transition: {} 0ms",
                transition_props,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_in_config() {
        let config = Transition::FadeIn.to_config();
        assert_eq!(config.from.opacity, Some(0.0));
        assert_eq!(config.to.opacity, Some(1.0));
        assert_eq!(config.duration_ms, 150);
    }

    #[test]
    fn fade_out_config() {
        let config = Transition::FadeOut.to_config();
        assert_eq!(config.from.opacity, Some(1.0));
        assert_eq!(config.to.opacity, Some(0.0));
    }

    #[test]
    fn slide_in_from_top() {
        let config = Transition::SlideInFromTop.to_config();
        assert!(config.from.transform.as_ref().unwrap().contains("translateY"));
        assert_eq!(config.to.opacity, Some(1.0));
    }

    #[test]
    fn slide_out_is_reversed_slide_in() {
        let slide_in = Transition::SlideInFromTop.to_config();
        let slide_out = Transition::SlideOutToTop.to_config();
        assert_eq!(slide_in.from, slide_out.to);
        assert_eq!(slide_in.to, slide_out.from);
    }

    #[test]
    fn scale_in_config() {
        let config = Transition::ScaleIn.to_config();
        assert!(config.from.transform.as_ref().unwrap().contains("scale"));
        assert_eq!(config.to.transform, Some("scale(1)".to_string()));
    }

    #[test]
    fn reversed_swaps_from_to() {
        let config = TransitionConfig {
            duration_ms: 200,
            easing: Easing::EaseOut,
            delay_ms: 0,
            from: CssState {
                opacity: Some(0.0),
                ..Default::default()
            },
            to: CssState {
                opacity: Some(1.0),
                ..Default::default()
            },
        };
        let reversed = config.reversed();
        assert_eq!(reversed.from.opacity, Some(1.0));
        assert_eq!(reversed.to.opacity, Some(0.0));
    }

    #[test]
    fn css_state_to_inline() {
        let state = CssState {
            opacity: Some(0.5),
            transform: Some("scale(0.95)".to_string()),
            ..Default::default()
        };
        let css = state.to_inline_css();
        assert!(css.contains("opacity: 0.5"));
        assert!(css.contains("transform: scale(0.95)"));
    }

    #[test]
    fn animation_style_from_transition() {
        let style = AnimationStyle::from_transition(&Transition::FadeIn, &Transition::FadeOut);
        assert!(style.enter_css.contains("opacity: 1"));
        assert!(style.exit_css.contains("opacity: 0"));
        assert!(style.transition_css.contains("150ms"));
        assert!(style.reduced_motion_css.contains("0ms"));
    }

    #[test]
    fn with_duration_override() {
        let config = Transition::FadeIn.to_config().with_duration(300);
        assert_eq!(config.duration_ms, 300);
    }

    #[test]
    fn with_easing_override() {
        let config = Transition::FadeIn.to_config().with_easing(Easing::Linear);
        assert_eq!(config.easing, Easing::Linear);
    }

    #[test]
    fn custom_transition() {
        let custom = Transition::Custom(Box::new(TransitionConfig {
            duration_ms: 500,
            easing: Easing::Linear,
            delay_ms: 100,
            from: CssState {
                opacity: Some(0.0),
                ..Default::default()
            },
            to: CssState {
                opacity: Some(1.0),
                ..Default::default()
            },
        }));
        let config = custom.to_config();
        assert_eq!(config.duration_ms, 500);
        assert_eq!(config.delay_ms, 100);
    }
}
