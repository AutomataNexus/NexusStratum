//! Separator primitive — simple visual divider.
//!
//! Supports semantic (role=separator) and decorative (role=none) modes.

use stratum_core::*;

/// Props for the Separator primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Orientation of the separator.
    pub orientation: Orientation,
    /// If true, the separator is purely decorative (role=none).
    pub decorative: bool,
}

impl Default for SeparatorProps {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            decorative: false,
        }
    }
}

/// Internal state for the Separator primitive (stateless).
#[derive(Debug, Clone)]
pub struct SeparatorState;

/// Headless separator primitive.
pub struct Separator;

impl Component for Separator {
    type Props = SeparatorProps;
    type State = SeparatorState;

    fn initial_state(_props: &Self::Props) -> Self::State {
        SeparatorState
    }

    fn render(props: &Self::Props, _state: &Self::State) -> RenderOutput {
        let aria = if props.decorative {
            AriaAttributes::new().with_role(AriaRole::None)
        } else {
            AriaAttributes::new()
                .with_role(AriaRole::Separator)
                .with_orientation(props.orientation)
        };

        RenderOutput::new().with_tag("hr").with_aria(aria)
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

    fn default_props() -> SeparatorProps {
        SeparatorProps::default()
    }

    #[test]
    fn initial_state_is_unit() {
        let props = default_props();
        let _state = Separator::initial_state(&props);
    }

    #[test]
    fn render_semantic_separator() {
        let props = default_props();
        let state = Separator::initial_state(&props);
        let output = Separator::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::Separator));
        assert_eq!(output.aria.orientation, Some(Orientation::Horizontal));
        assert_eq!(output.effective_tag(), "hr");
    }

    #[test]
    fn render_vertical_separator() {
        let props = SeparatorProps {
            orientation: Orientation::Vertical,
            decorative: false,
        };
        let state = Separator::initial_state(&props);
        let output = Separator::render(&props, &state);
        assert_eq!(output.aria.orientation, Some(Orientation::Vertical));
    }

    #[test]
    fn render_decorative_separator() {
        let props = SeparatorProps {
            decorative: true,
            ..default_props()
        };
        let state = Separator::initial_state(&props);
        let output = Separator::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::None));
        assert_eq!(output.aria.orientation, None);
    }

    #[test]
    fn events_are_noop() {
        let props = default_props();
        let mut state = Separator::initial_state(&props);
        let result = Separator::on_event(&props, &mut state, ComponentEvent::Focus);
        assert!(!result.state_changed);
    }
}
