//! Styled Link component.

use crate::common::{merge_classes, Size};
use stratum_core::aria::{AriaAttributes, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Link variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LinkVariant {
    #[default]
    Default,
    Muted,
    Destructive,
}

/// Properties for the styled Link component.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinkProps {
    pub variant: LinkVariant,
    pub size: Size,
    pub href: Option<String>,
    pub external: bool,
    pub disabled: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

pub struct Link;

impl Link {
    const BASE: &'static str = "underline-offset-4 hover:underline transition-colors";

    pub fn classes(props: &LinkProps) -> String {
        let variant_cls = match props.variant {
            LinkVariant::Default => "text-primary",
            LinkVariant::Muted => "text-muted-foreground",
            LinkVariant::Destructive => "text-destructive",
        };

        let size_cls = match props.size {
            Size::Xs => "text-xs",
            Size::Sm => "text-sm",
            Size::Md => "text-base",
            Size::Lg => "text-lg",
            Size::Xl => "text-xl",
        };

        let disabled_cls = if props.disabled {
            "pointer-events-none opacity-50"
        } else {
            ""
        };

        let computed = format!("{} {} {} {}", Self::BASE, variant_cls, size_cls, disabled_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &LinkProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Link);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }
        if props.disabled {
            aria = aria.with_disabled(true);
        }

        let mut output = RenderOutput::new()
            .with_tag("a")
            .with_class(classes)
            .with_aria(aria);

        if let Some(ref href) = props.href {
            output = output.with_attr("href", AttrValue::String(href.clone()));
        }
        if props.external {
            output = output
                .with_attr("target", AttrValue::String("_blank".to_string()))
                .with_attr("rel", AttrValue::String("noopener noreferrer".to_string()));
        }
        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_default_classes() {
        let props = LinkProps::default();
        let classes = Link::classes(&props);
        assert!(classes.contains("text-primary"));
        assert!(classes.contains("hover:underline"));
    }

    #[test]
    fn link_render_tag_is_a() {
        let props = LinkProps::default();
        let output = Link::render(&props);
        assert_eq!(output.effective_tag(), "a");
    }

    #[test]
    fn link_external_attrs() {
        let props = LinkProps {
            href: Some("https://example.com".to_string()),
            external: true,
            ..Default::default()
        };
        let output = Link::render(&props);
        assert!(output.attrs.iter().any(|(k, _)| k == "target"));
        assert!(output.attrs.iter().any(|(k, _)| k == "rel"));
    }

    #[test]
    fn link_disabled() {
        let props = LinkProps {
            disabled: true,
            ..Default::default()
        };
        let classes = Link::classes(&props);
        assert!(classes.contains("pointer-events-none"));
        assert!(classes.contains("opacity-50"));
    }
}
