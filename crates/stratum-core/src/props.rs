/// Trait for component property types.
///
/// All component props must be `Clone + PartialEq + 'static`.
/// The `merge_defaults` method allows props to fill in default values
/// after construction, enabling both builder-pattern and struct-literal usage.
pub trait Props: Clone + PartialEq + 'static {
    /// Merge default values into this props instance.
    ///
    /// Called after props are constructed but before they are passed to
    /// `Component::initial_state` or `Component::render`. This allows
    /// defaults to be applied without requiring `Default` on the entire type.
    fn merge_defaults(self) -> Self {
        self
    }
}

/// Blanket impl: any type satisfying the bounds is valid as Props.
/// Components can also implement Props explicitly for custom merge_defaults.
impl<T: Clone + PartialEq + 'static> Props for T {}
