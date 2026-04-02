/// Marker trait for component property types.
///
/// All component props must be `Clone + PartialEq + 'static`.
/// This trait is automatically implemented for any type meeting these bounds.
pub trait Props: Clone + PartialEq + 'static {}

/// Blanket impl: any type satisfying the bounds is valid as Props.
impl<T: Clone + PartialEq + 'static> Props for T {}
