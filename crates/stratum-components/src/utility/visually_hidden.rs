//! VisuallyHidden utility — renders content that is visually hidden but
//! accessible to screen readers.

use stratum_core::render::RenderOutput;

/// Properties for VisuallyHidden.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VisuallyHiddenProps {
    pub class: Option<String>,
}

pub struct VisuallyHidden;

impl VisuallyHidden {
    /// The standard sr-only class set used to visually hide content while
    /// keeping it accessible.
    const CLASSES: &'static str = "absolute w-[1px] h-[1px] p-0 -m-[1px] overflow-hidden whitespace-nowrap border-0";

    pub fn classes(props: &VisuallyHiddenProps) -> String {
        match &props.class {
            Some(extra) => format!("{} {}", Self::CLASSES, extra),
            None => Self::CLASSES.to_string(),
        }
    }

    pub fn render(props: &VisuallyHiddenProps) -> RenderOutput {
        RenderOutput::new()
            .with_tag("span")
            .with_class(Self::classes(props))
            .with_style("clip", "rect(0, 0, 0, 0)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visually_hidden_classes() {
        let props = VisuallyHiddenProps::default();
        let classes = VisuallyHidden::classes(&props);
        assert!(classes.contains("absolute"));
        assert!(classes.contains("overflow-hidden"));
    }

    #[test]
    fn visually_hidden_tag() {
        let props = VisuallyHiddenProps::default();
        let output = VisuallyHidden::render(&props);
        assert_eq!(output.effective_tag(), "span");
    }

    #[test]
    fn visually_hidden_clip_style() {
        let props = VisuallyHiddenProps::default();
        let output = VisuallyHidden::render(&props);
        assert!(output.styles.iter().any(|(k, v)| k == "clip" && v == "rect(0, 0, 0, 0)"));
    }
}
