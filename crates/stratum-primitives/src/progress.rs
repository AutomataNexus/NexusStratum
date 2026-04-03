//! Progress primitive — progress indicator.
//!
//! Renders with `role=progressbar` and ARIA value attributes.
//! When `value` is `None`, the progress bar is indeterminate.

use stratum_core::event::{ComponentEvent, EventResult};
use stratum_core::id::generators;
use stratum_core::render::{AttrValue, RenderOutput};
use stratum_core::{AriaAttributes, AriaRole, Component};

/// Props for the Progress primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct ProgressProps {
    /// Current progress value (0.0 to max). `None` means indeterminate.
    pub value: Option<f64>,
    /// Maximum value (default 100.0).
    pub max: f64,
    /// Optional explicit ID.
    pub id: Option<String>,
    /// Accessible label.
    pub aria_label: Option<String>,
}

impl Default for ProgressProps {
    fn default() -> Self {
        Self {
            value: None,
            max: 100.0,
            id: None,
            aria_label: None,
        }
    }
}

/// Internal state for the Progress primitive.
#[derive(Debug, Clone)]
pub struct ProgressState {
    /// Generated or provided element ID.
    pub id: String,
}

/// Headless progress indicator primitive.
pub struct Progress;

impl Component for Progress {
    type Props = ProgressProps;
    type State = ProgressState;

    fn initial_state(props: &Self::Props) -> Self::State {
        let id = props
            .id
            .clone()
            .unwrap_or_else(|| generators::GENERIC.next());
        ProgressState { id }
    }

    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput {
        let mut aria = AriaAttributes::new().with_role(AriaRole::ProgressBar);

        aria.valuemin = Some(0.0);
        aria.valuemax = Some(props.max);

        if let Some(value) = props.value {
            // Clamp value to [0, max]
            let clamped = value.clamp(0.0, props.max);
            aria.valuenow = Some(clamped);
        }

        if let Some(ref label) = props.aria_label {
            aria.label = Some(label.clone());
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_aria(aria)
            .with_attr("id", AttrValue::String(state.id.clone()));

        // Data attributes for styling
        if let Some(value) = props.value {
            let percentage = (value / props.max * 100.0).clamp(0.0, 100.0);
            output = output.with_data("value", format!("{:.0}", percentage));
            output = output.with_data("state", "determinate");
        } else {
            output = output.with_data("state", "indeterminate");
        }

        output = output.with_data("max", props.max.to_string());

        output
    }

    fn on_event(
        _props: &Self::Props,
        _state: &mut Self::State,
        _event: ComponentEvent,
    ) -> EventResult {
        // Progress bars don't handle events
        EventResult::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stratum_core::event::MouseButton;

    fn default_props() -> ProgressProps {
        ProgressProps::default()
    }

    #[test]
    fn initial_state_defaults() {
        let props = default_props();
        let state = Progress::initial_state(&props);
        assert!(state.id.starts_with("stratum-id-"));
    }

    #[test]
    fn initial_state_custom_id() {
        let props = ProgressProps {
            id: Some("my-progress".to_string()),
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        assert_eq!(state.id, "my-progress");
    }

    #[test]
    fn render_progressbar_role() {
        let props = default_props();
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.role, Some(AriaRole::ProgressBar));
    }

    #[test]
    fn render_value_range() {
        let props = ProgressProps {
            value: Some(50.0),
            max: 100.0,
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.valuemin, Some(0.0));
        assert_eq!(output.aria.valuemax, Some(100.0));
        assert_eq!(output.aria.valuenow, Some(50.0));
    }

    #[test]
    fn render_indeterminate() {
        let props = ProgressProps {
            value: None,
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.valuenow, None);
        assert!(
            output
                .data_attrs
                .contains(&("state".to_string(), "indeterminate".to_string()))
        );
    }

    #[test]
    fn render_determinate_data() {
        let props = ProgressProps {
            value: Some(75.0),
            max: 100.0,
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert!(
            output
                .data_attrs
                .contains(&("state".to_string(), "determinate".to_string()))
        );
        assert!(
            output
                .data_attrs
                .contains(&("value".to_string(), "75".to_string()))
        );
    }

    #[test]
    fn render_custom_max() {
        let props = ProgressProps {
            value: Some(5.0),
            max: 10.0,
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.valuemax, Some(10.0));
        assert_eq!(output.aria.valuenow, Some(5.0));
        assert!(
            output
                .data_attrs
                .contains(&("value".to_string(), "50".to_string()))
        );
    }

    #[test]
    fn render_clamps_value() {
        let props = ProgressProps {
            value: Some(150.0),
            max: 100.0,
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.valuenow, Some(100.0));
    }

    #[test]
    fn render_clamps_negative_value() {
        let props = ProgressProps {
            value: Some(-10.0),
            max: 100.0,
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.valuenow, Some(0.0));
    }

    #[test]
    fn render_aria_label() {
        let props = ProgressProps {
            aria_label: Some("Loading".to_string()),
            ..default_props()
        };
        let state = Progress::initial_state(&props);
        let output = Progress::render(&props, &state);
        assert_eq!(output.aria.label, Some("Loading".to_string()));
    }

    #[test]
    fn events_are_noop() {
        let props = default_props();
        let mut state = Progress::initial_state(&props);
        let event = ComponentEvent::Click {
            x: 0.0,
            y: 0.0,
            button: MouseButton::Left,
        };
        let result = Progress::on_event(&props, &mut state, event);
        assert!(!result.state_changed);
        assert!(!result.prevent_default);
    }
}
