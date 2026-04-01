use crate::event::{ComponentEvent, EventResult};
use crate::props::Props;
use crate::render::RenderOutput;
use crate::state::State;

/// The core trait all NexusStratum UI components implement.
///
/// `Component` defines the framework-agnostic lifecycle of a UI component:
/// - Initialize state from props
/// - Render a framework-agnostic description of the component
/// - Handle events and update state
///
/// Framework adapters (stratum-leptos, stratum-dioxus) bridge this trait
/// to each framework's specific rendering and reactivity model.
pub trait Component: Sized + 'static {
    /// The property type for this component.
    type Props: Props;

    /// The internal state type for this component.
    type State: State;

    /// Create the initial state from the given props.
    fn initial_state(props: &Self::Props) -> Self::State;

    /// Produce a framework-agnostic render description.
    ///
    /// The returned [`RenderOutput`] describes the component's attributes,
    /// CSS classes, ARIA attributes, and children. Framework adapters
    /// translate this into actual DOM/VDOM elements.
    fn render(props: &Self::Props, state: &Self::State) -> RenderOutput;

    /// Handle an event and optionally mutate state.
    ///
    /// Returns an [`EventResult`] indicating whether the event should
    /// be prevented, stopped, or if state changed (triggering re-render).
    fn on_event(
        props: &Self::Props,
        state: &mut Self::State,
        event: ComponentEvent,
    ) -> EventResult;

    /// Called when props change. Returns true if state needs updating.
    ///
    /// Default implementation always returns false (state is independent of props).
    /// Override for components where prop changes should sync to state.
    fn props_changed(
        _old_props: &Self::Props,
        _new_props: &Self::Props,
        _state: &mut Self::State,
    ) -> bool {
        false
    }

    /// Called before the component is unmounted.
    ///
    /// Default implementation is a no-op. Override for cleanup logic.
    fn cleanup(_props: &Self::Props, _state: &mut Self::State) {}
}
