//! Skeleton loader component for content placeholders.

use crate::common::merge_classes;
use stratum_core::aria::AriaAttributes;
use stratum_core::render::RenderOutput;

/// Properties for the Skeleton component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkeletonProps {
    pub class: Option<String>,
}

pub struct Skeleton;

impl Skeleton {
    const BASE: &'static str = "animate-pulse rounded-md bg-primary/10";

    pub fn classes(props: &SkeletonProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &SkeletonProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new();
        aria.busy = Some(true);
        aria.hidden = Some(true);

        RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_classes() {
        let props = SkeletonProps::default();
        let classes = Skeleton::classes(&props);
        assert!(classes.contains("animate-pulse"));
        assert!(classes.contains("rounded-md"));
    }

    #[test]
    fn skeleton_aria_busy() {
        let props = SkeletonProps::default();
        let output = Skeleton::render(&props);
        assert_eq!(output.aria.busy, Some(true));
        assert_eq!(output.aria.hidden, Some(true));
    }
}
