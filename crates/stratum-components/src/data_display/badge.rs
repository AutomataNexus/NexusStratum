//! Styled Badge component with variants.

use crate::common::merge_classes;
use stratum_core::render::RenderOutput;

/// Badge visual variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

/// Properties for the styled Badge.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BadgeProps {
    pub variant: BadgeVariant,
    pub class: Option<String>,
}

pub struct Badge;

impl Badge {
    const BASE: &'static str = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

    pub fn classes(props: &BadgeProps) -> String {
        let variant_cls = match props.variant {
            BadgeVariant::Default => {
                "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80"
            }
            BadgeVariant::Secondary => {
                "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            BadgeVariant::Destructive => {
                "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80"
            }
            BadgeVariant::Outline => "text-foreground",
        };

        let computed = format!("{} {}", Self::BASE, variant_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &BadgeProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("div")
            .with_class(Self::classes(props))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_badge_classes() {
        let props = BadgeProps::default();
        let classes = Badge::classes(&props);
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("bg-primary"));
    }

    #[test]
    fn destructive_badge() {
        let props = BadgeProps {
            variant: BadgeVariant::Destructive,
            ..Default::default()
        };
        let classes = Badge::classes(&props);
        assert!(classes.contains("bg-destructive"));
    }

    #[test]
    fn outline_badge() {
        let props = BadgeProps {
            variant: BadgeVariant::Outline,
            ..Default::default()
        };
        let classes = Badge::classes(&props);
        assert!(classes.contains("text-foreground"));
    }

    #[test]
    fn badge_render_tag() {
        let props = BadgeProps::default();
        let output = Badge::render(&props);
        assert_eq!(output.effective_tag(), "div");
    }
}
