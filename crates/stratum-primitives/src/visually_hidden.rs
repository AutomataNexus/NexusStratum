//! VisuallyHidden primitive — screen reader only content.
//!
//! Renders content that is visually hidden but accessible to screen readers.
//! Uses CSS-based hiding technique; no ARIA attributes needed.

use stratum_core::*;

/// Props for the VisuallyHidden primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct VisuallyHiddenProps;

/// Internal state for the VisuallyHidden primitive (stateless).
#[derive(Debug, Clone)]
pub struct VisuallyHiddenState;

/// Headless visually-hidden primitive.
pub struct VisuallyHidden;

impl Component for VisuallyHidden {
    type Props = VisuallyHiddenProps;
    type State = VisuallyHiddenState;

    fn initial_state(_props: &Self::Props) -> Self::State {
        VisuallyHiddenState
    }

    fn render(_props: &Self::Props, _state: &Self::State) -> RenderOutput {
        RenderOutput::new()
            .with_tag("span")
            .with_class("sr-only")
            .with_style("position", "absolute")
            .with_style("width", "1px")
            .with_style("height", "1px")
            .with_style("padding", "0")
            .with_style("margin", "-1px")
            .with_style("overflow", "hidden")
            .with_style("clip", "rect(0, 0, 0, 0)")
            .with_style("white-space", "nowrap")
            .with_style("border-width", "0")
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

    #[test]
    fn initial_state_is_unit() {
        let _state = VisuallyHidden::initial_state(&VisuallyHiddenProps);
    }

    #[test]
    fn render_produces_sr_only_class() {
        let props = VisuallyHiddenProps;
        let state = VisuallyHidden::initial_state(&props);
        let output = VisuallyHidden::render(&props, &state);
        assert_eq!(output.effective_tag(), "span");
        assert!(output.classes.contains(&"sr-only".to_string()));
    }

    #[test]
    fn render_has_hiding_styles() {
        let props = VisuallyHiddenProps;
        let state = VisuallyHidden::initial_state(&props);
        let output = VisuallyHidden::render(&props, &state);
        let style_str = output.style_string();
        assert!(style_str.contains("position: absolute"));
        assert!(style_str.contains("width: 1px"));
        assert!(style_str.contains("overflow: hidden"));
        assert!(style_str.contains("clip: rect(0, 0, 0, 0)"));
    }

    #[test]
    fn no_aria_attributes() {
        let props = VisuallyHiddenProps;
        let state = VisuallyHidden::initial_state(&props);
        let output = VisuallyHidden::render(&props, &state);
        assert_eq!(output.aria.role, None);
        assert_eq!(output.aria.to_attr_pairs().len(), 0);
    }

    #[test]
    fn events_are_noop() {
        let props = VisuallyHiddenProps;
        let mut state = VisuallyHidden::initial_state(&props);
        let result = VisuallyHidden::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.state_changed);
    }
}
