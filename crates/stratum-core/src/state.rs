/// Trait for component internal state types.
///
/// State is managed by the component itself (uncontrolled mode) or
/// by the consumer (controlled mode). Framework adapters bridge this
/// to each framework's reactivity model (Leptos signals, Dioxus signals).
pub trait State: Clone + 'static {}

/// Blanket impl: any type satisfying the bounds is valid as State.
impl<T: Clone + 'static> State for T {}
