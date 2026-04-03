//! Portal primitive — render outside DOM hierarchy.
//!
//! Marker component for framework adapters to handle portal/teleport logic.
//! The render output contains target info as data attributes.

use stratum_core::*;

/// Props for the Portal primitive.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PortalProps {
    /// Target container ID. If None, renders at document body.
    pub target: Option<String>,
}

/// Internal state for the Portal primitive (stateless marker).
#[derive(Debug, Clone)]
pub struct PortalState;

/// Headless portal marker primitive.
pub struct Portal;

impl Component for Portal {
    type Props = PortalProps;
    type State = PortalState;

    fn initial_state(_props: &Self::Props) -> Self::State {
        PortalState
    }

    fn render(props: &Self::Props, _state: &Self::State) -> RenderOutput {
        let mut output = RenderOutput::new().with_data("portal", "true");

        if let Some(ref target) = props.target {
            output = output.with_data("portal-target", target.clone());
        }

        output
    }

    fn on_event(
        _props: &Self::Props,
        _state: &mut Self::State,
        _event: ComponentEvent,
    ) -> EventResult {
        EventResult::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_props() -> PortalProps {
        PortalProps::default()
    }

    #[test]
    fn initial_state_is_unit() {
        let _state = Portal::initial_state(&default_props());
    }

    #[test]
    fn render_default_portal() {
        let props = default_props();
        let state = Portal::initial_state(&props);
        let output = Portal::render(&props, &state);
        assert!(
            output
                .data_attrs
                .contains(&("portal".to_string(), "true".to_string()))
        );
        assert!(!output.data_attrs.iter().any(|(k, _)| k == "portal-target"));
    }

    #[test]
    fn render_with_target() {
        let props = PortalProps {
            target: Some("modal-root".to_string()),
        };
        let state = Portal::initial_state(&props);
        let output = Portal::render(&props, &state);
        assert!(
            output
                .data_attrs
                .contains(&("portal-target".to_string(), "modal-root".to_string()))
        );
    }

    #[test]
    fn events_are_noop() {
        let props = default_props();
        let mut state = Portal::initial_state(&props);
        let result = Portal::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.state_changed);
    }
}
