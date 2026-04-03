//! Portal utility — a pass-through marker that framework adapters use to
//! render children into a different part of the DOM tree.

use stratum_core::render::RenderOutput;

/// Properties for Portal.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PortalProps {
    /// CSS selector of the container to portal into. Defaults to "body".
    pub container: Option<String>,
    pub class: Option<String>,
}

pub struct Portal;

impl Portal {
    pub fn render(props: &PortalProps) -> RenderOutput {
        let container = props.container.as_deref().unwrap_or("body");

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_data("portal", "true")
            .with_data("portal-container", container);

        if let Some(ref class) = props.class {
            output = output.with_class(class.clone());
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_default_container() {
        let props = PortalProps::default();
        let output = Portal::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "portal-container" && v == "body")
        );
    }

    #[test]
    fn portal_custom_container() {
        let props = PortalProps {
            container: Some("#modal-root".to_string()),
            ..Default::default()
        };
        let output = Portal::render(&props);
        assert!(
            output
                .data_attrs
                .iter()
                .any(|(k, v)| k == "portal-container" && v == "#modal-root")
        );
    }

    #[test]
    fn portal_tag() {
        let props = PortalProps::default();
        let output = Portal::render(&props);
        assert_eq!(output.effective_tag(), "div");
    }
}
